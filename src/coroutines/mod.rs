//! Module gathering I/O-free, composable and iterable state machines.
//!
//! Flows emit [`crate::Io`] requests that need to be processed by
//! [`crate::handlers`] in order to continue their progression.

#[path = "spawn-then-wait.rs"]
mod spawn_then_wait;
#[path = "spawn-then-wait-with-output.rs"]
mod spawn_then_wait_with_output;

#[doc(inline)]
pub use self::{
    spawn_then_wait::SpawnThenWait, spawn_then_wait_with_output::SpawnThenWaitWithOutput,
};
