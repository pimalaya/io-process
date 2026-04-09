//! I/O-free coroutine to spawn a process and capture its output.

use log::{debug, trace};
use thiserror::Error;

use alloc::vec::Vec;

use crate::{
    command::Command,
    io::{ProcessInput, ProcessOutput},
    status::ExitStatus,
};

/// Error emitted by the [`SpawnOut`] coroutine.
#[derive(Debug, Error)]
pub enum SpawnOutError {
    /// The coroutine received an unexpected [`ProcessOutput`] variant.
    #[error("Invalid spawn-out arg: {0:?}")]
    InvalidArg(ProcessOutput),

    /// [`SpawnOut::resume`] was called with `None` after the command
    /// was already consumed.
    #[error("Command not initialized")]
    NotInitialized,
}

/// Result emitted on each step of the [`SpawnOut`] coroutine.
#[derive(Debug)]
pub enum SpawnOutResult {
    /// The coroutine has successfully terminated its progression.
    Ok {
        status: ExitStatus,
        stdout: Vec<u8>,
        stderr: Vec<u8>,
    },
    /// A process I/O needs to be performed to make the coroutine
    /// progress.
    Io { input: ProcessInput },
    /// The coroutine encountered an unrecoverable error.
    Err { err: SpawnOutError },
}

/// I/O-free coroutine for spawning a process and capturing its stdout
/// and stderr.
///
/// The runtime captures both streams regardless of the [`Stdio`]
/// configuration on the command's stdout and stderr fields.
///
/// To only get the exit status without capturing output, see
/// [`Spawn`]. To feed bytes to stdin, see [`SpawnIn`].
///
/// [`Stdio`]: crate::stdio::Stdio
/// [`Spawn`]: super::spawn::Spawn
/// [`SpawnIn`]: super::spawn_in::SpawnIn
#[derive(Debug)]
pub struct SpawnOut {
    cmd: Option<Command>,
}

impl SpawnOut {
    /// Creates a new coroutine that will spawn the given command and
    /// capture its output.
    pub fn new(cmd: Command) -> Self {
        trace!("prepare command to be spawned: {cmd:?}");
        Self { cmd: Some(cmd) }
    }

    /// Makes the spawn-out progress.
    pub fn resume(&mut self, arg: Option<ProcessOutput>) -> SpawnOutResult {
        match arg {
            None => {
                let Some(cmd) = self.cmd.take() else {
                    return SpawnOutResult::Err {
                        err: SpawnOutError::NotInitialized,
                    };
                };
                trace!("wants process I/O to spawn command and capture output");
                SpawnOutResult::Io {
                    input: ProcessInput::SpawnOut { cmd },
                }
            }
            Some(ProcessOutput::SpawnOut {
                status,
                stdout,
                stderr,
            }) => {
                debug!("resume after spawning command: {:?}", status);
                SpawnOutResult::Ok {
                    status,
                    stdout,
                    stderr,
                }
            }
            Some(output) => SpawnOutResult::Err {
                err: SpawnOutError::InvalidArg(output),
            },
        }
    }
}
