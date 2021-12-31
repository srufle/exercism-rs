fn digits(num: u32) -> Vec<u32> {
    fn inner_digits(num: u32, digits: &mut Vec<u32>) {
        if num >= 10 {
            inner_digits(num / 10, digits);
        }
        digits.push(num % 10);
    }

    let mut digits = Vec::new();
    inner_digits(num, &mut digits);
    digits
}

pub fn is_armstrong_number(num: u32) -> bool {
    let digits = digits(num);
    println!("digits: {:?}", digits);
    let digits_len = digits.len() as u32;
    println!("digits_len: {:?}", digits_len);
    let sum = digits
        .iter()
        .fold(0, |acc, x| acc + u32::pow(*x, digits_len));
    println!("sum: {:?}", sum);
    sum == num
}
