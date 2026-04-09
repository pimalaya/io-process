//! I/O-free coroutine to spawn a process with bytes piped to its
//! stdin.

use alloc::vec::Vec;

use log::{debug, trace};
use thiserror::Error;

use crate::{
    command::Command,
    io::{ProcessInput, ProcessOutput},
    status::ExitStatus,
};

/// Error emitted by the [`SpawnIn`] coroutine.
#[derive(Debug, Error)]
pub enum SpawnInError {
    /// The coroutine received an unexpected [`ProcessOutput`] variant.
    #[error("Invalid spawn-in arg: {0:?}")]
    InvalidArg(ProcessOutput),

    /// [`SpawnIn::resume`] was called with `None` after the command
    /// was already consumed.
    #[error("Command not initialized")]
    NotInitialized,
}

/// Result emitted on each step of the [`SpawnIn`] coroutine.
#[derive(Debug)]
pub enum SpawnInResult {
    /// The coroutine has successfully terminated its progression.
    Ok { status: ExitStatus },
    /// A process I/O needs to be performed to make the coroutine
    /// progress.
    Io { input: ProcessInput },
    /// The coroutine encountered an unrecoverable error.
    Err { err: SpawnInError },
}

/// I/O-free coroutine for spawning a process and feeding bytes to its
/// stdin.
///
/// The runtime pipes `stdin` bytes into the process's standard input
/// regardless of the [`Stdio`] configuration on the command's stdin
/// field.
///
/// To also capture stdout and stderr, see [`SpawnOut`].
///
/// [`Stdio`]: crate::stdio::Stdio
/// [`SpawnOut`]: super::spawn_out::SpawnOut
#[derive(Debug)]
pub struct SpawnIn {
    inner: Option<(Command, Vec<u8>)>,
}

impl SpawnIn {
    /// Creates a new coroutine that will spawn the given command and
    /// pipe `stdin` bytes into its standard input.
    pub fn new(cmd: Command, stdin: Vec<u8>) -> Self {
        trace!("prepare command to be spawned: {cmd:?}");
        Self {
            inner: Some((cmd, stdin)),
        }
    }

    /// Makes the spawn-in progress.
    pub fn resume(&mut self, arg: Option<ProcessOutput>) -> SpawnInResult {
        match arg {
            None => {
                let Some((cmd, stdin)) = self.inner.take() else {
                    return SpawnInResult::Err {
                        err: SpawnInError::NotInitialized,
                    };
                };
                trace!("wants process I/O to spawn command with stdin bytes");
                SpawnInResult::Io {
                    input: ProcessInput::SpawnIn { cmd, stdin },
                }
            }
            Some(ProcessOutput::SpawnIn { status }) => {
                debug!("resume after spawning command: {status:?}");
                SpawnInResult::Ok { status }
            }
            Some(output) => SpawnInResult::Err {
                err: SpawnInError::InvalidArg(output),
            },
        }
    }
}
