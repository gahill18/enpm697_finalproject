use qscan::qscanner::QScanner;
use shodan_client::*;

pub fn spread() {
    println!("spread mode");

    let mut scanner = QScanner::new("127.0.0.1", "80");
    let lst = scanner.scan_tcp_connect();

    async {
        for port in lst.await {
            println!("{port:?}")
        }
    };
}
