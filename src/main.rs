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
struct AcctV3Inner {
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

impl AcctV3Inner {
    fn load_from_slice(buf: &[u8]) -> AcctV3Inner {
        let acct: AcctV3Inner = deserialize(buf).unwrap();

        acct
    }

    fn command(&self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.ac_comm.to_vec())
    }

    fn is_valid(&self) -> bool {
        self.ac_version == 3
    }
}

#[derive(Debug)]
pub struct AcctV3 {
    inner: AcctV3Inner,
    pub username: String,
    pub command: String,
}

impl AcctV3 {
    fn from_slice(buf: &[u8]) -> AcctV3 {
        let inner = AcctV3Inner::load_from_slice(buf);
        let command = inner.command().unwrap();
        let username = String::from("TODO");

        AcctV3 {
            inner: inner,
            command: command,
            username: username,
        }
    }

    fn is_valid(&self) -> bool {
        self.inner.is_valid()
    }
}

// TODO: maybe use #[inline] here?
fn expand_time(time: u16) -> u16 {
    let ret: u16 = (time & 0x1fff) << (((time >> 13) & 0x7) * 3);

    ret
}

fn is_file_valid_acct(file: &File) -> bool {
    file.metadata().unwrap()
        .len() % mem::size_of::<AcctV3Inner>() as u64 == 0
}

fn load_from_file(file: &mut File) -> Option<Vec<AcctV3>> {
    if !is_file_valid_acct(&file) {
        return None;
    }

    let size = mem::size_of::<AcctV3Inner>();
    println!("Size: {}", size);
    let chunks = (file.metadata().unwrap().len() / size as u64) as usize;
    println!("Chunks: {}", chunks);
    let mut all: Vec<AcctV3> = Vec::new();
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    println!("Buf len: {}", buf.len());

    for chunk in (0..buf.len()).step_by(size) {
        println!("Chunk: {}", chunk);
        let acct = AcctV3::from_slice(&buf[chunk..chunk+size]);
        if acct.is_valid() {
            all.push(acct);
        }
    }

    Some(all)
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
    println!("file: {}", acct_file);


    let mut file = File::open(acct_file).unwrap();
    println!("file: {:?}", file);

    let accts = load_from_file(&mut file).unwrap();
    for acct in accts {
        println!("{}", acct.command);
    }
}
