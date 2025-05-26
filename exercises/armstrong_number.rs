pub fn is_armstrong_number(num: u32) -> bool {
    let digits = num.to_string();
    let n = digits.len() as u32;
    let sum: u32 = digits.chars().map(|c| c.to_digit(10).unwrap().pow(n)).sum();
    sum == num
}
