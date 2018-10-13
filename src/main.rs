#[macro_use]
extern crate serde_derive;
extern crate clap;
extern crate bincode;


use clap::{Arg, App};
use bincode::deserialize;
use std::fs::File;
use std::io::Read;
use std::string::FromUtf8Error;
use std::string::String;
use std::mem;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct AcctV3 {
    ac_flag: u8,
    ac_version: u8,
    ac_tty: u16,
    ac_exitcode: u32,
    ac_uid: u32,
    ac_gid: u32,
    ac_pid: u32,
    ac_ppid: u32,
    ac_btime: u32,
    ac_etime: f32,
    ac_utime: u16,
    ac_stime: u16,
    ac_mem: u16,
    ac_io: u16,
    ac_rw: u16,
    ac_minflt: u16,
    ac_majflt: u16,
    ac_swaps: u16,
    ac_comm: [u8; 16],
}

impl AcctV3 {
    fn command(self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.ac_comm.to_vec())
    }
}

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

    let acct_file = matches.value_of("file").unwrap();

    let mut buf = [0u8; mem::size_of::<AcctV3>()];
    let mut file = File::open(acct_file).unwrap();
    file.read_exact(&mut buf).unwrap();
    println!("{:?}", matches);

    let acct: AcctV3 = deserialize(&buf).unwrap();
    println!("{:?}", acct);
    println!("{:?}", acct.command());
}
