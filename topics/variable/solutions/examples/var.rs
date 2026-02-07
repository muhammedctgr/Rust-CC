#![allow(unused)]

// Constants are immutable values known at compile time.
const NUM: u32 = 1;

fn main() {
    // Variable are immutable by default
    let x: i32 = -1;
    // This will not compile
    // x += 1;

    // Mutable variable
    let mut y: i32 = 1;
    y += 1;

    // Type inference: the compiler can infer types in most cases.
    let x = -1;


}
