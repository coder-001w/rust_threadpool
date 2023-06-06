use std::{
    sync::mpsc::{self, Receiver},
    sync::Arc,
    sync::Mutex,
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;
// 存放用于向信道中发送的闭包。
// struct Job;
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>, // 储存了使用一个空闭包创建的 JoinHandle<()>。
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap(); // 从信道中接收闭包并执行。

            println!("Worker {id} got a job; executing.");

            job();
        });

        Worker { id, thread }
    }
}
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// 创建线程池。
    ///
    /// 线程池中线程的数量。
    ///
    /// # Panics
    ///
    /// `new` 函数在 size 为 0 时会 panic。
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // 标准库提供的创建线程的方法，
            // thread::spawn，它期望获取一些一旦创建线程就应该执行的代码。
            // 然而，我们希望开始线程并使其等待稍后传递的代码。
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static, // FnOnce 代表闭包只能调用一次，Send 代表闭包可以在线程间传递，'static 代表闭包中的所有值都有 'static 生命周期
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}
