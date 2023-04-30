use std::io::Read;
use std::net::TcpListener;

pub fn solve() {
    let listener = TcpListener::bind("0.0.0.0:3050").expect("Could not bind to address");
    println!("Server listening on port 3050");

    loop {
        for stream in listener.incoming() {
            let mut buffer = [0; 1024];

            let bytes_read = stream.unwrap().read(&mut buffer).unwrap();

            let input = String::from_utf8_lossy(&buffer[..bytes_read]);
            
            let headers: Vec<&str> = input
                .lines()
                .skip(1)
                .take_while(|line| !line.is_empty())
                .collect();

            
            let mut json_response = String::from("{");

            for header in headers {
                let mut parts = header.split(": ");
                let key = parts.next().unwrap();
                let value = parts.next().unwrap();
                json_response.push_str(&format!("\"{}\": \"{}\",", key, value));
            }

            json_response.pop().unwrap();
            json_response.push_str("}");

            println!("{}", json_response);
        }
    }
}
