extern crate postscript;

use postscript::compact::FontSet;

mod compact;
mod type2;

fn setup() -> FontSet {
    use std::fs::File;
    use std::io::{Cursor, Read, Seek, SeekFrom};

    let mut file = File::open("tests/fixtures/SourceSerifPro-Regular.otf").unwrap();
    file.seek(SeekFrom::Start(17732)).unwrap();
    let mut buffer = vec![0; 37728];
    assert_eq!(file.read(&mut buffer).unwrap(), buffer.len());
    FontSet::read(&mut Cursor::new(buffer)).unwrap()
}
