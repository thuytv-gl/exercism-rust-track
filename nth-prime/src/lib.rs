fn is_prime(n: u32) -> bool {
    let lim = (n as f32).sqrt() as u32;
    (2..=lim).find(|i| n % i == 0) == None
}
pub fn nth(n: u32) -> u32 {
    let mut count = 0;
    let mut ret = 0;
    let mut iter = 2;
    while count <= n {
        if is_prime(iter) {
            count += 1;
            ret = iter;
        }
        iter += 1;
    }
    ret
}
