use std::net::{TcpListener, TcpStream};
use std::io::{self, prelude::*, Result as IoResult};
use std::thread::{self, JoinHandle};

mod args;
use args::NetCrabArgs;

fn main() {
    let args = NetCrabArgs::new();
    if let Some(addr) = args.addr {
        if args.listen {
            let listener = TcpListener::bind(addr).unwrap_or_else(|e| {
                eprintln!("Could not bind to {addr}.\n{e}");
                std::process::exit(1);
            });

            println!("Listening on {addr}");
            let (stream, caddr) = listener.accept().unwrap();
            println!("Connection received on ({caddr})");
            handle(stream).unwrap();
        } else {
            let stream = TcpStream::connect(addr).unwrap();
            handle(stream).unwrap();
        }
    }
}

fn handle(stream: TcpStream) -> IoResult<()> {
    let (t1, t2) = (
        pipe(io::stdin(), stream.try_clone()?),
        pipe(stream, io::stdout()),
    );

    t1.join().unwrap();
    t2.join().unwrap();

    Ok(())
}

fn pipe<R, W>(mut r: R, mut w: W) -> JoinHandle<()>
where
    R: Read + Send + 'static,
    W: Write + Send + 'static,
{
    thread::spawn(move || {
        let mut buf = vec![0u8; 1024];
        loop {
            match r.read(&mut buf) {
                Ok(0) => {
                    println!("Connection closed");
                    break;
                }
                Ok(_) => {
                    if let Err(e) = w.write_all(&buf) {
                        eprintln!("Write error: {e}");
                        break;
                    } 
                }
                Err(e) => {
                    eprintln!("Read error: {e}");
                    break;
                }
            }
            w.flush().unwrap();
        }
    })
}