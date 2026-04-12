//! Synchronous process runtime backed by [`std::process`].

use std::{
    io::{self, Write},
    process::{Command as StdCommand, Stdio as StdStdio},
};

use crate::{
    command::Command,
    io::{ProcessInput, ProcessOutput},
    status::ExitStatus,
    stdio::Stdio,
};

/// Processes a [`ProcessInput`] request synchronously using
/// [`std::process`].
pub fn handle(input: ProcessInput) -> io::Result<ProcessOutput> {
    match input {
        ProcessInput::Spawn { cmd } => spawn(cmd),
        ProcessInput::SpawnOut { cmd } => spawn_out(cmd),
        ProcessInput::SpawnIn { cmd, stdin } => spawn_in(cmd, stdin),
        ProcessInput::SpawnPipeline { cmds } => spawn_pipeline(cmds),
    }
}

/// Spawns a process and waits for its exit status.
pub fn spawn(cmd: Command) -> io::Result<ProcessOutput> {
    let mut command = StdCommand::from(cmd);
    let status = command.status()?;

    Ok(ProcessOutput::Spawned {
        status: ExitStatus::new(status.code()),
    })
}

/// Spawns a process, captures its stdout and stderr, and waits for
/// its exit status.
///
/// Overrides the command's stdout and stderr to [`StdStdio::piped`]
/// regardless of the [`Stdio`] configuration on the command.
pub fn spawn_out(cmd: Command) -> io::Result<ProcessOutput> {
    let mut command = StdCommand::from(cmd);
    command.stdout(StdStdio::piped());
    command.stderr(StdStdio::piped());

    let child = command.spawn()?;
    let output = child.wait_with_output()?;

    Ok(ProcessOutput::SpawnedOut {
        status: ExitStatus::new(output.status.code()),
        stdout: output.stdout,
        stderr: output.stderr,
    })
}

/// Spawns a process, feeds bytes to its stdin, and waits for its exit
/// status.
///
/// Overrides the command's stdin to [`StdStdio::piped`] regardless of
/// the [`Stdio`] configuration on the command.
pub fn spawn_in(cmd: Command, stdin: Vec<u8>) -> io::Result<ProcessOutput> {
    let mut command = StdCommand::from(cmd);
    command.stdin(StdStdio::piped());

    let mut child = command.spawn()?;

    if let Some(mut handle) = child.stdin.take() {
        handle.write_all(&stdin)?;
    }

    let status = child.wait()?;

    Ok(ProcessOutput::SpawnedIn {
        status: ExitStatus::new(status.code()),
    })
}

/// Spawns a pipeline of processes, piping each process's stdout into
/// the next process's stdin.
///
/// Returns the last process's exit status, stdout, and stderr.
pub fn spawn_pipeline(cmds: Vec<Command>) -> io::Result<ProcessOutput> {
    let n = cmds.len();
    if n == 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "empty pipeline",
        ));
    }

    let mut prev_stdout: Option<std::process::ChildStdout> = None;
    let mut early_children: Vec<std::process::Child> = Vec::new();
    let mut last_child = None;

    for (i, cmd) in cmds.into_iter().enumerate() {
        let is_last = i == n - 1;
        let mut command = StdCommand::from(cmd);

        if let Some(stdout) = prev_stdout.take() {
            command.stdin(stdout);
        }

        command.stdout(StdStdio::piped());
        if is_last {
            command.stderr(StdStdio::piped());
        }

        let mut child = command.spawn()?;

        if is_last {
            last_child = Some(child);
        } else {
            prev_stdout = child.stdout.take();
            early_children.push(child);
        }
    }

    let output = last_child.unwrap().wait_with_output()?;

    for mut child in early_children {
        let _ = child.wait();
    }

    Ok(ProcessOutput::SpawnedPipeline {
        status: ExitStatus::new(output.status.code()),
        stdout: output.stdout,
        stderr: output.stderr,
    })
}

/// Converts a [`Command`] builder into a [`std::process::Command`].
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
            command.current_dir(&dir);
        }
        match builder.stdin {
            Some(Stdio::Inherit) => {
                command.stdin(StdStdio::inherit());
            }
            Some(Stdio::Null) => {
                command.stdin(StdStdio::null());
            }
            Some(Stdio::Piped) => {
                command.stdin(StdStdio::piped());
            }
            None => (),
        };

        match builder.stdout {
            Some(Stdio::Inherit) => {
                command.stdout(StdStdio::inherit());
            }
            Some(Stdio::Null) => {
                command.stdout(StdStdio::null());
            }
            Some(Stdio::Piped) => {
                command.stdout(StdStdio::piped());
            }
            None => (),
        };

        match builder.stderr {
            Some(Stdio::Inherit) => {
                command.stderr(StdStdio::inherit());
            }
            Some(Stdio::Null) => {
                command.stderr(StdStdio::null());
            }
            Some(Stdio::Piped) => {
                command.stderr(StdStdio::piped());
            }
            None => (),
        };

        command
    }
}
