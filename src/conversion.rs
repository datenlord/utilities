//! A trait used to replace as conversion

pub use numeric_cast::NumericCast;

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
