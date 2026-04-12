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
