extern crate clap;
extern crate acct;

use acct::load_from_file;
use clap::{Arg, App};
use std::fs::File;

fn main() {
    let matches = App::new("acct-rs")
                      .version("0.1")
                      .author("Josh A. <sinisterpatrician@gmail.com>")
                      .about("Parse acct(2) files")
                      .arg(Arg::with_name("file")
                           .short("f")
                           .long("file")
                           .value_name("FILE")
                           .help("acct file to parse")
                           .required(true)
                           .takes_value(true))
                      .get_matches();

    let acct_file = matches.value_of("file").unwrap();
    println!("file: {}", acct_file);


    let mut file = File::open(acct_file).unwrap();
    println!("file: {:?}", file);

    let accts = load_from_file(&mut file).unwrap();
    for acct in accts {
        println!("{}", acct.command);
    }
}
