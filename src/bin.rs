extern crate acct;
extern crate clap;

use acct::AcctFile;
use clap::{App, Arg};
use std::fs::File;
use std::io::Write;

fn main() {
    let matches = App::new("acct-rs")
        .version("0.1")
        .author("Josh A. <sinisterpatrician@gmail.com>")
        .about("Parse acct(2) files")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("acct file to parse")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let acct_file = matches.value_of("file").unwrap();

    let mut file = File::open(acct_file).unwrap();

    let acct_file = AcctFile::load_from_file(&mut file).unwrap();
    for acct in &acct_file.records {
        println!("{}\t{}", acct.command, acct.username);
    }

    let mut out = File::create("optfile").unwrap();
    let bytes = acct_file.into_bytes();
    out.write_all(&bytes);
}
