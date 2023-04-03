use log::{error, info};
// use ssh::Session; // Currently causing linker error

type Url = String;

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
    url: Url,
}

impl SSHConn {
    fn new(url: Url) -> Self {
        Self { url }
    }

    fn snd_cmd(&mut self, _cmd: Command) -> Result<(), &str> {
        let _dst: Url = self.url.clone();
        Ok(())
    }
}

impl From<String> for SSHConn {
    fn from(value: String) -> Self {
        SSHConn::new(value)
    }
}

pub fn spread() {
    let targets: Vec<String> = vec![]; // TODO
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

fn ssh_to_url(dst: Url) -> Result<SSHConn, &'static str> {
    let conn = SSHConn::from(dst);
    // let mut session = Session::new().unwrap();
    // session.set_host(dst).unwrap();
    // session.parse_config(None).unwrap();
    // session.connect().unwrap();
    Ok(conn)
}
