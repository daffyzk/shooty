mod game;
mod player;

use axum::Router;
use game::Game;
use player::Player;
use map::{BitVectorMap, Level, Coordinates};

use serde_json::json;
use rmpv::Value;
use socketioxide::{
    extract::{Data, SocketRef}, layer::SocketIoLayer, socket::DisconnectReason, DisconnectError, ParserConfig, SocketIo
};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, services::ServeDir};
use tracing::{info, warn};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, filter::{EnvFilter, LevelFilter}};

use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let filter: EnvFilter = EnvFilter::builder().with_default_directive(LevelFilter::INFO.into()).from_env_lossy();
    tracing_subscriber::registry() // supalogging memes
        .with(fmt::layer())
        .with(filter)
        .init();

    info!("Starting server");

    // game logic stuff
    let map: Vec<u64> = vec![
        11111111111111,  
        10000000000111, 
        10000000000111, 
        10011000000001, 
        10010001000001,
        10110001000001,
        10111001111001,
        10001000010001,
        10101000010101,
        10001000000001,
        10101110100111,
        10001000000001,
        10000000000001,
        11111111111111 ];
    // info!("map bit-vec: {}", size_of_val(&map));
    let map_clone: Vec<u64> = Vec::clone(&map);

    let (map_height, map_width): (usize, usize) = (14, 14);

    let bvmap: BitVectorMap = BitVectorMap::new(map, map_width, map_height);
    let spawn_points: [Coordinates; 5] = [Coordinates{x:5, y:5}, Coordinates{x:5, y:5}, Coordinates{x:5, y:5}, Coordinates{x:5, y:5}, Coordinates{x:5, y:5}];
    let scenario: Level = Level::new(bvmap, 30, spawn_points);
    let game: Arc<Mutex<Game>> = Arc::new(Mutex::new(Game::new(scenario)));


    // i don't really understand how this could work if I'm cloning the game 
    // because it's literally a different game but whatever maybe uhh
    let game_clone: Arc<Mutex<Game>> = Arc::clone(&game);
    
    let mut game_connect: Arc<Mutex<Game>> = Arc::clone(&game_clone);
    let mut game_disconnect: Arc<Mutex<Game>> = Arc::clone(&game_clone);

    tokio::spawn(async move {
        game_clone.lock().unwrap().init();
    });

    // websocket stuff (socket-io of course)
    let (layer, io): (SocketIoLayer, SocketIo) = SocketIo::builder()
        .with_parser(ParserConfig::msgpack())
        .build_layer();

    info!("SocketNs???");
    io.ns("/", move |s: SocketRef| {

        info!("start socket: {}", s.id);

        s.on("connect", {
            info!("Client connected with id: {}", s.id);
            move |s: SocketRef| {
                
                let mut conn_game = game_connect.lock().unwrap();
                let player: Player = Player::new(s.id, Coordinates{x:35,y:35}, 5);
                conn_game.join(player);
                // update game's player list via fn "join"  
                // send to server info to new player
                // server info: map, idk
                let data = json!({"map": &map_clone, "game": "uhh"});
                s.emit("gameinfo", &data).unwrap();
            }
        });

        s.on("action", {
            info!("action");
            move |s: SocketRef, Data::<Value>(data)| {
                // send player pos and angle to the game loop
                //info!("action socket-id: {}, data: {}", s.id, data);
               
                s.emit("received", &format!("{{'msg':'{data}'}}")).unwrap();
                }
        });

        s.on_disconnect({
            warn!("Client disconnected, id: {:?}", s.id);
            move |s: SocketRef, reason: DisconnectReason| {
                info!("Socket disconnected on {} namespace with id and reason: {} {}", s.ns(), s.id, reason);

                let mut game_lock = game_disconnect.lock().unwrap();
                game_lock.disconnect(s.id);
            }
        });


    });

    info!("Serving btw");
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
