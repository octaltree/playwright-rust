use playwright::*;
use std::{env, process::Command};

fn main() {
    let driver = Driver::install().unwrap();
    let envs = env::vars_os();
    let args = {
        let mut a = env::args_os();
        a.next();
        a
    };
    Command::new(driver.executable())
        .args(args)
        .envs(envs)
        .status()
        .unwrap();
}
