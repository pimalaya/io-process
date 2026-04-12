//! I/O-free coroutine to spawn a process and wait for its exit
//! status.

use alloc::vec::Vec;
use core::mem;

use log::trace;
use thiserror::Error;

use crate::{
    command::Command,
    coroutines::spawn::ProcessSpawnState,
    io::{ProcessInput, ProcessOutput},
    status::ExitStatus,
};

/// Error emitted by the [`Spawn`] coroutine.
#[derive(Debug, Error)]
pub enum ProcessSpawnOutError {
    #[error("Invalid process spawn arg {arg:?} for state {state:?}")]
    Invalid {
        arg: Option<ProcessOutput>,
        state: ProcessSpawnState,
    },
}

/// Result emitted on each step of the [`Spawn`] coroutine.
#[derive(Debug)]
pub enum ProcessSpawnOutResult {
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
    Err { err: ProcessSpawnOutError },
}

/// I/O-free coroutine for spawning a process and waiting for its exit
/// status.
///
/// Use this when you only care about whether the process succeeded or
/// failed. To also capture stdout and stderr, see [`SpawnOut`].
///
/// [`SpawnOut`]: super::spawn_out::SpawnOut
#[derive(Debug)]
pub struct ProcessSpawnOut {
    state: ProcessSpawnState,
}

impl ProcessSpawnOut {
    /// Creates a new coroutine that will spawn the given command.
    pub fn new(cmd: impl Into<Command>) -> Self {
        let cmd = cmd.into();
        trace!("prepares process to be spawned: {cmd:?}");
        let state = ProcessSpawnState::WantsSpawn(cmd);
        Self { state }
    }

    /// Makes the spawn progress.
    pub fn resume(&mut self, arg: Option<ProcessOutput>) -> ProcessSpawnOutResult {
        match (mem::take(&mut self.state), arg) {
            (ProcessSpawnState::WantsSpawn(cmd), None) => {
                trace!("wants I/O to spawn process and collect output");
                let input = ProcessInput::SpawnOut { cmd };
                self.state = ProcessSpawnState::Spawning;
                ProcessSpawnOutResult::Io { input }
            }
            (
                ProcessSpawnState::Spawning,
                Some(ProcessOutput::SpawnedOut {
                    status,
                    stdout,
                    stderr,
                }),
            ) => {
                trace!("resumes after spawning process and collecting output");
                self.state = ProcessSpawnState::Spawned;
                ProcessSpawnOutResult::Ok {
                    status,
                    stdout,
                    stderr,
                }
            }
            (state, arg) => {
                let err = ProcessSpawnOutError::Invalid { arg, state };
                ProcessSpawnOutResult::Err { err }
            }
        }
    }
}
