#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! yansi = "0.5"
//! ```
extern crate yansi;
use std::process::Command;

macro_rules! run_command {
    ($cmd:expr , $($arg:expr),*) => (
        let mut command = command!($cmd, $($arg),*);
        let mut child = command.spawn().unwrap();
        let status = child.wait().unwrap();
        if !status.success() {
            print!("> {}",yansi::Paint::red("qualify terminates due to error"));
            std::process::exit(-1);
        }
    )
}

macro_rules! command {
    ($cmd:expr , $($arg:expr),*) => (
        {
            print!("\n> {}",yansi::Paint::yellow($cmd));
            let mut command = Command::new($cmd);
            $(
                print!(" {}",yansi::Paint::yellow(&$arg));
                command.arg($arg);
            )*
            print!("\n");
            command
        }
    )
}

// fn run_script(s: &str) {
//     let mut path = std::path::PathBuf::from(std::env::var("CARGO_SCRIPT_BASE_PATH").unwrap());
//     path.push(s);
//     let script = path.to_string_lossy().to_owned().to_string();
//     run_command!("cargo", "script", script);
// }

fn main() {
    // format
    run_command!("cargo", "fmt");

    // Build in important variants
    run_command!("cargo", "build");
    run_command!("cargo", "build", "--all-features");
    run_command!("cargo", "build", "--release");

    // Clippy
    run_command!("cargo", "clippy");

    // Run tests in important variants
    run_command!("cargo", "test", "--release");
    run_command!("cargo", "test");
    run_command!("cargo", "test", "--all-features");

    // doc
    run_command!("cargo", "doc", "--all-features", "--no-deps", "--open");

    // check git status
    let mut cmd = command!("git", "status", "-s");
    let child = cmd.stdout(std::process::Stdio::piped()).spawn().unwrap();
    let output = child.wait_with_output().unwrap();
    if output.stdout.len() > 0 {
        print!("> {}", yansi::Paint::red("there are unsubmitted files"));
        std::process::exit(-1);
    }

    // say goodbye
    println!("\n> all done :-)  Looks like you're ready to \"cargo publish\"?");
}
