#![feature(lazy_cell)]

extern crate core;

pub mod error; pub use error::*;
pub mod io;
pub mod path;
pub extern crate bisharper_filesystem_macros as macros;

mod filesystem; pub use filesystem::*;
mod implementation; pub use implementation::*;

pub const FILESYSTEM_VERSION: &'static str = env!("CARGO_PKG_VERSION");
