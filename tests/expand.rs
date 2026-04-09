#![cfg(feature = "expand")]

use io_process::{
    command::Command,
    coroutines::spawn_out::{SpawnOut, SpawnOutResult},
    runtimes::std::handle,
};

fn echo() -> Command {
    let mut command = Command::new("echo");
    command.arg("-n").arg("$TEST").env("TEST", "expanded");
    command
}

#[test]
pub fn expand() {
    let _ = env_logger::try_init();

    let mut command = echo();
    command.expand = true;

    let mut spawn = SpawnOut::new(command);
    let mut arg = None;

    let (_, stdout, _) = loop {
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

    assert_eq!("expanded", String::from_utf8_lossy(&stdout));
}

#[test]
pub fn no_expand() {
    let _ = env_logger::try_init();

    let mut spawn = SpawnOut::new(echo());
    let mut arg = None;

    let (_, stdout, _) = loop {
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

    assert_eq!("$TEST", String::from_utf8_lossy(&stdout));
}
