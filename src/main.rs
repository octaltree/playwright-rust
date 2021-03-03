use playwright::*;
use std::{env, process::Command};

fn main() {
    let dir = default_driver_dest();
    let driver = Driver::try_new(&dir).unwrap();
    let envs = {
        // my_env.pop("NODE_OPTIONS", None)
        // my_env["PW_CLI_TARGET_LANG"] = "python"
        env::vars_os()
    };
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
