//! Process I/O requests and responses.

use std::process::Output as SpawnOutput;

use crate::{command::Command, status::SpawnStatus};

/// The process I/O request enum, emitted by [coroutines] and
/// processed by [runtimes].
///
/// Represents all the possible I/O requests that a stream coroutine
/// can emit. Runtimes should be able to handle all variants.
///
/// [coroutines]: crate::coroutines
/// [runtimes]: crate::runtimes
#[derive(Debug)]
pub enum ProcessIo {
    /// I/O for spawning a process and waiting for its exit status.
    ///
    /// Input: command
    ///
    /// Output: spawn status
    SpawnThenWait(Result<SpawnStatus, Command>),

    /// I/O for spawning a process and waiting for its exit status and
    /// any potential output from stdout or stderr.
    ///
    /// Input: command
    ///
    /// Output: spawn output
    SpawnThenWaitWithOutput(Result<SpawnOutput, Command>),
}
