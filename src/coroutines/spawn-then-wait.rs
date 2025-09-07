//! I/O-free coroutine to spawn a process and wait for its child's exit status.

use log::{debug, trace};
use thiserror::Error;

use crate::{command::Command, io::ProcessIo, status::SpawnStatus};

/// Errors that can occur during the coroutine progression.
#[derive(Debug, Error)]
pub enum SpawnThenWaitError {
    /// The coroutine received an invalid argument.
    ///
    /// Occurs when the coroutine receives an I/O response from
    /// another coroutine, which should not happen if the runtime maps
    /// correctly the arguments.
    #[error("Invalid argument: expected {0}, got {1:?}")]
    InvalidArgument(&'static str, ProcessIo),

    /// The command was not initialized.
    #[error("Command not initialized")]
    NotInitialized,
}

/// Output emitted after a coroutine finishes its progression.
#[derive(Debug)]
pub enum SpawnThenWaitResult {
    /// The coroutine has successfully terminated its progression.
    Ok(SpawnStatus),

    /// A process I/O needs to be performed to make the coroutine progress.
    Io(ProcessIo),

    /// An error occurred during the coroutine progression.
    Err(SpawnThenWaitError),
}

/// I/O-free coroutine for spawning a process then waiting for its
/// child's exit status.
///
/// This coroutine should be used when you do not care about the
/// output, or when you need the output to be piped into another
/// process.
///
/// If you need to collect the output, see
/// [`super::spawn_then_wait_with_output::SpawnThenWaitWithOutput`].
#[derive(Debug)]
pub struct SpawnThenWait {
    cmd: Option<Command>,
}

impl SpawnThenWait {
    /// Creates a new coroutine from the given command builder.
    pub fn new(command: Command) -> Self {
        trace!("prepare command to be spawned: {command:?}");
        let cmd = Some(command);
        Self { cmd }
    }

    /// Makes the coroutine progress.
    pub fn resume(&mut self, arg: Option<ProcessIo>) -> SpawnThenWaitResult {
        let Some(arg) = arg else {
            let Some(cmd) = self.cmd.take() else {
                return SpawnThenWaitResult::Err(SpawnThenWaitError::NotInitialized);
            };

            trace!("break: need I/O to spawn command");
            return SpawnThenWaitResult::Io(ProcessIo::SpawnThenWait(Err(cmd)));
        };

        trace!("resume after spawning command");

        let ProcessIo::SpawnThenWait(io) = arg else {
            let err = SpawnThenWaitError::InvalidArgument("spawn output", arg);
            return SpawnThenWaitResult::Err(err);
        };

        let output = match io {
            Ok(output) => output,
            Err(cmd) => return SpawnThenWaitResult::Io(ProcessIo::SpawnThenWait(Err(cmd))),
        };

        debug!("spawned command: {:?}", output.status);
        SpawnThenWaitResult::Ok(output)
    }
}
