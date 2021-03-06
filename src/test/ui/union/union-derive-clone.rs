// FIXME: missing sysroot spans (#53081)
// ignore-i586-unknown-linux-gnu
// ignore-i586-unknown-linux-musl
// ignore-i686-unknown-linux-musl

#![feature(untagged_unions)]

use std::mem::ManuallyDrop;

#[derive(Clone)] //~ ERROR the trait bound `U1: std::marker::Copy` is not satisfied
union U1 {
    a: u8,
}

#[derive(Clone)]
union U2 {
    a: u8, // OK
}

impl Copy for U2 {}

#[derive(Clone, Copy)]
union U3 {
    a: u8, // OK
}

#[derive(Clone, Copy)]
union U4<T: Copy> {
    a: T, // OK
}

#[derive(Clone, Copy)]
union U5<T> {
    a: ManuallyDrop<T>, // OK
}

#[derive(Clone)]
struct CloneNoCopy;

fn main() {
    let u = U5 { a: ManuallyDrop::new(CloneNoCopy) };
    let w = u.clone(); //~ ERROR no method named `clone` found for union `U5<CloneNoCopy>`
}
