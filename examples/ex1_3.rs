fn plus_one(b: &mut i32) {
    *b += 1;
}

fn main() {
    let mut a = 10;
    println!("{}", a);
    plus_one(&mut a);
    println!("{}", a);
}