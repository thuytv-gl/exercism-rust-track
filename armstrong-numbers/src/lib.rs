pub fn is_armstrong_number(num: u64) -> bool {
    let str_num = num.to_string();
    let num_len = str_num.len() as u32;
    num == str_num
        .chars()
        .fold(0, |acc, n| {
            (n.to_digit(10).unwrap() as u64).pow(num_len) + acc
        })
}
