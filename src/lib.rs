pub struct ThreadPoll;

impl ThreadPoll {
    pub fn new(size: usize) -> ThreadPoll {
        ThreadPoll
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {

    }
}
