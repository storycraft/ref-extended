# Ref Extended
Extends lifetime of reference to same as lifetime of value by shortening entire program lifetime

## When it is useful?
1. You are doing programming without heap allocation (such as embed programming), and don't want to use static.
2. You run some diverging functions which exit process itself without returning.

## Example
```Rust
let mut a = 2_i32; // The lifetime of value itself(not reference) is 'static

// Safely extend lifetimes and abort after expression finish
ref_extended!(|&a| {
    identity::<&'static i32>(a); // This compiles
});

// Unreachable. Process abort
```
