#![allow(non_snake_case)]
pub fn compareString(x: &str, y: &str) -> bool {
    // if x is bigger than y in lexicographical order, return true
    // otherwise, return false
    let mut x_iter = x.chars();
    let mut y_iter = y.chars();
    loop {
        let x_char = x_iter.next();
        let y_char = y_iter.next();
        match (x_char, y_char) {
            (Some(x_char), Some(y_char)) => {
                if x_char > y_char {
                    return true;
                } else if x_char < y_char {
                    return false;
                }
            },
            (Some(_), None) => {
                return true;
            },
            (None, Some(_)) => {
                return false;
            },
            (None, None) => {
                return false;
            }
        }
    }
}