//! Process input and output.

use alloc::vec::Vec;

use crate::{command::Command, status::ExitStatus};

/// Process input emitted by [coroutines] and processed by [runtimes].
///
/// Represents all the possible operations that a process coroutine
/// can ask for. Runtimes must handle all variants and return a
/// matching [`ProcessOutput`].
///
/// [coroutines]: crate::coroutines
/// [runtimes]: crate::runtimes
#[derive(Debug)]
pub enum ProcessInput {
    /// Request to spawn a process and wait for its exit status.
    Spawn { cmd: Command },
    /// Request to spawn a process, capture its stdout and stderr,
    /// and wait for its exit status.
    SpawnOut { cmd: Command },
    /// Request to spawn a process, feed bytes to its stdin, and wait
    /// for its exit status.
    SpawnIn { cmd: Command, stdin: Vec<u8> },
    /// Request to spawn a pipeline of processes, feeding each
    /// process's stdout into the next process's stdin, and collecting
    /// the last process's stdout, stderr, and exit status.
    SpawnPipeline { cmds: Vec<Command> },
}

/// Process output returned by [runtimes] after processing a
/// [`ProcessInput`].
///
/// Each variant corresponds to the matching [`ProcessInput`] variant
/// and carries the data produced by the I/O operation.
///
/// [runtimes]: crate::runtimes
#[derive(Debug)]
pub enum ProcessOutput {
    /// Response to a [`ProcessInput::Spawn`] request.
    Spawn { status: ExitStatus },
    /// Response to a [`ProcessInput::SpawnOut`] request.
    SpawnOut {
        /// The exit status of the process.
        status: ExitStatus,
        /// The raw bytes written to stdout.
        stdout: Vec<u8>,
        /// The raw bytes written to stderr.
        stderr: Vec<u8>,
    },
    /// Response to a [`ProcessInput::SpawnIn`] request.
    SpawnIn { status: ExitStatus },
    /// Response to a [`ProcessInput::SpawnPipeline`] request.
    SpawnPipeline {
        /// The exit status of the process.
        status: ExitStatus,
        /// The raw bytes written to stdout.
        stdout: Vec<u8>,
        /// The raw bytes written to stderr.
        stderr: Vec<u8>,
    },
}
