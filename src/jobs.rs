use std::{
    collections::VecDeque,
    thread::{self, JoinHandle},
};

pub struct JobRunner<R> {
    jobs: VecDeque<JoinHandle<R>>,
}

impl<R: Sync + Send + 'static> JobRunner<R> {
    pub fn new() -> Self {
        Self {
            jobs: VecDeque::new(),
        }
    }

    pub fn run_on_thread<F: Fn() -> R + Send + 'static>(&mut self, func: F) {
        let handle = thread::spawn(func);
        self.jobs.push_back(handle);
    }

    pub fn wait_for_all_to_finish(&mut self) -> Vec<R> {
        let mut results: Vec<R> = Vec::new();
        while !self.jobs.is_empty() {
            if let Some(job_handle) = self.jobs.pop_front() {
                let res = job_handle.join().unwrap();
                results.push(res);
            }
        }

        results
    }

    pub fn remove_all_handles(&mut self) {
        self.jobs.clear();
    }
}
