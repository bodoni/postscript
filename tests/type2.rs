use postscript::compact::FontSet;

use read;

#[test]
fn program() {
    let _ = FontSet::read(&mut read()).unwrap();
}
