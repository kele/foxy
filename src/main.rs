use std::net;

const PROXY_PORT: u16 = 4000;

fn main() {
    let listener = net::TcpListener::bind(("127.0.0.1", PROXY_PORT)).unwrap();

    match listener.accept() {
        Ok((sock, _)) => handle_connection(sock),
        Err(e) => panic!("Error while accepting connection: {}", e),
    }
}

fn handle_connection(tcp: net::TcpStream) {
    println!("Opened connection: {:?}", tcp)
}
