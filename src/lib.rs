use telnet::Telnet;

pub struct Connection {
    pub ip: String,
    pub connection: Telnet,
}

pub struct Control {}


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
