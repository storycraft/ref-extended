use std::convert::identity;

use ref_extended::{ref_extended, pin_ref_extended};

#[test]
fn test() {
    fn _compile_test() {
        let mut a = 2_i32;
        ref_extended!(&mut a);

        pin_ref_extended!(mut b, 0_u32);

        identity::<&'static i32>(a);
        identity::<&'static mut u32>(b);
    }
}
