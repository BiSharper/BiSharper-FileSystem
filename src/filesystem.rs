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
        OwnedGfsPath::create(self.root().join(&Self::normalize_path(path)), &self)
    }

    fn normalize_path(path: &GfsPath) -> GfsPath;

    fn read_root(&self, recursive: bool) -> GfsResult<Box<[OwnedGfsPath<T, Self>]>> {
        self.read_dir(&self.root(), recursive)
    }

    fn read_entry(&self, path: &GfsPath) -> GfsResult<GfsFile<T>> {
        Ok(GfsFile::create(self.read_meta(path)?, self.read_data(path)?))
    }

    fn entry_reader(&self, path: &GfsPath) -> GfsResult<ReadableFile<T>> {
        Ok(ReadableFile::from(self.read_entry(path)?))
    }

    fn root(&self) -> &GfsPath;

    fn read_meta(&self, path: &GfsPath) -> GfsResult<T>;

    fn read_data(&self, path: &GfsPath) -> GfsResult<Arc<[u8]>>;

    fn read_dir(&self, path: &GfsPath, recursive: bool) -> GfsResult<Box<[OwnedGfsPath<T, Self>]>>;
}


pub trait GFS<T: GfsEntryMeta> : GfsSnapshot<T> {

    fn new(root: &GfsPath) -> Self;

    fn rename_entry(&self, path: &GfsPath, new_path: &GfsPath) -> GfsResult<OwnedGfsPath<T, Self>> {
        self.replace_entry(new_path,  self.drop_entry(path)?)
    }

    fn drop_entry(&self, path: &GfsPath) -> GfsResult<GfsFile<T>>;

    fn remove_entry(&self, path: &GfsPath) -> GfsResult<()> { self.drop_entry(path)?; Ok(()) }

    fn entry_writer(&self, path: &GfsPath) -> GfsResult<WritableFile<T, Self>> {
        let owned_path = self.create_path(path);
        return Ok(if let Ok(entry) = self.read_entry(path) {
            WritableFile::from_owned(&owned_path, entry.metadata.clone(), entry.contents.to_vec())
        } else { WritableFile::from_owned(&owned_path, Default::default(), vec![]) })
    }

    fn insert_entry(&self, path: &GfsPath, metadata: T, data: Box<[u8]>) -> GfsResult<OwnedGfsPath<T, Self>>;

    fn replace_entry(&self, path: &GfsPath, entry: GfsFile<T>) -> GfsResult<OwnedGfsPath<T, Self>> {
        self.insert_entry(path, entry.metadata, entry.contents.to_vec().into_boxed_slice())
    }
}
