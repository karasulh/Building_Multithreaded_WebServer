#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::thread::{self, JoinHandle};
use std::sync::mpsc;//for sender-receiver
use std::sync::Arc;
use std::sync::Mutex;

//thread::spawn signature:
//pub fn spawn<F,T>(f:F)->JoinHandle<T> where F:FnOnce()+T+Send+'static, T:Send+'static

enum Message{
    NewJob(Job),
    Terminate,
}


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
    sender : mpsc::Sender<Message>
}

impl ThreadPool{
    /// Create a new ThreadPool
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
    /// Send the closures as Message/Job 
    /// 
    /// The closure is the function which will be done by workers as threads
    /// 
    /// #Panics
    /// 
    /// The 'execute' function will panic if the sender could not send the message.
    //we deduce f from the signature of thread::spawn which takes closure as argument. 
    //Think FnOnce() as function without argument and return value
    //Send trait: transfer the closure from one thread to another
    //'static: we dont know how long the thread will take to execute
    pub fn execute<F>(&self,f:F) where F: FnOnce() + Send + 'static { 
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool{
    fn drop(&mut self){
        println!("Sending terminate message to all workers.");
        for _ in &mut self.workers{
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("Shutting down all workers");

        for worker in &mut self.workers{
            println!("Shutting down worker {}",worker.id);
            //worker.thread.join().unwrap(); //join try to take ownership, but thread is borrowed, so it doesnot allow to use like that. //Version6
            //To solve this, we need to move the thread out of Worker instance which own thread, then join can consume the thread. => use "take" method with Option
            if let Some(thread) = worker.thread.take(){
                thread.join().unwrap();
            }
        }
    }
}


pub struct Worker{
    id:usize,
    thread: Option<thread::JoinHandle<()>>, //use it to surpass the problem in 'drop' 
    //thread: thread::JoinHandle<()> //Version 6
}

impl Worker{
    /// Create a new Worker
    /// 
    /// The id is the spesific number of Worker
    /// 
    /// The receiver is the receiver side of the channel 
    /// 
    /// #Panics
    /// 
    /// The 'new' function will panic if the mutex locking has a problem or the receiver has a problem.
    fn new(id:usize,receiver:Arc<Mutex<mpsc::Receiver<Message>>>)->Worker{

        let thread = thread::spawn(move ||{ 
            loop{ //put a loop to recv waits until take any input to respond many requests by our threads //if no loop, only 4 request is ok. 
                let message = receiver.lock().unwrap().recv().unwrap();
                match message{
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing",id);
                        //(*job)(); //it doesnot allow to use it because it thinks this value will be moved out but the size is not known. 
                        //When the closure is called, closure needs to move itself out of the Box<T> because closure takes ownership of self when we call it.
                        //Instead, we take ownership value inside Box<T> using self:Box<Self>, then, once we have ownership of the closure, we can call it. 
                        //To surpass this problem, we created trait FnBox and use it like that:
                        job.call_box();
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate",id);
                        break;
                    },
                }
            }
        });
        Worker{
            id,
            thread:Some(thread), 
        }
    }
}



#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn is_id_of_worker_thesame_wgiven(){
        let id = 1;
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver)); 
        assert_eq!(Worker::new(id,receiver).id,id);
    }

    #[test]
    fn is_threadpool_workersvectorsize_3(){
        let mut vecworkers:Vec<Worker>=Vec::new();
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver)); 
        for id in 0..3{
            vecworkers.push(Worker::new(id,Arc::clone(&receiver)));
        }
        assert_eq!(ThreadPool::new(3).workers.len(),vecworkers.len());
    }

}
