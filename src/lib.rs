#![doc = "README.md"]

use core::mem;
use std::process;

/// This struct extends reference of value as long as lifetime of value
/// 
/// Dropping this value will abort process to ensure the lifetime of reference
#[derive(Debug)]
#[repr(transparent)]
pub struct RefExtended<T: ?Sized>(T);

impl<T> RefExtended<T> {
    pub const fn new(value: T) -> Self {
        Self(value)
    }
}

impl<'b, T: ?Sized + 'b> RefExtended<&'_ T> {
    /// Returns reference with lifetime of value
    /// 
    /// This method is unsafe because
    /// * Moving [`RefExtended`] struct can invalidate returned reference
    #[inline(always)]
    pub unsafe fn static_ref(&self) -> &'b T {
        mem::transmute::<&T, &'b T>(self.0)
    }
}

impl<'b, T: ?Sized + 'b> RefExtended<&'_ mut T> {
    /// Returns mutable reference with lifetime of value
    /// 
    /// This method is unsafe because
    /// * Moving [`RefExtended`] struct can invalidate returned reference
    /// * It can create multiple mutable reference
    #[inline(always)]
    pub unsafe fn static_mut(&mut self) -> &'b mut T {
        mem::transmute::<&mut T, &'b mut T>(self.0)
    }
}

impl<T: ?Sized> Drop for RefExtended<T> {
    fn drop(&mut self) {
        process::abort();
    }
}

/// Safely extends lifetime of reference as long as lifetime of it's value
/// 
/// ## Usage
/// ```Rust
/// let a = 0_u32;
/// 
/// ref_extended(&a); // Now a is &'static u32 because lifetime of a's value is 'static
///
/// let mut b = 0_u32;
/// 
/// ref_extended(&mut b); // Now b is &'static mut u32 because lifetime of b's value is 'static
/// ```
#[macro_export]
macro_rules! ref_extended {
    (&$name: ident) => {
        let $name = ::ref_extended::RefExtended::new(&$name);
        let $name = unsafe { $name.static_ref() };
    };

    (&mut $name: ident) => {
        let mut $name = ::ref_extended::RefExtended::new(&mut $name);
        let $name = unsafe { $name.static_mut() };
    };
}

/// Pin value to stack and safely extends lifetime of reference as long as lifetime of it's value
/// 
/// ## Usage
/// ```Rust
/// pin_ref_extended(a, 0_u32); // Now a is &'static u32 with value 0 because lifetime of a's value is 'static
///
/// pin_ref_extended(mut b, 0_u32); // Now b is &'static mut u32 with value 0 because lifetime of b's value is 'static
/// ```
#[macro_export]
macro_rules! pin_ref_extended {
    (&$name: ident, $expr: expr) => {
        let $name = $expr;
        ::ref_extended::ref_extended!(&$name);
    };
    
    (mut $name: ident, $expr: expr) => {
        let mut $name = $expr;
        ::ref_extended::ref_extended!(&mut $name);
    };
}

