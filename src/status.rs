//! Process exit status.

/// The exit status of a spawned process.
///
/// Wraps the raw exit code returned by the OS. A `None` code
/// typically means the process was terminated by a signal (Unix)
/// or did not exit normally.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExitStatus {
    code: Option<i32>,
}

impl ExitStatus {
    /// Creates an [`ExitStatus`] from a raw exit code.
    pub fn new(code: Option<i32>) -> Self {
        Self { code }
    }

    /// Returns `true` if the process exited with a zero exit code.
    pub fn success(&self) -> bool {
        self.code == Some(0)
    }

    /// Returns the raw exit code, or `None` if the process was
    /// terminated by a signal.
    pub fn code(&self) -> Option<i32> {
        self.code
    }
}
