use std::collections::HashMap;
use std::io::Cursor;
use std::sync::{RwLock};
use crate::error;
use crate::error::{GfsError, GfsResult};
use crate::filesystem::{GFS, GfsEntry, GfsEntryMeta, ReadableFile, WritableFile};
use crate::path::GfsPath;

pub type ReadableGameFile = ReadableFile<GameFileMeta>;
pub type WritableGameFile = WritableFile<GameFileMeta>;
pub type GameFile = GfsEntry<GameFileMeta>;
pub type GamePath = GfsPath<GameFileMeta>;

#[derive(Copy, Clone, Default)]
pub struct GameFileMeta {
    last_update: u64
}

impl GfsEntryMeta for GameFileMeta {

}

struct GameFileServer {
    entries: RwLock<HashMap<String, GameFile>>,
    root: GamePath
}

impl GFS<GameFileMeta> for GameFileServer {
    fn drop_entry(&self, path: &GamePath) -> GfsResult<GameFile> {
        self.entries.write().unwrap().remove(path.as_str()).ok_or(error::GfsError::EntryNotFound)
    }

    fn root(&self) -> &GamePath { &self.root }

    fn read_dir(&self, path: &str) -> Box<[GamePath]> {
        let handle = self.entries.read().unwrap();
        let path = self.create_path(path).to_directory_string();
        Box::from_iter(handle
            .iter()
            .filter(|(candidate, _)| candidate.starts_with(path.as_str()))
            .filter_map(|(candidate, _)| {
                match candidate.starts_with(path.as_str()) {
                    true => Some(self.create_path(candidate.as_str())),
                    false => None
                }
            }))
    }

    fn create_entry(&self, path: &GamePath) -> GfsResult<WritableGameFile> {
        let file = self.entries.write().unwrap().insert(
            path.to_string(),
            GameFile {
                metadata: Default::default(),
                content: Default::default()
            },
        ).ok_or(GfsError::Other())?;
        Ok(WritableFile {
            metadata: file.metadata.clone(),
            content: Cursor::new(vec![]),
            destination: path.to_string(),
            filesystem: path.fs.clone(),
        })
    }

    fn read_meta(&self, path: &GamePath) -> GfsResult<GameFileMeta> {
        let handle = self.entries.read().unwrap();
        Ok(handle.get(path.as_str()).ok_or(GfsError::EntryNotFound)?.metadata.clone())
    }

    fn read_data(&self, path: &GamePath) -> GfsResult<Vec<u8>> {
        let handle = self.entries.read().unwrap();
        Ok((*(handle.get(path.as_str()).ok_or(GfsError::EntryNotFound)?.content)).clone())
    }

    fn modify_entry(&self, path: &GamePath) -> GfsResult<WritableGameFile> {
        let handle = self.entries.write().unwrap();
        let file = handle.get(path.as_str()).ok_or(GfsError::EntryNotFound)?;
        Ok(WritableGameFile {
            metadata: file.metadata.clone(),
            content: Cursor::new((*(handle.get(path.as_str()).ok_or(GfsError::EntryNotFound)?.content)).clone()),
            destination: path.to_string(),
            filesystem: path.fs.clone(),
        })
    }
}



