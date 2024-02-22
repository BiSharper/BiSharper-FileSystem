use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::filesystem::{GFS, GfsSnapshot};
use crate::{GFS_SEPARATOR, GfsError, GfsResult, NoEntryMeta};
use crate::io::{GfsFile};
use crate::path::{GfsPath, OwnedGfsPath, PathLike};

pub type GameFileMeta = NoEntryMeta;
pub type GameFile = GfsFile<GameFileMeta>;
pub type GamePath<'a> = OwnedGfsPath<'a, GameFileMeta, GameFileSystem>;

pub struct GameFileSystem {
    entries: Arc<RwLock<HashMap<GfsPath, GameFile>>>,
    root: GfsPath
}

impl GfsSnapshot<GameFileMeta> for GameFileSystem {
    fn normalize_path(_path: &GfsPath) -> GfsPath {
        todo!()
    }

    fn read_entry(&self, path: &GfsPath) -> GfsResult<GameFile> {
        self.entries.read().unwrap().get(path).cloned().ok_or(GfsError::EntryNotFound)
    }

    fn root(&self) -> &GfsPath { &self.root }


    fn read_meta(&self, path: &GfsPath) -> GfsResult<GameFileMeta> {
        let handle = self.entries.read().unwrap();

        return handle.get(path).map(|file| file.metadata).ok_or(GfsError::EntryNotFound)
    }

    fn read_data(&self, path: &GfsPath) -> GfsResult<Arc<[u8]>> {
        let handle = self.entries.read().unwrap();

        return handle.get(path).map(|file| file.contents.clone()).ok_or(GfsError::EntryNotFound)
    }

    fn read_dir(&self, path: &GfsPath, recursive: bool) -> GfsResult<Box<[GamePath]>> {
        let handle = self.entries.read().unwrap();
        let path = self.create_path(path).to_directory_path();
        let prefix_len = path.as_str().len();
        Ok(Box::from_iter(
            handle
                .iter()
                .filter_map(| (candidate, _) | {
                    if path.as_path().is_child(candidate) && (!recursive || path.as_str()[prefix_len..].find(GFS_SEPARATOR).is_none()
                        ) {
                        Some(self.create_path(candidate))
                    } else { None }
                })
        ))
    }
}

impl GFS<GameFileMeta> for GameFileSystem {
    fn new(root: &GfsPath) -> Self {
        Self {
            entries: Arc::new(RwLock::new(Default::default())),
            root: root.clone(),
        }
    }

    fn drop_entry(&self, path: &GfsPath) -> GfsResult<GameFile> {
        self.entries.write().unwrap().remove(path).ok_or(GfsError::EntryNotFound)
    }

    fn insert_entry(&self, path: &GfsPath, metadata: GameFileMeta, data: Box<[u8]>) -> GfsResult<GamePath> {
        let mut handle = self.entries.write().unwrap();
        handle.insert(path.clone(), GameFile::create(metadata, Arc::from(data)));
        Ok(self.create_path(path))
    }

    fn replace_entry(&self, path: &GfsPath, entry: GfsFile<GameFileMeta>) -> GfsResult<GamePath >{
        let path = self.create_path(path);
        let mut handle = self.entries.write().unwrap();
        handle.insert(path.as_path().clone(), entry);
        Ok(path)
    }
}
