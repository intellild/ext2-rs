use std::fs::File;
use std::fs::OpenOptions;
use std::io::*;

#[test]
fn open_file() {
    let mut file = OpenOptions::new().read(true).write(true).create(false).open("test.img");
}