// Main modules of the program.
mod http;
mod write_to;

use std::collections::HashMap;
use std::net;
use std::vec::Vec;
use write_to::WriteTo;

const PROXY_PORT: u16 = 4000;

fn main() {
    let listener = net::TcpListener::bind(("127.0.0.1", PROXY_PORT)).unwrap();

    loop {
        match listener.accept() {
            Ok((sock, _)) => {
                match handle_connection(sock) {
                    Ok(()) => (),
                    Err(e) => println!("Connection ended with {:?}", e),
                }
            }
            Err(e) => println!("Error while accepting connection: {:?}", e),
        }
    }
}


fn handle_connection(tcp: net::TcpStream) -> std::io::Result<()> {
    let mut h = http::HttpStream::new(&tcp);

    while !h.is_closed() {
        let request = match h.get_request()? {
            Some(r) => r,
            None => return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "")),
        };

        let mut response_body = Vec::new();
        request.write_to(&mut response_body).unwrap();

        let mut fields = HashMap::new();
        fields.insert("Content-Length".to_owned(), response_body.len().to_string());

        h.send(&http::Response {
                   header: http::ResponseHeader {
                       fields: fields,
                       protocol: request.header.protocol.clone(),
                       status_code: 200,
                       status_desc: "OK".to_owned(),
                   },
                   body: response_body,
               });
    }

    Ok(())
}
