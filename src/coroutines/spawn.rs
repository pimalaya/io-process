//! I/O-free coroutine to spawn a process and wait for its exit
//! status.

use core::mem;

use log::trace;
use thiserror::Error;

use crate::{
    command::Command,
    io::{ProcessInput, ProcessOutput},
    status::ExitStatus,
};

/// Error emitted by the [`Spawn`] coroutine.
#[derive(Debug, Error)]
pub enum ProcessSpawnError {
    #[error("Invalid notify arg {arg:?} for state {state:?}")]
    Invalid {
        arg: Option<ProcessOutput>,
        state: ProcessSpawnState,
    },
}

/// Result emitted on each step of the [`Spawn`] coroutine.
#[derive(Debug)]
pub enum ProcessSpawnResult {
    /// The coroutine has successfully terminated its progression.
    Ok { status: ExitStatus },
    /// A process I/O needs to be performed to make the coroutine
    /// progress.
    Io { input: ProcessInput },
    /// The coroutine encountered an unrecoverable error.
    Err { err: ProcessSpawnError },
}

#[derive(Debug, Default)]
pub enum ProcessSpawnState {
    WantsSpawn(Command),
    Spawning,
    Spawned,
    #[default]
    Invalid,
}

/// I/O-free coroutine for spawning a process and waiting for its exit
/// status.
///
/// Use this when you only care about whether the process succeeded or
/// failed. To also capture stdout and stderr, see [`SpawnOut`].
///
/// [`SpawnOut`]: super::spawn_out::SpawnOut
#[derive(Debug)]
pub struct ProcessSpawn {
    state: ProcessSpawnState,
}

impl ProcessSpawn {
    /// Creates a new coroutine that will spawn the given command.
    pub fn new(cmd: impl Into<Command>) -> Self {
        let cmd = cmd.into();
        trace!("prepares process to be spawned: {cmd:?}");
        let state = ProcessSpawnState::WantsSpawn(cmd);
        Self { state }
    }

    /// Makes the spawn progress.
    pub fn resume(&mut self, arg: Option<ProcessOutput>) -> ProcessSpawnResult {
        match (mem::take(&mut self.state), arg) {
            (ProcessSpawnState::WantsSpawn(cmd), None) => {
                trace!("wants I/O to spawn process");
                let input = ProcessInput::Spawn { cmd };
                self.state = ProcessSpawnState::Spawning;
                ProcessSpawnResult::Io { input }
            }
            (ProcessSpawnState::Spawning, Some(ProcessOutput::Spawned { status })) => {
                trace!("resumes after spawning process");
                self.state = ProcessSpawnState::Spawned;
                ProcessSpawnResult::Ok { status }
            }
            (state, arg) => {
                let err = ProcessSpawnError::Invalid { arg, state };
                ProcessSpawnResult::Err { err }
            }
        }
    }
}
