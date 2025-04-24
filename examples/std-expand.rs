#![cfg(feature = "std")]
#![cfg(feature = "expand")]

use io_process::{coroutines::SpawnThenWaitWithOutput, runtimes::std::handle, Command};

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

    loop {
        match spawn.resume(arg.take()) {
            Ok(output) => break println!("output: {output:#?}"),
            Err(io) => arg = Some(handle(io).unwrap()),
        }
    }
}
