use std::io::{BufReader, BufWriter, Error, ErrorKind, Read, Result, Write};
use std::net;
use std::option::Option;
use std::string::String;
use write_to::WriteTo;

mod header;
pub use self::header::{ResponseHeader, RequestHeader, RequestMethod};

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
        Ok(Some(Request { header: RequestHeader::parse(self.read_raw_header()?.lines())? }))
    }

    pub fn send<WT: WriteTo>(&mut self, packet: &WT) -> Result<()> {
        packet.write_to(&mut self.tcp_writer)?;
        self.tcp_writer.flush()
    }

    fn read_raw_header(&mut self) -> Result<String> {
        let mut input = String::new();
        let mut bytes = self.tcp_reader.by_ref().bytes();
        while !input.ends_with("\r\n\r\n") {
            let x = match bytes.next() {
                None => return Err(Error::new(ErrorKind::UnexpectedEof, "")),
                Some(r) => r? as char,
            };
            input.push(x);
        }
        Ok(input)
    }

    pub fn release(mut self) -> Result<Vec<u8>> {
        use std::io::BufRead;
        self.tcp.set_nonblocking(true);
        let x = Ok(self.tcp_reader.fill_buf()?.to_vec());
        self.tcp.set_nonblocking(false);
        x
    }
}


#[derive (Clone, Debug)]
pub struct Request {
    pub header: RequestHeader,
}

impl WriteTo for Request {
    fn write_to<W: Write>(&self, w: &mut W) -> Result<()> {
        self.header.write_to(w.by_ref())
    }
}

#[derive (Clone,Debug)]
pub struct Response {
    pub body: Vec<u8>,
    pub header: ResponseHeader,
}

impl WriteTo for Response {
    fn write_to<W: Write>(&self, w: &mut W) -> Result<()> {
        self.header.write_to(w.by_ref())?;
        w.write_all(self.body.as_slice())
    }
}
