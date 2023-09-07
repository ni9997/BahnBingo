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

use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
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
    uuid: Uuid,
    player: Arc<Mutex<Option<Player>>>,
}

// impl PartialEq for Client {
//     fn eq(&self, other: &Self) -> bool {
//         self.socket_in == other.socket_in && self.socket_out == other.socket_out && self.game == other.game && self.game_map == other.game_map && self.self_arc == other.self_arc
//     }
// }

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
            uuid: Uuid::new_v4(),
            player: Arc::new(Mutex::new(None)),
        }
    }

    fn set_self_arc(&mut self, self_arc: Arc<RwLock<Client>>) {
        self.self_arc = Some(self_arc);
    }

    async fn self_disconnet(&self) {
        let mut game_lock = self.game.lock().await;
        if game_lock.is_some() {
            game_lock
                .as_mut()
                .unwrap()
                .lock()
                .await
                .disconnect_player(self.uuid)
                .await;
            let mut game_map_lock = self.game_map.lock().await;
            game_map_lock.remove(&game_lock.as_mut().unwrap().lock().await.id);
        }
    }

    async fn process_massages(&self) {
        let mut incoming_guard = self.socket_in.lock().await;
        while let Some(x) = incoming_guard.next().await {
            if let Ok(data) = x {
                if data.is_close() {
                    self.self_disconnet().await;
                    return;
                }
                println!("Processing Message: {:?}", data);
                let m_r: serde_json::Result<Message> =
                    serde_json::from_str(data.to_text().unwrap());
                match m_r {
                    Ok(m) => {
                        println!("{:?}", m);
                        match m {
                            Message::NewSession(()) => {
                                let mut session_id = None;
                                {
                                    println!("New session");
                                    let mut games_guard = self.game_map.lock().await;
                                    let new_game = Arc::new(Mutex::new(Game::new()));
                                    let session_info = {
                                        let game_lock = new_game.lock().await;
                                        game_lock.get_session_info()
                                    };
                                    session_id = Some(session_info.id.clone());
                                    games_guard.insert(session_info.id.clone(), new_game);
                                    self.socket_out
                                        .lock()
                                        .await
                                        .send(tokio_tungstenite::tungstenite::Message::Text(
                                            serde_json::to_string(&Message::SessionInfo(
                                                session_info,
                                            ))
                                            .unwrap(),
                                        ))
                                        .await
                                        .unwrap();
                                    println!("New session created");
                                }
                                {
                                    if let Some(session) = session_id {
                                        self.join_game(&session).await;
                                    }
                                }
                            }
                            Message::JoinSession(session) => {
                                self.join_game(session).await;
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
                                        let mut game_lock = game.lock().await;
                                        match game_lock.make_move(m, &self.uuid).await {
                                            Ok(p) => match p {
                                                MoveResult::NormalMove => {
                                                    self.send(&Message::MoveSuccess).await;
                                                }
                                                MoveResult::WinningMove(p) => {
                                                    self.send(&Message::MoveSuccess).await;
                                                    self.send(&Message::GameFinished(p)).await;
                                                }
                                            },
                                            Err(()) => {
                                                self.send(&Message::InvalidMove).await;
                                            }
                                        }
                                    }
                                    None => {
                                        println!("Error make move {:?}", m);
                                        self.send(&Message::Error).await
                                    }
                                }
                            }
                            Message::RequestGameStart => {
                                let game_lock = self.game.lock().await;
                                match game_lock.clone() {
                                    Some(game) => {
                                        let mut game_lock = game.lock().await;
                                        game_lock.begin_game().await.unwrap();
                                        println!("Starting game {}", game_lock.id);
                                    }
                                    None => {
                                        println!("Error starting game {:?}", m);
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
        // Disconnet from game here?
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

    async fn join_game(&self, session: &str) {
        println!("Joining session: {}", session);
        let game_map_guard = self.game_map.lock().await;
        if let Some(game) = game_map_guard.get(&String::from(session)) {
            match game
                .lock()
                .await
                .join(self.self_arc.as_ref().unwrap().clone())
            {
                Ok(()) => {
                    let mut self_game_lock = self.game.lock().await;
                    *self_game_lock = game_map_guard.get(&String::from(session)).cloned();
                    self.send(&Message::JoinSuccess(session)).await;
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

    async fn set_player(&self, player: Player) {
        let mut player_lock = self.player.lock().await;
        *player_lock = Some(player);
    }

    async fn send_game_start(&self) {
        self.send(&Message::GameStart(GameStartInfo {
            starting_player: Player::O,
            player: self.player.lock().await.unwrap(),
        }))
        .await;
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct GameStartInfo {
    starting_player: Player,
    player: Player,
}

#[derive(Debug)]
struct Game {
    id: String,
    player: [Option<Arc<RwLock<Client>>>; 2],
    fields: [[GameField; 3]; 3],
    next_player: Option<Uuid>,
    last_move: Option<Move>,
    running: bool,
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
            next_player: None,
            running: false,
            last_move: None,
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

    async fn begin_game(&mut self) -> Result<(), ()> {
        let temp: [Player; 2] = [Player::O, Player::X];
        for i in 0..self.player.len() {
            let p = &self.player[i];
            match p {
                Some(pl) => {
                    let player_lock = pl.read().await;
                    player_lock.set_player(temp[i]).await;
                    player_lock.send_game_start().await;
                }
                None => {
                    return Err(());
                }
            }
        }
        self.running = true;

        Ok(())
    }

    fn get_session_info(&self) -> SessionInfo {
        SessionInfo {
            id: self.id.clone(),
            player: [Player::O, Player::X],
            start_player: Player::O,
        }
    }

    async fn make_move(&mut self, m: Move, client_uuid: &Uuid) -> Result<MoveResult, ()> {
        match self.check_valid_move(&m) {
            Ok(()) => match self.running {
                true => {
                    for i in 0..2 {
                        if let Some(p) = &self.player[i] {
                            let player_lock = p.read().await;
                            if player_lock.uuid == *client_uuid {
                                continue;
                            }
                            player_lock.send(&Message::MakeMove(m)).await;
                        }
                    }
                    self.last_move = Some(m);
                    match self.check_winner() {
                        Some(p) => {
                            return Ok(MoveResult::WinningMove(p));
                        }
                        None => {
                            return Ok(MoveResult::NormalMove);
                        }
                    }
                }
                false => {
                    return Err(());
                }
            },
            Err(()) => {
                for i in 0..2 {
                    if let Some(p) = &self.player[i] {
                        let player_lock = p.read().await;
                        if player_lock.uuid == *client_uuid {
                            player_lock.send(&Message::InvalidMove).await;
                        }
                    }
                }
                return Err(());
            }
        }
    }

    fn check_valid_move(&self, m: &Move) -> Result<(), ()> {
        Ok(())
    }

    fn check_winner(&self) -> Option<Player> {
        None
    }

    async fn disconnect_player(&mut self, uuid: Uuid) {
        for i in 0..2 {
            if let Some(p) = self.player[i].clone() {
                if p.read().await.uuid == uuid {
                    self.player[i] = None;
                }
            }
        }
    }
}

enum MoveResult {
    NormalMove,
    WinningMove(Player),
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
    JoinSuccess(&'a str),
    SessionInfo(SessionInfo),
    MakeMove(Move),
    MoveSuccess,
    InvalidMove,
    GameStart(GameStartInfo),
    GameFinished(Player),
    RequestGameStart,
    NotifyPlayerJoined,
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

    let test = Message::GameStart(GameStartInfo {
        starting_player: Player::O,
        player: Player::X,
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
