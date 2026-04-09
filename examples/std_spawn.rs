//! Example: spawn a command and print its exit status (blocking).
//!
//! Run with:
//!
//! ```sh
//! cargo run --example std_spawn --features std
//! ```

use io_process::{
    command::Command,
    coroutines::spawn::{Spawn, SpawnResult},
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
    let mut spawn = Spawn::new(command);

    let status = loop {
        match spawn.resume(arg.take()) {
            SpawnResult::Ok { status } => break status,
            SpawnResult::Io { input } => arg = Some(handle(input).unwrap()),
            SpawnResult::Err { err } => panic!("{err}"),
        }
    };

    println!("exit status: {status:#?}");
}
