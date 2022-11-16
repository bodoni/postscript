use std::fs::File;
use std::path::PathBuf;

use postscript::compact1::FontSet;
use postscript::Value;

macro_rules! ok(($result:expr) => ($result.unwrap()));

pub enum Fixture {
    SourceSerifPro,
}

impl Fixture {
    pub fn path(&self) -> PathBuf {
        match *self {
            Fixture::SourceSerifPro => "tests/fixtures/SourceSerifPro-Regular.otf".into(),
        }
    }

    pub fn offset(&self) -> u64 {
        match *self {
            Fixture::SourceSerifPro => 17732,
        }
    }
}

pub fn setup(fixture: Fixture) -> FontSet {
    use std::io::{Seek, SeekFrom};

    let mut file = ok!(File::open(fixture.path()));
    ok!(file.seek(SeekFrom::Start(fixture.offset())));
    ok!(FontSet::read(&mut file))
}
