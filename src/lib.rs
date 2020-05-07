use std::thread;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        Worker {
            id,
            thread: thread::spawn(|| { }),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    /// Create a new ThreadPool of given size (threads number)
    ///
    /// # Panics
    ///
    /// The new function will panic if the size is zero.
    /// ```should_panic
    /// chapter20::ThreadPool::new(0);
    /// ```
    ///
    /// # Examples
    /// ```
    /// chapter20::ThreadPool::new(4);
    /// ```
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id))
        }
        ThreadPool { workers }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        thread::spawn(f);
    }
}
