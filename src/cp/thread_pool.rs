use core::str;
use std::sync::{mpsc, Arc, Mutex};
use std::thread::JoinHandle;
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: i32,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: i32, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            match message {
                Ok(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                // If the channel is closed, we exit the loop and terminate the thread.
                // This happens when the ThreadPool is dropped and all jobs are completed.
                // The `recv` method will return an error if the channel is closed.
                Err(_) => {
                    println!("Worker {} disconnected; shutting down.", id);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

pub struct ThreadPool {
    pub size: usize,
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0, "ThreadPool size must be greater than 0");
        let (sender, receiver) = mpsc::channel();

        // receiver need to be wrapped in Arc<Mutex> to be shared between threads.
        // Arc: Atomic Reference Counted, allows multiple ownership of the same data.
        // Mutex: Mutual Exclusion, allows only one thread to access the data at a time.
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id as i32, Arc::clone(&receiver)));
        }

        ThreadPool {
            size,
            workers,
            sender: Some(sender),
        }
    }

    pub fn exec<F>(&self, f: F)
    where 
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

// Drop trait is automatically called when the ThreadPool goes out of scope.

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("[ThreadPool] Sending shutdown signal to all workers.");

        // Drop the sender to close the channel and notify all workers.
        // This will cause all workers to receive an error when they try to receive a job.
        // The error will cause the worker threads to exit their loops and terminate.
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}.", worker.id);
            if let Some(thread) = worker.thread.take() {
                // Wait for the worker thread to finish.
                thread.join().unwrap();
            }
        }
    }
}