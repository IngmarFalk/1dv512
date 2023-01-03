use std::sync::{Condvar, Mutex};

#[derive(Debug)]
pub struct Semaphore<const MAX: i16> {
    mutex: Mutex<i16>,
    condvar: Condvar,
}

impl<const MAX: i16> Semaphore<MAX> {
    pub fn new() -> Self {
        Semaphore {
            mutex: Mutex::new(MAX),
            condvar: Condvar::new(),
        }
    }

    pub fn wait(&self) {
        let mut count = self.mutex.lock().unwrap();
        *count -= 1;
        while *count < 0 {
            count = self.condvar.wait(count).unwrap();
        }
    }

    pub fn signal(&self) {
        let mut count = self.mutex.lock().unwrap();
        *count += 1;
        if *count <= 0 {
            self.condvar.notify_one();
        }
    }

    pub fn status(&self) -> i16 {
        *self.mutex.lock().unwrap()
    }
}
