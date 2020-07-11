#![allow(unused_variables)]
#[derive(Clone, Copy)]
struct Variable {}

fn main() {
    let p = &Variable {};
    let mut v = Vec::new();
    v.push(p);
    let p2 = v[0];
}