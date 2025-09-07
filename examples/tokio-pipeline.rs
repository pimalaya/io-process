#![cfg(feature = "tokio")]

use std::process::Stdio;

use io_process::{
    command::Command,
    coroutines::{
        spawn_then_wait::{SpawnThenWait, SpawnThenWaitResult},
        spawn_then_wait_with_output::{SpawnThenWaitWithOutput, SpawnThenWaitWithOutputResult},
    },
    runtimes::tokio::handle,
};

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut command = Command::new("/bin/sh");
    command.arg("-c");
    command.arg("read line; echo $line");
    command.stdin(Stdio::inherit());
    command.stdout(Stdio::piped());
    println!("spawn 1: {command:#?}");
    println!();
    println!("What is your name? ");

    let mut arg = None;
    let mut spawn = SpawnThenWait::new(command);

    let (status, stdout) = loop {
        match spawn.resume(arg.take()) {
            SpawnThenWaitResult::Ok(output) => break (output.status, output.stdout.unwrap()),
            SpawnThenWaitResult::Io(io) => arg = Some(handle(io).await.unwrap()),
            SpawnThenWaitResult::Err(err) => panic!("{err}"),
        }
    };

    println!();
    println!("status: {status:#?}");
    println!();

    let mut command = Command::new("cat");
    command.arg("-E");
    command.stdin(stdout);
    println!("command 2: {command:#?}");
    println!();

    let mut arg = None;
    let mut spawn = SpawnThenWaitWithOutput::new(command);

    let output = loop {
        match spawn.resume(arg.take()) {
            SpawnThenWaitWithOutputResult::Ok(output) => break output,
            SpawnThenWaitWithOutputResult::Io(io) => arg = Some(handle(io).await.unwrap()),
            SpawnThenWaitWithOutputResult::Err(err) => panic!("{err}"),
        }
    };

    println!("output: {output:#?}")
}
