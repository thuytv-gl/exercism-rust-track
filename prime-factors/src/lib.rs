pub fn factors(n: u64) -> Vec<u64> {
    let mut num = n;
    let mut factor = 2;
    let mut factors: Vec<u64> = vec![];
    while num > 1 {
        if num % factor == 0 {
            num /= factor;
            factors.push(factor)
        } else {
            factor += 1;
        }
    }
    factors
}
