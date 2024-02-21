use std::marker;
use std::marker::PhantomData;
use std::sync::Arc;
use crate::filesystem::{GFS, GfsEntryMeta};
use crate::{GfsResult, GfsSnapshot};
use crate::io::{GfsFile, ReadableFile, WritableFile};

pub const GFS_SEPARATOR: char = '/';
pub const ALT_SEPARATOR: char = '\\';
const SEPARATORS: [char; 2] = [ALT_SEPARATOR, GFS_SEPARATOR];

pub struct OwnedGfsPath<M: GfsEntryMeta, F: GfsSnapshot<M>> {
    path: GfsPath,
    fs:   Arc<F>,
    _meta_data_marker: marker::PhantomData<M>,
}

pub trait PathLike: Clone{
    fn as_str(&self) -> &str;

    fn is_child(&self, path: &Self) -> bool;

    fn to_string(&self) -> String;

    fn to_directory_path(self) -> Self;

    fn join(&self, path: &str) -> Self;

    fn as_path(&self) -> &GfsPath;
}

#[derive(Clone)]
pub struct GfsPath {
    path: Arc<str>,
}

impl<T: AsRef<str>> From<T> for GfsPath {
    fn from(value: T) -> Self {
        Self {
            path: Arc::from(value.as_ref()),
        }
    }
}

impl<M: GfsEntryMeta, F: GFS<M>> From<OwnedGfsPath<M, F>> for GfsPath {
    fn from(value: OwnedGfsPath<M, F>) -> Self { value.path }
}


impl PathLike for GfsPath {
    fn as_str(&self) -> &str { &self.path }

    fn is_child(&self, path: &Self) -> bool { path.path.starts_with(self.as_str()) }

    fn to_string(&self) -> String {
        self.path.to_string()
    }

    fn to_directory_path(self) -> GfsPath {
        if self.path.ends_with(GFS_SEPARATOR) {
            return self;
        }
        return GfsPath::from(format!("{}{}", self.as_str(), GFS_SEPARATOR))
    }

    fn join(&self, path: &str) -> Self {
        GfsPath::from(format!("{}{}{}", self.as_str(), GFS_SEPARATOR, path.trim_start_matches(SEPARATORS)))
    }



    fn as_path(&self) -> &GfsPath { self }
}

impl<M: GfsEntryMeta, F: GfsSnapshot<M>> Clone for OwnedGfsPath<M, F> {
    fn clone(&self) -> Self {
        Self {
            path: self.path.clone(),
            fs: self.fs.clone(),
            _meta_data_marker: PhantomData
        }
    }
}

impl<M: GfsEntryMeta, F: GfsSnapshot<M>> PathLike for OwnedGfsPath<M, F> {

    fn as_str(&self) -> &str { self.path.as_str() }

    fn is_child(&self, path: &Self) -> bool { self.path.is_child(path.as_path()) }

    fn to_string(&self) -> String { self.path.to_string() }

    fn to_directory_path(self) -> Self {
        Self {
            path: self.path.to_directory_path(),
            fs: self.fs,
            _meta_data_marker: PhantomData
        }
    }

    fn join(&self, path: &str) -> Self {
        Self {
            path: self.path.join(path),
            fs: self.fs.clone(),
            _meta_data_marker: PhantomData
        }
    }


    fn as_path(&self) -> &GfsPath { &self.path }
}

impl<M: GfsEntryMeta, F: GFS<M>> OwnedGfsPath<M, F> {

    pub fn fs_new(&self, metadata: M, contents: Arc<Vec<u8>>) -> GfsResult<&GfsFile<M>> {
        self.fs.insert_entry(&self.path, metadata, contents)
    }

    pub fn fs_writer(&self) -> WritableFile<M, F> { self.fs.entry_writer(&self.path) }

    pub fn fs_drop_entry(&self) -> GfsResult<GfsFile<M>> { self.fs.drop_entry(&self.path) }

    pub fn fs_remove_entry(&self) -> GfsResult<()> { self.fs.remove_entry(&self.path) }

    pub fn fs_rename_entry(&self, new_path: &GfsPath) -> GfsResult<()>{ self.fs.rename_entry(&self.path, new_path) }
}

impl<M: GfsEntryMeta, F: GfsSnapshot<M>> OwnedGfsPath<M, F> {
    pub fn fs_meta(&self) -> Option<M> { self.fs.read_meta(&self.path) }

    pub fn fs_data(&self) -> Option<Arc<Vec<u8>>> { self.fs.read_data(&self.path) }

    pub fn fs_reader(&self) -> Option<ReadableFile<M>> { self.fs.entry_reader(&self.path) }
}
