use config::{Config, ConfigError, File, FileFormat};

fn read_config(path: &str) -> Result<Config, ConfigError> {
    let mut builder = Config::builder()
        .set_default("default", "1")?
        .add_source(File::new(path, FileFormat::Json))
        //  .add_async_source(...)
        .set_override("override", "1")?;

    builder.build()
}

fn main() {
    let conf: Config = read_config("test.json").unwrap();
    println!("{conf:?}");
}
