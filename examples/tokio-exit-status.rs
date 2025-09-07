#![cfg(feature = "tokio")]

use io_process::{
    command::Command,
    coroutines::spawn_then_wait::{SpawnThenWait, SpawnThenWaitResult},
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
    let mut spawn = SpawnThenWait::new(command);

    let status = loop {
        match spawn.resume(arg.take()) {
            SpawnThenWaitResult::Ok(output) => break output,
            SpawnThenWaitResult::Io(io) => arg = Some(handle(io).await.unwrap()),
            SpawnThenWaitResult::Err(err) => panic!("{err}"),
        }
    };

    println!("exit status: {status:#?}")
}
