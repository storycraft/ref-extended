use std::{convert::identity, process};

use ref_extended::ref_extended;

fn main() -> ! {
    let i = 0;
    let mut a = 0;

    ref_extended!(|&i, &mut a| {
        identity::<&'static i32>(i);
        identity::<&'static mut i32>(a);

        process::exit(0)
    });
}
