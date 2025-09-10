use std::{
    thread,
    sync::{Arc, Mutex, mpsc},
};
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> Self { // size kadar worker olusturacak.
                                     // her worker bir thread baslatacak.
        println!("{size} uzunlugundaki threadpool olusturuldu.");

        let (sender, receiver) : (mpsc::Sender<Job>, mpsc::Receiver<Job>) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));


        let mut workers = Vec::with_capacity(size);

        for i  in 0..size {

            let receiver = Arc::clone(&receiver);

            let t = thread::spawn(move || {
                println!("worker {} calisiyor",i);

                loop {
                    let message = receiver.lock().unwrap().recv();
                    match message {
                        Ok(job) => {
                            println!("{i} numarali worker bir job aldi");
                            job();
                        },
                        Err(_) => break,
                    }
                }
            });


            let worker = Worker{
                id : i,
                thread : t,
            };
            workers.push(worker);

        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }

    }

    pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in self.workers.drain(..) {
            println!("{} id'li worker kapatiliyor.", worker.id);
            worker.thread.join().unwrap();
        }
    }
}


struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}