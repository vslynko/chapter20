use std::thread;

pub struct ThreadPool;

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
        ThreadPool
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static
    {
        thread::spawn(f);
    }
}
