use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use axum::{Error, Extension};
use axum::extract::{Path, WebSocketUpgrade};
use axum::extract::ws::{Message, WebSocket};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use tokio::sync::{broadcast};
use futures::{SinkExt, StreamExt};
use futures::stream::{SplitSink, SplitStream};
use serde::Deserialize;
use serde_json::json;
use tracing::info;

pub fn router() -> axum::Router {
    info!("Initializing websocket.");
    let websocket_state = init_websocket();

    axum::Router::new()
        .route("/ws/ping", get(day19_ping_websocket_handler))
        .route("/reset", post(day19_room_reset_views))
        .route("/views", get(day19_room_get_views))
        .route("/ws/room/:num/user/:name", get(day19_room_websocket_handler))
        .layer(websocket_state)
}

#[derive(Clone)]
struct WsState {
    game_running: Arc<RwLock<bool>>,
    views: Arc<RwLock<u32>>,
    rooms: Arc<RwLock<HashMap<i32, broadcast::Sender<String>>>>,
}

fn init_websocket() -> Extension<WsState> {
    let state = WsState {
        game_running: Arc::new(RwLock::new(false)),
        views: Arc::new(RwLock::new(0)),
        rooms: Arc::new(RwLock::new(HashMap::new())),
    };
   Extension(state)
}

async fn day19_ping_websocket_handler(ws: WebSocketUpgrade, Extension(state): Extension<WsState>) -> impl IntoResponse {
    info!("Ping websocket handler called.");
    ws.on_upgrade(|socket| ping_websocket(socket, state))
}

async fn ping_websocket(mut stream: WebSocket, state: WsState) {
    while let Some(message) = stream.recv().await {
        if let Ok(message) = message {
            info!("Received message: {:?}", message);
            if message == Message::Text("serve".to_string()) {
                *state.game_running.write().expect("Could not get write lock to game_running") = true;
            } else if message == Message::Text("ping".to_string()) && *state.game_running.read().expect("Could not read lock game_running") == true {
                info!("Sending pong.");
                let _ = stream.send(Message::Text("pong".to_string())).await;
            }
        } else {
            // client disconnected
            return;
        };
    }
}

struct User {
    name: String,
    room: i32,
    state: WsState,
}

async fn day19_room_reset_views(Extension(state): Extension<WsState>) -> impl IntoResponse {
    //info!("Reset views called.");
    *state.views.write().expect("Could not get write lock for views") = 0;
    *state.game_running.write().expect("Could not get write lock for game_started") = false;
    StatusCode::OK
}

async fn day19_room_get_views(Extension(state): Extension<WsState>) -> impl IntoResponse {
    //info!("Get views called.");
    let views = state.views.read().expect("Could not get lock for state!");
    //info!("Views: {}", views);
    (StatusCode::OK, views.to_string())
}

async fn day19_room_websocket_handler(ws: WebSocketUpgrade, Extension(state): Extension<WsState>, Path((num, name)): Path<(i32, String)>) -> impl IntoResponse {
    //info!("Room websocket handler called.");
    let user = User {
        name,
        room: num,
        state,
    };
    ws.on_upgrade(move |socket| room_websocket(socket, user))
}

async fn room_websocket(stream: WebSocket, user: User) {
    //info!("Room websocket called for user {} and room {}.", &user.name, &user.room);

    let (sender, receiver) = stream.split();

    if user.room != 0 {
        let mut rooms = user.state.rooms.write().expect("Could not get lock for rooms!");
        if !rooms.contains_key(&user.room) {
            let (broadcast_sender, _) = broadcast::channel(1000);
            rooms.insert(user.room, broadcast_sender);
            //info!("Created room, now there are: {:?}.", rooms);
        } else {
            //info!("Room already exists, now there are: {:?}.", rooms);
        }
    }

    let broadcast_receiver = {
        //info!("Subscribing to broadcast channel for room {}.", user.room);
        user.state.rooms.read().expect("Could not get read lock for rooms!").get(&user.room).unwrap().subscribe()
    };
    let mut send_tweet = tokio::spawn(write(sender, broadcast_receiver, user.state.clone()));

    let broadcast_channel = {
        user.state.rooms.read().expect("Could not get read lock for rooms").get(&user.room).unwrap().clone()
    };
    let mut recv_tweet = tokio::spawn(read(receiver, broadcast_channel, user.name));

    tokio::select! {
		_ = (&mut send_tweet) => recv_tweet.abort(),
		_ = (&mut recv_tweet) => send_tweet.abort(),
	};
}

#[derive(Deserialize)]
struct Msg {
    message: String,
}

async fn read(mut receiver: SplitStream<WebSocket>, broadcast_channel: broadcast::Sender<String>, user: String) {
    while let Some(Ok(Message::Text(message))) = receiver.next().await {
        let msg: Msg = serde_json::from_str(message.as_str())
            .expect("Could not deserialize input message");
        if msg.message.chars().count() > 128 {
            //info!("User {} in room {} is sending a message that is too long: {}", user, room, msg.message);
            continue;
        }
        let broadcast_message = json!({"user":user.clone(),"message":msg.message}).to_string();
        //info!("User {:02} in room {} is sending message to broadcast: {}", user, room, broadcast_message);
        let _ = broadcast_channel.send(broadcast_message);
    }
}

async fn write(mut sender: SplitSink<WebSocket, Message>, mut broadcast_receiver: broadcast::Receiver<String>, state: WsState) -> Result<(), Error> {
    while let Ok(msg) = broadcast_receiver.recv().await {
        //info!("User {} in room {} received broadcast message: {}", user, room, msg);
        let result = sender.send(Message::Text(msg)).await;
        if result.is_err() {
            //info!("User {} has left the room {}.", user, room);
            
            return Ok(());
        } else {
            *state.views.write().expect("Could not get lock for state!") += 1;
        }
    }
    Ok(())
}