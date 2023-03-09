use std::{ops::Deref, path::PathBuf};

use clap::{Parser, Subcommand};
use config::{Config, ConfigError, File, FileFormat};

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

#[derive(Subcommand, Debug)]
enum Modes {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
    Snoop,
    Encrypt,
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
    // Check if the user provided a config path
    if let Some(config_path) = cli.config.as_deref() {
        println!("{config_path:?}");
        let conf: Config = read_config(config_path).unwrap();
        println!("{conf:?}")
    }

    // check if the user specified a mode
    if let Some(mode) = cli.mode {
        match mode {
            Modes::Snoop => println!("snoop mode"),
            Modes::Encrypt => println!("encrypt mode"),
            _ => println!("unknown mode {mode:?}"),
        }
    };
}
