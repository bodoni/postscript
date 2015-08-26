//! Parser for PostScript fonts.

extern crate random;

/// An error.
pub type Error = std::io::Error;

/// A result.
pub type Result<T> = std::result::Result<T, Error>;

macro_rules! raise(
    ($message:expr) => (return Err(::Error::new(::std::io::ErrorKind::Other, $message)));
    ($($argument:tt)+) => (raise!(format!($($argument)+)));
);

macro_rules! number {
    ($name:ident) => (
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum $name {
            Integer(i32),
            Real(f32),
        }

        impl $name {
            #[inline]
            pub fn as_i32(&self) -> i32 {
                use self::$name::*;
                match self {
                    &Integer(value) => value,
                    &Real(value) => value as i32,
                }
            }

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

mod band;

pub mod compact;
pub mod type2;
