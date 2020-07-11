#![allow(unused_variables)]
use std::collections::HashMap;

#[derive(Clone, Copy)]
struct Graph {}

fn main() {
    let g = &Graph {};
    let mut m = HashMap::new();
    m.insert("variable1", g);
    let g2 = m["variable1"];
}