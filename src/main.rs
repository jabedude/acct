extern crate clap;


use clap::{Arg, App};

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
