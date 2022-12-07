use super::cqueue::CQueue;

#[derive(Debug, Clone)]
pub struct Semaphore(usize, CQueue);

impl Semaphore {
    pub fn counting(initial_value: usize) -> Self {
        Self(initial_value, CQueue::new(10))
    }

    pub fn binary() -> Self {
        Self(1, CQueue::new(10))
    }

    pub fn wait(&mut self) {
        while self.0 == 0 {
            // spin
        }
        self.0 -= 1;
    }

    pub fn signal(&mut self) {
        if let Some(p) = self
            .1
            .iter_mut()
            .find(|p| p.is_some() && p.as_ref().unwrap().is_ready())
        {
            return p.as_mut().unwrap().run();
        }

        self.0 += 1;
    }
}

impl std::ops::Deref for Semaphore {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

unsafe impl Send for Semaphore {}
unsafe impl Sync for Semaphore {}
