/// https://docs.alpaca.markets/docs/websocket-streaming
use url::Url;
use tungstenite::{connect, Message};
use dotenv::dotenv;
use std::env;

// const TEST_BASE_URL: &str = "wss://stream.data.alpaca.markets/v2/test";
const BASE_URL: &str = "wss://paper-api.alpaca.markets/stream";

pub async fn alpaca_trading() {
    log::info!("trade with alpaca");

    dotenv().ok();
    let key_id = env::var("KEY_ID").expect("Cannot find password in .env file");
    let secret = env::var("SECRET").expect("Cannot find password in .env file");
    let (mut socket, _response) = connect(Url::parse(BASE_URL).unwrap()).expect("Can't connect");

    log::info!("{:#?}", _response);


    let authorization = format!(r#"{{
        "action": "auth",
        "key": "{key_id}",
        "secret": "{secret}"
    }}"#);

    let subscribe = r#"{
        "action":"subscribe",
        "params":"BTCUSD",
    }"#;

    // let listen = r#"{
    //     "action":"listen",
    //     "data":{
    //         "streams": ["BTCUSD"]
    //     }
    // }"#;

    socket.send(Message::Text(authorization.into())).unwrap();
    // expected response: {"stream":"authorization","data":{"action":"authenticate","status":"authorized"}}

    socket.send(Message::Text(subscribe.into())).unwrap();

    loop {
        log::info!("in loop");
        let msg = socket.read().expect("Error reading message").to_string();
        log::info!("response: {}", msg);
    }
}
