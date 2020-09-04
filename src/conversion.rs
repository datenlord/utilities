//! A trait used to replace as conversion

use std::any::type_name;
use std::convert::TryFrom;

/// A type cast trait used to replace as conversion.
pub trait Cast {
    /// Performs the conversion.
    #[inline]
    fn cast<T>(self) -> T
    where
        T: TryFrom<Self>,
        Self: Sized + std::fmt::Display + Copy,
    {
        T::try_from(self).unwrap_or_else(|_| {
            panic!(
                "Failed to convert from {}: {} to {}",
                type_name::<Self>(),
                self,
                type_name::<T>(),
            )
        })
    }
}

impl<U> Cast for U {}

/// Cast to pointer
#[inline]
pub const fn cast_to_ptr<T: ?Sized, U>(val: &T) -> *const U {
    let ptr: *const _ = val;
    ptr.cast()
}

/// Cast to mut pointer
#[inline]
pub fn cast_to_mut_ptr<T: ?Sized, U>(val: &mut T) -> *mut U {
    let ptr: *mut _ = val;
    ptr.cast()
}
