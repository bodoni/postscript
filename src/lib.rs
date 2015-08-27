//! Parser for PostScript fonts.

/// An error.
pub type Error = std::io::Error;

/// A result.
pub type Result<T> = std::result::Result<T, Error>;

macro_rules! deref {
    ($name:ident::$field:tt => $target:ty) => (itemize! {
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
    ($name:ident<$life:tt>::$field:tt => $target:ty) => (itemize! {
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

macro_rules! itemize(($($chunk:item)*) => ($($chunk)*));

macro_rules! number {
    ($name:ident) => (
        /// A number.
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum $name {
            /// An integer number.
            Integer(i32),
            /// A real number.
            Real(f32),
        }

        impl $name {
            /// Return the inner value, converting to `i32` if needed.
            #[inline]
            pub fn as_i32(&self) -> i32 {
                use self::$name::*;
                match self {
                    &Integer(value) => value,
                    &Real(value) => value as i32,
                }
            }

            /// Return the inner value, converting to `f32` if needed.
            #[inline]
            pub fn as_f32(&self) -> f32 {
                use self::$name::*;
                match self {
                    &Integer(value) => value as f32,
                    &Real(value) => value,
                }
            }
        }

        impl ::std::cmp::PartialOrd for $name {
            #[inline]
            fn partial_cmp(&self, that: &Self) -> Option<::std::cmp::Ordering> {
                use self::$name::*;
                match (self, that) {
                    (&Integer(this), &Integer(that)) => this.partial_cmp(&that),
                    (&Real(this), &Real(that)) => this.partial_cmp(&that),
                    (&Integer(this), &Real(that)) => (this as f32).partial_cmp(&that),
                    (&Real(this), &Integer(that)) => this.partial_cmp(&(that as f32)),
                }
            }
        }

        impl ::std::ops::Add for $name {
            type Output = Self;

            #[inline]
            fn add(self, that: Self) -> Self::Output {
                use self::$name::*;
                match (self, that) {
                    (Integer(this), Integer(that)) => Integer(this + that),
                    (Real(this), Real(that)) => Real(this + that),
                    (Integer(this), Real(that)) => Real(this as f32 + that),
                    (Real(this), Integer(that)) => Real(this + that as f32),
                }
            }
        }

        impl ::std::ops::Div for $name {
            type Output = Self;

            #[inline]
            fn div(self, that: Self) -> Self::Output {
                use self::$name::*;
                match (self, that) {
                    (Integer(this), Integer(that)) => Integer(this / that),
                    (Real(this), Real(that)) => Real(this / that),
                    (Integer(this), Real(that)) => Real(this as f32 / that),
                    (Real(this), Integer(that)) => Real(this / that as f32),
                }
            }
        }

        impl ::std::ops::Mul for $name {
            type Output = Self;

            #[inline]
            fn mul(self, that: Self) -> Self::Output {
                use self::$name::*;
                match (self, that) {
                    (Integer(this), Integer(that)) => Integer(this * that),
                    (Real(this), Real(that)) => Real(this * that),
                    (Integer(this), Real(that)) => Real(this as f32 * that),
                    (Real(this), Integer(that)) => Real(this * that as f32),
                }
            }
        }

        impl ::std::ops::Neg for $name {
            type Output = Self;

            #[inline]
            fn neg(self) -> Self::Output {
                use self::$name::*;
                match self {
                    Integer(value) => Integer(-value),
                    Real(value) => Real(-value),
                }
            }
        }

        impl ::std::ops::Sub for $name {
            type Output = Self;

            #[inline(always)]
            fn sub(self, that: Self) -> Self::Output {
                self + (-that)
            }
        }
    );
}

macro_rules! raise(
    ($message:expr) => (return Err(::Error::new(::std::io::ErrorKind::Other, $message)));
    ($($argument:tt)+) => (raise!(format!($($argument)+)));
);

mod tape;

pub mod compact;
pub mod type2;
