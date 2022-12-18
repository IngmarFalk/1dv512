use std::sync::atomic::AtomicUsize;

pub struct Semaphore {
    cnt: AtomicUsize,
    evt: Event,
}

impl Semaphore {
    pub fn new(cnt: usize) -> Self {
        Self {
            cnt: AtomicUsize::new(cnt),
            evt: Event::new(),
        }
    }

    pub fn wait(&self) {
        todo!()
    }

    pub fn signal(&self) {
        todo!()
    }
}

pub struct Event(bool);

impl Event {
    pub fn wait(&self) {
        while !self.0 {
            // spin
        }
    }

    pub fn signal(&mut self) {
        self.0 = true;
    }
}
