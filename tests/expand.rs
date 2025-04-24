use io_process::{coroutines::SpawnThenWaitWithOutput, runtimes::std::handle, Command};

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

    let mut spawn = SpawnThenWaitWithOutput::new(command);
    let mut arg = None;

    let output = loop {
        match spawn.resume(arg.take()) {
            Ok(output) => break output,
            Err(io) => arg = Some(handle(io).unwrap()),
        }
    };

    assert_eq!("expanded", String::from_utf8_lossy(&output.stdout));
}

#[test]
pub fn no_expand() {
    let _ = env_logger::try_init();

    let mut spawn = SpawnThenWaitWithOutput::new(echo());
    let mut arg = None;

    let output = loop {
        match spawn.resume(arg.take()) {
            Ok(output) => break output,
            Err(io) => arg = Some(handle(io).unwrap()),
        }
    };

    assert_eq!("$TEST", String::from_utf8_lossy(&output.stdout));
}
