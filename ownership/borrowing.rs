#[derive(Debug)]
struct Foo {}

fn main() {
    let mut vector = vec![Foo {}, Foo {}, Foo {}];

    let last_foo = vector.last();
    println!("last_foo: {:?}", last_foo);

    vector.pop();

    println!("last_foo: {:?}", last_foo);
}
