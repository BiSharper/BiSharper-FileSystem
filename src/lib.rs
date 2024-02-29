use std::collections::HashMap;
use rfsa::{GFS_SEPARATOR, PathLike, ReadableVFile, ReadableVMetadata, VDirectory, VFile, VFileContainer, VFileSystem, VPath, VPathIterator, WritableVFile, WritableVMetadata};
use rfsa::error::{VFSError, VFSResult};
use rfsa::macros::VMeta;

#[derive(VMeta, Copy, Clone, Default, Eq, PartialEq)]
pub struct GameMeta {

}

pub type GameFile = VFile<GameMeta>;
pub type ReadableGameFile<'a> = ReadableVFile<'a, GameMeta>;
pub type WritableGameFile<'a> = WritableVFile<'a, GameMeta>;
pub type ReadableGameMetadata<'a> = ReadableVMetadata<'a, GameMeta>;
pub type WritableGameMetadata<'a> = WritableVMetadata<'a, GameMeta>;
pub type GameDirectory<'a> = VDirectory<'a, GameMeta, GameFilesystem>;

pub struct GameFilesystem {
    entries: HashMap<VPath, GameFile>
}

impl GameFilesystem {
    pub fn create() -> Self { Self { entries: HashMap::new(), } }
}

impl VFileContainer<GameMeta, Self> for GameFilesystem {
    fn file_remove(&mut self, path: &VPath) -> VFSResult<GameFile> {
        self.entries.remove(path).ok_or(VFSError::EntryNotFound)
    }

    fn file_exists(&self, path: &VPath) -> VFSResult<bool> {
        Ok(self.entries.contains_key(path))
    }

    fn file_insert(&mut self, path: &VPath, file: GameFile) -> VFSResult<Option<GameFile>> {
        Ok(self.entries.insert(path.clone(), file))
    }

    fn file_mut(&mut self, path: &VPath) -> VFSResult<&mut GameFile> {
        self.entries.get_mut(path).ok_or(VFSError::EntryNotFound)
    }

    fn file_get(&self, path: &VPath) -> VFSResult<&GameFile> {
        self.entries.get(path).ok_or(VFSError::EntryNotFound)
    }

    fn dir_exists(&self, path: &VPath) -> VFSResult<bool> {
        Ok(self.entries.keys().find(|p| {
            p.starts_with(path.as_directory_string().as_str())
        }) != None)
    }
}

impl VFileSystem<GameMeta> for GameFilesystem {
    fn paths(&self) -> VFSResult<VPathIterator> {
        Ok(Box::new(self.entries.keys().cloned().collect::<Vec<VPath>>().into_iter()))
    }

    fn path_iter(&self, path_prefix: String, recursive: bool) -> VFSResult<VPathIterator> {
        let prefix_len = path_prefix.len();
        Ok(Box::new(self.entries.keys().filter( |candidate| {
            candidate.starts_with(path_prefix.as_str()) && (!recursive || !candidate[prefix_len..].contains(GFS_SEPARATOR))
        }).cloned().collect::<Vec<VPath>>().into_iter()))
    }
}