//! Module dedicated to the standard, blocking runtime.

use std::{
    io,
    process::{Command as StdCommand, Output},
};

use crate::{Command, Io, SpawnOutput};

/// The main runtime I/O handler.
///
/// This handler makes use of the standard module [`std::process`] to
/// spawn processes and wait for exit status or output.
pub fn handle(io: Io) -> io::Result<Io> {
    match io {
        Io::Error(err) => Err(io::Error::new(io::ErrorKind::Other, err)),
        Io::SpawnThenWait(io) => spawn_then_wait(io),
        Io::SpawnThenWaitWithOutput(io) => spawn_then_wait_with_output(io),
    }
}

/// Spawns a process then wait for its child's exit status.
///
/// This function builds a [`std::process::Command`] from the flow's
/// command builder, spawns a process, collects std{in,out,err} then
/// waits for the exit status.
pub fn spawn_then_wait(input: Result<SpawnOutput, Command>) -> io::Result<Io> {
    let Err(command) = input else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing command"));
    };

    let mut command = StdCommand::from(command);
    let mut child = command.spawn()?;

    let stdin = child.stdin.take();
    let stdout = child.stdout.take();
    let stderr = child.stderr.take();

    let output = SpawnOutput {
        status: child.wait()?,
        stdin: stdin.map(Into::into),
        stdout: stdout.map(Into::into),
        stderr: stderr.map(Into::into),
    };

    Ok(Io::SpawnThenWait(Ok(output)))
}

/// Spawns a process then wait for its child's output.
///
/// This function builds a [`std::process::Command`] from the flow's
/// command builder, spawns a process, then waits for the output.
pub fn spawn_then_wait_with_output(input: Result<Output, Command>) -> io::Result<Io> {
    let Err(command) = input else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing command"));
    };

    let mut command = StdCommand::from(command);
    let output = command.output()?;

    Ok(Io::SpawnThenWaitWithOutput(Ok(output)))
}

/// Converts a [`Command`] builder to a [`std::process::Command`].
impl From<Command> for StdCommand {
    fn from(builder: Command) -> Self {
        let mut command = StdCommand::new(&*builder.get_program());

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
