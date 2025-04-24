#![cfg(feature = "tokio")]

use io_process::{coroutines::SpawnThenWait, runtimes::tokio::handle, Command};
use tempdir::TempDir;

#[tokio::main]
async fn main() {
    env_logger::init();

    let workdir = TempDir::new("std-exit-status").unwrap();

    let mut command = Command::new("touch");
    command.arg(workdir.path().join("file.tmp"));

    println!("spawn: {command:#?}");
    println!();

    let mut arg = None;
    let mut spawn = SpawnThenWait::new(command);

    loop {
        match spawn.resume(arg.take()) {
            Ok(status) => break println!("exit status: {status:#?}"),
            Err(io) => arg = Some(handle(io).await.unwrap()),
        }
    }

    workdir.close().unwrap();
}
