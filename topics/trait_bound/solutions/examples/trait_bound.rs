#![allow(unused)]

// Trait bound - specifies constraints on a generic type

trait A {}
trait B {}
trait C {}

impl A for u32 {}
impl B for u32 {}
impl C for i32 {}

// Trait bounds
// x must implement A
fn c<T: A>(x: T) {}

// Multiple trait bounds
// x must implement A and B
fn m<T: A + B>(x: T) {}

// Where clause
// x must implement A and B
// y must implement C
fn w<T, U>(x: T, y: U)
where
    T: A + B,
    U: C,
{
}

