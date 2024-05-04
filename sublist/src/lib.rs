use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T>(_first_list: &[T], _second_list: &[T]) -> Comparison
where
    T: PartialEq<T>,
    T: Debug,
{
    println!("fist: {:?}, second: {:?}", _first_list, _second_list);
    let mut result = Comparison::Unequal;
    if _first_list == _second_list {
        result = Comparison::Equal;
    } else {
        if _first_list.len() > _second_list.len() {
            let mut start = 0;
            for (i, v) in _second_list.iter().enumerate() {
                if v == &_first_list[0] {
                    start = i;
                }
            }
        }
        if _first_list.len() < _second_list.len() {
            if _first_list.iter().all(|i| _second_list.contains(i)) {
                result = Comparison::Sublist;
            }
        }
    }

    result
}
