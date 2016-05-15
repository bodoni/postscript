use postscript::type2::Program;
use test::Bencher;

use setup;

#[bench]
fn program(bencher: &mut Bencher) {
    let set = setup();
    let global = &set.global_subroutines;
    let local = &set.local_subroutines[0];

    bencher.iter(|| {
        for code in set.char_strings[0].iter() {
            let mut program = Program::new(code, global, local);
            while let Some(..) = program.next().unwrap() {
            }
        }
    })
}
