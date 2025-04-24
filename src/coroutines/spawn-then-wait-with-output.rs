use std::process::Output;

use log::{debug, trace};

use crate::{Command, Io};

/// The I/O-free coroutine for spawning a process then waiting for its
/// child's output.
///
/// This coroutine should be used when you need to collect the child
/// process' output, from stdout and stderr.
///
/// If you do not need to collect the output, or if you need to pipe
/// the output to another process, see [`super::SpawnThenWait`].
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
    pub fn resume(&mut self, arg: Option<Io>) -> Result<Output, Io> {
        let Some(arg) = arg else {
            let Some(cmd) = self.cmd.take() else {
                return Err(Io::err("Command not initialized"));
            };

            trace!("break: need I/O to spawn command");
            return Err(Io::SpawnThenWaitWithOutput(Err(cmd)));
        };

        trace!("resume after spawning command");

        let Io::SpawnThenWaitWithOutput(io) = arg else {
            let err = format!("Expected spawn output, got {arg:?}");
            return Err(Io::err(err));
        };

        let output = match io {
            Ok(output) => output,
            Err(cmd) => return Err(Io::SpawnThenWaitWithOutput(Err(cmd))),
        };

        debug!("spawned command: {:?}", output.status);
        Ok(output)
    }
}
