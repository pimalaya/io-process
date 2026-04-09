//! Example: spawn a command and print its exit status (async).
//!
//! Run with:
//!
//! ```sh
//! cargo run --example tokio_spawn --features tokio
//! ```

use io_process::{
    command::Command,
    coroutines::spawn::{Spawn, SpawnResult},
    runtimes::tokio::handle,
};
use tempfile::tempdir;

#[tokio::main]
async fn main() {
    env_logger::init();

    let workdir = tempdir().unwrap();

    let mut command = Command::new("touch");
    command.arg(workdir.path().join("file.tmp").to_string_lossy());

    println!("spawn: {command:#?}");
    println!();

    let mut arg = None;
    let mut spawn = Spawn::new(command);

    let status = loop {
        match spawn.resume(arg.take()) {
            SpawnResult::Ok { status } => break status,
            SpawnResult::Io { input } => arg = Some(handle(input).await.unwrap()),
            SpawnResult::Err { err } => panic!("{err}"),
        }
    };

    println!("exit status: {status:#?}");
}
