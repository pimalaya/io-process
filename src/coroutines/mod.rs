//! Collection of I/O-free, resumable and composable process state
//! machines.
//!
//! Coroutines emit [I/O] requests that need to be processed by
//! [runtimes] in order to continue their progression.
//!
//! [I/O]: crate::io::ProcessIo
//! [runtimes]: crate::runtimes

#[path = "spawn-then-wait.rs"]
pub mod spawn_then_wait;
#[path = "spawn-then-wait-with-output.rs"]
pub mod spawn_then_wait_with_output;
