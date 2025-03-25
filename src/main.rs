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


pub const SIZE: usize = 10;
pub const ROWS: usize = SIZE;
pub const COLUMNS: usize = SIZE * 2;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Cords(pub usize, pub usize);

#[derive(Clone, Debug)]
pub struct RelCords(pub i32, pub i32);



// To run your program
    // cargo run

// When you finish tasks and your code compiles with no errors
    // git add .
    // git commit -m "why are you reading this?"
    // git push

#[tokio::main]
async fn main()-> Result<(), String> { 
    execute!(std::io::stdout(), terminal::Clear(terminal::ClearType::All), cursor::MoveTo(0, 0));
    let mut game = GameState::new();

    game.add_ship(Cords(2, 3), Ship::new_fly())?;
    game.add_ship(Cords(3, 4), Ship::new_fly())?;
    game.add_ship(Cords(2, 5), Ship::new_fly())?;
    game.add_ship(Cords(3, 6), Ship::new_fly())?;
    game.add_ship(Cords(2, 7), Ship::new_fly())?;
    game.add_ship(Cords(3, 8), Ship::new_fly())?;
    game.add_ship(Cords(2, 9), Ship::new_fly())?;
    game.add_ship(Cords(3, 10), Ship::new_fly())?;

    game.start_game().await?;

    Ok(())
}

pub struct GameState {
    pub game_board: HashMap<Cords, Ship>, 
    pub tick_count: u32,
    pub player: Player,
    pub gamelevel: GameLevel,
}

impl GameState {
    pub fn new() -> GameState {
        let game_level: GameLevel::new(Level::Easy) // Fix: Add semi-colon. // Fix: Replace : with =
        let level_status: game_level.get_level_status(); // Fix: Replace : with +
        
        GameState {
            game_board: HashMap::new(),
            tick_count: 0,
            player: Player::new(),
            game_level: game_level,
        }
    }
    

    pub fn display_board(&self) {
        execute!(std::io::stdout(), crossterm::cursor::MoveTo(0, 0));
    
        print!("           +");
        for _ in 0..COLUMNS {
            print!("-");
        }
        println!("+           ");

        for row in 0..ROWS {
            print!("           |");
            for col in 0..COLUMNS {
                let current_position = Cords(row, col);
                if row == ROWS - 1 && col < (self.player.lives - 1) as usize {
                    print!("{}", self.player.display_char);   
                } else if self.player.current_position == Some (current_position) {
                    print!("{}", self.player.display_char);
                } else if let Some(ship) = self.game_board.get(&current_position) {
                    print!("{}", ship.display_char()); 
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

    pub fn add_ship(&mut self, cords: Cords, ship: Ship) -> Result<(), String>  {
        if let Some(_existing_ship) = self.remove_ship(cords) {
            self.game_board.insert(cords, Ship::new_explosion());
        } else {
            self.game_board.insert(cords, ship);
        }     
    
        Ok(())
    }
   

    pub fn remove_ship(&mut self, Cords) -> Option<Ship> { // Fix: Give the Cords parameter a name (cords)
        self.game_board.remove(&cords)
    }
    
    pub fn move_ship(&mut self, old_cords: Cords, new_cords: Cords) {
        if let Some(ship) = self.remove_ship(old_cords) {
            self.add_ship(new_cords, ship).ok();
        }
    }

    pub fn ship_actions(&mut self)-> Result<(), String> {
        let to_update: Vec<(Coords, Uuid)> = self // Fix: Replace Coords with Cords
            .game_board
            .iter()
            .map(|(coords, ship)| (*coords, ship.get_id()))
            .collect::<Vec<(Coords, Uuid)>>(); // Fix: Replace Coords with Cords
        
        for (coords, ship_id) in to_update {          
            if let Some(mut current_ship) = self.game_board.remove(&coords) {
                match current_ship.get_action(coords, &mut self.game_board) {
                    ShipAction::Remove => {} // Fix: Add comma here
                    ShipAction::Shoot => {
                        let shoot_position = Coords(coords.0, coords.1 - 1); // Fix: change Coords() to Cords() (variable coords is fine)
                        self.current_ship(shoot_position, Ship::new_bullet(true)); // Fix: current_ship is not a function, this should be add_ship()
                    } // Fix: Add comma here
                    ShipAction::Move(new_coords, wrapped) => {
                        if wrapped || (wrapped && current_ship.wrap())

                    } // Fix: Add comam here
                    Ok(()) // Fix: Move this line below the match statement
                }
            }
                    
        }
    }   

    pub async fn player_actions(&mut self) {
        if let Some(player_pos) = self.player.current_position {
            self.game_board.get(&player_pos).is_some() { // Fix: Add `if` keyword to the start of this line
                self.remove_ship(player_position);
                self.game_board.insert(player_pos, Ship::new_explosion());
            }
        }

        if let Some(lives_left) = self.player.handle_collision() {
            println!("Oh no... lives left: {}", lives_left - 1);
        } else {
            println!("Ouch, you died.");
            exit(0);
        }

        // We use this if statement to check if the player is not being displayed (implying it was exploded, and removed)
        if self.game_board.get(&self.player.start_position).is_none() {
            self.player.respawn(); // Fix: Pass true in to respawn() to let respawn know it can respawn the character. // Will improve this logic in the next iteration
        }

        if let Some(bullet_position) = self.player.use_key().await {
            self.add_ship(bullet_position, Ship::new_bullet(false));
            
        }
    }    

    pub async fn start_game(&mut self) -> Result<(), String> {
        loop {
            thread::sleep(Duration::from_millis(10)) // Fix: Add semi colon
            self.display_board();
            self.tick_count += 1;
            self.ship_actions();
            player.actions().await; // Fix: player is a field of self. Prefix player with `self.`
        }
    }
}


pub struct Player {
    pub display_char: char,
    pub lives: u8 // Fix: Add comma
    pub current_position: Option<Cords>, 
    pub start_position: Cords // Fix: Add comma
    pub death_timer: Timer, 
    pub key_reader: KeyReader,
}

impl Player {
    pub fn new(lives: u8) -> Self { 
        let start_position = Cords(ROWS - 2, COLUMNS / 2);
        Player {
            display_char: '^', 
            lives,
            current_ position // Fix: remove space after current_ // Fix: Add comma
            start_position: start_position // Fix: Add comma
            death_timer: Timer::new(200),
            key_reader: KeyReader::new(),
        }
    }

    pub async fn use_key(&mut self) -> Option<Coords> { // Fix: Change Option<Coords> to Option<Cords>
        if let Some(Cords(x, y)) = self.current_position {
            match self.key_reader.read_key().await {
                Some(Key::ArrowLeft) => {
                    if y > 0 {
                        self.current_position = Some(Cords(x, y - 1));
                    }
                }
                Some(Key::ArrowRight) => {
                    if y < COLUMNS - 1 {
                        self.current_position = Some(Cords(x, y + 1));
                    }
                }
                Some(Key::ArrowUp) => {
                    return Some(Cords(x - 1, y)); 
                }
                Some(Key::CtrlC) => exit(0),
                _ => {}
            };
        }
        None 
    }
    
    pub fn handle_collision(&mut self)-> Option<u8> {
        self.lives -= 1;
        if self.lives == 0{
            return None
        } else{
            self.current_position = None;
            return Some(self.lives);
        }
    }

    pub fn respawn(&mut self, can_respawn: bool) {
        if can_respawn && self.current_position.is_none() && self.death_timer.tick() {
            self.current position = Some(self.start_position);
        }
    }
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

pub enum Ship {
    Fly(ShipAI, bool, Uuid),
    Explosion(ShipAI, bool, Uuid),
    Bullet(ShipAI, bool, Uuid),
}

impl Ship{
    pub fn display_char(&self)-> char{
        match self {
            // Enum variants require the (_, _, _) even when not using them
            Ship::Fly => 'F', // Fix: Change to Ship::Fly(_, _, _)
            Ship::Explosion => 'X',  // Fix: Change to Ship::Explosion(_, _, _)
            Ship::Bullet => '|',  // Fix: Change to Ship::Bullet(_, _, _)
        }
    }

    pub fn get_id(&self)-> Uuid {
        // Match statements return references to the value. We want the value, not a reference
        match self {
            Ship::Fly(_, _, uuid) => uuid, // Fix: Prefix with * to dereference. '=> *uuid'
            Ship::Explosion(_, _, uuid) => uuid, // Fix: Change to '=> *uuid'
            Ship::Bullet(_, _, uuid) => uuid, // Fix: Change to '=> *uuid'
        }
    }

    pub fn get_action(&mut self, cords: Cords, hashbrowns: &mut HashMap<Cords, Ship>)-> ShipActions {
        // This match statement returns ai, but isn't storing the value anywhere
        match self {
            Ship::Fly(ai, _, _) => ai, // Fix: Change to '=> *ai'
            Ship::Explosion(ai, _, _) => ai, // Fix: Change to '=> *ai'
            Ship::Bullet(ai, _, _) => ai, // Fix: Change to '=> *ai'
        }
        // We try to use ai here, but we didn't store the value anywhere.
        // Can you solve this?
        ai.get_action(cords, hashbrowns)
    }

    pub fn wrap(&self) -> bool {
        match self {
            Ship::Fly(_, fool, _) => fool, // Fix: Change to '=> *fool'
            Ship::Explosion(_, fool, _) => fool, // etc
            Ship::Bullet(_, fool, _) => fool, // etc
        }
    }

    pub fn new_fly()->Self {
        Self::Fly( 
            ShipAI::new(
                100,
                vec![
                    (None, AIAction::MoveOrNothing(RelCords(1, 0))),                         
                    (None, AIAction::MoveOrNothing(RelCords(0, -1))), 
                    (None, AIAction::MoveOrNothing(RelCords(-1, 0))), 
                    (Some(Condition::DontShootIfShipsAreBelow(RelCords(1, 0))), AIAction::Shoot),
                ],
            ),
            true,
            Uuid::new_v4(),
        )
    }

    pub fn new_bullet(moving_down: bool)->Self{
        let movement: if moving_down { // Fix: replace : with =
            RelCords(1, 0)
        } else {
            RelCords(-1, 0)
        } // Fix: This is a let statement, add a semi-colon here

        Self::Bullet(
            ShipAI::new(
                10, 
                vec![(None, AIAction::RelativeMove(movement))],
            ), 
            false, 
            Uuid::new_v4(),
        )
    }

    pub fn new_explosion()-> Self {
        Self::Explosion(
            ShipAI::new(
                10, 
                vec![(None, AIAction::Remove)],
            ), 
            false, 
            Uuid::new_v4(),
        )
    }
}
  
pub struct ShipAI {
    pub timer: Timer,
    pub actions: Vec<(Option<Condition>, AIAction)>,
    pub action_index: usize,
}

impl ShipAI {
    pub fn new(action_interval: u64, actions: Vec<Option<Condition>, AIAction>) -> Self {
        timer: action_interval, // Fix: Surround these three fields in: ShipAI {}
        actions: actions,
        action_index: 0,
    }

    pub fn get_ai_action(&mut self, cords: Cords, game_board: &HashMap<Cords, Ship>)-> AIaction { // Return type has typo (AIAction)
        if self.actions.is_empty() {
            AIAction::Nothing
        }

        if self.timer.tick() {
            let (condition, action) = self.actions[self.action_index];
            if let Some(conditioner) = condition{
                match conditioner.evaluate(cords, game_board) {
                    true => {},
                    false => {
                        self.next_action();
                        return self.get_ai_action(cords, game_board)
                    }
                }
            }
            match self.action_index == self.actions.len() - 1{
                true => self.action_index = 0,
                false => self.action_index += 1,
            }
            return action;
        }
        AIAction::Nothing
    }
    pub fn next_action(&mut self) {
        if self.action_index == self.actions.len() - 1 {
            self.action_index = 0;
        } else {
            self.action_index += 1;
        }
    }

    pub fn get_action(&mut self, cords: Cords, game_board: &HashMap<Cords, Ship>)-> ShipAction {
        self.get_ai_action(cords, game_board).to_ship_action(cords, game_board);
    }
}

pub enum Condition {
    ShipExists(Cords),
    PositionAvailable(RelCords),
    DontShootIfShipsAreBelow(RelCords),
}

impl Condition{
    pub fn evaluate(&self, cords: Cords, game_board: &HashMap<Cords, Ship>)-> bool {
        match self {
            Condition::ShipExists(ref target_cords) => {
                return game_board.contains_key(target_cords);
            },

            Condition::PositionAvailable(rel_cords) => {
                return game_board.get(&rel_cords.evaluate(cords).0).is_none();
            },

            Condition::DontShootIfShipsAreBelow(_) => {
                let mut below_cords = cords;
                loop {
                    if !game_board.contains_key(&below_cords) {
                        break;
                    }
                    if let Some(ship) = game_board.get(&below_cords) {
                        if let Ship::Fly(_, _, _) = ship{
                            return false;
                        }
                    }
                    below_cords.0 += 1;
                }
                return true;
            }
        }
    }
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

impl AIAction{
    pub fn to_ship_action(self, cords: Cords, game_board: &HashMap<Cords, Ship>)-> ShipAction {
        match self {
            AIAction::Remove => {
                return ShipAction::Remove;
            },

            AIAction::Shoot => {
                return ShipAction::Shoot;
            },
            // We want to use the cords that AIAction::Move provides us with.
            AIAction::Move => { // Fix: Change to AIAction::Move(move_cords)
                return ShipAction::Move(cords, false); // Fix: Use move_cords, not cords, here
            },

            AIAction::MoveOrNothing(rel_cords) => {
                let condition = Condition::PositionAvailable(rel_cords);
                match condition.evaluate(cords, game_board){
                    true => {
                        let (new_cords, wrap) = rel_cords.evaluate(cords);
                        ShipAction::Move(new_cords, wrap)
                    },
                    false => ShipAction::Nothing
                }
            }

            AIAction::RelativeMove(rel_cords) => {
                let (new_cords, wrap) = rel_cords.evaluate(cords) // Fix: Add semi-colon here
                ShipAction::Move(new_cords, wrap)
            }

            AIAction::ShootOrNothing => {
                let condition = Condition::DontShootIfShipsAreBelow(RelCords(1, 0));
                match condition.evaluate(cords, game_board) => { // Fix: This part of a match statement doesn't use the =>. Remove it.
                    true => ShipAction::Shoot,
                    false => ShipAction::Nothing,
                }
            }

            AIAction::Nothing => {
                return ShipAction::Nothing;
            }



        }
    }
}

#[derive(Debug)]
pub struct GameLevel {
    current_level: Level,
}

impl GameLevel {
    pub fn new(level: Level)-> Self {
        return GameLevel{current_level: level}
    }

    pub fn level_status(&self)-> (u64, u8) {
        match self.current_level {
            Level::Easy => Self::easy(),
            Level::Medium => Self::medium(),
            Level::Hard => Self::hard(),
        }
    }

    pub fn easy()-> (u64, u8){
        let speed = 500;
        let lives = 5;
        return (speed, lives)
    }

    pub fn medium()-> (u64, u8) {
        let speed = 300;
        let lives = 3;
        return (speed, lives)
    }

    pub fn hard() -> (u64, u8) {
        let speed = 100;
        let lives = 1;
        return (speed, lives)
    }
}

#[derive(Clone, Debug)]
pub enum Level {
    Easy,
    Medium,
    Hard,
}

impl RelCords {
    pub fn evaluate(&self, cords: Cords) -> (Cords, bool) {
        
        let new_cords = (
            (cords.0 as i32 + self.0),
            (cords.1 as i32 + self.1),
        );
        
        let mut wrapped = false;
        let new_cords = Cords(
            if new_cords.0 < 0 {
                wrapped = true;
                ROWS as usize - 1
            } else if new_cords.0 >= ROWS as i32 {
                wrapped = true;
                0
            } else {
                new_cords.0 as usize
            },
            if new_cords.1 < 0 {
                wrapped = true;
                COLUMNS as usize - 1
            } else if new_cords.1 >= COLUMNS as i32 {
                wrapped = true;
                0
            } else {
                new_cords.1 as usize
            },
        );

        (new_cords, wrapped)
    }
}

#[derive(Clone)]
pub enum ShipAction {
    Nothing,
    Remove,
    Shoot,
    Move(Cords, bool),
}

pub struct Timer {
    current_time: u64,
    interval: u64,
}

impl Timer {
    pub fn new(interval: u64) -> Self {
        Timer {
            current_time: 0,
            interval,
        }
    }

    pub fn tick(&mut self) -> bool {
        self.current_time += 1;
        if self.current_time >= self.interval {
            self.current_time = 0; 
            true
        } else {
            false
        }
    }
}
