use std::io::prelude::*;

use flate2::read::GzDecoder;
use tungstenite::connect;
use url::Url;
mod models;

static HUOBI_WS_URL: &str = "wss://api-cloud.huobi.co.jp/ws";

fn main() {
    let (mut socket, response) =
        connect(Url::parse(&HUOBI_WS_URL).unwrap()).expect("Can't connect.");
    println!("Connected to Huobi stream.");
    println!("HTTP status code: {}", response.status());
    println!("Response headers:");
    for (ref header, ref header_value) in response.headers() {
        println!("- {}: {:?}", header, header_value);
    }

    // Receive ping
    let msg = read_message(&mut socket);
    let ping_data: models::Ping = serde_json::from_str(&msg).expect("Can't parse");
    println!("ping id: {}", ping_data.ping);

    // Send pong
    let pong_data: models::Pong = models::Pong {
        pong: ping_data.ping,
    };
    socket.write_message(tungstenite::Message::Text(serde_json::to_string(&pong_data).unwrap())).unwrap();

    // Subscribe
    let subscription: models::Subscription = models::Subscription {
        id: "1".to_owned(),
        sub: "market.ethbtc.depth.step0".to_owned(),
    };
    socket.write_message(tungstenite::Message::Text(serde_json::to_string(&subscription).unwrap())).unwrap();

    let msg = read_message(&mut socket);
    let subscription_response: models::SubscriptionResponse = serde_json::from_str(&msg).expect("Can't parse");
    println!("status: {}", subscription_response.status);

    let request: models::Request = models::Request {
        req: "market.ethbtc.depth.step0".to_owned(),
        id: "1".to_owned(),
    };
    socket.write_message(tungstenite::Message::Text(serde_json::to_string(&request).unwrap())).unwrap();

    loop {
        let msg = read_message(&mut socket);
        let huobi_depth: models::HuobiMessage = serde_json::from_str(&msg).expect(&msg);
        match huobi_depth {
            models::HuobiMessage::Depth(depth) => {    
                for i in 0..depth.data.asks.len() {
                    println!(
                        "{}: {}. ask: {}, size: {}",
                        depth.rep, i, depth.data.asks[i][0], depth.data.asks[i][1]
                    );
                }
            },
            models::HuobiMessage::Tick(tick) => {    
                for i in 0..tick.tick.asks.len() {
                    println!(
                        "{}: {}. ask: {}, size: {}",
                        tick.ch, i, tick.tick.asks[i][0], tick.tick.asks[i][1]
                    );
                }
            },
            models::HuobiMessage::Ping(ping) => {    
                let pong_data: models::Pong = models::Pong {
                    pong: ping.ping,
                };
                socket.write_message(tungstenite::Message::Text(serde_json::to_string(&pong_data).unwrap())).unwrap();
            }
        }
    }
}

fn read_message<T: Read + Write>(socket: &mut tungstenite::protocol::WebSocket<T>) -> std::string::String {
    let msg = socket.read_message().expect("Error reading message");
    match msg {
        tungstenite::Message::Binary(bytes) => {
            let mut gz = GzDecoder::new(&bytes[..]);
            let mut s = String::new();
            gz.read_to_string(&mut s).expect("Error decompressing message");
            s
        },
        _ => {
            panic!("Error getting text");
        }
    }
}
