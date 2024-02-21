use std::sync::Arc;
use crate::GfsResult;
use crate::io::{GfsFile, WritableFile, ReadableFile};
use crate::path::GfsPath;

pub const GFS_SEPARATOR: char = '/';

pub trait GfsEntryMeta : Copy + Clone + Default {

}

pub trait GfsSnapshot<T: GfsEntryMeta> {
    fn create_path(&self, path: &str) -> GfsPath<T> { self.root().join(self.normalize_path(path)) }

    fn normalize_path(&self, path: &str) -> &str;

    fn read_root(&self) -> Box<[GfsPath<T>]> { self.read_dir(self.root().as_str()) }

    fn read_entry(&self, path: &GfsPath<T>) -> Option<GfsFile<T>> {
        Some(GfsFile::create(self.read_meta(path)?, self.read_data(path)?))
    }

    fn entry_reader(&self, path: &GfsPath<T>) -> Option<ReadableFile<T>> {
        Some(ReadableFile::from(self.read_entry(path)?))
    }

    fn read_meta(&self, path: &GfsPath<T>) -> Option<T>;

    fn read_data(&self, path: &GfsPath<T>) -> Option<Arc<Vec<u8>>>;

    fn root(&self) -> &GfsPath<T>;

    fn read_dir(&self, path: &str) -> Box<[GfsPath<T>]>;
}

pub trait GFS<T: GfsEntryMeta> : GfsSnapshot<T> {

    fn rename_entry(&self, path: &GfsPath<T>, new_path: &GfsPath<T>);

    fn remove_entry(&self, path: &GfsPath<T>) -> GfsResult<()> { self.drop_entry(path)?; Ok(()) }

    fn entry_writer(&self, path: &GfsPath<T>) -> WritableFile<T> {
        if let Some(entry) = self.read_entry(path) {
            return WritableFile::create(path, entry.metadata, entry.contents.to_vec())
        }
        WritableFile::create(path, Default::default(), vec![])
    }

    fn insert_entry(&self, path: &GfsPath<T>, metadata: T, data: Arc<Vec<u8>>) -> GfsResult<&GfsFile<T>>;

    fn drop_entry(&self, path: &GfsPath<T>) -> GfsResult<GfsFile<T>>;
}