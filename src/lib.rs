use telnet::Telnet;
use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

pub struct Connection {
    pub ip: String,
    pub connection: Telnet,
}

pub struct Control {}

pub struct Hosts {

}

impl Hosts {
    pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
}

impl Connection {
    pub fn connect(ip: &str) -> Connection {
        let mut conn_mut = Connection {
            ip: String::from(ip),
            connection: Telnet::connect((ip, 9761), 256).expect("error establishing connection"),
        };
        conn_mut
    }
}

impl Control {
    pub const ON: bool = true;
    pub const OFF: bool = false;
    pub fn power(mut connection: Telnet, p: bool) {
        let buffer: &[u8] = if p {
            b"POWER on"
        }else {
            b"POWER off"
        };
        connection.write(buffer).expect("Write Error");
    }
}
