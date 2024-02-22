use std::marker::PhantomData;
use std::sync::Arc;
use crate::filesystem::{GFS, GfsEntryMeta};
use crate::{GfsResult, GfsSnapshot};
use crate::io::{GfsFile, ReadableFile, WritableFile};

pub const GFS_SEPARATOR: char = '/';
pub const ALT_SEPARATOR: char = '\\';
const SEPARATORS: [char; 2] = [ALT_SEPARATOR, GFS_SEPARATOR];

pub struct OwnedGfsPath<'a, M: GfsEntryMeta, F: GfsSnapshot<M>> {
    path: GfsPath,
    fs:   &'a F,
    _meta_data_marker: PhantomData<M>,
}

pub trait PathLike: Clone{
    fn as_str(&self) -> &str;

    fn is_child(&self, path: &Self) -> bool;

    fn to_string(&self) -> String;

    fn to_directory_path(self) -> Self;

    fn join(&self, path: &GfsPath) -> Self;

    fn to_owned_path<M: GfsEntryMeta, F: GfsSnapshot<M>>(self, filesystem: &F) -> OwnedGfsPath<M, F> {
        OwnedGfsPath::create(self, filesystem)
    }

    fn as_owned_path<'a, M: GfsEntryMeta, F: GfsSnapshot<M>>(&self, filesystem: &'a F) -> OwnedGfsPath<'a, M, F> {
        OwnedGfsPath::create(self.as_path().clone(), filesystem)
    }

    fn as_path(&self) -> &GfsPath;

    fn to_path(self) -> GfsPath;

}

#[derive(Clone, Hash, Eq, PartialEq)]
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

impl<'a, M: GfsEntryMeta, F: GfsSnapshot<M>> OwnedGfsPath<'a, M, F> {
    pub fn create(path: impl PathLike, snapshot: &'a F) -> Self {
        Self {
            path: path.to_path(),
            fs: snapshot,
            _meta_data_marker: Default::default(),
        }
    }

    pub fn filesystem(&self) -> &'a F { self.fs }
}

impl<M: GfsEntryMeta, F: GFS<M>> From<OwnedGfsPath<'_, M, F>> for GfsPath {
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

    fn join(&self, path: &GfsPath) -> Self {
        GfsPath::from(format!("{}{}{}", self.as_str(), GFS_SEPARATOR, path.path.trim_start_matches(SEPARATORS)))
    }

    fn as_path(&self) -> &GfsPath { self }

    fn to_path(self) -> GfsPath { self }
}

impl<M: GfsEntryMeta, F: GfsSnapshot<M>> Clone for OwnedGfsPath<'_, M, F> {
    fn clone(&self) -> Self {
        Self {
            path: self.path.clone(),
            fs: self.fs,
            _meta_data_marker: PhantomData
        }
    }
}

impl<M: GfsEntryMeta, F: GfsSnapshot<M>> PathLike for OwnedGfsPath<'_, M, F> {

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

    fn join(&self, path: &GfsPath) -> Self {
        Self {
            path: self.path.join(path),
            fs: self.fs,
            _meta_data_marker: PhantomData
        }
    }

    fn as_path(&self) -> &GfsPath { &self.path }

    fn to_path(self) -> GfsPath { self.path }
}

impl<'a, M: GfsEntryMeta, F: GFS<M>> OwnedGfsPath<'a, M, F> {

    pub fn fs_new(&self, metadata: M, contents: Box<[u8]>) -> GfsResult<OwnedGfsPath<'a, M, F>> {
        self.fs.insert_entry(&self.path, metadata, contents)
    }

    pub fn fs_writer(&self) -> GfsResult<WritableFile<M, F>> { self.fs.entry_writer(&self.path) }

    pub fn fs_drop_entry(&self) -> GfsResult<GfsFile<M>> { self.fs.drop_entry(&self.path) }

    pub fn fs_remove_entry(&self) -> GfsResult<()> { self.fs.remove_entry(&self.path) }

    pub fn fs_rename_entry(&self, new_path: &GfsPath) -> GfsResult<OwnedGfsPath<'a, M, F>>{
        self.fs.rename_entry(&self.path, new_path)
    }
}

impl<M: GfsEntryMeta, F: GfsSnapshot<M>> OwnedGfsPath<'_, M, F> {
    pub fn fs_meta(&self) -> GfsResult<M> { self.fs.read_meta(&self.path) }

    pub fn fs_data(&self) -> GfsResult<Arc<[u8]>> { self.fs.read_data(&self.path) }

    pub fn fs_reader(&self) -> GfsResult<ReadableFile<M>> { self.fs.entry_reader(&self.path) }
}
