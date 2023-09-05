fn char_add_one(pre: &[char]) -> Vec<char> {
    let iter = pre.iter().map(|c| char::from_u32(*c as u32 + 1).unwrap());
    iter.collect()
}

fn main() {
    let pre = vec!['a', 'b', 'c', 'd', 'e'];
    let post = char_add_one(&pre);
    println!("{:?}", post);
}

#[cfg(test)]
mod tests {
    use crate::char_add_one;

    #[test]
    fn test() {
        let pre = vec!['a', 'b', 'c', 'd', 'e'];
        assert_eq!(char_add_one(&pre), vec!['b', 'c', 'd', 'e', 'f']);
    }
}