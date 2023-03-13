use std::path::PathBuf;

use clap::{Parser, Subcommand};
use config::{Config, ConfigError, File, FileFormat};
use env_logger::{fmt::Target, Builder};
use log::{debug, error, info, log_enabled, LevelFilter};

mod borrow;
mod ransom;
mod snoop;
mod spread;

use borrow::borrow;
use ransom::ransom;
use snoop::snoop;
use spread::spread;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<String>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    mode: Option<Modes>,
}

#[derive(Subcommand)]
enum Modes {
    Borrow, // Utilize system resources
    Ransom, // Hold the infected system for ransom
    Snoop,  // Gather information without being detected
    Spread, // Spread the running program to other devices
}

fn read_config(path: &str) -> Result<Config, ConfigError> {
    let builder = Config::builder()
        .set_default("default", "1")?
        .add_source(File::new(path, FileFormat::Json))
        .set_override("override", "1")?;

    builder.build()
}

fn main() {
    let cli = Cli::parse();

    // Show the disable mode
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    // Check if the user provided a config path
    if let Some(config_path) = cli.config.as_deref() {
        println!("{config_path:?}");
        let conf: Config = read_config(config_path).unwrap();
        println!("{conf:?}")
    }

    // check if the user specified a mode
    if let Some(mode) = cli.mode {
        match mode {
            Modes::Borrow => borrow(),
            Modes::Ransom => ransom(),
            Modes::Snoop => snoop(Some(PathBuf::from("./sys.json"))),
            Modes::Spread => spread(),
            _ => unreachable!(), // panics if code becomes not unreachable
        }
    };

    info!("finished!");
}
