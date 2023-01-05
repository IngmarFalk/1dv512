use std::sync::MutexGuard;

use super::sem;

pub trait MessageQueue {
    fn send(&mut self, msg: char) -> bool;
    fn recv(&mut self) -> Option<char>;
}

pub struct Writer<'a> {
    pub mq: MutexGuard<'a, Mq>,
    pub msg: char,
}

impl<'a> Writer<'a> {
    pub fn send(&mut self) -> bool {
        self.mq.send(self.msg)
    }
}

pub struct Reader<'a> {
    mq: MutexGuard<'a, Mq>,
}

impl<'a> Reader<'a> {
    pub fn new(mq: MutexGuard<'a, Mq>) -> Self {
        Self { mq }
    }

    pub fn recv(&mut self) -> Option<char> {
        self.mq.recv()
    }
}

pub struct Mq {
    buf: Vec<char>,
    sem: sem::Sem<3, i16>,
    rx_lock: sem::Sem<1, i16>,
    sz: usize,
}

impl Mq {
    pub fn new() -> Self {
        let sz = 3;
        Self {
            buf: Vec::with_capacity(sz),
            sem: sem::Sem::from(sz as i16),
            rx_lock: sem::Sem::from(1),
            sz: 3,
        }
    }

    fn is_full(&self) -> bool {
        self.sem.status() == 0
    }

    pub fn is_empty(&self) -> bool {
        self.sem.status() == self.sz as i16
    }
}

impl MessageQueue for Mq {
    fn send(&mut self, msg: char) -> bool {
        if self.is_full() {
            return false;
        }
        self.sem.wait();
        self.buf.push(msg);
        true
    }

    fn recv(&mut self) -> Option<char> {
        self.rx_lock.wait();

        if self.is_empty() {
            self.rx_lock.signal();
            return None;
        }

        let msg = self.buf.remove(0);
        self.sem.signal();
        self.rx_lock.signal();
        Some(msg)
    }
}
