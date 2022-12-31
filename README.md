This repo contains the example from Rust book. I wrote it with comments and with suggestions for Rust coding.
It creates a web server which works with multithread. HTTP and TCP protocols should be a bit known to understand well. 
Load the "127.0.0.1:7878" address from your browser to check it after calling "cargo run".
Btw in some browsers, we could not load this page, so check it with another browsers if you couldnot succeed to see result.  
Also it can take 10 request, then drop method will be active. If you need unlimited request, you should uncomment "for stream in listener.incoming()" in main.rs

- to run it by: **cargo run**
- to see the documents of functions: **cargo doc --open** 
- to load the web in browser: **127.0.0.1:7878/** 
- to load the web in browser with 5 sec delay: **127.0.0.1:7878/sleep**
