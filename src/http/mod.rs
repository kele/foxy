use std::io::{Bytes, BufReader, BufWriter, Error, ErrorKind, Read, Result, Write};
use std::net;
use std::option::Option;
use std::string::String;
use write_to::WriteTo;

use std::boxed::Box;

mod header;
pub use self::header::{ResponseHeader, RequestHeader};

pub struct HttpStream<'a> {
    tcp: &'a net::TcpStream,
    tcp_reader: BufReader<&'a net::TcpStream>,
    tcp_writer: BufWriter<&'a net::TcpStream>,
}


impl<'a> HttpStream<'a> {
    pub fn new(tcp: &'a net::TcpStream) -> Self {
        HttpStream {
            tcp: tcp,
            tcp_reader: BufReader::new(tcp),
            tcp_writer: BufWriter::new(tcp),
        }
    }

    pub fn get_request(&mut self) -> Result<Option<Request>> {
        let mut input = String::new();
        let mut bytes = self.tcp_reader.by_ref().bytes();
        while !input.ends_with("\r\n\r\n") {
            let x = match bytes.next() {
                None => return Err(Error::new(ErrorKind::UnexpectedEof, "")),
                Some(r) => r? as char,
            };
            input.push(x);
        }

        let header = RequestHeader::parse(input.lines())?;
        let body = Vec::new(); // TODO: read the body

        Ok(Some(Request {
                    header: header,
                    body: body,
                }))
    }

    pub fn send<WT>(&mut self, packet: &WT) -> Result<()>
        where WT: WriteTo<BufWriter<&'a net::TcpStream>>
    {
        packet.write_to(&mut self.tcp_writer)?;
        self.tcp_writer.flush()
    }

    pub fn is_closed(&self) -> bool {
        // TODO
        false
    }
}


pub struct Request {
    pub header: RequestHeader,
    pub body: Vec<u8>,
}

impl<W: Write> WriteTo<W> for Request {
    fn write_to(&self, w: &mut W) -> Result<()> {
        self.header.write_to(w.by_ref())?;
        w.write_all(self.body.as_slice())
    }
}

pub struct Response {
    pub body: Vec<u8>,
    pub header: ResponseHeader,
}

impl<W: Write> WriteTo<W> for Response {
    fn write_to(&self, w: &mut W) -> Result<()> {
        self.header.write_to(w.by_ref())?;
        w.write_all(self.body.as_slice())
    }
}
