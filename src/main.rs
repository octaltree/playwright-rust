#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use playwright::*;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use std::{env, io, process};

fn main() {
    let envs = env::vars_os();
    let args = {
        let mut a = env::args_os();
        a.next();
        a
    };
    let status = run(args, envs).unwrap();
    if let Some(status) = status.code() {
        std::process::exit(status)
    }
}

fn run(args: env::ArgsOs, envs: env::VarsOs) -> io::Result<process::ExitStatus> {
    let driver = Driver::install().unwrap();
    let mut command = process::Command::new(driver.executable());
    let child = command.args(args).envs(envs);

    #[cfg(target_os = "windows")]
    child.creation_flags(0x08000000);
    child.status()
}
