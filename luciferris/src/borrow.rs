use config::Config;
use log::{error, info, warn};
use std::path::PathBuf;
use std::process::Command;

pub fn borrow(exe: PathBuf, exearg: Option<String>) {
    info!("executing {exe:?} with arg {exearg:?}");

    // set up the command with arguments
    let mut runexe = Command::new(exe);
    if let Some(arg) = exearg {
        runexe.arg(arg);
    }

    // log the output of the command
    match runexe.output() {
        Ok(msg) => info!("{msg:?}"),
        Err(e) => error!("{e:?}"),
    }
}
