/*
a TCP client needs to perform two main actions:
-> connect to the server address (ip address and port), eg: 127.0.0.1:8080
-> send a byte message and read server's response
(sends data to server, reads server's serponse)
*/

use std::io::{Read, Write};
use std::net::TcpStream;
/*
TcpStream represents an active TCP connection

unlike TcpListener, which waits for connecting,
TcpStream is used to communicate with the other endpoint

Read and Write are traits that provide methods for
recieving and sending bytes through the stream
*/

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    println!("connected to server");
    /*
    establishing a TCP connection to the server 

    connect() returns a Result because the connection can fail
    if the server is not running

    ? => 
    -> 1f connection succeeds give us TcpStream
    -> if it fails return the error from main
    */

    let message = "hello from client...";
    stream.write_all(message.as_bytes())?;
    /*
    send data to the server

    TCP works with bytes, so string must be converted
    into a byte slice .as_bytes()

    write_all() keeps writing until all bytes have been sent
    or an error occurs
    */

    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;
    /*
    a temporary buffer for the response

    mut is required as read()
    writes the recieved bytes into the buffer

    read() reads upto 1024 bytes from the TCP stream
    it returns the number of bytes actually recieved

    TCP gives us a byte stream 
    */

    let response = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("server response: {}", response);

    /*
    only the first 'bytes_read' bytes contain the reaponse

    '&buffer[..bytes_read]' creates a slice containing
    exactly the bytes that were recieved

    `from_utf8_lossy()` interprets those bytes as UTF-8
    text so that we can print them.
    */

    Ok(())
    /*
    if execution reaches thsi point, all operations above succeeded

    `()` is Rust's unit value, meaning there is no meaningful
    value to return.
    Therefore `Ok(())` means the program completed successfully.
    */
}