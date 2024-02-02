#![allow(dead_code)]

use std::fs::File;
use std::io::{Seek, SeekFrom};
use std::path::PathBuf;

use postscript::compact1::FontSet;
use postscript::value::Read;

macro_rules! ok(($result:expr) => ($result.unwrap()));

pub enum Fixture {
    NotoSansJP,
    SourceSerifPro,
}

impl Fixture {
    pub fn path(&self) -> PathBuf {
        let file_name = match *self {
            Fixture::NotoSansJP => "NotoSansJP-Regular.otf",
            Fixture::SourceSerifPro => "SourceSerifPro-Regular.otf",
        };
        PathBuf::from("tests").join("fixtures").join(file_name)
    }

    pub fn offset(&self) -> u64 {
        match *self {
            Fixture::NotoSansJP => 337316,
            Fixture::SourceSerifPro => 17732,
        }
    }
}

pub fn setup(fixture: Fixture) -> File {
    let mut file = ok!(File::open(fixture.path()));
    ok!(file.seek(SeekFrom::Start(fixture.offset())));
    file
}

pub fn setup_font_set(fixture: Fixture) -> FontSet {
    let mut file = ok!(File::open(fixture.path()));
    ok!(file.seek(SeekFrom::Start(fixture.offset())));
    let count = ok!(FontSet::count(&mut file));
    ok!(file.seek(SeekFrom::Start(fixture.offset())));
    let table = ok!(FontSet::read(&mut file));
    assert_eq!(table.operations.len(), count);
    table
}
