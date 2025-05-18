#[derive(Clone, Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        active: false,
        ..user1.clone()
    };

    struct Color(i32, i32, i32);

    let black = Color(255, 255, 255);

    println!("{user1:?}");

    let rec = Rectangle {
        width: 10,
        height: 10,
    };

    println!("Area: {:?}", rec.area());

    let square = Rectangle::square(5);
    println!("Square are: {:?}", square.area());
}
