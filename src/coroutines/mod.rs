//! Collection of I/O-free, resumable and composable process state
//! machines.
//!
//! Coroutines emit [`ProcessInput`] requests that need to be processed
//! by [runtimes] in order to continue their progression.
//!
//! [`ProcessInput`]: crate::io::ProcessInput
//! [runtimes]: crate::runtimes

pub mod spawn;
pub mod spawn_in;
pub mod spawn_out;
pub mod spawn_pipeline;
