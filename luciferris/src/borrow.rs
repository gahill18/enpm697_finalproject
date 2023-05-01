use log::{error, info};
use std::path::PathBuf;
use std::process::Command;

pub fn borrow(exe: PathBuf, exeargs: Option<Vec<String>>) {
    info!(
        "executing {exe:?} with args {exeargs:?} in pwd {:?}",
        std::env::current_dir()
    );

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
