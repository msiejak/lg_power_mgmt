use std::env;
use lg_power_mgmt::Connection;
use lg_power_mgmt::Control;
use lg_power_mgmt::Hosts;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("options are:\n\npoweron\nshutdown")
    }
    match &args[1][..] {
        "poweron" => run(Control::ON),
        "shutdown" => run(Control::OFF),
        _ => println!("options are:\n\npoweron\nshutdown"),
    };
}

fn run(power: bool) {
    if let lines = Hosts::read_lines("/etc/lg_power_mgmt/displays").expect("error reading file") {
        for line in lines {
            if let ip = line.expect("error in file") {
                println!("{}", ip);
                Control::power(
                    Connection::connect(&ip[..]).connection,
                    power,
                )
            }
        }
    }
}
