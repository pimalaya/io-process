#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![doc = include_str!("../README.md")]

extern crate alloc;

pub mod command;
pub mod coroutines;
pub mod io;
pub mod runtimes;
#[cfg(feature = "serde")]
pub mod serde;
pub mod status;
pub mod stdio;
