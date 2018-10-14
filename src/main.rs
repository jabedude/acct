#![feature(iterator_step_by)]

#[macro_use]
extern crate serde_derive;
extern crate clap;
extern crate bincode;


use clap::{Arg, App};
use bincode::deserialize;
use std::fs::File;
use std::io::Read;
use std::io;
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
    fn command(&self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.ac_comm.to_vec())
    }

    fn is_valid(&self) -> bool {
        self.ac_version == 3
    }
}

// TODO: maybe use #[inline] here?
fn expand_time(time: u16) -> u16 {
    let ret: u16 = (time & 0x1fff) << (((time >> 13) & 0x7) * 3);

    ret
}

fn is_file_valid_acct(file: &File) -> bool {
    file.metadata().unwrap()
        .len() % mem::size_of::<AcctV3>() as u64 == 0
}

fn load_from_slice(buf: &[u8]) -> AcctV3 {
    let acct: AcctV3 = deserialize(buf).unwrap();

    acct
}

fn load_from_file(file: &mut File) -> Vec<AcctV3> {
    let size = mem::size_of::<AcctV3>();
    println!("Size: {}", size);
    let chunks = (file.metadata().unwrap().len() / size as u64) as usize;
    println!("Chunks: {}", chunks);
    let mut all: Vec<AcctV3> = Vec::new();
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    println!("Buf len: {}", buf.len());

    for chunk in (0..chunks).step_by(size) {
        println!("Chunk: {}", chunk);
        all.push(load_from_slice(&buf[chunk..chunk+size]));
    }

    all
}

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


    let mut file = File::open(acct_file).unwrap();

    /*
    let mut buf = [0u8; mem::size_of::<AcctV3>()];
    println!("{}", is_file_valid_acct(&file));
    file.read_exact(&mut buf).unwrap();
    println!("{:?}", matches);
    let acct: AcctV3 = deserialize(&buf).unwrap();
    println!("{:?}", acct);
    println!("{:?}", acct.command());
    */

    let accts = load_from_file(&mut file);
    for acct in accts {
        println!("{}", acct.command().unwrap());
    }
}
