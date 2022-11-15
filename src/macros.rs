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
    ($(#[$attribute:meta])* pub $name:ident { $($field:ident ($kind:ty),)* }) => (
        table! { @define $(#[$attribute])* pub $name { $($field ($kind),)* } }
        table! { @implement pub $name { $($field,)* } }
    );
    (@define $(#[$attribute:meta])* pub $name:ident { $($field:ident ($kind:ty),)* }) => (
        $(#[$attribute])*
        #[derive(Clone, Debug)]
        pub struct $name { $(pub $field: $kind,)* }
    );
    (@implement pub $name:ident { $($field:ident,)* }) => (
        impl crate::Value for $name {
            fn read<T: crate::Tape>(tape: &mut T) -> crate::Result<Self> {
                let mut table: $name = unsafe { ::std::mem::zeroed() };
                $(::std::mem::forget(::std::mem::replace(&mut table.$field, tape.take()?));)+
                Ok(table)
            }
        }
    );
}
