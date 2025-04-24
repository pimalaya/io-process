use log::{debug, trace};

use crate::{Command, Io, SpawnOutput};

/// The I/O-free coroutine for spawning a process then waiting for its
/// child's exit status.
///
/// This coroutine should be used when you do not care about the
/// output, or when you need the output to be piped into another
/// process.
///
/// If you need to collect the output, have a look at
/// [`super::SpawnThenWaitWithOutput`].
#[derive(Debug)]
pub struct SpawnThenWait {
    cmd: Option<Command>,
}

impl SpawnThenWait {
    /// Creates a new coroutine from the given command builder.
    pub fn new(command: Command) -> SpawnThenWait {
        trace!("prepare command to be spawned: {command:?}");
        let cmd = Some(command);
        Self { cmd }
    }

    /// Makes the coroutine progress.
    pub fn resume(&mut self, arg: Option<Io>) -> Result<SpawnOutput, Io> {
        let Some(arg) = arg else {
            let Some(cmd) = self.cmd.take() else {
                return Err(Io::err("Command not initialized"));
            };

            trace!("break: need I/O to spawn command");
            return Err(Io::SpawnThenWait(Err(cmd)));
        };

        trace!("resume after spawning command");

        let Io::SpawnThenWait(io) = arg else {
            let err = format!("Expected spawn output, got {arg:?}");
            return Err(Io::err(err));
        };

        let output = match io {
            Ok(output) => output,
            Err(cmd) => return Err(Io::SpawnThenWait(Err(cmd))),
        };

        debug!("spawned command: {:?}", output.status);
        Ok(output)
    }
}
