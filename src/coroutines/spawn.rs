//! I/O-free coroutine to spawn a process and wait for its exit
//! status.

use log::{debug, trace};
use thiserror::Error;

use crate::{
    command::Command,
    io::{ProcessInput, ProcessOutput},
    status::ExitStatus,
};

/// Error emitted by the [`Spawn`] coroutine.
#[derive(Debug, Error)]
pub enum SpawnError {
    /// The coroutine received an unexpected [`ProcessOutput`] variant.
    #[error("Invalid spawn arg: {0:?}")]
    InvalidArg(ProcessOutput),

    /// [`Spawn::resume`] was called with `None` after the command was
    /// already consumed.
    #[error("Command not initialized")]
    NotInitialized,
}

/// Result emitted on each step of the [`Spawn`] coroutine.
#[derive(Debug)]
pub enum SpawnResult {
    /// The coroutine has successfully terminated its progression.
    Ok { status: ExitStatus },
    /// A process I/O needs to be performed to make the coroutine
    /// progress.
    Io { input: ProcessInput },
    /// The coroutine encountered an unrecoverable error.
    Err { err: SpawnError },
}

/// I/O-free coroutine for spawning a process and waiting for its exit
/// status.
///
/// Use this when you only care about whether the process succeeded or
/// failed. To also capture stdout and stderr, see [`SpawnOut`].
///
/// [`SpawnOut`]: super::spawn_out::SpawnOut
#[derive(Debug)]
pub struct Spawn {
    cmd: Option<Command>,
}

impl Spawn {
    /// Creates a new coroutine that will spawn the given command.
    pub fn new(cmd: Command) -> Self {
        trace!("prepare command to be spawned: {cmd:?}");
        Self { cmd: Some(cmd) }
    }

    /// Makes the spawn progress.
    pub fn resume(&mut self, arg: Option<ProcessOutput>) -> SpawnResult {
        match arg {
            None => {
                let Some(cmd) = self.cmd.take() else {
                    return SpawnResult::Err {
                        err: SpawnError::NotInitialized,
                    };
                };
                trace!("wants process I/O to spawn command");
                SpawnResult::Io {
                    input: ProcessInput::Spawn { cmd },
                }
            }
            Some(ProcessOutput::Spawn { status }) => {
                debug!("resume after spawning command: {status:?}");
                SpawnResult::Ok { status }
            }
            Some(output) => SpawnResult::Err {
                err: SpawnError::InvalidArg(output),
            },
        }
    }
}
