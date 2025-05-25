fn main() {
    let mut v1 = Vec::new();
    v1.push(1);

    let v2 = vec![1, 2, 3];

    println!("{v1:?}");

    let third = v2.get(2);

    for i in &v1 {
        println!("{i}");
    }

    let mut v3 = vec![1, 2, 3];
    for i in &mut v3 {
        *i += 10;
    }

    println!("{:?}", v3);
}
