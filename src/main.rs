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
    pub game_board: HashMap<Coords, char>,
    pub player_position: Coords
}

// To run your program
    // cargo run

// When you finish tasks and your code compiles with no errors
    // git add .
    // git commit -m "nancy is still the best"
    // git push

impl GameState{
    pub fn new() -> GameState { 
        GameState{
            game_board: HashMap::new(),
            player_position: Coords(10, 8),
        }
    }

    pub fn add_ship(&mut self, coords: Coords, ship: char) { 
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
            self.player_position = on_press(self.player_position).await; // delete this line, we will rewrite it differently below

            // use an if let statement to check if self.player.use_key().await returned Some
            // if so, run self.add_ship()
            // the first parameter of add_ship() should be the coordinates that were returned in the if statement
            // the second parameter should be a '|' this will represent a bullet
        }
    }
}

#[tokio::main]
pub async fn main() {
    execute!(std::io::stdout(), terminal::Clear(terminal::ClearType::All), cursor::MoveTo(0, 0));

    let mut game = GameState::new();
    game.add_ship(Coords(10, 4), 'F');

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

pub struct Player {
    pub display_char: char,
    pub current_position: Option<Coords>,
    // pub death_timer: Timer, 
    pub key_reader: KeyReader,
}

// Create an impl block for Player
// Add a new() function
// This function should return Self
// Return an instance of Player
// where display character is '^'
// set current_position to Coords(COLUMNS / 2, ROWS - 2);
// death_timer to Timer::new(200),
// and key_reader KeyReader::new(),


// move this function into a impl block for the Player struct
// change this so it takes in &mut self instead of player_position
// rename to use_key
pub async fn on_press(mut player_position: Coords) -> Coords { // change to Option<Coords>
    match KeyReader::await_key_press().await {
        Key::ArrowLeft => {
            player_position.0 -= 1; // change to self.current_position.0
        }
        Key::ArrowRight => {
            player_position.0 += 1; // change to self.current_position.0
        }
        Key::ArrowUp => {
            // ArrowUp is the key that makes the player shoot
            // This function now returns an Option<Coords>
            // Meaning if we return Some(Coords(x, y)) then the player shot a bullet from that position
            // If we return None, the player did not shoot
            // Here, the player shot, so let's return the player's current position
            // Return Some(self.current_position.0)
        }
        Key::CtrlC => exit(0),
        _ => {}
    }
    // When we change self.current_position.0 we directly change the position of the player
    // We do not need to return the player_position anymore
    // Instead we will return a set of coordinates, only when the player shoots
    // That return will happen above in the ArrowUp match arm
    // Return None here instead of player_position
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


