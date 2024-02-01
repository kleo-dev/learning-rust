use std;
use std::net::{TcpListener, TcpStream};

fn main() {}

fn handle_client(mut stream: TcpStream) {
    // read 20 bytes at a time from stream echoing back to stream
    loop {
        let mut read = [0; 1028];
        match stream.read(&mut read) {
            Ok(n) => {
                if n == 0 {
                    // connection was closed
                    break;
                }
                println!("Friend: {}", &read[0..n]);
                let input_to_client = String::new();
                std::io::stdin().read_line(&mut input_to_client);
                stream.write_all(input_to_client.as_bytes()).unwrap();
            }
            Err(err) => {
                panic!(err);
            }
        }
    }
}

fn server() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(_) => {
                println!("Error");
            }
        }
    }
}

fn client() {
    let mut stream = TcpStream::connect("127.0.0.1:6969").unwrap();
    stream.write_all("Hello, World").unwrap();
}

pub fn run() {
    let mut input = String::new();
    println!("[1] SERVER\n[2] CLIENT");
    std::io::stdin().read_line(&mut input);
    let _value = input.trim();
    if _value == "1" {
        server();
    } else if _value == "2" {
        client();
    }
}
