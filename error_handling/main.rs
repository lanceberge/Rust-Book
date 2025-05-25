fn main() {
    let value: Option<i32> = None;
    let res = value.expect("value was none");

    println!("{res}");
}

fn process_input(input: i32) -> Result<(), &str> {
    let res = expects_ten(input)?; // return the error if there is one
    Ok();
}

fn expects_ten(i: i32) -> Result<bool, &str> {
    match i {
        case 10 => Ok(true)
        _ => Err("Expected 10")
    }
}
