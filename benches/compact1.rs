#![feature(test)]

extern crate postscript;
extern crate random;
extern crate test;

#[macro_use]
mod support;

mod encoding {
    use postscript::compact1::Encoding;
    use random::Source;
    use test::{black_box, Bencher};

    #[bench]
    fn get(bencher: &mut Bencher) {
        let mut source = random::default(42);
        let codes = source
            .iter::<u64>()
            .take(1000)
            .map(|number| (number as u16) % 256)
            .collect::<Vec<_>>();
        let encoding = Encoding::Standard;
        bencher.iter(|| {
            for &code in &codes {
                black_box(encoding.get(code));
            }
        });
    }
}

mod operation {
    use postscript::compact1::Operator;
    use random::Source;
    use test::{black_box, Bencher};

    #[bench]
    fn operator_default(bencher: &mut Bencher) {
        let mut source = random::default(42);
        let operators = generate_operators(&mut source, 1000);
        bencher.iter(|| {
            for &operator in &operators {
                black_box(operator.default());
            }
        });
    }

    #[bench]
    fn operator_get(bencher: &mut Bencher) {
        let mut source = random::default(42);
        let codes = generate_codes(&mut source, 1000);
        bencher.iter(|| {
            for &code in &codes {
                black_box(ok!(Operator::from(code)));
            }
        });
    }

    fn generate_codes<T: Source>(source: &mut T, count: usize) -> Vec<u16> {
        let mut codes = vec![];
        while codes.len() != count {
            if (source.read::<u64>() as i64) > 0 {
                loop {
                    let number = source.read::<u64>();
                    match (number % (0x15 + 1)) as u16 {
                        0x0c => continue,
                        code => codes.push(code),
                    }
                    break;
                }
            } else {
                loop {
                    let number = source.read::<u64>();
                    match 0x0c00 | (number % (0x26 + 1)) as u16 {
                        0x0c0f..=0x0c10 | 0x0c18..=0x0c1d => continue,
                        code => codes.push(code),
                    }
                    break;
                }
            }
        }
        codes
    }

    fn generate_operators<T: Source>(source: &mut T, count: usize) -> Vec<Operator> {
        generate_codes(source, count)
            .iter()
            .map(|&code| ok!(Operator::from(code)))
            .collect()
    }
}

mod noto_sans {
    use test::Bencher;

    use crate::support::{setup_font_set, Fixture};

    #[bench]
    fn font_set(bencher: &mut Bencher) {
        bencher.iter(|| {
            setup_font_set(Fixture::NotoSansJP);
        });
    }
}
