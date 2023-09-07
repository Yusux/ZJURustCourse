use core::fmt;
use std::cell::UnsafeCell;
use std::ops::Deref;

pub struct Rc<T> {
    rcbox: *mut UnsafeCell<(T, i32)>,
}

impl<T> Rc<T> {
    pub fn new(value_: T) -> Rc<T> {
        println!("new Rc (count: 1)");
        let rc_box = Box::leak(Box::new(UnsafeCell::new((value_, 1))));
        let rc = Rc {
            rcbox: rc_box,
        };
        rc
    }

    pub fn ref_count(rc: &Rc<T>) -> usize {
        rc.ref_count()
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe {
            &(*self.inner().get()).0
        }
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        self.dec_ref();
        println!("drop Rc (count: {})", self.ref_count());
        
        if self.ref_count() == 0 {
            unsafe {
                println!("free Rc memory");
                let _: Box<UnsafeCell<(T, i32)>> = Box::from_raw(self.rcbox);
            }
        }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Rc<T> {
        self.inc_ref();
        println!("clone Rc (count: {})", self.ref_count());
        Rc {
            rcbox: self.rcbox
        }
    }
}

trait RcBoxPtr<T> {
    fn inner(&self) -> &UnsafeCell<(T, i32)>;

    fn ref_count(&self) -> usize {
        unsafe {
            (*self.inner().get()).1 as usize
        }
    }

    fn inc_ref(&self) {
        unsafe {
            (*self.inner().get()).1 += 1;
        }
    }

    fn dec_ref(&self) {
        unsafe {
            (*self.inner().get()).1 -= 1;
        }
    }
}

impl<T> RcBoxPtr<T> for Rc<T> {
    fn inner(&self) -> &UnsafeCell<(T, i32)> {
        unsafe {
            &(*self.rcbox)
        }
    }
}

impl<T> fmt::Display for Rc<T> where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}
