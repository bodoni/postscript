#![feature(test)]

extern crate postscript;
extern crate random;
extern crate test;

use postscript::Value;
use postscript::compact::FontSet;

macro_rules! ok(($result:expr) => ($result.unwrap()));

mod compact;
mod type2;

fn setup() -> FontSet {
    use std::fs::File;
    use std::io::{Cursor, Read, Seek, SeekFrom};

    let mut file = ok!(File::open("tests/fixtures/SourceSerifPro-Regular.otf"));
    ok!(file.seek(SeekFrom::Start(17732)));
    let mut buffer = vec![0; 37728];
    assert_eq!(ok!(file.read(&mut buffer)), buffer.len());
    ok!(FontSet::read(&mut Cursor::new(buffer)))
}
