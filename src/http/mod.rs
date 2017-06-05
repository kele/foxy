use std::net;
use std::io;

pub struct HttpStream {
    tcp: net::TcpStream,
}

impl HttpStream {
    pub fn new(tcp: net::TcpStream) -> HttpStream {
        HttpStream { tcp: tcp }
    }

    pub fn get(&mut self) -> io::Result<HttpPacket> {
        // TODO
        Ok(HttpPacket {})
    }

    pub fn send(&mut self, packet: &HttpPacket) -> io::Result<()> {
        // TODO
        Ok(())
    }

    pub fn is_closed(&self) -> bool {
        // TODO
        false
    }
}

pub struct HttpPacket {}
