use std::io::{Error, Read};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let addr = std::env::args()
        .skip(1)
        .next()
        .unwrap_or_else(|| "0.0.0.0:8080".to_string());
    println!("Listenning on {}", addr);

    let listener = TcpListener::bind(addr).expect("TCP listener err");
    for stream in listener.incoming() {
        match stream {
            Err(e) => {
                eprintln!("failed: {}", e)
            }
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("Incoming from: {}", stream.peer_addr()?);
    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
        // stream.write(&buf[..bytes_read])?;
        // println!("read len={}", bytes_read);
        let s = std::str::from_utf8(&buf[..bytes_read]).unwrap();
        println!("{}", s);
    }
}
