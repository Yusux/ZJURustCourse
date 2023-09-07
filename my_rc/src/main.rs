mod my_rc;

fn main() {
    let five = my_rc::Rc::new(5);
    let fivel = five.clone();
    println!("{}", fivel);
    println!("{}", my_rc::Rc::ref_count(&fivel));
}

#[cfg(test)]
mod tests {
    use crate::my_rc::Rc;

    #[test]
    fn test_base() {
        let five = Rc::new(5);
        let fivel = five.clone();
        // Rc 实现了 Deref trait, 可以自动解引用, 因此下面格式化成功
        assert_eq!(format!("{}", fivel), "5");
        // 可以通过调用 strong_count 查看引用计数
        assert_eq!(Rc::ref_count(&fivel), 2);
    }

    #[test]
    fn test_count_and_drop() {
        // Rc::new() 创建一个引用计数为 1 的 Rc<T>
        let a = Rc::new("不要用裸指针");
        // Rc::clone() 会增加引用计数
        let b = Rc::clone(&a);
        // Rc::strong_count() 获取引用计数
        assert_eq!(Rc::ref_count(&a), 2);
        assert_eq!(Rc::ref_count(&b), 2);
        // 检查 Rc::drop() 的实现
        drop(a);
        assert_eq!(Rc::ref_count(&b), 1);
        assert_eq!(format!("{}", b), "不要用裸指针");
        let c = Rc::clone(&b);
        let d = Rc::clone(&b);
        assert_eq!(Rc::ref_count(&b), 3);
        assert_eq!(Rc::ref_count(&c), 3);
        assert_eq!(Rc::ref_count(&d), 3);
    }
}