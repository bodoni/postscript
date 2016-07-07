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

macro_rules! table {
    ($(#[$attribute:meta])* pub $structure:ident {
        $($field:ident ($kind:ty),)+
    }) => (
        table! { @define $(#[$attribute])* pub $structure { $($field ($kind),)+ } }
        table! { @implement pub $structure { $($field,)+ } }
    );
    (@define $(#[$attribute:meta])* pub $structure:ident {
        $($field:ident ($kind:ty),)+
    }) => (itemize! {
        $(#[$attribute])*
        #[derive(Clone, Debug, Default, Eq, PartialEq)]
        pub struct $structure { $(pub $field: $kind,)+ }
    });
    (@implement pub $structure:ident {
        $($field:ident,)+
    }) => (
        impl ::tape::Value for $structure {
            fn read<T: ::tape::Tape>(tape: &mut T) -> ::Result<Self> {
                let mut table = $structure::default();
                $(table.$field = try!(::tape::Value::read(tape));)+
                Ok(table)
            }
        }
    );
}
