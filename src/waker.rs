use std::task::Wake;
use std::sync::{Mutex, Condvar, Arc};


pub struct Signal {
    state: Mutex<State>,
    cond: Condvar,
}

enum State {
    Empty,
    Waiting,
    Notified,
}

impl Signal {
    pub fn new() -> Self {
        Signal {
            state: Mutex::new(State::Empty),
            cond: Condvar::new(),
        }
    }

    pub fn wait(&self) {
        let mut state = self.state.lock().unwrap();
        println!("wait");
        match *state {
            State::Notified => {
                println!("wait at Notified");
                *state = State::Empty;
            }
            State::Empty => {
                println!("wait at Empty");
                *state = State::Waiting;
                while let State::Waiting = *state {
                    state = self.cond.wait(state).unwrap();
                    println!("wait at while let")
                }
            }
            State::Waiting => {
                println!("wait at Waiting");
                panic!("multiple wait");
            }
        }
    }

    pub fn notify(&self) {
        let mut state = self.state.lock().unwrap();
        println!("notify");
        match *state {
            State::Notified => {
                println!("notify at Notified");
            }
            State::Empty => {
                println!("notify at Empty");
                *state = State::Notified;
            }
            State::Waiting => {
                println!("notify at Waiting");
                *state = State::Empty;
                self.cond.notify_one();
            }
        }
    }
}

impl Wake for Signal {
    fn wake(self: Arc<Self>) {
        self.notify();
    }
}