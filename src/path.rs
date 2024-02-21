use std::io::Read;
use std::sync::Arc;
use crate::error::GfsResult;
use crate::filesystem::{GFS, GfsEntryMeta, ReadableFile};

pub const GFS_SEPARATOR: char = '/';

#[derive(Clone)]
pub struct GfsPath<M: GfsEntryMeta> {
    path: Arc<str>,
    pub fs:  Arc<Box<dyn GFS<M>>>,
}

impl<M: GfsEntryMeta> GfsPath<M> {
    pub fn as_str(&self) -> &str {
        &self.path
    }

    pub fn is_child(&self, path: &str) -> bool { path.starts_with(self.as_str()) }

    pub fn to_string(&self) -> String {
        self.path.to_string()
    }

    pub fn to_directory_string(&self) -> String {
        if self.path.ends_with(GFS_SEPARATOR) {
            return self.to_string();
        }
        return format!("{}{}", self.as_str(), GFS_SEPARATOR)
    }

    pub fn read_meta(&self) -> GfsResult<M> { self.fs.read_meta(&self) }

    pub fn open_entry_reader(&self) -> GfsResult<ReadableFile<M>> { self.fs.open_entry_reader(&self) }

    pub fn join(&self, path: &str) -> Self {
        Self {
            path: Arc::from(path),
            fs: self.fs.clone(),
        }
    }
}
