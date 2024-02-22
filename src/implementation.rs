use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::filesystem::{GFS, GfsSnapshot, GfsEntryMeta};
use crate::GfsResult;
use crate::io::{GfsFile, ReadableFile};
use crate::path::{GfsPath, OwnedGfsPath};

pub type ReadableGameFile = ReadableFile<GameFileMeta>;

pub type GameFile = GfsFile<GameFileMeta>;


#[derive(Copy, Clone, Default)]
pub struct GameFileMeta {
    last_update: u64
}


pub struct GameFileServer {
    entries: Arc<RwLock<HashMap<GfsPath, GameFile>>>,
    root: GfsPath
}

impl GfsSnapshot<GameFileMeta> for GameFileServer {

    fn root(&self) -> &GfsPath { &self.root }

    fn read_meta(&self, path: &GfsPath) -> Option<GameFileMeta> {
        todo!()
    }

    fn read_data(&self, path: &GfsPath) -> Option<Arc<Vec<u8>>> {
        todo!()
    }

    fn read_dir(&self, path: &GfsPath) -> Box<[OwnedGfsPath<GameFileMeta, Self>]> {
        todo!()
    }
}

impl GFS<GameFileMeta> for GameFileServer {
    fn new(root: &GfsPath) -> Arc<Self> {
        todo!()
    }


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
