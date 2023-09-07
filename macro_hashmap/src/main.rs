use std::collections::HashMap;

macro_rules! hash_map {
    ($($key:expr => $val:expr), *) => {
        {
            let mut map = HashMap::new();
            $(
                map.insert($key, $val);
            )*
            map
        }
    };
}

fn main() {
    let map = hash_map!{
        "one" => 1,
        "two" => 2,
        "three" => 3
    };
    println!("{:?}", map);
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    
    #[test]
    fn test_1() {
        let map = hash_map!{
            "one" => 1,
            "two" => 2,
            "three" => 3
        };
        
        let mut tradition_map = HashMap::new();
        tradition_map.insert("one", 1);
        tradition_map.insert("two", 2);
        tradition_map.insert("three", 3);

        assert_eq!(map, tradition_map);
    }
}
