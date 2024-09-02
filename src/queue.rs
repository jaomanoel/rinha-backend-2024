use std::{
    collections::VecDeque,
    sync::{Condvar, Mutex},
};

pub struct Queue<T> {
    pub data: Mutex<VecDeque<T>>,
    pub cvar: Condvar,
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Self {
            data: Mutex::new(VecDeque::new()),
            cvar: Condvar::new(),
        }
    }
}

impl<T> Queue<T> {
    pub fn push_front(&self, value: T) -> &Self {
        self.data.lock().unwrap().push_back(value);

        self.cvar.notify_one();
        self
    }

    pub fn pop_back(&self) -> T {
        let mut data = self.data.lock().unwrap();

        while data.is_empty() {
            data = self.cvar.wait(data).unwrap();
        }

        data.pop_front().unwrap()
    }
}
