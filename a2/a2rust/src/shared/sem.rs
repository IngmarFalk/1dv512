use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Sem<const MAX: i16> {
    mtx: Arc<Mutex<i16>>,
}

impl<const MAX: i16> Sem<MAX> {
    pub fn new() -> Self {
        Sem {
            mtx: Arc::new(Mutex::new(0)),
        }
    }

    pub fn from(val: i16) -> Self {
        if val > MAX {
            panic!("Value is greater than MAX");
        }
        Sem {
            mtx: Arc::new(Mutex::new(val)),
        }
    }

    pub fn wait(&self) {
        while *self.mtx.lock().unwrap() < 0 {}
        self.decr();
    }

    pub fn signal(&self) {
        if *self.mtx.lock().unwrap() == MAX - 1 {
            return *self.mtx.lock().unwrap() = 0;
        }
        self.incr();
    }

    pub fn set(&self, val: i16) {
        *self.mtx.lock().unwrap() = val;
    }

    pub fn is_turn(&self, turn: i16) -> bool {
        *self.mtx.lock().unwrap() == turn
    }

    pub fn wait_turn(&self, turn: i16) {
        while !self.is_turn(turn) {}
    }

    pub fn status(&self) -> i16 {
        *self.mtx.lock().unwrap()
    }

    fn incr(&self) {
        *self.mtx.lock().unwrap() += 1;
    }

    fn decr(&self) {
        *self.mtx.lock().unwrap() -= 1;
    }
}
