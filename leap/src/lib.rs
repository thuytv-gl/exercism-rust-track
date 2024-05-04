pub fn is_leap_year(year: u64) -> bool {
    match year % 100 == 0 {
        true => year % 400 == 0,
        false => year % 4 == 0,
    }
}
