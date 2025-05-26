use std::fmt::Display;

struct Pair<T> {
    a: T,
    b: T,
}

fn main() {
    println!("Hello, world!");
    let p1 = Pair { a: 3, b: 9 };
}

fn foobar<L: Display, R>(left: L, right: R) {}
