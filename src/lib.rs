

mod interop;
#[allow(unused_imports)] pub use interop::*;

use std::ops::{Deref, DerefMut};
use rfsa::{ReadableVFile, ReadableVMetadata, VDirectory, VFile, WritableVFile, WritableVMetadata};
use rfsa::impls::memory::MemoryFileSystem;
use rfsa::macros::VMeta;

pub type GameFile = VFile<GameMeta>;
pub type ReadableGameFile = ReadableVFile<GameMeta>;
pub type WritableGameFile<'a> = WritableVFile<'a, GameMeta, GameFileSystem>;
pub type ReadableGameMetadata = ReadableVMetadata<GameMeta>;
pub type WritableGameMetadata<'a> = WritableVMetadata<'a, GameMeta, GameFileSystem>;
pub type GameDirectory<'a> = VDirectory<'a, GameMeta, GameFileSystem>;

#[derive(VMeta, Copy, Clone, Default, Eq, PartialEq)]
pub struct GameMeta {

}

pub struct GameFileSystem { file_system: MemoryFileSystem<GameMeta> }

impl Deref for GameFileSystem {
    type Target = MemoryFileSystem<GameMeta>;

    fn deref(&self) -> &Self::Target { &self.file_system }
}

impl DerefMut for GameFileSystem {
    fn deref_mut(&mut self) -> &mut Self::Target  { &mut self.file_system }
}




