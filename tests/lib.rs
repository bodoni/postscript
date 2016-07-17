extern crate postscript;

use postscript::Value;
use postscript::compact::FontSet;

macro_rules! ok(($result:expr) => ($result.unwrap()));

mod compact;
mod type2;

fn setup() -> FontSet {
    use std::fs::File;
    use std::io::{Seek, SeekFrom};

    let mut file = ok!(File::open("tests/fixtures/SourceSerifPro-Regular.otf"));
    ok!(file.seek(SeekFrom::Start(17732)));
    ok!(FontSet::read(&mut file))
}
