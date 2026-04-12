//! I/O-free coroutine to spawn a pipeline of processes.

use alloc::vec::Vec;

use log::{debug, trace};
use thiserror::Error;

use crate::{
    command::Command,
    io::{ProcessInput, ProcessOutput},
    status::ExitStatus,
};

/// Error emitted by the [`SpawnPipeline`] coroutine.
#[derive(Debug, Error)]
pub enum SpawnPipelineError {
    /// The coroutine received an unexpected [`ProcessOutput`] variant.
    #[error("Invalid spawn-pipeline arg: {0:?}")]
    InvalidArg(ProcessOutput),

    /// [`SpawnPipeline::resume`] was called with `None` after the
    /// commands were already consumed.
    #[error("Commands not initialized")]
    NotInitialized,
}

/// Result emitted on each step of the [`SpawnPipeline`] coroutine.
#[derive(Debug)]
pub enum SpawnPipelineResult {
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
    Err { err: SpawnPipelineError },
}

/// I/O-free coroutine for spawning a pipeline of processes.
///
/// Each process's stdout is piped into the next process's stdin. The
/// runtime captures the stdout and stderr of the last process.
///
/// On success, [`SpawnPipelineResult::Ok`] carries the last process's
/// exit status, stdout, and stderr.
#[derive(Debug)]
pub struct SpawnPipeline {
    cmds: Option<Vec<Command>>,
}

impl SpawnPipeline {
    /// Creates a new coroutine that will spawn the given commands as a
    /// pipeline.
    pub fn new(cmds: impl IntoIterator<Item = Command>) -> Self {
        let cmds: Vec<Command> = cmds.into_iter().collect();
        trace!(
            "prepare {} commands to be spawned as a pipeline",
            cmds.len()
        );
        Self { cmds: Some(cmds) }
    }

    /// Makes the spawn-pipeline progress.
    pub fn resume(&mut self, arg: Option<ProcessOutput>) -> SpawnPipelineResult {
        match arg {
            None => {
                let Some(cmds) = self.cmds.take() else {
                    return SpawnPipelineResult::Err {
                        err: SpawnPipelineError::NotInitialized,
                    };
                };
                trace!("wants process I/O to spawn pipeline");
                SpawnPipelineResult::Io {
                    input: ProcessInput::SpawnPipeline { cmds },
                }
            }
            Some(ProcessOutput::SpawnedPipeline {
                status,
                stdout,
                stderr,
            }) => {
                debug!("resume after spawning pipeline: {:?}", status);
                SpawnPipelineResult::Ok {
                    status,
                    stdout,
                    stderr,
                }
            }
            Some(output) => SpawnPipelineResult::Err {
                err: SpawnPipelineError::InvalidArg(output),
            },
        }
    }
}
