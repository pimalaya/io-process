//! Example: pipe the output of one process into another (blocking).
//!
//! Run with:
//!
//! ```sh
//! cargo run --example std_spawn_pipeline --features std
//! ```

use io_process::{
    command::Command,
    coroutines::spawn_pipeline::{SpawnPipeline, SpawnPipelineResult},
    runtimes::std::handle,
};

fn main() {
    env_logger::init();

    let mut echo = Command::new("echo");
    echo.arg("hello world");

    let mut grep = Command::new("grep");
    grep.arg("world");

    println!("pipeline: {echo:#?} | {grep:#?}");
    println!();

    let mut arg = None;
    let mut spawn = SpawnPipeline::new([echo, grep]);

    let (status, stdout, stderr) = loop {
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

    println!("status: {status:#?}");
    println!("stdout: {}", String::from_utf8_lossy(&stdout));
    println!("stderr: {}", String::from_utf8_lossy(&stderr));
}
