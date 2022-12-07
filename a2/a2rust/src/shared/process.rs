#[derive(Debug, Clone)]
pub struct Process {
    pub name: String,
    pub pid: usize,
    pub state: State,
}

impl Process {
    pub fn new(name: String, pid: usize) -> Self {
        Self {
            name,
            pid,
            state: State::Ready,
        }
    }

    pub fn run(&mut self) {
        self.state = State::Running;
    }

    pub fn block(&mut self) {
        self.state = State::Blocked;
    }

    pub fn terminate(&mut self) {
        self.state = State::Terminated;
    }

    pub fn is_ready(&self) -> bool {
        self.state == State::Ready
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum State {
    Ready,
    Running,
    Blocked,
    Terminated,
}
