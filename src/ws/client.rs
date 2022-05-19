use tungstenite::connect;


pub struct Client {
}

pub fn test_connect() {
    let (mut _socket, response) = connect("wss://fsserver-gyd.glodon.com/").unwrap();
    println!("websocket connected");
    println!("status: {}", response.status());
}
