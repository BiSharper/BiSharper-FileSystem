use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::filesystem::{GFS, GfsSnapshot, GfsEntryMeta};
use crate::GfsResult;
use crate::io::{GfsFile, ReadableFile, WritableFile};
use crate::path::GfsPath;

pub type ReadableGameFile = ReadableFile<GameFileMeta>;
pub type WritableGameFile = WritableFile<GameFileMeta>;
pub type GamePath = GfsPath<GameFileMeta>;
pub type GameFile = GfsFile<GameFileMeta>;


#[derive(Copy, Clone, Default)]
pub struct GameFileMeta {
    last_update: u64
}


struct GameFileServer {
    entries: RwLock<HashMap<String, GameFile>>,
    root: GamePath
}

impl GFS<GameFileMeta> for GameFileServer {
    fn rename_entry(&self, path: &GamePath, new_path: &GamePath) {
        todo!()
    }

    fn insert_entry(&self, path: &GamePath, metadata: GameFileMeta, data: Arc<Vec<u8>>) -> GfsResult<&GameFile> {
        todo!()
    }

    fn drop_entry(&self, path: &GfsPath<GameFileMeta>) -> GfsResult<GameFile> {
        todo!()
    }
}

impl GfsSnapshot<GameFileMeta> for GameFileServer {
    fn normalize_path(&self, path: &str) -> &str {
        todo!()
    }

    fn read_meta(&self, path: &GamePath) -> Option<GameFileMeta> {
        todo!()
    }

    fn read_data(&self, path: &GamePath) -> Option<Arc<Vec<u8>>> {
        todo!()
    }

    fn root(&self) -> &GamePath {
        todo!()
    }

    fn read_dir(&self, path: &str) -> Box<[GamePath]> {
        todo!()
    }
}

impl GfsEntryMeta for GameFileMeta {

}
