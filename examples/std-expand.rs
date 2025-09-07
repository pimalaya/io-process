#![cfg(feature = "std")]
#![cfg(feature = "expand")]

use io_process::{
    command::Command,
    coroutines::spawn_then_wait_with_output::{
        SpawnThenWaitWithOutput, SpawnThenWaitWithOutputResult,
    },
    runtimes::std::handle,
};

fn main() {
    env_logger::init();

    let mut command = Command::new("echo");
    command.expand = true;
    command.env("HELLO", "world");
    command.arg("-n").arg("$USER").arg("$HELLO").arg("~");

    println!("spawn: {command:#?}");
    println!();

    let mut arg = None;
    let mut spawn = SpawnThenWaitWithOutput::new(command);

    let output = loop {
        match spawn.resume(arg.take()) {
            SpawnThenWaitWithOutputResult::Ok(output) => break output,
            SpawnThenWaitWithOutputResult::Io(io) => arg = Some(handle(io).unwrap()),
            SpawnThenWaitWithOutputResult::Err(err) => panic!("{err}"),
        }
    };

    println!("output: {output:#?}")
}
