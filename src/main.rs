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

#[derive(Clone, Debug)]
pub struct RelCords(pub i32, pub i32);

pub const ROWS: usize = 10;
pub const COLUMNS: usize = 20;

/*
    The next step is to add enemy ships, or flies. (represented by the character F)

    To do this, we need a better way to keep track of which 
    characters are on which squares of our game board
*/

// Here we have a GameState struct. One of the fields is game_board
// game_board is a "hashmap" of characters on our board and their respective coordinates
pub struct GameState {
    pub game_board: HashMap<Cords, Char>
}

// Create an impl block for GameState


/* FOLLOW THESE INSTRUCTIONS INSIDE THE GameState impl BLOCK */

// Create function called new()
// new() needs to return an instance of GameState where game_board is HashMap::new()

// Create a function called add_ship()
// add_ship() needs three parameters. 
    // #1 &self
    // #2 The coords of the new ship (Cords)
    // #3 The representing character (Char)

// This function will return nothing

// Inside this function we will add a key and value pair to our game_board hashmap

// Add this line: self.game_board.insert(cords, ship);

// self means get whichever instance of GameState we are calling add_ship() on
// .game_board means get the game_board field
// .insert(cords, ship) means add a key (cords) and value (ship) to the game_board hashmap
    // "cords" here is the #2 parameter
    // "ship" here is the #3 parameter

// End of impl block here

// After you finish all comments (there are more below)
// GameState impl should have these 4 functions
    // new()
    // add_ship()
    // display_board()
    // run_game()

#[tokio::main]
pub async fn main() {
    execute!(std::io::stdout(), terminal::Clear(terminal::ClearType::All), cursor::MoveTo(0, 0));

    // Create a variable called game and set it to GameState::new()

    // now run game.add_ship() with a set of coordinates to place this enemy somewhere on the board and the character "F"
    
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

// move this function into the GameBoard impl block
pub async fn run_game() {
    let mut player_position = Cords(10, 8);

    loop {
         // Move the code inside this loop into a function called display_board() inside the GameBoard impl block
         // Leave the loop here, and inside it call display_board()
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
                if player_position == current_position {
                    print!("^");
                } else if /* check if there is a ship at current_position */ {
                    // print the character F
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
    
        player_position = on_press(player_position).await;
    }
}

pub async fn on_press(mut player_position: Cords)-> Cords {
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
