#![allow(unused)]
mod waker;
pub mod executor;

#[cfg(test)]
mod tests {
    use super::*;

    async fn demo() -> i32{
        executor::Executor::spawn(demo2());
        println!("Hello, world!");
        419
    }

    async fn demo2() {
        println!("This is demo2.");
    }
    #[test]
    fn simple_async() {
        let ex = executor::Executor::new(2);
        println!("# Test Multi Tasks without Spin #");
        let ret = ex.block_on(demo());
        assert_eq!(ret, 419);
    }
}