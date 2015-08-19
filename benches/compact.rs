use postscript::compact::compound::Operator;
use random::{self, Source};
use test::{Bencher, black_box};

const SAMPLES: usize = 1000;

#[bench]
fn encoding_get(bencher: &mut Bencher) {
    use postscript::compact::compound::Encoding;

    let mut source = random::default().seed([42, 69]);
    let codes = source.iter::<u64>().take(SAMPLES).map(|number| (number as u16) % 256)
                                                  .collect::<Vec<_>>();
    let encoding = Encoding::Standard;
    bencher.iter(|| {
        for &code in &codes {
            black_box(encoding.get(code));
        }
    });
}

#[bench]
fn operator_default(bencher: &mut Bencher) {
    let mut source = random::default().seed([69, 42]);
    let operators = generate_operators(&mut source, SAMPLES);
    bencher.iter(|| {
        for &operator in &operators {
            black_box(operator.default());
        }
    });
}

#[bench]
fn operator_get(bencher: &mut Bencher) {
    let mut source = random::default().seed([42, 69]);
    let codes = generate_codes(&mut source, SAMPLES);
    bencher.iter(|| {
        for &code in &codes {
            black_box(Operator::get(code));
        }
    });
}

#[bench]
fn string_index_get(bencher: &mut Bencher) {
    use postscript::compact::compound::StringIndex;
    use postscript::compact::primitive::StringID;

    let mut source = random::default().seed([69, 42]);
    let sids = source.iter::<u64>().take(SAMPLES).map(|number| (number as StringID) % 391)
                                                 .collect::<Vec<_>>();
    let index = StringIndex::default();
    bencher.iter(|| {
        for &sid in &sids {
            black_box(index.get(sid));
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
                    0x0c0f...0x0c10 | 0x0c18...0x0c1d => continue,
                    code => codes.push(code),
                }
                break;
            }
        }
    }
    codes
}

fn generate_operators<T: Source>(source: &mut T, count: usize) -> Vec<Operator> {
    generate_codes(source, count).iter().map(|&code| Operator::get(code).unwrap()).collect()
}
