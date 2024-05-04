pub fn abbreviate(phrase: &str) -> String {
    let mut take = true;
    let mut ret = String::new();
    for (i, ch) in phrase.chars().enumerate() {
        if ch.is_alphabetic() && take {
            ret.push(ch);
            take = false;
        }
        if " -".contains(ch) {
            take = true;
        }
        if let Some(next) = phrase.chars().nth(i+1) {
            if ch.is_lowercase() && next.is_uppercase() {
                take = true;
            }
        }
    }
    ret.to_ascii_uppercase()
}
