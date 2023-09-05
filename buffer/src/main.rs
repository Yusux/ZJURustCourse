mod buffer;
mod point;

fn main() {
    let mut buf = buffer::Buffer::new();
    buf.push(1);
    buf.push(2);
    buf.push(3);
    println!("sum: {:?}", buf.sum().unwrap());
    buf.remove(1);
    println!("sum: {:?}", buf.sum().unwrap());

    let point_a = point::Point::new(1, 2);
    let point_b = point::Point::new(3, 4);
    let point_c = point_a + point_b;
    println!("point_a: {}", point_a);
    println!("point_b: {}", point_b);
    println!("point_c: {}", point_c);
}

#[cfg(test)]
mod tests {
    use crate::{buffer, point::Point};

    #[test]
    fn test_i32() {
        let mut buf: buffer::Buffer<i32> = buffer::Buffer::new();
        buf.push(1);
        buf.push(2);
        buf.push(3);
        assert_eq!(buf.sum().unwrap(), 6);
        buf.remove(0);
        assert_eq!(buf.sum().unwrap(), 5);
        buf.push(4);
        assert_eq!(buf.sum().unwrap(), 9);
    }

    #[test]
    fn test_i64() {
        let mut buf = buffer::Buffer::new();
        buf.push(1 as i64);
        buf.push(2 as i64);
        buf.push(3 as i64);
        assert_eq!(buf.sum().unwrap(), 6 as i64);
        buf.remove(0);
        assert_eq!(buf.sum().unwrap(), 5 as i64);
        buf.push(4 as i64);
        assert_eq!(buf.sum().unwrap(), 9 as i64);
    }

    #[test]
    fn test_f32() {
        let mut buf = buffer::Buffer::new();
        buf.push(1.0 as f32);
        buf.push(2.0 as f32);
        buf.push(3.0 as f32);
        assert_eq!(buf.sum().unwrap(), 6.0 as f32);
        buf.remove(0);
        assert_eq!(buf.sum().unwrap(), 5.0 as f32);
        buf.push(4.0 as f32);
        assert_eq!(buf.sum().unwrap(), 9.0 as f32);
    }

    #[test]
    fn test_f64() {
        let mut buf = buffer::Buffer::new();
        buf.push(1.0);
        buf.push(2.0);
        buf.push(3.0);
        assert_eq!(buf.sum().unwrap(), 6.0);
        buf.remove(0);
        assert_eq!(buf.sum().unwrap(), 5.0);
        buf.push(4.0);
        assert_eq!(buf.sum().unwrap(), 9.0);
    }

    #[test]
    fn test_empty() {
        let buf: buffer::Buffer<i32> = buffer::Buffer::new();
        assert_eq!(buf.sum(), None);
        assert_eq!(buf.sum(), None);
    }

    #[test]
    fn test_point() {
        let mut buf = buffer::Buffer::new();
        buf.push(Point::new(1, 2));
        buf.push(Point::new(3, 4));
        buf.push(Point::new(5, 6));
        assert_eq!(buf.sum().unwrap(), Point::new(9, 12));
        buf.remove(1);
        assert_eq!(buf.sum().unwrap(), Point::new(6, 8));
        buf.push(Point::new(7, 8));
        assert_eq!(buf.sum().unwrap(), Point::new(13, 16));
    }
}