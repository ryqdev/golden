struct TcpMessageBus{
}

impl TcpMessageBus {
    fn new() -> Self{
        TcpMessageBus{}
    }

}

struct Client {
    client_id: usize,
    message_bus: TcpMessageBus
}

impl Client {
    fn new(addr: &str, client_id: usize) -> Self {
        Client{
            client_id,
            message_bus: TcpMessageBus::new()
        }
    }
    fn handshake() {
        todo!()
    }
}