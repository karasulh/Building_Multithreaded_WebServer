This repo contains the example from Rust book. I wrote it with comments and with suggestions for Rust coding.
It creates a web server which works with multithread. HTTP and TCP protocols should be a bit known to understand well. 
Load the "127.0.0.1:7878" address from your browser to check it after calling "cargo run".
Btw in some browsers, we couldnot load this page, so check it with another browsers if you couldnot succeed to see result.  

- to unset environment variable: **$env:CASE_INSENSITIVE=""** 
- to set environment variable: **$env:CASE_INSENSITIVE=1** 
- to run it by: **cargo run body poem.txt**
- to run it to see results with standard output stream(stdout) in output.txt file by: **cargo run body poem.txt > output.txt** 
