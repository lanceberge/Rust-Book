struct Number {
    value: i32,
    positive: bool,
}

trait Signed {
    fn is_negative(self) -> bool;
}

impl Signed for Number {
    fn is_negative(self) -> bool {
        self.value < 0
    }
}

// Trait receivers
impl std::clone::Clone for Number {
    fn clone(&self) -> Self {
        Self { ..*self }
    }
}

trait SignedDefault {
    fn is_negative(self) ->
}

fn main() {
    let n = Number {
        value: -44,
        positive: false,
    };
    println!("is n negative: {}", n.is_negative());
}
