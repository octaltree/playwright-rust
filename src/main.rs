use playwright::*;
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
    process::Command::new(driver.executable())
        .args(args)
        .envs(envs)
        .status()
}
