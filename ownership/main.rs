fn main() {
    // Pointer to a string on the heap
    let s = String::from("Hello World");

    let s2 = s;

    // s is shadowed so this will error
    // println!("{s}");

    let s2 = takes_ownership(s2);

    println!("{s2}");

    uses_reference(&s2);

    println!("s2 is still valid");

    let mut s = String::from("Hello"); // you can only have one mutable reference to a piece of data

    uses_mutable_reference(&mut s);
    println!("{s}");
}

fn takes_ownership(some_string: String) -> String {
    println!("{some_string}");
    some_string
}

fn uses_reference(some_string: &String) {
    println!("{some_string}")
}

fn uses_mutable_reference(some_string: &mut String) {
    some_string.push_str(", world");
}
