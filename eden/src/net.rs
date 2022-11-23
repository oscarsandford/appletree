use std::{
	thread, sync::{mpsc, Arc, Mutex}
};

type Job = Box<dyn Send + 'static + FnOnce()>;

struct Worker {
	id: usize,
	handle: Option<thread::JoinHandle<()>>,
}
impl Worker {
	fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
		let handle = thread::spawn(move || loop {
			let job = receiver.lock().expect("Mutex could not be locked").recv();
			if let Ok(job) = job {
				println!("~> Worker {id} : processing job.");
				job();
			} 
			else {
				println!("Worker {id} : sender disconnected.");
				break;
			}
		});
		Worker { id, handle: Some(handle) }
	}
}

pub struct ThreadPool {
	workers: Vec<Worker>,
	sender: Option<mpsc::Sender<Job>>,
}
impl ThreadPool {
	pub fn new(size: usize) -> ThreadPool {
		let (sender, receiver) = mpsc::channel();
		let receiver = Arc::new(Mutex::new(receiver));
		let mut workers = Vec::with_capacity(size);

		for i in 0..size {
			workers.push(Worker::new(i, Arc::clone(&receiver)));
		}
		ThreadPool { workers, sender: Some(sender) }
	}

	pub fn execute<F: Send + 'static + FnOnce()>(&self, f: F) {
		let job = Box::new(f);
		match self.sender.as_ref() {
			Some(sender) => {
				if let Err(e) = sender.send(job) {
					eprintln!("Error while sending: {e:?}");
				}
			},
			None => { eprintln!("Sender already dropped."); }
		}
	}
}
impl Drop for ThreadPool {
	fn drop(&mut self) {
		drop(self.sender.take());

		for w in &mut self.workers {
			if let Some(handle) = w.handle.take() {
				println!("Worker {} : shutting down.", w.id);
				handle.join().expect("Could not join on thread");
			};
		}
	}
}