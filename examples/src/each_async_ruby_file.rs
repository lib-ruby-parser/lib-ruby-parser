use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use std::env;

use crate::each_ruby_file;

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;
enum Message {
    Job(Job),
    Exit,
}

#[derive(Debug)]
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let job = receiver
                .lock()
                .expect("Failed to lock mpsc receiver")
                .recv()
                .expect("mspc receiver is empty");

            // println!("Worker {} got a job; executing.", id);
            match job {
                Message::Job(job) => job.call_box(),
                Message::Exit => break,
            }
        });
        Self {
            id,
            thread: Some(thread),
        }
    }
}

#[derive(Debug)]
pub struct Pool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl Pool {
    pub fn new(size: usize) -> Self {
        let mut workers = vec![];
        let (sender, receiver) = mpsc::channel::<Message>();
        let receiver = Arc::new(Mutex::new(receiver));
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)))
        }
        Self { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender
            .send(Message::Job(Box::new(f)))
            .expect("Failed to send JOB message to the thread");
    }
}

impl Drop for Pool {
    fn drop(&mut self) {
        println!("Sending EXIT");
        for _ in &mut self.workers {
            self.sender
                .send(Message::Exit)
                .expect("Failed to send EXIT message to the thread");
        }

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().expect("Failed to join a worker thread");
            }
        }
    }
}

pub fn each_async_ruby_file<F>(path: &str, cb: &'static F) -> Result<(), Box<dyn std::error::Error>>
where
    F: Fn(&str) + Send + Sync,
{
    let threads_count = env::var("THREADS")
        .map(|s| s.parse::<usize>().expect("THREADS must be a number"))
        .unwrap_or(5);

    let pool = Pool::new(threads_count);

    each_ruby_file(path, &|path| {
        let path = path.to_string();
        pool.execute(move || cb(&path));
        Ok(())
    })
}
