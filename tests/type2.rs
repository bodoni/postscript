use postscript::compact::FontSet;
use postscript::type2::Machine;

use read;

#[test]
fn machine() {
    let set = FontSet::read(&mut read()).unwrap();

    let code = &set.charstrings[0][134];
    let global = &set.global_subroutines;
    let local = &set.local_subroutines[0];

    let mut machine = Machine::new(global, local);
    let operations = machine.execute(code).unwrap();

    assert_eq!(&operations, &vec![]);
}
