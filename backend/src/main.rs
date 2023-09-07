use std::{collections::HashMap, io::Error, sync::Arc};

use async_std::sync::RwLock;
use futures_util::{
    lock::Mutex,
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use tokio::net::{TcpListener, TcpStream};

use serde::{Deserialize, Deserializer, Serialize};
use tokio_tungstenite::WebSocketStream;

use rand::{distributions::Alphanumeric, Rng};
// use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
enum Player {
    O,
    X,
    None,
}

#[derive(Debug)]
struct Client {
    socket_in: Arc<Mutex<SplitStream<WebSocketStream<TcpStream>>>>,
    socket_out: Arc<
        Mutex<
            SplitSink<
                WebSocketStream<TcpStream>,
                tokio_tungstenite::tungstenite::protocol::Message,
            >,
        >,
    >,
    game: Arc<Mutex<Option<Arc<Mutex<Game>>>>>,
    game_map: Arc<Mutex<HashMap<String, Arc<Mutex<Game>>>>>,
    self_arc: Option<Arc<RwLock<Client>>>,
}

impl Client {
    fn new(
        mut socket: WebSocketStream<TcpStream>,
        game_map: Arc<Mutex<HashMap<String, Arc<Mutex<Game>>>>>,
    ) -> Client {
        let (mut outgoing, mut incoming) = socket.split();
        Client {
            socket_in: Arc::new(Mutex::new(incoming)),
            socket_out: Arc::new(Mutex::new(outgoing)),
            game: Arc::new(Mutex::new(None)),
            game_map,
            self_arc: None,
        }
    }

    fn set_self_arc(&mut self, self_arc: Arc<RwLock<Client>>) {
        self.self_arc = Some(self_arc);
    }

    async fn process_massages(&self) {
        let mut incoming_guard = self.socket_in.lock().await;
        while let Some(x) = incoming_guard.next().await {
            if let Ok(data) = x {
                println!("{:?}", data);
                let m_r: serde_json::Result<Message> =
                    serde_json::from_str(data.to_text().unwrap());
                match m_r {
                    Ok(m) => {
                        println!("{:?}", m);
                        match m {
                            Message::NewSession(()) => {
                                println!("New session");
                                let mut games_guard = self.game_map.lock().await;
                                let new_game = Arc::new(Mutex::new(Game::new()));
                                let session_info = {
                                    let game_lock = new_game.lock().await;
                                    game_lock.get_session_info()
                                };
                                games_guard.insert(session_info.id.clone(), new_game);
                                self.socket_out
                                    .lock()
                                    .await
                                    .send(tokio_tungstenite::tungstenite::Message::Text(
                                        serde_json::to_string(&Message::SessionInfo(session_info))
                                            .unwrap()
                                    ))
                                    .await
                                    .unwrap();
                                println!("New session created")
                            }
                            Message::JoinSession(session) => {
                                // let games_guard = self.games.lock().await;
                                // println!("{:?}", games_guard);
                                println!("Joining session: {}", session);
                                let mut game_map_guard = self.game_map.lock().await;
                                println!("1");
                                if let Some(game) = game_map_guard.get(&String::from(session)) {
                                    match game
                                        .lock()
                                        .await
                                        .join(self.self_arc.as_ref().unwrap().clone())
                                    {
                                        Ok(()) => {
                                            println!("2");
                                            let mut self_game_lock = self.game.lock().await;
                                            println!("3");
                                            *self_game_lock = game_map_guard
                                                .get(&String::from(session))
                                                .cloned();
                                            println!("4");
                                        }
                                        Err(()) => {
                                            println!("Join Error")
                                        }
                                    }
                                } else {
                                    println!("No game found for session id: {}", session);
                                }
                                println!("Session join finished");
                            }
                            Message::MakeMove(m) => {
                                println!("Move received");
                                let game_lock = self.game.lock().await;

                                // if game_lock.is_some() {
                                //     let game = game_lock.unwrap();
                                // }
                                // let game = game_lock.clone();
                                match game_lock.clone() {
                                    Some(game) => {
                                        println!("Processing make move {:?}", m);
                                        let game_lock = game.lock().await;
                                        println!("Moeve 2");
                                        game_lock
                                            .make_move(m, self.self_arc.clone().unwrap())
                                            .await;
                                        println!("Moeve 3");
                                    }
                                    None => {
                                        println!("Error make move {:?}", m);
                                        self.send(&Message::Error).await
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    Err(_) => {
                        self.send(&Message::Error).await;
                    }
                }
            }
        }
    }

    async fn send(&self, m: &Message<'_>) {
        self.socket_out
            .lock()
            .await
            .send(tokio_tungstenite::tungstenite::Message::Text(
                serde_json::to_string(m).unwrap(),
            ))
            .await
            .unwrap();
    }
}

#[derive(Debug)]
struct Game {
    id: String,
    player: [Option<Arc<RwLock<Client>>>; 2],
    fields: [[GameField; 3]; 3],
}

impl Game {
    fn new() -> Game {
        let id = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(6)
            .map(char::from)
            .collect();
        Game {
            id,
            player: [None, None],
            fields: [
                [GameField::new(), GameField::new(), GameField::new()],
                [GameField::new(), GameField::new(), GameField::new()],
                [GameField::new(), GameField::new(), GameField::new()],
            ],
        }
    }

    fn join(&mut self, client: Arc<RwLock<Client>>) -> Result<(), ()> {
        for i in 0..2 {
            if self.player[i].is_none() {
                self.player[i] = Some(client);
                return Ok(());
            }
        }
        Err(())
    }

    fn get_session_info(&self) -> SessionInfo {
        SessionInfo {
            id: self.id.clone(),
            player: [Player::O, Player::X],
            start_player: Player::O,
        }
    }

    async fn make_move(&self, m: Move, client: Arc<RwLock<Client>>) {
        for i in 0..2 {
            if let Some(p) = &self.player[i] {
                println!("Make Move {} 1", i);
                let player_lock = p.write().await;
                println!("Make Move {} 2", i);
                player_lock.send(&Message::MakeMove(m)).await;
                println!("Make Move {} 3", i);
            }
        }
    }
}

#[derive(Debug)]
struct GameField {
    fields: [[Player; 3]; 3],
}

impl GameField {
    fn new() -> GameField {
        GameField {
            fields: [
                [Player::None, Player::None, Player::None],
                [Player::None, Player::None, Player::None],
                [Player::None, Player::None, Player::None],
            ],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
enum MessageType {
    NewSession,
    JoinSession,
    SessionInfo,
    MakeMove,
    InvalidMove,
    GameStart,
    Queue,
    QueueStatus,
    #[default]
    Error,
}

#[derive(Serialize, Deserialize, Debug)]
struct QueueStatus {
    waiting: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct SessionInfo {
    id: String,
    player: [Player; 2],
    start_player: Player,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
struct Move {
    global_grid_x: u8,
    global_grid_y: u8,
    local_grid_x: u8,
    local_grid_y: u8,
}

#[derive(Serialize, Deserialize, Debug)]
enum Message<'a> {
    NewSession(()),
    JoinSession(&'a str),
    SessionInfo(SessionInfo),
    MakeMove(Move),
    InvalidMove,
    GameStart,
    Queue,
    QueueStatus(QueueStatus),
    Error,
}

async fn handle_connection(
    raw_stream: TcpStream,
    game_map: Arc<Mutex<HashMap<String, Arc<Mutex<Game>>>>>,
) {
    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during websocket handshake");

    // let (mut outgoing, mut incoming) = ws_stream.split();
    let c = Client::new(ws_stream, game_map);
    let c_arc = Arc::new(RwLock::new(c));
    {
        let mut c_arc_guard = c_arc.write().await;
        c_arc_guard.set_self_arc(c_arc.clone());
    }
    // println!("{:?}", c_arc);
    c_arc.read().await.process_massages().await;
    // c_arc_guard.process_massages().await;
    // c_arc_guard.lock().process_massages().await;
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Hello, world!");

    let test = Message::MakeMove(Move {
        global_grid_x: 0,
        global_grid_y: 1,
        local_grid_x: 2,
        local_grid_y: 3,
    });
    println!("{}", serde_json::to_string(&test).unwrap());

    let addr = "127.0.0.1:8080";

    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    let game_map: Arc<Mutex<HashMap<String, Arc<Mutex<Game>>>>> =
        Arc::new(Mutex::new(HashMap::new()));

    while let Ok((stream, _)) = listener.accept().await {
        let game_map_ref = game_map.clone();
        tokio::spawn(handle_connection(stream, game_map_ref));
    }

    Ok(())
}
