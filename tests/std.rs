#![cfg(feature = "std")]

use io_process::{
    command::Command,
    coroutines::{
        spawn::{Spawn, SpawnResult},
        spawn_in::{SpawnIn, SpawnInResult},
        spawn_out::{SpawnOut, SpawnOutResult},
        spawn_pipeline::{SpawnPipeline, SpawnPipelineResult},
    },
    runtimes::std::handle,
};

#[test]
fn spawn() {
    let _ = env_logger::try_init();

    let mut arg = None;
    let mut spawn = Spawn::new(Command::new("true"));

    let status = loop {
        match spawn.resume(arg.take()) {
            SpawnResult::Ok { status } => break status,
            SpawnResult::Io { input } => arg = Some(handle(input).unwrap()),
            SpawnResult::Err { err } => panic!("{err}"),
        }
    };

    assert!(status.success());
}

#[test]
fn spawn_out() {
    let _ = env_logger::try_init();

    let mut command = Command::new("echo");
    command.arg("hello");

    let mut arg = None;
    let mut spawn = SpawnOut::new(command);

    let (status, stdout, _stderr) = loop {
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

    assert!(status.success());
    assert_eq!("hello\n", String::from_utf8_lossy(&stdout));
}

#[test]
fn spawn_in() {
    let _ = env_logger::try_init();

    let command = Command::new("cat");
    let stdin = b"hello\n".to_vec();

    let mut arg = None;
    let mut spawn = SpawnIn::new(command, stdin);

    let status = loop {
        match spawn.resume(arg.take()) {
            SpawnInResult::Ok { status } => break status,
            SpawnInResult::Io { input } => arg = Some(handle(input).unwrap()),
            SpawnInResult::Err { err } => panic!("{err}"),
        }
    };

    assert!(status.success());
}

#[test]
fn spawn_pipeline() {
    let _ = env_logger::try_init();

    let mut echo = Command::new("echo");
    echo.arg("hello world");

    let mut grep = Command::new("grep");
    grep.arg("world");

    let mut arg = None;
    let mut spawn = SpawnPipeline::new([echo, grep]);

    let (status, stdout, _stderr) = loop {
        match spawn.resume(arg.take()) {
            SpawnPipelineResult::Ok {
                status,
                stdout,
                stderr,
            } => break (status, stdout, stderr),
            SpawnPipelineResult::Io { input } => arg = Some(handle(input).unwrap()),
            SpawnPipelineResult::Err { err } => panic!("{err}"),
        }
    };

    assert!(status.success());
    assert_eq!("hello world\n", String::from_utf8_lossy(&stdout));
}
