//! This is documentation for the `acct` crate.
//!
//! The acct crate is meant to be used for handling and
//! processing the acct(5) file generated by UNIX process
//! accounting.

#![feature(iterator_step_by)]

#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate users;

use bincode::{serialize, deserialize};
use std::io::Read;
use std::{fmt, mem, result};
use std::string::FromUtf8Error;
use std::string::String;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use users::get_user_by_uid;

const AFORK: u8 = 0x01;
const ASU: u8 = 0x02;
const ACORE: u8 = 0x08;
const AXSIG: u8 = 0x10;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidFile,
    BadReader,
    Er,
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(_: std::string::FromUtf8Error) -> Error { Error::Er }
}

impl From<std::ffi::OsString> for Error {
    fn from(_: std::ffi::OsString) -> Error { Error::Er }
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Error { Error::BadReader }
}

impl From<std::boxed::Box<bincode::ErrorKind>> for Error {
    fn from(_: std::boxed::Box<bincode::ErrorKind>) -> Error { Error::BadReader }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidFile => write!(f, "Invalid file"),
            Error::BadReader => write!(f, "Invalid reader"),
            Error::Er => write!(f, "Invalid data"),
        }
    }
}

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

    fn command(&self) -> Result<String> {
        let res = String::from_utf8(self.ac_comm.to_vec())?;
        Ok(res)
    }

    fn is_valid(&self) -> bool {
        self.ac_version == 3
    }
}

/// Represents a acct(5) v3 record structure
///
/// see https://linux.die.net/man/5/acct
#[derive(Debug)]
pub struct AcctV3 {
    inner: AcctV3Inner,
    /// The accounting username
    pub username: String,
    /// The command name of executed command
    pub command: String,
    /// The time the command was created
    pub creation_time: SystemTime,
}

impl AcctV3 {
    /// Constructs a AcctV3 object from a byte slice
    pub fn from_slice(buf: &[u8]) -> Result<AcctV3> {
        let inner = AcctV3Inner::load_from_slice(buf);
        let command = inner.command()?;
        let username = get_user_by_uid(inner.ac_uid).ok_or(Error::Er)?;
        let username = username.name()
            .to_os_string()
            .into_string()?;
        let ctime = inner.ac_btime as u64;
        let creation_time = UNIX_EPOCH + Duration::from_secs(ctime);

        Ok(AcctV3 {
            inner: inner,
            command: command,
            username: username,
            creation_time: creation_time,
        })
    }

    fn is_valid(&self) -> bool {
        self.inner.is_valid()
    }

    /// Shows if command was forked
    pub fn was_forked(&self) -> bool {
        self.inner.ac_flag & AFORK == AFORK
    }

    /// Shows if the command's user was root
    pub fn was_super_user(&self) -> bool {
        self.inner.ac_flag & ASU == ASU
    }

    /// Shows if the command produced a core dump
    pub fn was_core_dumped(&self) -> bool {
        self.inner.ac_flag & ACORE == ACORE
    }

    /// Shows if the command was killed via a signal
    pub fn was_killed(&self) -> bool {
        self.inner.ac_flag & AXSIG == AXSIG
    }
}

/// Represents an acct(5) log file.
/// Iterate over records field to examine contents
pub struct AcctFile {
    /// Vector of acct records
    pub records: Vec<AcctV3>,
}

impl AcctFile {
    fn is_valid(buf: &[u8]) -> bool {
        buf.len() % mem::size_of::<AcctV3Inner>() == 0
    }

    /// Construct a new AcctFile struct from a Reader
    pub fn new<R: Read + ?Sized>(reader: &mut R) -> Result<AcctFile> {
        let size = mem::size_of::<AcctV3Inner>();
        let mut all: Vec<AcctV3> = Vec::new();
        let mut buf: Vec<u8> = Vec::new();
        reader.read_to_end(&mut buf)?;
        //reader.read_to_end(&mut buf).unwrap();

        if !AcctFile::is_valid(&buf) {
            return Err(Error::Er);
        }

        for chunk in (0..buf.len()).step_by(size) {
            let acct = AcctV3::from_slice(&buf[chunk..chunk + size])?;
            if acct.is_valid() {
                all.push(acct);
            }
        }

        Ok(
            AcctFile {
                records: all,
            }
        )
    }

    /// Convert the AcctFile object into bytes for writing back into file.
    /// Consumes the object.
    pub fn into_bytes(self) -> Result<Vec<u8>> {
        let mut all_bytes: Vec<u8> = Vec::new();
        for acct in self.records {
            let mut buf = serialize(&acct.inner)?;
            all_bytes.append(&mut buf);
        }

        Ok(all_bytes)
    }
}

fn expand_time(time: u16) -> u16 {
    let ret: u16 = (time & 0x1fff) << (((time >> 13) & 0x7) * 3);

    ret
}
