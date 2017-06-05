use std::net;

const PROXY_PORT: u16 = 4000;

fn main() {
    let listener = net::TcpListener::bind(("127.0.0.1", PROXY_PORT)).unwrap();

    match listener.accept() {
        Ok((sock, _)) => handle_connection(sock),
        Err(e) => panic!("Error while accepting connection: {}", e),
    }
}

mod http;

fn handle_connection(tcp: net::TcpStream) {
    let mut h = http::HttpStream::new(tcp);

    while !h.is_closed() {
        let request = match h.get() {
            Ok(r) => r,
            Err(e) => {
                println!("Error while getting http request: {}", e);
                return;
            }
        };
        h.send(&http::HttpPacket{});
    }
}
