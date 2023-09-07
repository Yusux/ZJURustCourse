mod my_stack;

fn main() {
    let stack = my_stack::MyStack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("Popped: {:?}", stack.pop());
    println!("Popped: {:?}", stack.pop());

    stack.push(4);

    println!("Popped: {:?}", stack.pop());
    println!("Popped: {:?}", stack.pop());
    println!("Popped: {:?}", stack.pop());
}

#[cfg(test)]
mod tests {
    use crate::my_stack::MyStack;

    #[test]
    fn test_my_stack() {
        let stack =MyStack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));

        stack.push(4);

        assert_eq!(stack.pop(), Some(4));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }
}
