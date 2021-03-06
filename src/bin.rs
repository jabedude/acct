use acct::AcctFile;
use chrono::prelude::DateTime;
use chrono::Utc;
use clap::{App, Arg};
use std::fs::File;

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

    let acct_file = AcctFile::new(&mut file).unwrap();
    for acct in acct_file.iter() {
        let datetime = DateTime::<Utc>::from(acct.creation_time);
        let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string();
        println!(
            "{}\t{}\t{:?}\tSU:{}",
            acct.command,
            acct.username,
            timestamp_str,
            acct.was_super_user()
        );
    }
}
