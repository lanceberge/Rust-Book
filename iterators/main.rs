fn main() {
    let natural_numbers = 1..;

    println!("{}", natural_numbers.contains(&10));

    for i in &[10, 20, 30] {
        println!("{i}");
    }

    for c in "abcd".chars() {}

    let filtered_nums: Vec<i32> = vec![1, 2, 20].into_iter().filter(|x| *x >= 2).collect();
    println!("filtered nums: {:?}", filtered_nums);
}
