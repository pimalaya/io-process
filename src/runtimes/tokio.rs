//! The Tokio-based, async process runtime.

use std::{io, process::Output};

use tokio::process::Command as TokioCommand;

use crate::{command::Command, io::ProcessIo, status::SpawnStatus};

/// The Tokio-based, async process runtime.
///
/// This handler makes use of the [`tokio::process`] module to spawn
/// processes and wait for exit status or output.
pub async fn handle(io: ProcessIo) -> io::Result<ProcessIo> {
    match io {
        ProcessIo::SpawnThenWait(io) => spawn_then_wait(io).await,
        ProcessIo::SpawnThenWaitWithOutput(io) => spawn_then_wait_with_output(io).await,
    }
}

/// Spawns a process then wait for its child's exit status.
///
/// This function builds a [`std::process::Command`] from the flow's
/// command builder, spawns a process, collects std{in,out,err} then
/// waits for the exit status.
pub async fn spawn_then_wait(input: Result<SpawnStatus, Command>) -> io::Result<ProcessIo> {
    let Err(command) = input else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing command"));
    };

    let mut command = TokioCommand::from(command);
    let mut child = command.spawn()?;

    #[cfg(unix)]
    let stdin = child.stdin.take().and_then(|io| io.into_owned_fd().ok());
    #[cfg(windows)]
    let stdin = child
        .stdin
        .take()
        .and_then(|io| io.into_owned_handle().ok());

    #[cfg(unix)]
    let stdout = child.stdout.take().and_then(|io| io.into_owned_fd().ok());
    #[cfg(windows)]
    let stdout = child
        .stdout
        .take()
        .and_then(|io| io.into_owned_handle().ok());

    #[cfg(unix)]
    let stderr = child.stderr.take().and_then(|io| io.into_owned_fd().ok());
    #[cfg(windows)]
    let stderr = child
        .stderr
        .take()
        .and_then(|io| io.into_owned_handle().ok());

    let output = SpawnStatus {
        status: child.wait().await?,
        stdin: stdin.map(Into::into),
        stdout: stdout.map(Into::into),
        stderr: stderr.map(Into::into),
    };

    Ok(ProcessIo::SpawnThenWait(Ok(output)))
}

/// Spawns a process then wait for its child's output.
///
/// This function builds a [`std::process::Command`] from the flow's
/// command builder, spawns a process, then waits for the output.
pub async fn spawn_then_wait_with_output(input: Result<Output, Command>) -> io::Result<ProcessIo> {
    let Err(command) = input else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing command"));
    };

    let mut command = TokioCommand::from(command);
    let output = command.output().await?;

    Ok(ProcessIo::SpawnThenWaitWithOutput(Ok(output)))
}

/// Converts a [`Command`] builder to a [`std::process::Command`].
impl From<Command> for TokioCommand {
    fn from(builder: Command) -> Self {
        let mut command = TokioCommand::new(&*builder.get_program());

        if let Some(args) = builder.get_args() {
            for arg in args {
                command.arg(&*arg);
            }
        }

        if let Some(envs) = builder.envs {
            for (key, val) in envs {
                command.env(key, val);
            }
        }

        if let Some(dir) = builder.current_dir {
            command.current_dir(dir);
        }

        if let Some(cfg) = builder.stdin {
            command.stdin(cfg);
        }

        if let Some(cfg) = builder.stdout {
            command.stdout(cfg);
        }

        if let Some(cfg) = builder.stderr {
            command.stderr(cfg);
        }

        command
    }
}
