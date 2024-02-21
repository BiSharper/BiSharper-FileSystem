use std::io::Cursor;
use std::sync::Arc;
use crate::error::GfsResult;
use crate::implementation::GamePath;
use crate::path::GfsPath;

pub const GFS_SEPARATOR: char = '/';

pub(super) trait GfsEntryMeta : Copy + Clone + Default {

}

pub struct GfsEntry<T: GfsEntryMeta> {
    pub metadata: T,
    pub(crate) content:  Arc<Vec<u8>>,
}

impl<T: GfsEntryMeta> GfsEntry<T> {
    pub fn read_meta(&self) -> T { self.metadata }
    pub fn modify_meta(&mut self) -> &mut T { &mut self.metadata }
}

impl<T: GfsEntryMeta> ReadableFile<T> {
    pub fn read_meta(&self) -> T { self.metadata }
    pub fn create(metadata: T, content: Vec<u8>) -> Self {
        Self {
            metadata,
            content: Arc::new(content),
            position: 0,
        }
    }
}

impl<M: GfsEntryMeta> WritableFile<M> {
    pub fn read_meta(&self) -> M { self.metadata }
    pub fn modify_meta(&mut self) -> &mut M { &mut self.metadata }
}

pub struct ReadableFile<T: GfsEntryMeta> {
    pub metadata: T,
    pub(crate) content:  Arc<Vec<u8>>,
    pub(crate) position: u64,
}

pub struct WritableFile<M: GfsEntryMeta> {
    pub metadata:        M,
    pub(crate) content:         Cursor<Vec<u8>>,
    pub(crate) destination:     String,
    pub(crate) filesystem:      Arc<Box<dyn GFS<M>>>
}

pub trait GFS<T: GfsEntryMeta> {
    fn create_path(&self, path: &str) -> GfsPath<T> { self.root().join(path) }
    
    fn remove_entry(&self, path: &GfsPath<T>) -> GfsResult<()> { self.drop_entry(path)?; Ok(()) }

    fn read_root(&self) -> Box<[GamePath]> { self.read_dir(self.root().as_str()) }

    fn replace_meta(&self, path: &GfsPath<T>, metadata: T) -> GfsResult<()> {
        Ok(self.modify_entry(path)?.metadata = metadata)
    }

    fn read_entry(&self, path: &GfsPath<T>) -> GfsResult<(Vec<u8>, T)> {
        Ok((self.read_data(path)?, self.read_meta(path)?))
    }

    fn open_entry_reader(&self, path: &GfsPath<T>) -> GfsResult<ReadableFile<T>> {
        let (data, meta) = self.read_entry(path)?;
        Ok(ReadableFile {
            metadata: meta.clone(),
            content: Arc::new(data),
            position: 0
        })
    }

    fn drop_entry(&self, path: &GfsPath<T>) -> GfsResult<GfsEntry<T>>;

    fn root(&self) -> &GfsPath<T>;

    fn read_dir(&self, path: &str) -> Box<[GamePath]>;

    fn create_entry(&self, path: &GfsPath<T>) -> GfsResult<WritableFile<T>>;

    fn read_meta(&self, path: &GfsPath<T>) -> GfsResult<T>;

    fn read_data(&self, path: &GfsPath<T>) -> GfsResult<Vec<u8>>;

    fn modify_entry(&self, path: &GfsPath<T>) -> GfsResult<WritableFile<T>>;
}