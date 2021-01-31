use postscript::Value;
use postscript::compact1::FontSet;

macro_rules! ok(($result:expr) => ($result.unwrap()));

pub fn setup() -> FontSet {
    use std::fs::File;
    use std::io::{Seek, SeekFrom};

    let mut file = ok!(File::open("tests/fixtures/SourceSerifPro-Regular.otf"));
    ok!(file.seek(SeekFrom::Start(17732)));
    ok!(FontSet::read(&mut file))
}
