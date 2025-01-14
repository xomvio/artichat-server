use std::collections::HashMap;
use std::net::{SocketAddr, TcpListener, TcpStream, UdpSocket};
use std::io::Read;

struct User {
    pub name: String,
    pub addr: SocketAddr
}
fn main() {

    let socket = UdpSocket::bind("303:c8b5:db69:fc6d::3131:9595").unwrap();

    let mut rooms: HashMap<String, Vec<User>> = HashMap::new();
    loop {
        let mut buffer = [0; 1024];
        match socket.recv_from(&mut buffer) {
            Ok((size, addr)) => {
                println!("Request: {}", String::from_utf8_lossy(&buffer));
                let room: String = String::from_utf8_lossy(&buffer[..32]).to_string();
                match rooms.get_mut(&room) {// is there a room with this id
                    Some(users) => {// there is
                        // check if the user is already in the room
                        if users.iter().find(|user| user.addr == addr).is_none() {// user is not in the room yet
                            // add him to the room
                            users.push(User { name: String::from_utf8_lossy(&buffer[32..size]).to_string(), addr });
                            // send all usernames in the room to the new user
                            for user in users.iter() {
                                if user.addr == addr {
                                    continue;
                                }
                                // send message
                                socket.send_to(user.name.as_bytes(), addr).unwrap();
                            }
                        }
                        
                        // send message to all users in the room
                        for user in users.iter() { // for each user
                            // send message
                            socket.send_to(&buffer[32..size], user.addr).unwrap();
                        }
                    },
                    None => {// there isn't
                        // create a new room
                        rooms.insert(room.clone(), vec![User { name: String::from_utf8_lossy(&buffer[32..size]).to_string(), addr }]);
                        // only send his own name to room creator because he's the only one in the room. lonely.
                        socket.send_to(&buffer[32..size], addr).unwrap();
                    }
                }
            },
            Err(e) => println!("Failed to read from connection: {}", e),
        }
    }
}

#[test]
fn test() {
    let mut socket = UdpSocket::bind("303:c8b5:db69:fc6d::3131:9595").unwrap();

    let mut buffer = [0; 1024];
    let x = socket.recv_from(&mut buffer).unwrap();
    socket.send_to(b"hi! from server", x.1).unwrap();
    println!("{}", String::from_utf8_lossy(&buffer));
}


#[test]
fn test2() {
    let mut stream = TcpStream::connect("109.176.250.101:65534").unwrap();
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("AGA! {}", String::from_utf8_lossy(&buffer));
}