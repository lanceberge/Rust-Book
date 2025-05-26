fn file_ext(name: &str) -> Option<&str> {
    name.split(".").last()
}

fn main() {
    let ext = {
        let name = String::from("Read me. Or don't.txt");
        file_ext(&name).unwrap_or("")
        // error: `name` does not live long enough
    };
    println!("extension: {:?}", ext);
}
