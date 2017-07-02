// Main modules of the program.
mod http;
mod write_to;

use std::net;
use std::thread;

const PROXY_PORT: u16 = 4000;


fn main() {
    let listener = net::TcpListener::bind(("127.0.0.1", PROXY_PORT)).unwrap();

    loop {
        match listener.accept() {
            Ok((sock, _)) => {
                thread::spawn(move || handle_connection(sock));
            }
            Err(e) => println!("Error while accepting connection: {:?}", e),
        }
    }
}


use std::sync::Arc;

fn handle_connection(tcp: net::TcpStream) -> std::io::Result<()> {
    use http::RequestMethod;
    use std::io::Write;
    use write_to::WriteTo;

    let remote = {
        let mut h = http::HttpStream::new(&tcp);
        let initial_request = h.get_request()?.ok_or_else(|| eof("TODO"))?;

        let remote_port = match initial_request.header.method {
            RequestMethod::Get => 80,
        };

        let mut remote = {
            let remote_addr = (initial_request.header.fields.get("Host").unwrap().as_str(),
                               remote_port);
            net::TcpStream::connect(remote_addr)?
        };

        // Forward the initial request.
        initial_request.write_to(&mut remote);

        // Forward what was left in the HttpStream buffer.
        let leftovers = h.release().unwrap_or_default();

        remote.write_all(leftovers.as_slice());
        remote
    };

    let tcp = Arc::new(tcp);
    let remote = Arc::new(remote);

    spawn_tunnel(tcp.clone(), remote.clone());
    spawn_tunnel(remote.clone(), tcp.clone());

    Ok(())
}


fn spawn_tunnel(r: Arc<net::TcpStream>,
                w: Arc<net::TcpStream>)
                -> std::thread::JoinHandle<std::io::Result<()>> {
    thread::spawn(move || {
        loop {
            let n = std::io::copy(&mut r.as_ref(), &mut w.as_ref())?;
            println!("Transferred {} bytes between {:?} and {:?}", n, r, w);
            if n == 0 {
                break;
            }
        }
        Ok(())
    })
}

fn eof(s: &str) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::UnexpectedEof, s)
}
