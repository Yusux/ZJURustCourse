mod cmpstr;

fn main() {
    let a = "abe";
    let b = "abd";
    println!("{}", cmpstr::compareString(a, b));
}

// check the result using another function or library
#[cfg(test)]
mod tests {
    use crate::cmpstr;

    #[test]
    fn test_same_length_front_bigger() {
        let a = "abe";
        let b = "abd";
        assert_eq!(cmpstr::compareString(a, b), a.cmp(b) == std::cmp::Ordering::Greater);
    }

    #[test]
    fn test_same_length_same_order() {
        let a = "abe";
        let b = "abe";
        assert_eq!(cmpstr::compareString(a, b), a.cmp(b) == std::cmp::Ordering::Greater);
    }

    #[test]
    fn test_same_length_back_bigger() {
        let a = "abd";
        let b = "abe";
        assert_eq!(cmpstr::compareString(a, b), a.cmp(b) == std::cmp::Ordering::Greater);
    }

    #[test]
    fn test_front_longer() {
        let a = "abed";
        let b = "abe";
        assert_eq!(cmpstr::compareString(a, b), a.cmp(b) == std::cmp::Ordering::Greater);
    }

    #[test]
    fn test_back_longer() {
        let a = "abe";
        let b = "abed";
        assert_eq!(cmpstr::compareString(a, b), a.cmp(b) == std::cmp::Ordering::Greater);
    }

    #[test]
    fn test_empty() {
        let a = "";
        let b = "";
        assert_eq!(cmpstr::compareString(a, b), a.cmp(b) == std::cmp::Ordering::Greater);
    }

    #[test]
    fn test_empty_front() {
        let a = "";
        let b = "a";
        assert_eq!(cmpstr::compareString(a, b), a.cmp(b) == std::cmp::Ordering::Greater);
    }

    #[test]
    fn test_empty_back() {
        let a = "a";
        let b = "";
        assert_eq!(cmpstr::compareString(a, b), a.cmp(b) == std::cmp::Ordering::Greater);
    }

    #[test]
    fn test_unicode() {
        let a = "这是一串字符";
        let b = "这也是一串字符";
        assert_eq!(cmpstr::compareString(a, b), a.cmp(b) == std::cmp::Ordering::Greater);
    }
}