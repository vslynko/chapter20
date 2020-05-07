use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
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

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {}
        ThreadPool { threads }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        thread::spawn(f);
    }
}
