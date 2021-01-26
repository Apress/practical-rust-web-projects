extern crate ws;

use std::{thread, time};
use ws::util::{Timeout, Token};
use ws::{
    CloseCode, Error, ErrorKind, Frame, Handler, Handshake,
    OpCode, Result, Sender, WebSocket,
};

const PING: Token = Token(0);
const CLIENT_UNRESPONSIVE: Token = Token(1);

struct Server {
    out: Sender,
    ping_timeout: Option<Timeout>,
    client_unresponsive_timeout: Option<Timeout>,
}
impl Handler for Server {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        println!("Opened a connetion");
        self.out.timeout(15_000, CLIENT_UNRESPONSIVE)?;
        self.out.timeout(5_000, PING)
    }

    fn on_timeout(&mut self, event: Token) -> Result<()> {
        println!("event: {:?}", event);
        match event {
            PING => {
                println!("Pinging the client");
                self.out.ping("".into())?;
                println!(
                    "{:?}",
                    self.client_unresponsive_timeout
                );
                match self.client_unresponsive_timeout {
                    Some(_) => self.out.timeout(5_000, PING),
                    None => Ok(()), // skip
                }
            }
            CLIENT_UNRESPONSIVE => {
                println!("Client is unresponsive, closing the connection");

                // Otherwise, stop the timer and wait until it get dropped
                // TODO: Implement the disconnect
                self.client_unresponsive_timeout.take();
                if let Some(timeout) = self.ping_timeout.take() {
                    println!("timeout: {:?}", timeout);
                    self.out.cancel(timeout)?;
                    println!("canceled");
                }

                // If the clinet still respond to a close frame, try to disconnect nicely
                self.out.close(CloseCode::Away)
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
        println!("new timeout: {:?}", timeout);
        match event {
            PING => {
                if let Some(timeout) = self.ping_timeout.take() {
                    self.out.cancel(timeout)?
                }
                match self.client_unresponsive_timeout {
                    Some(_) => self.ping_timeout = Some(timeout),
                    None => self.ping_timeout = None,
                }
            }
            CLIENT_UNRESPONSIVE => {
                if let Some(timeout) =
                    self.client_unresponsive_timeout.take()
                {
                    self.out.cancel(timeout)?
                }
                self.client_unresponsive_timeout = Some(timeout)
            }
            _ => {
                eprintln!("Unknown event: {:?}", event);
            }
        }
        Ok(())
    }

    fn on_frame(
        &mut self,
        frame: Frame,
    ) -> Result<Option<Frame>> {
        if frame.opcode() == OpCode::Pong {
            println!("Received a pong");
            // Reset the CLIENT_UNRESPONSIVE timeout
            self.out.timeout(15_000, CLIENT_UNRESPONSIVE)?;
        }

        Ok(Some(frame))

        // TODO: default frame validation
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
        client_unresponsive_timeout: None,
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
