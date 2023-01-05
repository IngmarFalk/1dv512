use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Sem<const MAX: usize, V: Sized> {
    mtx: Arc<Mutex<V>>,
}

impl<const RESOURCES: usize> Sem<RESOURCES, i16> {
    pub fn new() -> Self {
        Sem {
            mtx: Arc::new(Mutex::new(1)),
        }
    }

    pub fn from(val: i16) -> Self {
        if val > RESOURCES as i16 {
            panic!("Value is greater than MAX");
        }
        Sem {
            mtx: Arc::new(Mutex::new(val)),
        }
    }

    pub fn wait(&self) {
        while *self.mtx.lock().unwrap() <= 0 {
            // println!("Waiting...");
            // std::thread::sleep(std::time::Duration::from_millis(200));
        }
        *self.mtx.lock().unwrap() -= 1;
    }

    pub fn signal(&self) {
        *self.mtx.lock().unwrap() += 1;
    }

    pub fn next(&self) {
        if *self.mtx.lock().unwrap() == RESOURCES as i16 {
            return *self.mtx.lock().unwrap() = 1;
        }
        *self.mtx.lock().unwrap() += 1;
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

    pub fn as_idx(&self) -> usize {
        *self.mtx.lock().unwrap() as usize - 1
    }
}
