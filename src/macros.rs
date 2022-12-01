macro_rules! deref {
    (@itemize $($one:item)*) => ($($one)*);
    ($name:ident::$field:tt => $target:ty) => (deref! {
        @itemize

        impl ::std::ops::Deref for $name {
            type Target = $target;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.$field
            }
        }

        impl ::std::ops::DerefMut for $name {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$field
            }
        }
    });
    ($name:ident<$life:tt>::$field:tt => $target:ty) => (deref! {
        @itemize

        impl<$life> ::std::ops::Deref for $name<$life> {
            type Target = $target;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.$field
            }
        }

        impl<$life> ::std::ops::DerefMut for $name<$life> {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$field
            }
        }
    });
}

macro_rules! raise(
    ($($argument:tt)*) => (
        return Err(crate::Error::new(::std::io::ErrorKind::Other, format!($($argument)*)))
    );
);

macro_rules! table {
    ($(#[$attribute:meta])* pub $name:ident {
        $($field:ident ($($kind:tt)+) $(= $value:block)* $(|$($argument:tt),+| $body:block)*,)*
    }) => (
        table! { @define $(#[$attribute])* pub $name { $($field ($($kind)+),)* } }
        table! {
            @implement
            pub $name { $($field ($($kind)+) [$($value)*] $(|$($argument),+| $body)*,)* }
        }
    );
    (@define $(#[$attribute:meta])* pub $name:ident { $($field:ident ($kind:ty),)* }) => (
        $(#[$attribute])*
        #[derive(Clone, Debug, Default)]
        pub struct $name { $(pub $field: $kind,)* }
    );
    (@implement pub $name:ident {
        $($field:ident ($($kind:tt)+) [$($value:block)*] $(|$($argument:tt),+| $body:block)*,)*
    }) => (
        impl crate::Value for $name {
            fn read<T: crate::Tape>(tape: &mut T) -> crate::Result<Self> {
                let mut table: $name = $name::default();
                $({
                    let value = table!(@read $name, table, tape [$($kind)+] [$($value)*]
                                       $(|$($argument),+| $body)*);
                    ::std::mem::forget(::std::mem::replace(&mut table.$field, value));
                })*
                Ok(table)
            }
        }
    );
    (@read $name:ident, $this:ident, $tape:ident [$kind:ty] []) => ($tape.take()?);
    (@read $name:ident, $this:ident, $tape:ident [$kind:ty] [$value:block]) => ({
        let value = $tape.take()?;
        if value != $value {
            raise!(concat!("found a malformed table of type ", stringify!($name)));
        }
        value
    });
    (@read $name:ident, $this:ident, $tape:ident [$kind:ty] []
     |$this_:tt, $tape_:tt| $body:block) => ({
        #[inline(always)]
        fn read<T: crate::Tape>($this_: &$name, $tape_: &mut T) -> crate::Result<$kind> $body
        read(&$this, $tape)?
    });
}
