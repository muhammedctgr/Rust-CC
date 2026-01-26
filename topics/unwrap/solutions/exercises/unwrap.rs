#![allow(unused)]

fn main() {
    // Unwrap and expect
    let x: Option<u32> = Some(1);
    // Unwraps the inner value. Panics if None
    let i = x.unwrap();
    println!("{}", i);

    
}
