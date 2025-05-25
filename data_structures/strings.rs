fn main() {
    let mut s = String::from("foo");
    s.push_str("bar");

    let s1 = String::from("Hello");
    let s2 = String::from("World");
    println!("{}", format!("{} {}", s1, s2));

    let s1 = "Hello".to_string();
}
