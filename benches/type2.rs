#![feature(test)]

extern crate postscript;
extern crate test;

#[macro_use]
mod common;

mod source_serif {
    use postscript::compact1::font_set::Record;
    use postscript::type2::Program;
    use test::Bencher;

    use crate::common::{setup_font_set, Fixture};

    #[bench]
    fn program(bencher: &mut Bencher) {
        let set = setup_font_set(Fixture::SourceSerifPro);
        let global = &set.subroutines;
        let local = match &set.records[0] {
            Record::CharacterNameKeyed(ref record) => &*record.subroutines,
            _ => unreachable!(),
        };
        bencher.iter(|| {
            for code in set.char_strings[0].iter() {
                let mut program = Program::new(code, global, local);
                while let Some(..) = ok!(program.next()) {}
            }
        })
    }
}
