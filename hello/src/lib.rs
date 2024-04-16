pub struct ThreadPool;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        ThreadPool
    }

    pub fn execute<F>(&self, _f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
