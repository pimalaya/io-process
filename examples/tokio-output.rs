#![cfg(feature = "tokio")]

use io_process::{coroutines::SpawnThenWaitWithOutput, runtimes::tokio::handle, Command};

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut command = Command::new("echo");
    command.arg("hello");
    command.arg("world");
    println!("spawn: {command:#?}");
    println!();

    let mut arg = None;
    let mut spawn = SpawnThenWaitWithOutput::new(command);

    loop {
        match spawn.resume(arg.take()) {
            Ok(output) => break println!("output: {output:#?}"),
            Err(io) => arg = Some(handle(io).await.unwrap()),
        }
    }
}
