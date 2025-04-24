use std::process::{ExitStatus, Stdio};

#[derive(Debug)]
pub struct SpawnOutput {
    pub status: ExitStatus,
    pub stdin: Option<Stdio>,
    pub stdout: Option<Stdio>,
    pub stderr: Option<Stdio>,
}
