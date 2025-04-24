#![cfg(feature = "std")]

use io_process::{coroutines::SpawnThenWait, runtimes::std::handle, Command};
use tempdir::TempDir;

fn main() {
    env_logger::init();

    let workdir = TempDir::new("std-exit-status").unwrap();

    let mut command = Command::new("touch");
    command.arg(workdir.path().join("file.tmp").to_string_lossy());

    println!("spawn: {command:#?}");
    println!();

    let mut arg = None;
    let mut spawn = SpawnThenWait::new(command);

    loop {
        match spawn.resume(arg.take()) {
            Ok(status) => break println!("exit status: {status:#?}"),
            Err(io) => arg = Some(handle(io).unwrap()),
        }
    }

    workdir.close().unwrap();
}
