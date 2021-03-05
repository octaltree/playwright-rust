use playwright::*;
// use std::{env, process::Command};

fn main() {
    env_logger::init();
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let p = Playwright::initialize().await.unwrap(); // if drop all resources are disposed
                                                             // p.prepare().unwrap(); // install browsers
        });
    //    let driver = Driver::install().unwrap();
    //    let envs = {
    //        // my_env.pop("NODE_OPTIONS", None)
    //        // my_env["PW_CLI_TARGET_LANG"] = "python"
    //        env::vars_os()
    //    };
    //    let args = {
    //        let mut a = env::args_os();
    //        a.next();
    //        a
    //    };
    //    Command::new(driver.executable())
    //        .args(args)
    //        .envs(envs)
    //        .status()
    //        .unwrap();
}
