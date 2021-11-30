use std::{
    collections::VecDeque,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

pub struct ThreadPool<R> {
    threads: Vec<JoinHandle<()>>,

    results: Arc<Mutex<Vec<R>>>,
    work_queue: Arc<Mutex<VecDeque<Box<dyn Fn() -> R + Send + 'static>>>>,
    busy_count: Arc<AtomicUsize>,

    should_kill_workers: Arc<AtomicBool>,
}

impl<R: Sync + Send + 'static> ThreadPool<R> {
    pub fn new(thread_count: usize) -> Self {
        let mut thread_pool = Self {
            threads: Vec::<JoinHandle<()>>::with_capacity(thread_count),
            results: Arc::new(Mutex::new(Vec::<R>::new())),
            work_queue: Arc::new(Mutex::new(
                VecDeque::<Box<dyn Fn() -> R + Send + 'static>>::new(),
            )),
            busy_count: Arc::new(AtomicUsize::new(0)),
            should_kill_workers: Arc::new(AtomicBool::new(false)),
        };

        for _ in 0..thread_count {
            let results = Arc::clone(&thread_pool.results);
            let work_queue = Arc::clone(&thread_pool.work_queue);
            let busy_count = Arc::clone(&thread_pool.busy_count);
            let should_kill_workers = Arc::clone(&thread_pool.should_kill_workers);

            thread_pool.threads.push(thread::spawn(move || {
                while !should_kill_workers.load(Ordering::Relaxed) {
                    let work: Option<Box<dyn Fn() -> R + Send + 'static>> = {
                        let mut work_queue_local = work_queue.lock().unwrap();
                        if !work_queue_local.is_empty() {
                            work_queue_local.pop_front()
                        } else {
                            None
                        }
                    };

                    match work {
                        Some(work_fn) => {
                            busy_count.fetch_add(1, Ordering::Relaxed);
                            let result = work_fn.as_ref()();
                            results.lock().unwrap().push(result);
                            busy_count.fetch_sub(1, Ordering::Relaxed);
                        }
                        None => thread::sleep(Duration::from_millis(1)),
                    }
                }
            }))
        }

        thread_pool
    }

    pub fn run<F: Fn() -> R + Send + 'static>(&mut self, func: F) {
        self.work_queue.lock().unwrap().push_back(Box::new(func));
    }

    pub fn wait_for_finish(&mut self) -> Vec<R> {
        while !self.work_queue.lock().unwrap().is_empty() {}
        while self.busy_count.load(Ordering::Relaxed) > 0 {}

        let mut result_lock = self.results.lock().unwrap();
        (0..result_lock.len())
            .into_iter()
            .map(|_| result_lock.pop().unwrap())
            .collect()
    }

    pub fn stop_threads(&mut self) {
        self.should_kill_workers.store(true, Ordering::Relaxed);
        let mut i = 0;
        while let Some(thread) = self.threads.pop() {
            thread.join().unwrap();
            i += 1;
        }
    }
}
