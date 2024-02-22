use std::sync::Arc;
use crate::{GfsResult};
use crate::io::{GfsFile, WritableFile, ReadableFile};
use crate::path::{GfsPath, OwnedGfsPath, PathLike};

pub const GFS_SEPARATOR: char = '/';
pub const GFS_ROOT: &'static str = "";

pub trait GfsEntryMeta : Copy + Clone + Default {

}

#[derive(Copy, Clone, Default)]
pub struct NoEntryMeta;

impl GfsEntryMeta for NoEntryMeta {

}

pub trait GfsSnapshot<T: GfsEntryMeta> : Sized {
    fn create_path(&self, path: &GfsPath) -> OwnedGfsPath<T, Self> {
        OwnedGfsPath::create(self.root().join(path), &self)
    }

    fn read_root(&self, recursive: bool) -> Box<[OwnedGfsPath<T, Self>]> { self.read_dir(&self.root(), recursive) }

    fn root(&self) -> &GfsPath;

    fn read_meta(&self, path: &GfsPath) -> Option<T>;

    fn read_data(&self, path: &GfsPath) -> Option<Arc<Vec<u8>>>;

    fn read_entry(&self, path: &GfsPath) -> Option<GfsFile<T>> {
        Some(GfsFile::create(self.read_meta(path)?, self.read_data(path)?))
    }

    fn read_dir(&self, path: &GfsPath, recursive: bool) -> Box<[OwnedGfsPath<T, Self>]>;

    fn entry_reader(&self, path: &GfsPath) -> Option<ReadableFile<T>> {
        Some(ReadableFile::from(self.read_entry(path)?))
    }
}


pub trait GFS<T: GfsEntryMeta> : GfsSnapshot<T> {

    fn new(root: &GfsPath) -> Self;

    fn rename_entry(&self, path: &GfsPath, new_path: &GfsPath) -> GfsResult<OwnedGfsPath<T, Self>> {
        let data = self.drop_entry(path)?;
        self.insert_entry(new_path, data.metadata, data.contents)
    }

    fn drop_entry(&self, path: &GfsPath) -> GfsResult<GfsFile<T>>;

    fn remove_entry(&self, path: &GfsPath) -> GfsResult<()> { self.drop_entry(path)?; Ok(()) }

    fn entry_writer(&self, path: &GfsPath) -> WritableFile<T, Self> {
        let owned_path = self.create_path(path);
        if let Some(entry) = self.read_entry(path) {
            return WritableFile::from_owned(&owned_path, entry.metadata.clone(), entry.contents.to_vec())
        }
        WritableFile::from_owned(&owned_path, Default::default(), vec![])
    }

    fn insert_entry(&self, path: &GfsPath, metadata: T, data: Arc<Vec<u8>>) -> GfsResult<OwnedGfsPath<T, Self>>;

}