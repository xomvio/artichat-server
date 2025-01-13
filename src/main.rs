use std::net::{TcpListener, UdpSocket};
use std::io::Read;
fn main() {

    let socket = UdpSocket::bind("127.0.0.1:9595").unwrap();

    loop {
        let mut buffer = [0; 1024];
        match socket.recv_from(&mut buffer) {
            Ok((size, addr)) => {
                println!("Request: {}", String::from_utf8_lossy(&buffer));
                socket.send_to(&buffer, addr).unwrap();
            },
            Err(e) => println!("Failed to read from connection: {}", e),
        }
    }
}
