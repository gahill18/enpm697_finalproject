use qscan::qscanner::QScanner;
use ssh::*;

type URL = String;

pub fn spread() {
    let targets: Vec<String> = Vec::new();
}

fn ssh_to_url(dst: &URL) -> Result<(), &'static str> {
    let mut session = Session::new().unwrap();
    session.set_host(dst).unwrap();
    session.parse_config(None).unwrap();
    session.connect().unwrap();
    Ok(())
}
