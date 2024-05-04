/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code_without_space = code.replace(" ", "");
    if code_without_space == "0" {
        return false;
    }
    let mut sum = 0;
    let mut is_second = false;
    for c in code_without_space.chars().rev() {
        let try_digit = c.to_digit(10);
        if let None = try_digit {
            return false;
        }
        let mut digit = try_digit.unwrap();
        if is_second {
            digit *= 2;
            if digit > 9 {
                digit -= 9;
            }
        }
        sum += digit;
        is_second = !is_second;
    }
    sum % 10 == 0
}
