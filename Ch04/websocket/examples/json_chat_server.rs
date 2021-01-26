extern crate ws;
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::time::{SystemTime, UNIX_EPOCH};
use ws::{listen, Message};

#[derive(Serialize, Deserialize)]
struct JSONMessage {
    name: String,
    message: String,
}

fn main() {
    listen("127.0.0.1:8080", |out| {
        move |msg: Message| {
            let msg_text = msg.as_text().unwrap();
            if let Ok(json_message) =
                serde_json::from_str::<JSONMessage>(msg_text)
            {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards");
                println!(
                    "{} said: {} at {:?}",
                    json_message.name,
                    json_message.message,
                    now.as_millis()
                );
                let output_msg = json!({
                    "name": json_message.name,
                    "message": json_message.message,
                    "received_at": now.as_millis().to_string()
                });

                out.broadcast(Message::Text(
                    output_msg.to_string(),
                ))?;
            }
            Ok(())
        }
    }).unwrap();
}
