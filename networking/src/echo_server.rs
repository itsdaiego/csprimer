use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

pub fn solve() {
    let listener = TcpListener::bind("0.0.0.0:3050").expect("Could not bind to address");
    println!("Server listening on port 3050");

    loop {
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut buffer = [0; 1024];

                    match stream.read(&mut buffer) {
                        Ok(bytes_read) => {

                            println!("Received a request: {}", String::from_utf8_lossy(&buffer[..bytes_read]));

                            let contents = &buffer[..bytes_read];
                            let input = String::from_utf8_lossy(contents);

                            println!("{}", input.to_uppercase());

                            let response = input.into_owned();

                            thread::spawn(move || {
                                stream.write(response.to_uppercase().as_bytes()).unwrap();
                            });

                        }
                        Err(e) => {
                            panic!("Connection failed: {}", e);
                        }
                    }
                }

                Err(e) => {
                    panic!("Connection failed: {}", e);
                }
            }

        }
    }
}
