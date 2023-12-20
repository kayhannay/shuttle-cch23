use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use axum::{Error, Extension};
use axum::extract::{Path, WebSocketUpgrade};
use axum::extract::ws::{Message, WebSocket};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tokio::sync::{broadcast};
use futures::{SinkExt, StreamExt};
use futures::stream::{SplitSink, SplitStream};
use tracing::log::info;

pub struct WsState {
    game_running: bool,
    views: i32,
    rooms: HashMap<i32, Room>,
}

pub fn init_websocket() -> Extension<Arc<Mutex<WsState>>> {
    let state = Arc::new(Mutex::new(WsState {
        game_running: false,
        views: 0,
        rooms: HashMap::new(),
    }));
   Extension(state)
}

pub async fn day19_ping_websocket_handler(ws: WebSocketUpgrade, Extension(state): Extension<Arc<Mutex<WsState>>>) -> impl IntoResponse {
    info!("Ping websocket handler called.");
    ws.on_upgrade(|socket| ping_websocket(socket, state))
}

async fn ping_websocket(mut stream: WebSocket, state: Arc<Mutex<WsState>>) {
    // By splitting we can send and receive at the same time.
    //let (mut sender, mut receiver) = stream.split();

    while let Some(message) = stream.recv().await {
        if let Ok(message) = message {
            info!("Received message: {:?}", message);
            if message == Message::Text("serve".to_string()) {
                state.lock().unwrap().game_running = true;
            } else if message == Message::Text("ping".to_string()) && state.lock().unwrap().game_running == true {
                info!("Sending pong.");
                let _ = stream.send(Message::Text("pong".to_string())).await;
            }
        } else {
            // client disconnected
            return;
        };
    }
}

struct Room {
    num: i32,
    broadcast_channel: broadcast::Sender<String>,
}

pub async fn day19_room_reset_views(Extension(state): Extension<Arc<Mutex<WsState>>>) -> impl IntoResponse {
    info!("Reset views called.");
    let mut st = state.lock().unwrap();
    st.views = 0;
    st.game_running = false;
    StatusCode::OK
}

pub async fn day19_room_get_views(Extension(state): Extension<Arc<Mutex<WsState>>>) -> impl IntoResponse {
    info!("Get views called.");
    (StatusCode::OK, format!("{}", state.lock().unwrap().views))
}

pub async fn day19_room_websocket_handler(ws: WebSocketUpgrade, Extension(state): Extension<Arc<Mutex<WsState>>>, Path(num): Path<i32>, Path(name): Path<String>) -> impl IntoResponse {
    info!("Room websocket handler called.");
    ws.on_upgrade(move |socket| room_websocket(socket, state, num, name))
}

async fn room_websocket(mut stream: WebSocket, state: Arc<Mutex<WsState>>, num: i32, name: String) {
    // By splitting we can send and receive at the same time.
    let (mut sender, mut receiver) = stream.split();

    if num != 0 {
        let mut state = state.lock().unwrap();
        let (broadcast_sender, broadcast_receiver) = broadcast::channel(10);
        state.rooms.insert(num, Room { num, broadcast_channel: broadcast_sender });
    }

    //tokio::spawn(write(sender.clone()));
    let broadcast_channel = {
        let mut state = state.lock().unwrap();
        state.rooms.get_mut(&num).unwrap().broadcast_channel.clone()
    };
    tokio::spawn(read(receiver, broadcast_channel));

    let mut rx = {
        let mut state = state.lock().unwrap();
        state.rooms.get_mut(&num).unwrap().broadcast_channel.subscribe()
    };

    while let Ok(msg) = rx.recv().await {
        info!("Received broadcast message: {:?}", msg);

        if sender.send(Message::Text(msg)).await.is_err() {
            break;
        }
    }
}

async fn read(receiver: SplitStream<WebSocket>, broadcast_channel: broadcast::Sender<String>) {
    receiver.for_each(|message| async {
        match message {
            Ok(message) => {
                info!("Received message: {:?}", message);
                if let Message::Text(text) = message {
                    info!("Sending message to broadcast.");
                    let _ = broadcast_channel.send(text);
                }
            }
            Err(e) => {
                info!("Error reading message: {:?}", e);
            }
        }
    }).await;
}

async fn write(mut sender: SplitSink<WebSocket, Message>) -> Result<(), Error> {
    sender.send(Message::Text("Hello".to_string())).await
}