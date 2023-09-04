use std::{io::Error, sync::Arc};

use futures_util::{
    lock::Mutex,
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use tokio::net::{TcpListener, TcpStream};

use serde::{Deserialize, Serialize};
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
    socket_in: SplitStream<WebSocketStream<TcpStream>>,
    socket_out:
        SplitSink<WebSocketStream<TcpStream>, tokio_tungstenite::tungstenite::protocol::Message>,
    game: Option<Arc<Mutex<Game>>>,
    games: Arc<Mutex<Vec<Game>>>,
}

impl Client {
    fn new(mut socket: WebSocketStream<TcpStream>, games: Arc<Mutex<Vec<Game>>>) -> Client {
        let (mut outgoing, mut incoming) = socket.split();
        Client {
            socket_in: incoming,
            socket_out: outgoing,
            game: None,
            games
        }
    }

    async fn process_massages(&mut self) {
        while let Some(x) = self.socket_in.next().await {
            if let Ok(data) = x {
                println!("{:?}", data);
                let m_r: serde_json::Result<Message> =
                    serde_json::from_str(data.to_text().unwrap());
                match m_r {
                    Ok(m) => {
                        println!("{:?}", m);
                        match m.message_type {
                            MessageType::NewSession => {
                                println!("New session");
                                let mut games_guard = self.games.lock().await;
                                let new_game = Game::new();
                                games_guard.push(new_game);
                                self.socket_out
                                    .send(tokio_tungstenite::tungstenite::Message::Text(
                                        serde_json::to_string(&Message {
                                            message_type: MessageType::SessionInfo,
                                            session_info: Some(SessionInfo {
                                                id: "".to_string(),
                                                player: Player::O,
                                                start_player: Player::O,
                                            }),
                                            ..Default::default()
                                        })
                                        .unwrap(),
                                    ))
                                    .await
                                    .unwrap();
                            }
                            MessageType::JoinSession => {
                                let games_guard = self.games.lock().await;
                                println!("{:?}", games_guard);
                            }
                            _ => {}
                        }
                    }
                    Err(_) => {
                        self.socket_out
                            .send(tokio_tungstenite::tungstenite::Message::Text(
                                serde_json::to_string(&Message {
                                    message_type: MessageType::Error,
                                    ..Default::default()
                                })
                                .unwrap(),
                            ))
                            .await
                            .unwrap();
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
struct Game {
    id: String,
    player: [Option<Arc<Mutex<Client>>>; 2],
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

    fn join(&mut self, client: Arc<Mutex<Client>>) -> Result<(), ()> {
        for i in 0..2 {
            if self.player[i].is_none() {
                self.player[i] = Some(client);
                return Ok(());
            }
        }
        Err(())
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
    player: Player,
    start_player: Player,
}

#[derive(Serialize, Deserialize, Debug)]
struct Move {
    global_grid_x: u8,
    global_grid_y: u8,
    local_grid_x: u8,
    local_grid_y: u8,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Message {
    message_type: MessageType,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_move: Option<Move>,
    #[serde(skip_serializing_if = "Option::is_none")]
    queue_status: Option<QueueStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_info: Option<SessionInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
enum Message2<'a> {
    NewSession(()),
    JoinSession(&'a str),
    SessionInfo(&'a str),
    MakeMove(Move),
    InvalidMove,
    GameStart,
    Queue,
    QueueStatus(QueueStatus),
    Error,
}

async fn handle_connection(raw_stream: TcpStream, games: Arc<Mutex<Vec<Game>>>) {
    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during websocket handshake");

    // let (mut outgoing, mut incoming) = ws_stream.split();
    let mut c = Client::new(ws_stream, games);
    c.process_massages().await;
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Hello, world!");

    let test = Message2::NewSession(());
    println!("{}", serde_json::to_string(&test).unwrap());

    let addr = "127.0.0.1:8080";

    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    let games = Arc::new(Mutex::new(vec![]));

    while let Ok((stream, _)) = listener.accept().await {
        let games_ref = games.clone();
        tokio::spawn(handle_connection(stream, games_ref));
    }

    Ok(())
}
