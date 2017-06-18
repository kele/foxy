use std::collections::HashMap;
use std::fmt;
use std::io::{Error, ErrorKind, Result, Write};
use std::str::Lines;
use std::vec::Vec;
use write_to::WriteTo;

#[derive (Clone, Copy)]
pub enum RequestMethod {
    Get,
    // TODO: add more methods here
}

impl fmt::Display for RequestMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RequestMethod::Get => write!(f, "GET"),
        }
    }
}

pub struct RequestHeader {
    pub method: RequestMethod,
    pub uri: String,
    pub protocol: String,
    pub fields: HashMap<String, String>,
}

impl RequestHeader {
    pub fn parse(mut lines: Lines) -> Result<Self> {
        let request_line = lines.next().unwrap_or_default();
        let parts: Vec<_> = request_line.trim().split_whitespace().collect();
        if parts.len() != 3 {
            let err_msg = format!("Request header should consist of three parts. Got: {:?}",
                                  request_line);
            return Err(Error::new(ErrorKind::InvalidData, err_msg));
        }
        let (method, uri, protocol) = (parts[0], parts[1], parts[2]);

        println!("{} {} {}", method, uri, protocol);
        let method = match method {
            "GET" => RequestMethod::Get,
            _ => {
                let err_msg = format!("Unsupported request method. Request line: {:?}",
                                      request_line);
                return Err(Error::new(ErrorKind::InvalidData, err_msg));
            }
        };

        Ok(RequestHeader {
               method: method,
               uri: uri.to_owned(),
               protocol: protocol.to_owned(),
               fields: parse_fields(lines)?,
           })
    }
}

impl<W: Write> WriteTo<W> for RequestHeader {
    fn write_to(&self, w: &mut W) -> Result<()> {
        write!(w,
               "{method} {uri} {protocol}\r\n",
               method = self.method,
               uri = self.uri,
               protocol = self.protocol)?;

        for (key, value) in self.fields.iter() {
            write!(w, "{}: {}\r\n", key, value)?
        }
        write!(w, "\r\n")
    }
}


#[derive (Clone)]
pub struct ResponseHeader {
    pub fields: HashMap<String, String>,

    pub protocol: String,
    pub status_code: u16,
    pub status_desc: String,
}

impl ResponseHeader {
    pub fn write_to<W: Write>(&self, mut w: W) -> Result<()> {
        write!(w,
               "{protocol} {status_code} {status_desc}\r\n",
               protocol = self.protocol,
               status_code = self.status_code,
               status_desc = self.status_desc)?;

        for (key, value) in self.fields.iter() {
            write!(w, "{}: {}\r\n", key, value)?
        }
        write!(w, "\r\n")
    }

    pub fn parse(mut lines: Lines) -> Result<Self> {
        let response_line = lines.next().unwrap_or_default();
        let parts: Vec<_> = response_line.trim().split_whitespace().collect();
        if parts.len() != 3 {
            let err_msg = format!("Response header should consist of three parts. Got: {:?}",
                                  response_line);
            return Err(Error::new(ErrorKind::InvalidData, err_msg));
        }
        let (protocol, status_code, status_desc) = (parts[0], parts[1], parts[2]);
        let status_code = status_code
            .parse()
            .map_err(|_| {
                         Error::new(ErrorKind::InvalidData,
                                    format!("Status code cannot be parsed. Got: {:?}", status_code))
                     })?;
        Ok(ResponseHeader {
               protocol: protocol.to_owned(),
               status_code: status_code,
               status_desc: status_desc.to_owned(),
               fields: parse_fields(lines)?,
           })
    }
}


fn parse_fields(lines: Lines) -> Result<HashMap<String, String>> {
    let mut fields = HashMap::new();

    for line in lines.filter(|x| !x.is_empty()) {
        let parts: Vec<_> = line.trim().splitn(2, ": ").collect();
        if parts.len() != 2 {
            let err_msg = format!("Header field should have form \"X: Y\". Got: {:?}", line);
            return Err(Error::new(ErrorKind::InvalidData, err_msg));
        }
        fields.insert(parts[0].to_owned(), parts[1].to_owned());
    }
    Ok(fields)
}
