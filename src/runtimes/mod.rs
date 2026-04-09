//! Collection of process runtimes.
//!
//! A runtime contains all the I/O logic, and is responsible for
//! processing [`ProcessInput`] requests emitted by [coroutines] and
//! returning the corresponding [`ProcessOutput`].
//!
//! If you miss a runtime matching your requirements, you can easily
//! implement your own by taking example on the existing ones. PRs are
//! welcomed!
//!
//! [`ProcessInput`]: crate::io::ProcessInput
//! [`ProcessOutput`]: crate::io::ProcessOutput
//! [coroutines]: crate::coroutines

#[cfg(feature = "std")]
pub mod std;
#[cfg(feature = "tokio")]
pub mod tokio;
