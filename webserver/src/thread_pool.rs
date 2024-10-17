use std::sync::{mpsc, Arc, Mutex};

use worker::Worker;

mod worker;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

#[derive(Debug)]
pub enum ThreadPoolError {
    PoolCreationError,
}

impl ThreadPool {
    pub fn build(size: usize) -> Result<ThreadPool, ThreadPoolError> {
        if size < 1 {
            return Err(ThreadPoolError::PoolCreationError);
        }

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Ok(Self { workers, sender })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}
