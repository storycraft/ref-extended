#![doc = include_str!("../README.md")]

use core::mem;
use std::{process, hint::unreachable_unchecked};

/// Returns reference with lifetime of value
///
/// # Safety
/// * You must not move original variable
/// * You must not create immutable reference while having mutable reference
/// * The reference must not outlive
#[inline(always)]
pub unsafe fn extend_ref<'a, T: 'a>(target: &T) -> &'a T {
    mem::transmute::<&T, &'a T>(target)
}

/// Returns mutable reference with lifetime of value
///
/// # Safety
/// * You must not move original variable
/// * You must not create mutable reference while having immutable references
/// * You must not create multiple mutable reference
/// * The reference must not outlive
#[inline(always)]
pub unsafe fn extend_mut<'a, T: 'a>(target: &mut T) -> &'a mut T {
    mem::transmute::<&mut T, &'a mut T>(target)
}

#[doc(hidden)]
#[inline(always)]
pub fn run_abort_guarded<F: FnOnce()>(func: F) -> ! {
    struct AbortGuard;

    impl Drop for AbortGuard {
        fn drop(&mut self) {
            process::abort();
        }
    }

    {
        let _guard = AbortGuard;

        func();
    }

    // Safety: AbortGuard always abort process on drop and cannot be touched from outside
    unsafe { unreachable_unchecked() }
}

/// Safely extends lifetime of reference as long as lifetime of it's value.
///
/// ## Usage
/// ```Rust
/// ref_extended(|&a| {}); // a is &'static u32 because lifetime of a's value is 'static
///
/// ref_extended(|&a, &mut b| {}); // a is &'static u32 and b is &'static mut u32 because lifetime of a and b' value is 'static
/// ```
#[macro_export]
macro_rules! ref_extended {
    (|$(& $($var: ident)+ ),*| $body: expr) => {
        $(let ::ref_extended::extract_ref_name!($($var)*) = unsafe { ::ref_extended::extend_lifetime!($($var)*) };)*

        ::ref_extended::run_abort_guarded(move || {$body})
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! extract_ref_name {
    ($name: ident) => {
        $name
    };

    (mut $name: ident) => {
        $name
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! extend_lifetime {
    ($name: ident) => {
        ::ref_extended::extend_ref(&$name)
    };

    (mut $name: ident) => {
        ::ref_extended::extend_mut(&mut $name)
    };
}
