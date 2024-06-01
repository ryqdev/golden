use url::Url;
use tungstenite::{connect, Message};
use serde_json;

// const TEST_BASE_URL: &str = "wss://stream.data.alpaca.markets/v2/test";
const BASE_URL: &str = "wss://stream.data.alpaca.markets/v1beta2/crypto";

pub async fn alpaca_trading() {
    log::info!("trade with alpaca");

    let (mut socket, _response) = connect(Url::parse(BASE_URL).unwrap()).expect("Can't connect");

    log::info!("{:#?}", _response);

    let authorization = r#"{"action": "auth", "key": "{KEY ID}", "secret": "{SECRET KEY}"}"#;
    let subscribe = r#"{"action":"subscribe","trades":["BTC/USD","ETH/USD"],"quotes":[],"bars":[]}"#;

    socket.send(Message::Text(authorization.into())).unwrap();
    socket.send(Message::Text(subscribe.into())).unwrap();

    loop {
        let msg = socket.read().expect("Error reading message").to_string();
        if msg != "" {
            let msg_vector: Vec<serde_json::Value> = serde_json::from_str(&msg).unwrap();
            for m in msg_vector {
                if m["T"] == "t" {
                    log::info!("{:#?}", &m);
                }
            }
        }
    }
}
