use url::Url;
use tungstenite::{connect, Message};
use serde_json;

const BASE_URL: &str = "wss://stream.data.alpaca.markets/v2/test";

fn main() {
    let (mut socket, response) = connect(Url::parse(BASE_URL).unwrap()).expect("Can't connect");
    println!("{:#?}", response);
}