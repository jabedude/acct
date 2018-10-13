extern crate clap;


use clap::{Arg, App};

// TODO: maybe use #[inline] here?
fn expand_time(time: u16) -> u16 {
    let ret: u16 = (time & 0x1fff) << (((time >> 13) & 0x7) * 3);
    ret
}

fn main() {
    let matches = App::new("acct-rs")
                      .version("0.1")
                      .author("Josh A. <sinisterpatrician@gmail.com>")
                      .about("Parse acct(2) files")
                      .arg(Arg::with_name("file")
                           .short("f")
                           .value_name("FILE")
                           .help("IPv4 address to connect to")
                           .required(true)
                           .takes_value(true))
                      .get_matches();
    println!("{:?}", matches);
}
