use config::Config;
use log::{info, warn};
use std::path::PathBuf;

pub fn borrow(exe: PathBuf) {
    info!("executing {exe:?}");
}
