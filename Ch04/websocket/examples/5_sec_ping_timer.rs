extern crate ws;

use std::{thread, time};
use ws::util::{Timeout, Token};
use ws::{
    CloseCode, Error, ErrorKind, Handler, Handshake,
    Result, Sender, WebSocket,
};

const PING: Token = Token(0);

struct Server {
    out: Sender,
    ping_timeout: Option<Timeout>,
}

impl Handler for Server {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        self.out.timeout(5_000, PING)
    }

    fn on_timeout(&mut self, event: Token) -> Result<()> {
        match event {
            PING => {
                println!("Pinging the client");
                self.out.ping("".into())?;
                self.out.timeout(5_000, PING)
            }
            _ => Err(Error::new(
                ErrorKind::Internal,
                "Invalid timeout token encountered!",
            )),
        }
    }

    fn on_new_timeout(
        &mut self,
        event: Token,
        timeout: Timeout,
    ) -> Result<()> {
        match event {
            PING => {
                if let Some(timeout) =
                    self.ping_timeout.take() {
                    self.out.cancel(timeout)?
                }
                self.ping_timeout = Some(timeout);
            }
            _ => {
                eprintln!("Unknown event: {:?}", event);
            }
        }
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!(
            "WebSocket closing for ({:?}) {}",
            code, reason
        );

        if let Some(timeout) = self.ping_timeout.take() {
            self.out.cancel(timeout).unwrap()
        }
    }
}

fn main() {
    let server = WebSocket::new(|out| Server {
        out: out,
        ping_timeout: None,
    })
    .unwrap();

    let broadcaster = server.broadcaster();

    let periodic = thread::spawn(move || loop {
        broadcaster.send("Meow").unwrap();
        thread::sleep(time::Duration::from_secs(1));
    });
    server.listen("127.0.0.1:8080").unwrap();
    periodic.join().unwrap();
}
