//! A trait for overflowing arithmetic

use crate::conversion::NumericCast;
use std::any::type_name;

/// Macros for checking arithmetic overflow, panicing when overflow happens.
macro_rules! impl_overflow_arithmetic {
    ($target: ty) => {
        #[allow(clippy::use_self)]
        impl OverflowArithmetic for $target {
            #[inline]
            fn overflow_add(self, other: Self) -> Self {
                let (res, overflow) = self.overflowing_add(other);
                assert!(
                    !overflow,
                    "number = {}({}) add number = {}({}) overflowed",
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
                assert!(
                    !overflow,
                    "number = {}({}) substract number = {}({}) overflowed",
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
                assert!(
                    !overflow,
                    "number = {}({}) multiply number = {}({}) overflowed",
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
                assert!(
                    !overflow,
                    "number = {}({}) divide number = {}({}) overflowed",
                    self,
                    type_name::<Self>(),
                    other,
                    type_name::<Self>()
                );
                res
            }

            #[inline]
            fn overflow_shl(self, other: Self) -> Self {
                let (res, overflow) = self.overflowing_shl(other.numeric_cast());
                assert!(
                    !overflow,
                    "number = {}({}) left shift number = {}({}) overflowed",
                    self,
                    type_name::<Self>(),
                    other,
                    type_name::<Self>()
                );
                res
            }

            #[inline]
            fn overflow_shr(self, other: Self) -> Self {
                let (res, overflow) = self.overflowing_shr(other.numeric_cast());
                assert!(
                    !overflow,
                    "number = {}({}) right shift number = {}({}) overflowed",
                    self,
                    type_name::<Self>(),
                    other,
                    type_name::<Self>()
                );
                res
            }

            #[inline]
            fn overflow_neg(self) -> Self {
                let (res, overflow) = self.overflowing_neg();
                assert!(
                    !overflow,
                    "number = {}({}) negate overflowed",
                    self,
                    type_name::<Self>(),
                );
                res
            }

            #[inline]
            fn overflow_rem(self, other: Self) -> Self {
                let (res, overflow) = self.overflowing_rem(other.numeric_cast());
                assert!(
                    !overflow,
                    "number = {}({}) remainder number = {}({}) overflowed",
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
    #[must_use]
    fn overflow_add(self, other: Self) -> Self;

    /// Overflow sub.
    #[must_use]
    fn overflow_sub(self, other: Self) -> Self;

    /// Overflow mul.
    #[must_use]
    fn overflow_mul(self, other: Self) -> Self;

    /// Overflow div.
    #[must_use]
    fn overflow_div(self, other: Self) -> Self;

    /// Overflow shl.
    #[must_use]
    fn overflow_shl(self, other: Self) -> Self;

    /// Overflow shr.
    #[must_use]
    fn overflow_shr(self, other: Self) -> Self;

    /// Overflow neg.
    #[must_use]
    fn overflow_neg(self) -> Self;

    /// Overflow rem.
    #[must_use]
    fn overflow_rem(self, other: Self) -> Self;
}
