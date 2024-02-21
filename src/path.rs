use std::sync::Arc;
use crate::filesystem::{GFS, GfsEntryMeta};
use crate::{GfsResult};
use crate::io::{GfsFile, ReadableFile, WritableFile};

pub const GFS_SEPARATOR: char = '/';

#[derive(Clone)]
pub struct GfsPath<M: GfsEntryMeta> {
    path: Arc<str>,
    fs:  Arc<dyn GFS<M>>,
}

impl<M: GfsEntryMeta> GfsPath<M> {
    pub fn filesystem(&self) -> Arc<dyn GFS<M>> { self.fs.clone() }

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

    pub fn fs_new(&self, metadata: M, contents: Arc<Vec<u8>>) -> GfsResult<&GfsFile<M>> {
        self.fs.insert_entry(self, metadata, contents)
    }

    pub fn fs_meta(&self) -> Option<M> { self.fs.read_meta(&self) }

    pub fn fs_data(&self) -> Option<Arc<Vec<u8>>> { self.fs.read_data(&self) }

    pub fn fs_reader(&self) -> Option<ReadableFile<M>> { self.fs.entry_reader(&self) }

    pub fn fs_writer(&self) -> WritableFile<M> { self.fs.entry_writer(&self) }

    pub fn fs_drop_entry(&self) -> GfsResult<GfsFile<M>> { self.fs.drop_entry(&self) }

    pub fn fs_remove_entry(&self) -> GfsResult<()> { self.fs.remove_entry(&self) }

    pub fn rename_entry(&self, new_path: &GfsPath<M>) { self.fs.rename_entry(&self, new_path) }

    pub fn join(&self, path: &str) -> Self {
        Self {
            path: Arc::from(path),
            fs: self.fs.clone(),
        }
    }
}
