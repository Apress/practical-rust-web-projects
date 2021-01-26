extern crate ws;

fn main() {
    ws::listen("127.0.0.1:8080", |out| {
        move |msg| {
            println!("Received message: {}", msg);
            out.broadcast(msg)
        }
    })
    .unwrap()
}
