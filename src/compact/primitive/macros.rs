macro_rules! fill(
    ($tape:ident, $count:expr, $buffer:ident) => (
        if try!(::std::io::Read::read($tape, &mut $buffer)) != $count {
            return raise!("failed to read as much as needed");
        }
    );
);

macro_rules! read(
    ($tape:ident, $size:expr) => (unsafe {
        let mut buffer: [u8; $size] = ::std::mem::uninitialized();
        fill!($tape, $size, buffer);
        ::std::mem::transmute(buffer)
    });
);
