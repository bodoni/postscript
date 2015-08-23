use postscript::compact::FontSet;
use postscript::type2::Program;

use read;

#[test]
fn program() {
    let set = FontSet::read(&mut read()).unwrap();

    let code = &set.charstrings[0][134];
    let global = &set.global_subroutines;
    let local = &set.local_subroutines[0];

    let mut program = Program::new(code, global, local);

    let mut operations = vec![];
    while let Some(operation) = program.next().unwrap() {
        operations.push(operation);
    }

    assert_eq!(&operations, &vec![]);
}
