//! Example: spawn a command and collect its output (blocking).
//!
//! Run with:
//!
//! ```sh
//! cargo run --example std_spawn_out --features std
//! ```

use io_process::{
    command::Command,
    coroutines::spawn_out::{SpawnOut, SpawnOutResult},
    runtimes::std::handle,
};

fn main() {
    env_logger::init();

    let mut command = Command::new("echo");
    command.arg("hello");
    command.arg("world");

    println!("spawn: {command:#?}");
    println!();

    let mut arg = None;
    let mut spawn = SpawnOut::new(command);

    let (status, stdout, stderr) = loop {
        match spawn.resume(arg.take()) {
            SpawnOutResult::Ok {
                status,
                stdout,
                stderr,
            } => break (status, stdout, stderr),
            SpawnOutResult::Io { input } => arg = Some(handle(input).unwrap()),
            SpawnOutResult::Err { err } => panic!("{err}"),
        }
    };

    println!("status: {status:#?}");
    println!("stdout: {}", String::from_utf8_lossy(&stdout));
    println!("stderr: {}", String::from_utf8_lossy(&stderr));
}
