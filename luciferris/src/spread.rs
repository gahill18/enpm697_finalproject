use log::{error, info};
// use ssh::Session; // Currently causing linker error

type URL = String;

struct Command;

impl Command {
    fn new() -> Self {
        Command {}
    }
}

impl Default for Command {
    fn default() -> Self {
        Command::new()
    }
}

struct SSHConn {
    url: URL,
}

impl SSHConn {
    fn new(url: URL) -> Self {
        Self { url }
    }

    fn snd_cmd(&mut self, cmd: Command) -> Result<(), &str> {
        Ok(())
    }
}

impl From<String> for SSHConn {
    fn from(value: String) -> Self {
        SSHConn::new(value)
    }
}

pub fn spread() {
    let targets: Vec<String> = todo!();
    for target in targets {
        match ssh_to_url(target) {
            Ok(mut conn) => match conn.snd_cmd(Command::default()) {
                Ok(result) => info!("{result:?}"),
                Err(e) => error!("{e:?}"),
            },
            Err(e) => error!("{e:?}"),
        }
    }
}

fn ssh_to_url(dst: URL) -> Result<SSHConn, &'static str> {
    let conn = SSHConn::from(dst);
    // let mut session = Session::new().unwrap();
    // session.set_host(dst).unwrap();
    // session.parse_config(None).unwrap();
    // session.connect().unwrap();
    Ok(conn)
}
