use postscript::compact::compound::Operator;
use random::{self, Source};
use test::{Bencher, black_box};

#[bench]
fn operator_default(bencher: &mut Bencher) {
    let mut source = random::default().seed([42, 69]);
    let operators = generate_operators(&mut source, 1000);
    bencher.iter(|| {
        for &operator in &operators {
            black_box(operator.default());
        }
    });
}

#[bench]
fn operator_get(bencher: &mut Bencher) {
    let mut source = random::default().seed([69, 42]);
    let operators = generate_codes(&mut source, 1000);
    bencher.iter(|| {
        for &operator in &operators {
            black_box(Operator::get(operator));
        }
    });
}

fn generate_operators<T: Source>(source: &mut T, count: usize) -> Vec<Operator> {
    generate_codes(source, count).iter().map(|&code| Operator::get(code).unwrap()).collect()
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
