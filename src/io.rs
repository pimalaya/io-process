use std::{fmt, process::Output};

use crate::{Command, SpawnOutput};

#[derive(Debug)]
pub enum Io {
    Error(String),

    /// I/O for spawning a process and waiting for its exit status.
    SpawnThenWait(Result<SpawnOutput, Command>),

    /// I/O for spawning a process and waiting for its exit status and
    /// any potential output from stdout or stderr.
    SpawnThenWaitWithOutput(Result<Output, Command>),
}

impl Io {
    pub fn err(message: impl fmt::Display) -> Io {
        let message = format!("Process error: {message}");
        Io::Error(message.to_string())
    }
}
