extern crate tokio;
extern crate tokio_timer;
extern crate tokio_dns;

use std::time::{Duration, Instant};
use tokio::prelude::*;
use self::tokio::net::TcpStream;
use self::tokio::net::tcp::ConnectFuture;
use self::tokio::io;
use std::net::{IpAddr, SocketAddr, Ipv4Addr};
use self::tokio_dns::IoFuture;

pub struct ResolveAndConnect {
    state: State,
}

enum State {
    // Currently resolving the host name
    Resolving(IoFuture<Vec<IpAddr>>),

    // Establishing a TCP connection to the remote host
    Connecting(ConnectFuture),
}

pub fn resolve_and_connect(host: &str) -> ResolveAndConnect {
    let state = State::Resolving(tokio_dns::resolve_ip_addr(host));
    ResolveAndConnect { state }
}

impl Future for ResolveAndConnect {
    type Item = TcpStream;
    type Error = io::Error;

    fn poll(&mut self) -> Result<Async<TcpStream>, io::Error> {
        use self::State::*;

        loop {
            let addr = match self.state {
                Resolving(ref mut fut) => {
                    try_ready!(fut.poll())
                }
                Connecting(ref mut fut) => {
                    return fut.poll();
                }
            };

            println!("Resolved DNS: {:?}", &addr);
            let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 52459);
            let connecting = TcpStream::connect(&socket);
            self.state = Connecting(connecting);
        }
    }
}


pub fn run() {
    let f = resolve_and_connect("tribute.macmiller.life")
        .and_then(|mut stream| {
            println!("Sending http request"); // sending works
            let request = "GET / HTTP/1.1
User-Agent: Mozilla/4.0 (compatible; MSIE5.01; Windows NT)
Host: tribute.macmiller.life
Accept-Language: en-us
Accept-Encoding: gzip, deflate
Connection: Keep-Alive";
            stream.write_all(request.clone().as_bytes());
            future::ok(stream)
        })
        .and_then(|stream| { // TODO: Didn't manage to get HTTP response, just hangs or receives early eof
            println!("Reading response stream");
            // We need to create a buffer for read_exact to write into.
            // A Vec<u8> is a good starting point.
            // read_exact will read buffer.len() bytes, so we need
            // to make sure the Vec isn't empty!
            let mut buf = vec![0; 8];

            // read_exact returns a Future that resolves when
            // buffer.len() bytes have been read from stream.
            tokio::io::read_exact(stream, buf)
        })
        .inspect(|(_stream, buf)| {
            // Notice that we get both the buffer and the stream back
            // here, so that we can now continue using the stream to
            // send a reply for example.
            println!("got eight bytes: {:x?}", buf);
        })
        .map(|_| ())
        .map_err(|err| panic!("Problem with resolve+connect+process! {:?}", err));
    tokio::run(f);
}
