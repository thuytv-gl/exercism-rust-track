use std::str::Chars;

const SOME_MINE: Option<char> = Some('*');

fn check_neighbors(row: Chars, i: usize) -> u8 {
    let mut mine_count = 0;
    if row.to_owned().nth(i) == SOME_MINE {
        mine_count += 1;
    }
    if row.to_owned().nth(i+1) == SOME_MINE {
        mine_count += 1;
    }
    if row.to_owned().nth(i.wrapping_sub(1)) == SOME_MINE {
        mine_count += 1;
    }
    mine_count
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    minefield.iter().enumerate().map(|(i, row)| {
        let mut annotated_row = String::new();
        for (j, c) in row.chars().to_owned().enumerate() {
            if Some(c) == SOME_MINE {
                annotated_row.push(c);
                continue;
            }
            let mut mine_count: u8 = 0;
            mine_count += check_neighbors(row.chars().to_owned(), j);
            if let Some(next_row) = minefield.get(i+1) {
                mine_count += check_neighbors(next_row.chars(), j);
            }
            if let Some(prev_row) = minefield.get(i.wrapping_sub(1)) {
                mine_count += check_neighbors(prev_row.chars(), j);
            }
            match mine_count > 0 {
                true => annotated_row.push_str(format!("{}", mine_count).as_str()),
                false => annotated_row.push(' ')
            }
        }

        annotated_row
    }).collect()
}
