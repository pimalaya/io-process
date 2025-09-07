//! I/O-free coroutine to spawn a process and wait for its child's output.

use log::{debug, trace};
use std::process::Output;
use thiserror::Error;

use crate::{command::Command, io::ProcessIo};

/// Errors that can occur during the coroutine progression.
#[derive(Debug, Error)]
pub enum SpawnThenWaitWithOutputError {
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
pub enum SpawnThenWaitWithOutputResult {
    /// The coroutine has successfully terminated its progression.
    Ok(Output),

    /// A process I/O needs to be performed to make the coroutine progress.
    Io(ProcessIo),

    /// An error occurred during the coroutine progression.
    Err(SpawnThenWaitWithOutputError),
}

/// I/O-free coroutine for spawning a process then waiting for its
/// child's output.
///
/// This coroutine should be used when you need to collect the child
/// process' output, from stdout and stderr.
///
/// If you do not need to collect the output, or if you need to pipe
/// the output to another process, see
/// [`super::spawn_then_wait::SpawnThenWait`].
#[derive(Debug)]
pub struct SpawnThenWaitWithOutput {
    cmd: Option<Command>,
}

impl SpawnThenWaitWithOutput {
    /// Creates a new coroutine from the given command builder.
    pub fn new(cmd: Command) -> Self {
        trace!("prepare command to be spawned: {cmd:?}");
        let cmd = Some(cmd);
        Self { cmd }
    }

    /// Makes the coroutine progress.
    pub fn resume(&mut self, arg: Option<ProcessIo>) -> SpawnThenWaitWithOutputResult {
        let Some(arg) = arg else {
            let Some(cmd) = self.cmd.take() else {
                return SpawnThenWaitWithOutputResult::Err(
                    SpawnThenWaitWithOutputError::NotInitialized,
                );
            };

            trace!("break: need I/O to spawn command");
            return SpawnThenWaitWithOutputResult::Io(ProcessIo::SpawnThenWaitWithOutput(Err(cmd)));
        };

        trace!("resume after spawning command");

        let ProcessIo::SpawnThenWaitWithOutput(io) = arg else {
            let err = SpawnThenWaitWithOutputError::InvalidArgument("spawn output", arg);
            return SpawnThenWaitWithOutputResult::Err(err);
        };

        let output = match io {
            Ok(output) => output,
            Err(cmd) => {
                let io = ProcessIo::SpawnThenWaitWithOutput(Err(cmd));
                return SpawnThenWaitWithOutputResult::Io(io);
            }
        };

        debug!("spawned command: {:?}", output.status);
        SpawnThenWaitWithOutputResult::Ok(output)
    }
}
