use message_io::network::{NetEvent, Transport};
use message_io::node::{self, NodeHandler, NodeListener};

pub struct GameSocket {
    handler: NodeHandler<()>,
    send_event: fn(&[u8]) -> (),
}

impl GameSocket {
    pub fn new(
      connection_string: String,
      send_event: fn(&[u8]) -> (),
      handle_receive: fn(&[u8]) -> ()
    ) -> Self {
        let (handler, listener) = node::split::<()>();
        handler
            .network()
            .listen(Transport::FramedTcp, connection_string)
            .unwrap();
        GameSocket::create_socket(listener, handle_receive);
        Self {
            handler,
            send_event
        }
    }

    pub fn create_socket(listener: NodeListener<()>, callback: fn(&[u8]) -> ()) {
        listener.for_each(move |event| match event.network() {
            NetEvent::Connected(_, _) => unreachable!(),
            NetEvent::Accepted(_endpoint, _listener) => println!("Client connected"),
            NetEvent::Message(endpoint, data) => callback(data),
            NetEvent::Disconnected(_endpoint) => println!("Client disconnected"),
        });
    }

    fn receive_event(data: &[u8]) -> () {
        // Here's where we'll receive game state
        // change to send back this games state
        println!("Received: {}", String::from_utf8_lossy(data));
    }

    fn send_event(&self, data: &[u8]) -> () {

        // self.handler.network().send(endpoint, data);

    }
}
