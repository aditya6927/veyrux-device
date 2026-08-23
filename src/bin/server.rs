/*
a tcp server:
-> binds to an ip address and port
-> accepts incoming connections in a loop
-> read/write data to the connected client stream
*/

use std::io::{Read, Write};
use std::net::TcpListener;
/*
`TcpListener` is the server-side socket.
It listens for clients that want to establish a TCP connection.

`Read` and `Write` are traits that provide methods for receiving
and sending bytes through the TCP stream.
*/

fn main() -> std::io::Result<()> {
    // bind the listener to a local port
    let listener = TcpListener::bind( "127.0.0.1:8080")?;
    println!("Server listening on 127.0.0.1:8080...");
    /*
    bind returns a Result because creating a listening socket can fail
    '?' means 
    -> if binding succeeds, give us TcpListener
    -> else return the error from main immediately
    */

    // accept incoming client connections
    for stream in listener.incoming() {
        // 'incoming()' produces connection attempts
        // each item is a Result because accepting a connection can fail
        match stream {
            Ok(mut stream) => {
                println!("new connection established...");
                // reading incoming bytes
                let mut buffer = [0; 1024];
                let bytes_read = stream.read(&mut buffer)?;
                /*
                TCP gives a stream of raw bytes, not complete 'messages'
                we provide a 1024-byte buffer as temporary storage for bytes

                'mut' is required because 'read()' will write recieved data
                read() may fill only part of the buffer

                bytes_read tells us how many bytes this read()
                call actually placed into the buffer

                if 18 bytes recieved:
                -> only interpret buffer[0...18] as the recieved data
                */

                let recievd_msg = String::from_utf8_lossy(&buffer[..bytes_read]);
                println!("Recieved: {}", recievd_msg);

                // send reply back to client
                let response = "hello from server!";
                stream.write_all(response.as_bytes())?;
            }
            Err(e) => {
                println!("connection failed: {}", e);
            }
        }
    }
    Ok(())
}