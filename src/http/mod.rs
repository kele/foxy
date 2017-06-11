use std::io::Read;
use std::io::Write;
use std::io;
use std::net;
use std::string;

pub struct HttpStream<'a> {
    tcp: &'a net::TcpStream,
    tcp_bytes: io::Bytes<io::BufReader<&'a net::TcpStream>>,
}

impl<'a> HttpStream<'a> {
    pub fn new(tcp: &'a net::TcpStream) -> HttpStream<'a> {
        HttpStream {
            tcp: tcp,
            tcp_bytes: io::BufReader::new(tcp).bytes(),
        }
    }

    pub fn get_request(&mut self) -> io::Result<HttpRequest> {
        // TODO: error handling
        let mut lastChar = '\0';
        let mut packet = string::String::new();

        loop {
            let x = self.tcp_bytes.next().unwrap().unwrap() as char;
            packet.push(x);
            if lastChar == '\n' && x == '\n' {
                break;
            }
            lastChar = x;
        }

        Ok(HttpRequest { data: packet })
    }

    pub fn send(&mut self, resp: &HttpResponse) -> io::Result<()> {
        // TODO: error handling
        self.tcp.write(resp.data.to_string().as_bytes());
        Ok(())
    }

    pub fn is_closed(&self) -> bool {
        // TODO
        false
    }
}

pub struct HttpRequest {
    pub data: string::String,
}

pub struct HttpResponse {
    pub data: string::String,
}
