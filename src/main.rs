use crossterm::execute;
use crossterm::cursor;
use crossterm::terminal;
use std::collections::HashMap;
use std::process::exit;
use std::thread;
use std::time::Duration;
use uuid::Uuid;
use console::Key;
use console::Term;

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub struct Coords(pub usize, pub usize);

#[derive(Clone, Debug)]
pub struct RelCoords(pub i32, pub i32);

pub const ROWS: usize = 10;
pub const COLUMNS: usize = 20;

pub struct GameState {
    pub game_board: HashMap<Coords, char>, // 'Char' should be lowercase 'char'
    pub player_position: Coords
}

// ACHTUNG!
// I changed Cords to the correct spelling Coords
// John is primitive rock child

// To run your program
    // cargo run

// When you finish tasks and your code compiles with no errors
    // git add .
    // git commit -m "nancy is the best"
    // git push

impl GameState{
    pub fn new() -> GameState{ // We are not calling a function, remove the parentheses after GameState
        GameState{
            game_board: HashMap::new(),
            player_position: Coords(10, 8),
        }
    }

    pub fn add_ship(&mut self, coords: Coords, ship: char) { // 'Char' should be lowercase 'char'
        self.game_board.insert(coords, ship);  
    }

    pub fn display_board(&mut self){ 
        execute!(std::io::stdout(), crossterm::cursor::MoveTo(0, 0));

        print!("           +");
        for _ in 0..COLUMNS {
            print!("-");
        }
        println!("+           ");

        for row in 0..ROWS {
            print!("           |");
            for col in 0..COLUMNS {
            let current_position = Coords(col, row);
                if self.player_position == current_position {
                    print!("^");
                } else if let Some(ship) = self.game_board.get(&current_position) {
                    print!("{}", ship);
                } else {
                    print!(" ");
                }
            }
            println!("|           ");
        }

        print!("           +");
        for _ in 0..COLUMNS {
            print!("-");
        }
        println!("+           ");
    }

    pub async fn run_game(&mut self) { 
    
        loop {
            self.display_board();
            self.player_position = on_press(self.player_position).await;
        }
    }
}

#[tokio::main]
pub async fn main() {
    execute!(std::io::stdout(), terminal::Clear(terminal::ClearType::All), cursor::MoveTo(0, 0));

    let mut game = GameState::new();
    game.add_ship(Coords(8, 4), 'F');

    game.run_game().await;
}

pub struct KeyReader {
    jh: Option<tokio::task::JoinHandle<Key>>,
}

impl KeyReader {
    pub fn new() -> KeyReader {
        KeyReader {
            jh: Some(tokio::spawn(Self::await_key_press())),
        }
    }

    async fn await_key_press() -> Key {
        let term = Term::stdout();
        term.read_key().unwrap()
    }

    pub async fn read_key(&mut self) -> Option<Key> {
        if self.jh.as_ref().unwrap().is_finished() {
            let key = self.jh.take().unwrap().await.unwrap();
            self.jh = Some(tokio::spawn(Self::await_key_press()));
            Some(key)
        } else {
            None
        }
    }
}



pub async fn on_press(mut player_position: Coords)-> Coords {
    match KeyReader::await_key_press().await {
        Key::ArrowLeft => {
            player_position.0 -= 1;
        }
        Key::ArrowRight => {
            player_position.0 += 1;
        }
        Key::ArrowUp => {
        }
        Key::CtrlC => exit(0),
        _ => {}
    }
    player_position
}

#[derive(Clone, Debug)]
pub enum Level {
    Easy,
    Medium,
    Hard,
}

#[derive(Debug)]
pub struct GameLevel {
    current_level: Level,
}

#[derive(Clone)]
pub enum AIAction {
    Nothing,
    Remove,
    Shoot,
    Move(Coords),
    MoveOrNothing(RelCoords),
    ShootOrNothing,
    RelativeMove(RelCoords),
}

pub struct ShipAI {
    // pub timer: Timer,
    pub actions: Vec<(Option<Condition>, AIAction)>,
    pub action_index: usize,
}

pub enum Condition {
    ShipExists(Coords),
    PositionAvailable(RelCoords),
    DontShootIfShipsAreBelow(RelCoords),
}

pub enum Ship {
    Fly(ShipAI, bool, Uuid),
    Explosion(ShipAI, bool, Uuid),
    Bullet(ShipAI, bool, Uuid),
}

pub struct Player {
    pub display_char: char,
    pub lives: u8,
    pub current_position: Option<Coords>,
    pub start_position: Coords,
    // pub death_timer: Timer, 
    pub key_reader: KeyReader,
}
