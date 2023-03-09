use std::collections::HashMap;

use sysinfo::{
    Component, Disk, NetworkData, NetworkExt, Networks, NetworksExt, Pid, Process, ProcessExt,
    System, SystemExt,
};

#[derive(Debug)]
struct Status {
    disks: Vec<Disk>,
    kernel: Option<String>,
    os: Option<String>,
    procs: HashMap<Pid, String>,
}

impl From<System> for Status {
    fn from(value: System) -> Self {
        let mut disks = vec![];
        for disk in value.disks() {}

        let kernel = value.kernel_version();
        let os = value.os_version();

        let mut procs = HashMap::new();
        for ps in value.processes().to_owned() {
            let (k, v): (Pid, String) = (ps.0.to_owned(), String::from(ps.1.name()));
            procs.insert(k, v);
        }

        Self {
            disks,
            kernel,
            os,
            procs,
        }
    }
}

pub fn snoop() -> () {
    println!("snoop mode");
    let mut sys = System::new_all();
    sys.refresh_all();

    let status = Status::from(sys);
    println!("{:?}", status.procs);
}
