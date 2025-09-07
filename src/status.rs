//! Module dedicated to spawn status.

use std::process::{ExitStatus, Stdio};

/// The process spawn exit status.
///
/// Wrapper containing the standard exit status, stdin, stdout and
/// stderr of a spawned command.
#[derive(Debug)]
pub struct SpawnStatus {
    pub status: ExitStatus,
    pub stdin: Option<Stdio>,
    pub stdout: Option<Stdio>,
    pub stderr: Option<Stdio>,
}
