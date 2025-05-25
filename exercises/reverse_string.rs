pub fn reverse(input: &str) -> String {
    input.chars().rev().collect::<_>()
}

fn main() {
    println!("{}", reverse("abcd"));
}
