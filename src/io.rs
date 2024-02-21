use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use std::{cmp, mem};
use std::sync::Arc;
use crate::{GfsEntryMeta};
use crate::path::GfsPath;

pub struct ReadableFile<T: GfsEntryMeta> {
    metadata:  T,
    position:  usize,
    content:   Arc<Vec<u8>>,
}

pub struct WritableFile<T: GfsEntryMeta> {
    metadata:        T,
    cursor:          Cursor<Vec<u8>>,
    destination:     GfsPath<T>
}

pub struct GfsFile<T: GfsEntryMeta> {
    pub(crate) metadata: T,
    pub(crate) contents: Arc<Vec<u8>>,
}

impl<T: GfsEntryMeta> From<GfsFile<T>> for ReadableFile<T> {
    fn from(value: GfsFile<T>) -> Self {
        Self {
            metadata: value.metadata,
            position: 0,
            content: value.contents,
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

impl<T: GfsEntryMeta> WritableFile<T> {
    pub fn create(path: &GfsPath<T>, metadata: T, contents: Vec<u8>) -> Self {
        WritableFile {
            metadata,
            cursor: Cursor::new(contents),
            destination: path.clone(),
        }
    }

    pub fn metadata(&self) -> &T { &self.metadata }

    pub fn modify_metadata(&mut self) -> &mut T { &mut self.metadata }
}

impl<T: GfsEntryMeta> Write for WritableFile<T> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> { self.cursor.write(buf) }

    fn flush(&mut self) -> std::io::Result<()> { self.cursor.flush() }
}

impl<T: GfsEntryMeta> Read for WritableFile<T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> { self.cursor.read(buf) }
}

impl<T: GfsEntryMeta> Seek for WritableFile<T> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> { self.cursor.seek(pos) }
}

impl<T: GfsEntryMeta> Drop for WritableFile<T> {
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
                &self.content.as_slice()[self.position..self.position + amt],
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
            SeekFrom::End(offset) => self.position = (self.content.len() + offset as usize),
        }
        Ok(self.position as u64)
    }
}