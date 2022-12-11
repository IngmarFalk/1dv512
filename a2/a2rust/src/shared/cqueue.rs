use super::process::Process;

#[derive(Debug, Clone)]
pub struct CQueue {
    head: usize,
    tail: usize,
    size: usize,
    prcs: Vec<Option<Process>>,
    waiting: Vec<usize>,
}

impl CQueue {
    pub fn new(size: usize) -> Self {
        Self {
            head: 0,
            tail: 0,
            size,
            prcs: vec![],
            waiting: vec![],
        }
    }

    pub fn enqueue(&mut self, p: Process) {
        if self.waiting.len() > 0 {
            let i = self.waiting.pop().unwrap();
            // self[i] = Some(p);
            return;
        }

        if self.tail == self.head && self.prcs[self.tail].is_some() {
            self.waiting.push(self.tail);
            self.tail = (self.tail + 1) % self.size;
            return self.enqueue(p);
        }

        self.prcs[self.tail] = Some(p);
        self.tail = (self.tail + 1) % self.size;
    }

    pub fn dequeue(&mut self) -> Option<Process> {
        if self.head == self.tail && self.prcs[self.head].is_none() {
            return None;
        }

        let p = self.prcs[self.head].take();
        self.head = (self.head + 1) % self.size;
        p
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Option<Process>> {
        self.prcs.iter_mut()
    }
}

impl std::ops::Index<usize> for CQueue {
    type Output = Option<Process>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.prcs[index]
    }
}

impl Iterator for CQueue {
    type Item = Process;

    fn next(&mut self) -> Option<Self::Item> {
        self.dequeue()
    }
}
