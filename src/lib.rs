#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

mod loader;
mod plugin;

pub use loader::KritaDocumentLoader;
pub use plugin::KritaPlugin;
