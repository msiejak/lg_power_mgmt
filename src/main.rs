use std::env;
use lg_power_mgmt::Connection;
use lg_power_mgmt::Control;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("options are:\n\npoweron\nshutdown")
    }
    match &args[1][..] {
        "poweron" => Control::power(
            Connection::connect("127.0.0.1").connection,
            Control::ON
        ),
        "shutdown" => Control::power(
            Connection::connect("127.0.0.1").connection,
            Control::OFF
        ),
        _ => println!("options are:\n\npoweron\nshutdown"),
    };
}
