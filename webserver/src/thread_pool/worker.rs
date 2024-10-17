use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

use super::Job;

pub(crate) struct Worker {
    id: usize,
    // NOTE(aver): in a production environment, `std::thread::Builder` and the `spawn` method would
    // be more sensible, as it handles OS error, in case threads cannot be created.
    thread: thread::JoinHandle<()>,
}

impl Worker {
    pub(crate) fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {id} got a job; executing...");
            job();
        });
        Self { id, thread }
    }
}
