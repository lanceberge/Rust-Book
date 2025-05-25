#[derive(Debug)]
struct Foo {}

#[derive(Debug)]
struct Number {
    value: i32,
    positive: bool,
}

#[derive(Clone, Debug)]
struct CopyNumber {
    value: i32,
    positive: bool,
}

fn main() {}

fn main() {
    let mut vector = vec![Foo {}, Foo {}, Foo {}];

    let last_foo = vector.last();
    println!("last_foo: {:?}", last_foo);

    vector.pop();

    println!("last_foo: {:?}", last_foo);

    let n = Number {
        value: -44,
        positive: false,
    };

    let copy_n = CopyNumber {
        value: 44,
        positive: false,
    };
    // m is moved
    let m = n;
    // println!("{:?}", n); - error because n is moved to m

    let m2 = copy_n; // creates a copy of n
    println!("2");
}
