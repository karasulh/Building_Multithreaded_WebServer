use std::thread::{self, JoinHandle};

//thread::spawn signature:
//pub fn spawn<F,T>(f:F)->JoinHandle<T> where F:FnOnce()+T+Send+'static, T:Send+'static

pub struct ThreadPool{
    workers : Vec<Worker> //We deduce the type from signature of thread::spawn, there is no return value so we use ().
}

impl ThreadPool{
    ///Create a new ThreadPool
    /// 
    /// The size is the number of threads in Pool
    /// 
    /// #Panics
    /// 
    /// The 'new' function will panic if the size is zero.
    pub fn new(size:usize)->ThreadPool{
        assert!(size>0);

        let mut workers = Vec::with_capacity(size);
        for id in 0..size{
            workers.push(Worker::new(id));
        }
        ThreadPool{
            workers
        }
    }
    //we deduce f from the signature of thread::spawn which takes closure as argument. 
    //Think FnOnce() as function without argument and return value
    //Send trait: transfer the closure from one thread to another
    //'static: we dont know how long the thread will take to execute
    pub fn execute<F>(&self,f:F) where F: FnOnce() + Send + 'static { 
        f();
    }
}

pub struct Worker{
    id:usize,
    thread: thread::JoinHandle<()>
}

impl Worker{
    fn new(id:usize)->Worker{
        let thread = thread::spawn(||{});
        Worker{
            id,
            thread 
        }
    }
}