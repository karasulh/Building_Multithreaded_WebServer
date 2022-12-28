use std::io::prelude::*; //to get access to traits which let us read and write to stream
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use std::thread;
use std::time::Duration;
use web_server::ThreadPool; //we added lib.rs into src folder and moved main.rs into bin folder, than our web_server crate becomes a lib crate.

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); //HTTP default port but we can choose any.
    let pool = ThreadPool::new(4);

    for stream in listener.incoming(){   
        let stream = stream.unwrap();

        pool.execute(||{
            handle_connection(stream);
        });  
    }  
}

fn handle_connection(mut stream:TcpStream){
   
    let mut buffer= [0;512];
    stream.read(&mut buffer).unwrap(); //write stream to buffer
    
    //println!("Request:{}",String::from_utf8_lossy(&buffer[..]));//lossy means if there is invalid utf-8, replace invalid sequence with '?' char. //Version 1
    //let response = "HTTP/1.1 200 OK\r\n\r\n"; //Version 2

    let get = b"GET / HTTP/1.1\r\n"; //thanks to b we transform string to byte string: &str => &[u8;16]
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line,filename) = if buffer.starts_with(get){
        ("HTTP/1.1 200 OK\r\n\r\n","web.html")
    } else if buffer.starts_with(sleep){
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n","web.html")
    } else{
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n","404.html") 
    };

    let contents = fs::read_to_string(filename).unwrap(); //Version 3
    let response = format!("{}{}",status_line,contents);//Version 3

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}

//call "cargo run" here. Then look at the "127.0.0.1:7878" address from your browser. 
//You can load the address as "127.0.0.1:7878/" or "127.0.0.1:7878/something"