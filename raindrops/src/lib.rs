pub fn raindrops(n: u32) -> String {
    let mut ret = String::new();
    let drops = vec![
        (3, "Pling"),
        (5, "Plang"),
        (7, "Plong")
    ];
    for drop in drops.iter() {
        if n % drop.0 == 0 {
            ret.push_str(drop.1);
        }
    }
    if ret.is_empty() {
        return n.to_string();
    }
    ret
}
