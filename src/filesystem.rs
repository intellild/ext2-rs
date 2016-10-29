use std::io::Read;
use std::io::Write;
use std::io::Seek;

pub trait Disk: Read + Write + Seek {}
