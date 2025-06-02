use std::io::BufWriter;

use clap::ValueEnum;
use log::{debug, info, trace};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream, ToSocketAddrs},
};

use crate::{
    Game, Play,
    game::{End, GamePlayError},
};

/// Role in a game
#[derive(Debug, PartialEq, Eq, Clone, Copy, ValueEnum)]
pub enum Roles {
    /// Client
    ///
    /// Plays First
    Client,

    /// Host of the game
    ///
    /// Plays Second
    Host,
}

/// AI type
#[derive(Debug, PartialEq, Eq, Clone, Copy, ValueEnum)]
pub enum AIType {
    /// Random Player
    Random,
    /// Short sight
    Short,
    /// Long sight
    Long,
    /// Short sight, multithreaded
    ShortT,
    /// Long sight, multithreaded
    LongT,
}

/// Handle the network logic
pub struct RemoteGame {
    /// Local state of the game
    current_game: Game,
    /// Network connection to the other player
    stream: TcpStream,
}

impl RemoteGame {
    /// Create a new `RemoteGame` for the client side   
    pub async fn new_client<T: ToSocketAddrs>(addr: T) -> Self {
        //todo!("Can't create a client to play with a friend !")

        // - Connect a TCPStream to the `addr`
        let stream = TcpStream::connect(addr)
            .await
            .expect("Failed to connect to server");

        // - Initialise a default game
        let current_game = Game::default();

        // - Return
        Self {
            stream,
            current_game,
        }
    }

    /// Create a new `RemoteGame` for the host side    
    pub async fn new_server<T: ToSocketAddrs>(addr: T) -> Self {
        //todo!("Can't create a host to play with a friend")

        // - Connect a TCPListeber to the `addr`
        let listener = TcpListener::bind(addr)
            .await
            .expect("Failed to bind the TCP listener");

        // - Wait for a connection
        let (stream, _) = listener
            .accept()
            .await
            .expect("Failed to accept the connection");

        // - Initialise a default game
        let mut remote_game = Self {
            stream,
            current_game: Game::default(),
        };

        // - Wait for the client to make the first move
        let _ = remote_game.await_other_payer().await;

        // - Return
        remote_game
    }

    /// Send the current state  
    async fn send_state(&mut self) {
        //todo!("Can't send the history of the game to my friend");

        // Use `serde_json::to_string` to convert the history of the current game into a string
        let history = self.current_game.history();
        let json = serde_json::to_string(&history).expect("Failed to make the game history");

        // Send the whole string throught the connection
        let bytes = json.as_bytes();
        self.stream
            .write_all(bytes)
            .await
            .expect("Failed to send the game state");

        // Log for the send function
        info!("Play sended");
    }

    /// Await for the other player to send his move
    /// Use `serde_json::from_str` to try to convert a `str` into a `Vec<Play>`.
    async fn await_other_payer(&mut self) -> Option<End> {
        // Hint : read the stream one `u8` at a time and store them into a buffer
        // Hint : use the `From<T>` trait of `Game` to recreate a new current game.
        let mut buffer: Vec<u8> = Vec::new();
        let mut history: Vec<Play>;
        let mut json: String;
        history = loop {
            let mut reception: [u8; 1] = [0; 1];
            let n = self.stream.read(&mut reception).await.unwrap();
            debug!("Recived {n} bytes");
            buffer.push(reception[0]);
            json = match String::from_utf8(buffer.clone()) {
                Ok(s) => s,
                Err(e) => {
                    trace!("Unicode : {e:?}");
                    continue;
                }
            };
            match serde_json::from_str(json.as_str()) {
                Ok(v) => break v,
                Err(e) => {
                    trace!("Serde : {e:?}");
                    continue;
                }
            };
        };
        let new_move = history.pop().unwrap();
        self.current_game.play(new_move.column()).unwrap()
    }

    pub async fn play(&mut self, column: usize) -> Result<Option<End>, GamePlayError> {
        let next = self.current_game.play(column)?;
        self.send_state().await;
        if let Some(e) = next {
            return Ok(Some(e));
        };
        let e = self.await_other_payer().await;
        Ok(e)
    }

    pub fn render<T: std::io::Write>(&self, buff: &mut BufWriter<T>) {
        self.current_game.render(buff);
    }

    pub fn history(&self) -> Vec<Play> {
        self.current_game.history()
    }

    pub fn game(&self) -> Game {
        self.current_game.clone()
    }
}
