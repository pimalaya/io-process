//! Process standard I/O stream configuration.

/// Configuration for a child process's standard I/O stream.
///
/// Used in [`Command`] to specify how stdin, stdout, and stderr
/// should behave when the process is spawned.
///
/// [`Command`]: crate::command::Command
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum Stdio {
    /// Inherit the stream from the parent process.
    #[default]
    Inherit,
    /// Redirect the stream to `/dev/null` (discard).
    Null,
    /// Create a new pipe for the stream.
    Piped,
}

#[cfg(feature = "std")]
impl From<Stdio> for std::process::Stdio {
    fn from(stdio: Stdio) -> Self {
        match stdio {
            Stdio::Inherit => std::process::Stdio::inherit(),
            Stdio::Null => std::process::Stdio::null(),
            Stdio::Piped => std::process::Stdio::piped(),
        }
    }
}
