pub struct Todo {
    pub name: String,
    pub finished: bool
}

impl Todo {
    pub fn flip(&mut self) {
        self.finished = !self.finished;
    }
}