use std::io::prelude::*; //to get access to traits which let us read and write to stream
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); //HTTP default port but we can choose any.
    for stream in listener.incoming(){   
        let stream = stream.unwrap();
        handle_connection(stream);
    } 
    
}

fn handle_connection(mut stream:TcpStream){
   
    let mut buffer= [0;512];
    stream.read(&mut buffer).unwrap(); //write stream to buffer
    
    //println!("Request:{}",String::from_utf8_lossy(&buffer[..]));//lossy means if there is invalid utf-8, replace invalid sequence with '?' char. //Version 1
    //let response = "HTTP/1.1 200 OK\r\n\r\n"; //Version 2

    let get = b"GET / HTTP/1.1\r\n"; //thanks to b we transform string to byte string: &str => &[u8;16]
    if buffer.starts_with(get){

        let contents = fs::read_to_string("web.html").unwrap();
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}",contents);
    
        stream.write(response.as_bytes()).unwrap(); //write response to stream
        stream.flush().unwrap(); //flush means wait until all stream bytes are written. 
    }
    else{
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let contents = fs::read_to_string("404.html").unwrap();
        let response = format!("{}{}",status_line,contents);
    
        stream.write(response.as_bytes()).unwrap(); //write response to stream
        stream.flush().unwrap(); //flush means wait until all stream bytes are written. 
    }

}

//call "cargo run" here. Then look at the "127.0.0.1:7878" address from your browser. 
//You can load the address as "127.0.0.1:7878/" or "127.0.0.1:7878/something"