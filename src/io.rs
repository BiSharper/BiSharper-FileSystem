use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use std::{cmp, mem};
use std::sync::Arc;
use crate::{GFS, GfsEntryMeta};
use crate::path::{GfsPath, OwnedGfsPath};

pub struct ReadableFile<T: GfsEntryMeta> {
    metadata:  T,
    position:  usize,
    content:   Arc<[u8]>,
}

pub struct WritableFile<'a, M: GfsEntryMeta, F: GFS<M>> {
    metadata:        M,
    cursor:          Cursor<Vec<u8>>,
    destination:     OwnedGfsPath<'a, M, F>
}

#[derive(Clone)]
pub struct GfsFile<T: GfsEntryMeta> {
    pub(crate) metadata: T,
    pub(crate) contents: Arc<Vec<u8>>,
}

impl<'a, T: GfsEntryMeta> From<GfsFile<T>> for ReadableFile<T> {
    fn from(value: GfsFile<T>) -> Self {
        Self {
            metadata: value.metadata,
            position: 0,
            content: Arc::from(value.contents.as_slice())
        }

    }
}

impl<T: GfsEntryMeta> GfsFile<T> {
    pub fn create(metadata: T, contents: Arc<Vec<u8>>) -> Self {
        GfsFile { metadata, contents}
    }
}

impl<T: GfsEntryMeta> ReadableFile<T> {
    pub fn metadata(&self) -> &T { &self.metadata }

    pub fn len(&self) -> usize { self.content.len() }
}

impl<'a, M: GfsEntryMeta, F: GFS<M>> WritableFile<'a, M, F> {
    pub fn from_owned(path: &OwnedGfsPath<'a, M, F>, metadata: M, contents: Vec<u8>) -> WritableFile<'a, M, F> {
        WritableFile {
            metadata,
            cursor: Cursor::new(contents),
            destination: path.clone(),
        }
    }

    pub fn create(path: &GfsPath, filesystem: &'a F, metadata: M, contents: Vec<u8>) -> WritableFile<'a, M, F> {
        WritableFile {
            metadata,
            cursor: Cursor::new(contents),
            destination: filesystem.create_path(&path),
        }
    }

    pub fn metadata(&self) -> &M { &self.metadata }

    pub fn modify_metadata(&mut self) -> &mut M { &mut self.metadata }
}

impl<M: GfsEntryMeta, F: GFS<M>> Write for WritableFile<'_, M, F> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> { self.cursor.write(buf) }

    fn flush(&mut self) -> std::io::Result<()> { self.cursor.flush() }
}

impl<M: GfsEntryMeta, F: GFS<M>> Read for WritableFile<'_, M, F> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> { self.cursor.read(buf) }
}

impl<M: GfsEntryMeta, F: GFS<M>> Seek for WritableFile<'_, M, F> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> { self.cursor.seek(pos) }
}

impl<M: GfsEntryMeta, F: GFS<M>> Drop for WritableFile<'_, M, F> {
    fn drop(&mut self) {
        let mut content = vec![];
        mem::swap(&mut content, self.cursor.get_mut());
        self.destination.fs_new(self.metadata, Arc::new(content)).unwrap();
    }
}

impl<T: GfsEntryMeta> Read for ReadableFile<T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let amt = cmp::min(buf.len(), self.len());

        if amt == 1 {
            buf[0] = self.content[self.position];
        } else {
            buf[..amt].copy_from_slice(
                &self.content[self.position..self.position + amt],
            );
        }
        self.position += amt;
        Ok(amt)
    }
}

impl<T: GfsEntryMeta> Seek for ReadableFile<T> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        match pos {
            SeekFrom::Start(offset) => self.position = offset as usize,
            SeekFrom::Current(offset) => self.position = self.position + offset as usize,
            SeekFrom::End(offset) => self.position = self.content.len() + offset as usize,
        }
        Ok(self.position as u64)
    }
}