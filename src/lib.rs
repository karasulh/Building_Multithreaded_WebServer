#![allow(unused_imports)]
#![allow(dead_code)]

use std::thread::{self, JoinHandle};
use std::sync::mpsc;//for sender-receiver
use std::sync::Arc;
use std::sync::Mutex;

//thread::spawn signature:
//pub fn spawn<F,T>(f:F)->JoinHandle<T> where F:FnOnce()+T+Send+'static, T:Send+'static

trait FnBox{ //a tricky way to surpass the problem in Worker::new().
    fn call_box(self:Box<Self>);
}
impl<F: FnOnce()> FnBox for F{
    fn call_box(self:Box<F>) {
        (*self)()
    }
}

//struct Job;//Version 4
//type Job = Box<dyn FnOnce() + Send + 'static>; //Version 5
type Job = Box<dyn FnBox + Send + 'static>;

pub struct ThreadPool{
    workers : Vec<Worker>, //We deduce the type from signature of thread::spawn, there is no return value so we use ().
    sender : mpsc::Sender<Job>
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

        let (sender, receiver) = mpsc::channel();

        //we must have many sender but only one receiver acc to channel so we give an atomic reference(arc) to the same receiver to all threads.
        //Also this receiver should be protected with mutex.
        let receiver = Arc::new(Mutex::new(receiver)); 
        
        let mut workers = Vec::with_capacity(size);
        for id in 0..size{
            workers.push(Worker::new(id,Arc::clone(&receiver)));
        }


        ThreadPool{
            workers,
            sender
        }
    }
    //we deduce f from the signature of thread::spawn which takes closure as argument. 
    //Think FnOnce() as function without argument and return value
    //Send trait: transfer the closure from one thread to another
    //'static: we dont know how long the thread will take to execute
    pub fn execute<F>(&self,f:F) where F: FnOnce() + Send + 'static { 
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}


pub struct Worker{
    id:usize,
    thread: thread::JoinHandle<()>
}

impl Worker{
    fn new(id:usize,receiver:Arc<Mutex<mpsc::Receiver<Job>>>)->Worker{

        let thread = thread::spawn(move ||{ 
            loop{ //put a loop to recv waits until take any input to respond many requests by our threads //if no loop, only 4 request is ok. 
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {} got a job; executing",id);
                //(*job)(); //it doesnot allow to use it because it thinks this value will be moved out but the size is not known. 
                //When the closure is called, closure needs to move itself out of the Box<T> because closure takes ownership of self when we call it.
                //Instead, we take ownership value inside Box<T> using self:Box<Self>, then, once we have ownership of the closure, we can call it. 
                //To surpass this problem, we created trait FnBox and use it like that:
                job.call_box();
            }
        });
        Worker{
            id,
            thread 
        }
    }
}