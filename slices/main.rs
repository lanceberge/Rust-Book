fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..];
}

fn main() {
    let s = String::from("Hello world");

    let byte = first_word(&s);

    println!("byte: {byte}");

    let slice = &s[0..5];

    println!("slice: {slice}");

    println!("slice: {}", &s[6..]);
}
