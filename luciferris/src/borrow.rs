use log::{error, info};
use std::path::PathBuf;
use std::process::Command;

pub fn borrow(exe: PathBuf, exeargs: Option<Vec<String>>) {
    info!("executing {exe:?} with args {exeargs:?}");

    // set up the command with arguments
    let mut runexe = Command::new(exe);
    if let Some(args) = exeargs {
        runexe.args(args);
    }

    // log the output of the command
    match runexe.output() {
        Ok(msg) => info!("{msg:?}"),
        Err(e) => error!("{e:?}"),
    }
}

pub fn recent_config() {
    // TODO: dynamically update exe path
    let exe: PathBuf = PathBuf::from("./target/release/luciferris");
    let exeargs: Option<Vec<String>> = Some(vec![
        String::from("-dd"),              // verbose debugging
        String::from("-c ./recent.json"), // default name for recent config
        String::from("-o out.log"),       // where to save logging info to
        String::from("get-command"),      // mode
    ]);
    borrow(exe, exeargs);
}
