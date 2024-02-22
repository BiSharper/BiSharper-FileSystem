#![feature(lazy_cell)]

pub mod error; pub use error::*;
pub mod io;
pub mod path;
mod filesystem; pub use filesystem::*;
mod implementation; pub use implementation::*;

pub const FILESYSTEM_VERSION: &'static str = env!("CARGO_PKG_VERSION");
