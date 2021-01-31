use postscript::compact1::FontSet;
use postscript::Value;

macro_rules! ok(($result:expr) => ($result.unwrap()));

#[allow(dead_code)]
pub fn setup() -> FontSet {
    use std::fs::File;
    use std::io::{Cursor, Read, Seek, SeekFrom};

    let mut file = ok!(File::open("tests/fixtures/SourceSerifPro-Regular.otf"));
    ok!(file.seek(SeekFrom::Start(17732)));
    let mut buffer = vec![0; 37728];
    assert!(ok!(file.read(&mut buffer)) == buffer.len());
    ok!(FontSet::read(&mut Cursor::new(buffer)))
}
