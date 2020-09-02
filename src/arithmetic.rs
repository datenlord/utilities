use crate::conversion::Cast;
use std::any::type_name;

macro_rules! impl_overflow_arithmetic {
    ($target: ty) => {
        #[allow(clippy::use_self)]
        impl OverflowArithmetic for $target {
            #[inline]
            fn overflow_add(self, other: Self) -> Self {
                let (res, overflow) = self.overflowing_add(other);
                debug_assert!(
                    !overflow,
                    "number = {}({}) add number = {}({}) overflowing",
                    self,
                    type_name::<Self>(),
                    other,
                    type_name::<Self>()
                );
                res
            }

            #[inline]
            fn overflow_sub(self, other: Self) -> Self {
                let (res, overflow) = self.overflowing_sub(other);
                debug_assert!(
                    !overflow,
                    "number = {}({}) substract number = {}({}) overflowing",
                    self,
                    type_name::<Self>(),
                    other,
                    type_name::<Self>()
                );
                res
            }

            #[inline]
            fn overflow_mul(self, other: Self) -> Self {
                let (res, overflow) = self.overflowing_mul(other);
                debug_assert!(
                    !overflow,
                    "number = {}({}) multiply number = {}({}) overflowing",
                    self,
                    type_name::<Self>(),
                    other,
                    type_name::<Self>()
                );
                res
            }

            #[inline]
            fn overflow_div(self, other: Self) -> Self {
                let (res, overflow) = self.overflowing_div(other);
                debug_assert!(
                    !overflow,
                    "number = {}({}) divide number = {}({}) overflowing",
                    self,
                    type_name::<Self>(),
                    other,
                    type_name::<Self>()
                );
                res
            }

            #[inline]
            fn overflow_shr(self, other: Self) -> Self {
                let (res, overflow) = self.overflowing_shr(other.cast());
                debug_assert!(
                    !overflow,
                    "number = {}({}) right shift number = {}({}) overflowing",
                    self,
                    type_name::<Self>(),
                    other,
                    type_name::<Self>()
                );
                res
            }
        }
    };
}

impl_overflow_arithmetic!(u8);
impl_overflow_arithmetic!(u16);
impl_overflow_arithmetic!(u32);
impl_overflow_arithmetic!(u64);
impl_overflow_arithmetic!(u128);
impl_overflow_arithmetic!(i8);
impl_overflow_arithmetic!(i16);
impl_overflow_arithmetic!(i32);
impl_overflow_arithmetic!(i64);
impl_overflow_arithmetic!(i128);
impl_overflow_arithmetic!(usize);
impl_overflow_arithmetic!(isize);

/// A type cast trait used to do the integer arithmetic.
pub trait OverflowArithmetic {
    /// Overflow add.
    fn overflow_add(self, other: Self) -> Self;

    /// Overflow sub.
    fn overflow_sub(self, other: Self) -> Self;

    /// Overflow mul.
    fn overflow_mul(self, other: Self) -> Self;

    /// Overflow div.
    fn overflow_div(self, other: Self) -> Self;

    /// Overflow shr.
    fn overflow_shr(self, other: Self) -> Self;
}