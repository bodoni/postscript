extern crate postscript;

use postscript::Value;
use postscript::compact::FontSet;

mod compact;
mod type2;

fn setup() -> FontSet {
    use std::fs::File;
    use std::io::{Seek, SeekFrom};

    let mut file = File::open("tests/fixtures/SourceSerifPro-Regular.otf").unwrap();
    file.seek(SeekFrom::Start(17732)).unwrap();
    FontSet::read(&mut file).unwrap()
}
