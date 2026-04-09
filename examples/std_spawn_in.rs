//! Example: spawn a command with bytes piped to its stdin (blocking).
//!
//! Run with:
//!
//! ```sh
//! cargo run --example std_spawn_in --features std
//! ```

use io_process::{
    command::Command,
    coroutines::spawn_in::{SpawnIn, SpawnInResult},
    runtimes::std::handle,
};

fn main() {
    env_logger::init();

    let command = Command::new("cat");
    let stdin = b"hello from stdin\n".to_vec();

    println!("spawn: {command:#?}");
    println!();

    let mut arg = None;
    let mut spawn = SpawnIn::new(command, stdin);

    let status = loop {
        match spawn.resume(arg.take()) {
            SpawnInResult::Ok { status } => break status,
            SpawnInResult::Io { input } => arg = Some(handle(input).unwrap()),
            SpawnInResult::Err { err } => panic!("{err}"),
        }
    };

    println!("exit status: {status:#?}");
}
