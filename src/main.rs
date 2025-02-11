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
pub struct Cords(pub usize, pub usize);

// We will be using this Cords struct to define where objects are
// The first usize will be the x
// The second usize will be the y

// x meaning top to bottom
// y meaning left to right

// Cords(0, 0) is the top left corner
// Cords(20, 10) is the bottom right corner

#[derive(Clone, Debug)]
pub struct RelCords(pub i32, pub i32);

pub const ROWS: usize = 10;
pub const COLUMNS: usize = 20;

pub struct GameState {
    pub game_board: HashMap<Cords, Ship>,
    pub tick_count: u32,
    pub player: Player,
    pub gamelevel: GameLevel,
}


#[tokio::main]
pub async fn main() {
    execute!(std::io::stdout(), terminal::Clear(terminal::ClearType::All), cursor::MoveTo(0, 0));
    run_game().await;
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


pub async fn run_game() {
    let mut player_position = Cords(10, 8);

    // Create a mutable variable called player_position
    // player_position should be a cords struct  // Cords(10, 8)

    loop {
        execute!(std::io::stdout(), crossterm::cursor::MoveTo(0, 0));

        print!("           +");
        for _ in 0..COLUMNS {
            print!("-");
        }
        println!("+           ");

        for row in 0..ROWS {
            print!("           |");
            for col in 0..COLUMNS {
            let current_position = Cords(col, row);
                // Create a variable called current_position
                // Set current_position to be a Cord struct where
                // the x is col
                // the y is row

                // Change this statememnt to compare player_position to current_position
                if player_position == current_position {
                    print!("^");
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
    

        // Calling your on_press function here:
        // Follow the instructions below then, come back here and:
        // 1: Pass player_position as the parameter
        // 2: Set player_position to be the returned value of calling on_press()
        player_position = on_press(player_position).await;
    }
}

// I moved your code for checking key presses here -->
// Make this function's return type: Cords
// Make this function take in mut player_position as a parameter
pub async fn on_press(mut player_position: Cords)-> Cords {
    match KeyReader::await_key_press().await {
        Key::ArrowLeft => {
            player_position.0 -= 1;
            // We want to adjust the player_position to the left by one
            // Add code here to subtract 1 from the x value
        }
        Key::ArrowRight => {
            player_position.0 += 1;
            // We want to adjust the player_position to the right by one
            // Add code here to add 1 to the x value
        }
        Key::ArrowUp => {
            // Make player shoot
            // Don't do anything here
        }
        Key::CtrlC => exit(0),
        _ => {}
    }
    player_position
    // Return the player_position variable
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
    Move(Cords),
    MoveOrNothing(RelCords),
    ShootOrNothing,
    RelativeMove(RelCords),
}

pub struct ShipAI {
    // pub timer: Timer,
    pub actions: Vec<(Option<Condition>, AIAction)>,
    pub action_index: usize,
}

pub enum Condition {
    ShipExists(Cords),
    PositionAvailable(RelCords),
    DontShootIfShipsAreBelow(RelCords),
}

pub enum Ship {
    Fly(ShipAI, bool, Uuid),
    Explosion(ShipAI, bool, Uuid),
    Bullet(ShipAI, bool, Uuid),
}

pub struct Player {
    pub display_char: char,
    pub lives: u8,
    pub current_position: Option<Cords>,
    pub start_position: Cords,
    // pub death_timer: Timer, 
    pub key_reader: KeyReader,
}
