enum IpAddr {
    V4(String),
    V6(u8, u8, u8, u8),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));

    let home = IpAddr::V4(127, 0, 0, 1);

    let config_max = Some(3u8);
}
