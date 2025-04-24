#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![doc = include_str!("../README.md")]

mod command;
pub mod coroutines;
mod io;
mod output;
pub mod runtimes;
#[cfg(feature = "serde")]
mod serde;

#[doc(inline)]
pub use self::{command::Command, io::Io, output::SpawnOutput};
