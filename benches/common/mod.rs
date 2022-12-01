use std::fs::File;
use std::path::PathBuf;

use postscript::compact1::FontSet;
use postscript::Value;

macro_rules! ok(($result:expr) => ($result.unwrap()));

#[allow(dead_code)]
pub enum Fixture {
    NotoSansJP,
    SourceSerifPro,
}

impl Fixture {
    pub fn path(&self) -> PathBuf {
        match *self {
            Fixture::NotoSansJP => "tests/fixtures/NotoSansJP-Regular.otf".into(),
            Fixture::SourceSerifPro => "tests/fixtures/SourceSerifPro-Regular.otf".into(),
        }
    }

    pub fn offset(&self) -> u64 {
        match *self {
            Fixture::NotoSansJP => 337316,
            Fixture::SourceSerifPro => 17732,
        }
    }
}

pub fn setup(fixture: Fixture) -> File {
    use std::io::{Seek, SeekFrom};

    let mut file = ok!(File::open(fixture.path()));
    ok!(file.seek(SeekFrom::Start(fixture.offset())));
    file
}

pub fn setup_font_set(fixture: Fixture) -> FontSet {
    let mut file = setup(fixture);
    ok!(FontSet::read(&mut file))
}
