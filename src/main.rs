mod game;
mod level;
mod player;

use axum::Router;
use game::Game;
use player::{Player, KeyInput};
use level::{BitVectorMap, Level, Coordinates};

use serde_json::json;
use socketioxide::{
    extract::{Data, SocketRef}, layer::SocketIoLayer, socket::DisconnectReason, SocketIo
};
use tokio::net::TcpListener;
use tokio::time::{interval, Duration};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, services::ServeDir};
use tracing::info;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, filter::{EnvFilter, LevelFilter}};

use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let filter: EnvFilter = EnvFilter::builder().with_default_directive(LevelFilter::INFO.into()).from_env_lossy();
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(filter)
        .init();

    info!("Starting server");

    // game logic stuff
    let map: Vec<u64> = vec![
        0b11111111111111,
        0b10000000000111,
        0b10000000000111,
        0b10011000000001,
        0b10010001000001,
        0b10110001000001,
        0b10111001111001,
        0b10001000010001,
        0b10101000010101,
        0b10001000000001,
        0b10101110100111,
        0b10001000000001,
        0b10000000000001,
        0b11111111111111 ];
    let map_clone: Vec<u64> = Vec::clone(&map);

    let (map_height, map_width): (usize, usize) = (14, 14);

    let bvmap: BitVectorMap = BitVectorMap::new(map, map_width, map_height);
    let spawn_points: [Coordinates; 5] = [Coordinates{x:2, y:2}, Coordinates{x:2, y:2}, Coordinates{x:2, y:2}, Coordinates{x:2, y:2}, Coordinates{x:2, y:2}];
    let scenario: Level = Level::new(bvmap, 1, spawn_points); // tile_size=1, server thinks in raw grid units
    let game: Arc<Mutex<Game>> = Arc::new(Mutex::new(Game::new(scenario)));

    // websocket stuff (socket-io)
    let (layer, io): (SocketIoLayer, SocketIo) = SocketIo::builder()
        .build_layer();

    // game loop — broadcasts player state every tick
    let game_loop = Arc::clone(&game);
    let io_loop = io.clone();
    tokio::spawn(async move {
        let mut tick = interval(Duration::from_millis(1000 / 60));
        loop {
            tick.tick().await;
            let state = {
                let mut game = game_loop.lock().unwrap();
                game.update();
                game.get_state()
            };
            if !state.is_empty() {
                let _ = io_loop.emit("tick", &state);
            }
        }
    });

    // socket handlers
    let game_connect = Arc::clone(&game);
    let game_keyinput = Arc::clone(&game);
    let game_disconnect = Arc::clone(&game);

    io.ns("/", move |s: SocketRef| {
        info!("Socket connected: {}", s.id);

        // on connect — add player to game, send map info
        {
            let mut game = game_connect.lock().unwrap();
            game.join(s.id);
        }
        let data = json!({"map": &map_clone, "width": map_width, "height": map_height});
        s.emit("gameinfo", &data).unwrap();

        // on keyinput — set player movement flags
        let game_keyinput = Arc::clone(&game_keyinput);
        s.on("keyinput", move |s: SocketRef, Data::<KeyInput>(input)| {
            let mut game = game_keyinput.lock().unwrap();
            game.set_key(s.id, &input.key, input.pressed);
        });

        // on disconnect — remove player
        let game_disconnect = Arc::clone(&game_disconnect);
        s.on_disconnect(move |s: SocketRef, reason: DisconnectReason| {
            info!("Socket disconnected: {} {}", s.id, reason);
            let mut game = game_disconnect.lock().unwrap();
            game.disconnect(s.id);
        });
    });

    info!("Serving on 0.0.0.0:4269");
    let app: Router = axum::Router::new()
        .nest_service("/", ServeDir::new("web"))
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(layer),
        );

    let listener: TcpListener = tokio::net::TcpListener::bind("0.0.0.0:4269").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
