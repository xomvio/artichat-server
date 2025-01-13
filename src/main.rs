use std::collections::HashMap;
use std::net::{SocketAddr, TcpListener, UdpSocket};
use std::io::Read;
fn main() {

    let socket = UdpSocket::bind("127.0.0.1:9595").unwrap();

    let mut rooms: HashMap<String, Vec<SocketAddr>> = HashMap::new();
    loop {
        let mut buffer = [0; 1024];
        match socket.recv_from(&mut buffer) {
            Ok((size, addr)) => {
                println!("Request: {}", String::from_utf8_lossy(&buffer));
                let room: String = String::from_utf8_lossy(&buffer[..32]).to_string();
                match rooms.get_mut(&room) {
                    Some(users) => {
                        if !users.contains(&addr) {
                            users.push(addr);
                        }
                        for user in users.iter() {
                            socket.send_to(&buffer[32..size], *user).unwrap();
                        }
                    },
                    None => {
                        rooms.insert(room.clone(), vec![addr]);
                    }
                }
                //socket.send_to(&buffer[32..size], addr).unwrap();
            },
            Err(e) => println!("Failed to read from connection: {}", e),
        }
    }
}
