use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::filesystem::{GFS, GfsSnapshot, GfsEntryMeta};
use crate::GfsResult;
use crate::io::{GfsFile, ReadableFile, WritableFile};
use crate::path::{GfsPath, OwnedGfsPath};

pub type ReadableGameFile = ReadableFile<GameFileMeta>;
pub type WritableGameFile = WritableFile<GameFileMeta, GameFileServer>;
pub type GamePath = OwnedGfsPath<GameFileMeta, GameFileServer>;
pub type GameFile = GfsFile<GameFileMeta>;


#[derive(Copy, Clone, Default)]
pub struct GameFileMeta {
    last_update: u64
}


struct GameFileServer {
    entries: RwLock<HashMap<String, GameFile>>,
    root: GamePath
}

impl GfsSnapshot<GameFileMeta> for GameFileServer {
    fn root(&self) -> &OwnedGfsPath<GameFileMeta, Self> {
        todo!()
    }

    fn normalize_path(&self, path: String) -> String {
        todo!()
    }

    fn read_meta(&self, path: &GfsPath) -> Option<GameFileMeta> {
        todo!()
    }

    fn read_data(&self, path: &GfsPath) -> Option<Arc<Vec<u8>>> {
        todo!()
    }

    fn read_dir(&self, path: &str) -> Box<[OwnedGfsPath<GameFileMeta, Self>]> {
        todo!()
    }
}

impl GFS<GameFileMeta> for GameFileServer {
    fn rename_entry(&self, path: &GfsPath, new_path: &GfsPath) -> GfsResult<()> {
        todo!()
    }

    fn drop_entry(&self, path: &GfsPath) -> GfsResult<GfsFile<GameFileMeta>> {
        todo!()
    }

    fn insert_entry(&self, path: &GfsPath, metadata: GameFileMeta, data: Arc<Vec<u8>>) -> GfsResult<&GfsFile<GameFileMeta>> {
        todo!()
    }
}

impl GfsEntryMeta for GameFileMeta {

}
