use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

use super::Job;

pub(crate) struct Worker {
    pub(crate) id: usize,
    /// NOTE(aver): in a production environment, `std::thread::Builder` and the `spawn` method would
    /// be more sensible, as it handles OS error, in case threads cannot be created.
    pub(crate) thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub(crate) fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing...");

                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down...");
                    break;
                }
            };
        });
        Self {
            id,
            thread: Some(thread),
        }
    }
}
