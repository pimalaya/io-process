//! Example: spawn a command and print its exit status (blocking).
//!
//! Run with:
//!
//! ```sh
//! cargo run --example std_spawn --features std
//! ```

use io_process::{
    command::Command,
    coroutines::spawn::{ProcessSpawn, ProcessSpawnResult},
    runtimes::std::handle,
};
use tempfile::tempdir;

fn main() {
    env_logger::init();

    let workdir = tempdir().unwrap();

    let mut command = Command::new("touch");
    command.arg(workdir.path().join("file.tmp").to_string_lossy());

    println!("spawn: {command:#?}");
    println!();

    let mut arg = None;
    let mut spawn = ProcessSpawn::new(command);

    let status = loop {
        match spawn.resume(arg.take()) {
            ProcessSpawnResult::Ok { status } => break status,
            ProcessSpawnResult::Io { input } => arg = Some(handle(input).unwrap()),
            ProcessSpawnResult::Err { err } => panic!("{err}"),
        }
    };

    println!("exit status: {status:#?}");
}
