extern crate ws;

use ws::{connect, CloseCode, Frame, Message, OpCode, Result};

struct Client {
    out: ws::Sender,
}
impl ws::Handler for Client {
    fn on_frame(
        &mut self,
        frame: Frame,
    ) -> Result<Option<Frame>> {
        if frame.opcode() == OpCode::Ping {
            println!(
                "Received a ping, but we are not responding"
            );
        }

        Ok(None)

        // TODO: default frame validation
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("Received message {:?}", msg);
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!(
            "WebSocket closing for ({:?}) {}",
            code, reason
        );
    }
}

fn main() {
    connect("ws://127.0.0.1:8080", |out| Client { out: out })
        .unwrap();
}
