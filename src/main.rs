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

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub struct Cords(pub usize, pub usize);

#[derive(Clone, Debug)]
pub struct RelCords(pub i32, pub i32);

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
    pub game_level: GameLevel,
}

impl GameState {
    pub fn new() -> GameState {
        let game_level = GameLevel::new(Level::Easy);
        let (speed, lives) = game_level.level_status();
        
        GameState {
            game_board: HashMap::new(),
            tick_count: 0,
            player: Player::new(lives),
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
        if cords.0 >= ROWS || cords.1 >= COLUMNS {
            return Err(format!("Coordinates are out of bounds."));
        } else if let Some(_existing_ship) = self.remove_ship(cords) {
            self.game_board.insert(cords, Ship::new_explosion());
        } else {
            self.game_board.insert(cords, ship);
        }     
    
        Ok(())
    }
   

    pub fn remove_ship(&mut self, cords :Cords) -> Option<Ship> { 
        self.game_board.remove(&cords)
    }
    
    pub fn move_ship(&mut self, old_cords: Cords, new_cords: Cords) {
        if let Some(ship) = self.remove_ship(old_cords) {
            self.add_ship(new_cords, ship).ok();
        }
    }

    pub fn ship_actions(&mut self)-> Result<(), String> {
        let to_update: Vec<(Cords, Uuid)> = self
            .game_board
            .iter()
            .map(|(cords, ship)| (*cords, ship.get_id()))
            .collect::<Vec<(Cords, Uuid)>>(); 
            
        for (cords, ship_id) in to_update {          
            if let Some(mut current_ship) = self.game_board.remove(&cords) { // This line tries to remove a ship off the board at the given location (cords). The reason we remove the ship, is cause we want to move it. So if it can be removed, then we continue with the rest of this code block in order to move or shoot or whatever
                if current_ship.get_id() != ship_id {
                    continue;
                } // If the ship that we removed, does not match the id of the ship we're trying to update, then we run continue, to skip the next block of code
                
                match current_ship.get_action(cords, &mut self.game_board) {
                    ShipAction::Nothing => {
                        self.add_ship(cords, current_ship)?
                        // If we are not supposed to do anything, we need to add the ship back onto the board at the original coordinates
                        // Run self.add_ship() and pass in the cords and current_ship variables. add_ship() returns a Result so make sure to add ? to the end of the line.
                    },
                    ShipAction::Remove => {},  // Remove does nothing since we've already removed the ship
                    ShipAction::Shoot => {
                        let shoot_position = Cords(cords.0, cords.1 - 1);
                        self.add_ship(shoot_position, Ship::new_bullet(true));
                    }, 
                    ShipAction::Move(new_cords, wrapped) => {
                       // Here we need to write the code to move the ship. 
                       if !wrapped || wrapped && current_ship.wrap() {
                        self.add_ship(new_cords, current_ship)?
                       }
                       // Create an if statement that checks if wrapped is false (!wrapped) or if wrapped is true and (&&) the result of current_ship.wrap() is true
                       // Inside the if statement, run self.add_ship() and pass in the new_cords variable and current_ship variable. (add ? to the end of the line)
                    }, 
                }
                  // We don't need to return Ok here since its then next thing that runs outside the for loop (you can delete this line)
            }
        }
        Ok(())
    }   

    pub async fn player_actions(&mut self) {
        if let Some(player_pos) = self.player.current_position {
            if self.game_board.get(&player_pos).is_some() {
                self.remove_ship(player_pos);
                self.game_board.insert(player_pos, Ship::new_explosion());
                if let Some(lives_left) = self.player.handle_collision() {
                    println!("Oh no... lives left: {}", lives_left - 1);
                } else {
                    println!("Ouch, you died.");
                    exit(0);
                }
            }
        }

        if self.game_board.get(&self.player.start_position).is_none() {
            self.player.respawn(true); 
        }

        if let Some(bullet_position) = self.player.use_key().await {
            self.add_ship(bullet_position, Ship::new_bullet(false));
            
        }
    }    

    pub async fn start_game(&mut self) -> Result<(), String> {
        loop {
            thread::sleep(Duration::from_millis(10)); 
            self.display_board();
            self.tick_count += 1;
            self.ship_actions();
            self.player_actions().await; 
        }
    }
}


pub struct Player {
    pub display_char: char,
    pub lives: u8,
    pub current_position: Option<Cords>, 
    pub start_position: Cords, 
    pub death_timer: Timer, 
    pub key_reader: KeyReader,
}

impl Player {
    pub fn new(lives: u8) -> Self { 
        let start_position = Cords(ROWS - 2, COLUMNS / 2);
        Player {
            display_char: '^', 
            lives,
            current_position: Some(start_position), 
            start_position: start_position,
            death_timer: Timer::new(200),
            key_reader: KeyReader::new(),
        }
    }

    pub async fn use_key(&mut self) -> Option<Cords> { 
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
            self.current_position = Some(self.start_position);
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
            Ship::Fly(_, _, _)=> 'F', 
            Ship::Explosion(_, _, _)=> '*',
            Ship::Bullet(_, _, _)=> '|',  
        }
    }

    pub fn get_id(&self)-> Uuid {
        match self {
            Ship::Fly(_, _, uuid) => *uuid,
            Ship::Explosion(_, _, uuid) => *uuid, 
            Ship::Bullet(_, _, uuid) => *uuid, 
        }
    }

    pub fn get_action(&mut self, cords: Cords, hashbrowns: &mut HashMap<Cords, Ship>)-> ShipAction {
        let ai = match self {
            Ship::Fly(ai, _, _) => ai,
            Ship::Explosion(ai, _, _) => ai,
            Ship::Bullet(ai, _, _) => ai,
        };
        ai.get_action(cords, hashbrowns)
    }

    pub fn wrap(&self) -> bool {
        match self {
            Ship::Fly(_, fool, _) => *fool, 
            Ship::Explosion(_, fool, _) => *fool, 
            Ship::Bullet(_, fool, _) => *fool, 
        }
    }

    pub fn new_fly()->Self {
        Self::Fly( 
            ShipAI::new( // This code is correct. Here's a couple notes to explain it better.
                100, // This number represents how often to run an action
                vec![
                    (None, AIAction::MoveOrNothing(RelCords(1, 0))),   // First action is to move up                      
                    (None, AIAction::MoveOrNothing(RelCords(0, -1))), // Then move right
                    (None, AIAction::MoveOrNothing(RelCords(-1, 0))), // Then move down
                    (Some(Condition::DontShootIfShipsAreBelow(RelCords(1, 0))), AIAction::Shoot), // Then shoot
                ], // The ShipAI causes these actions to repeat in a loop
            ),
            true, // Wrap is set to true, if the fly moves into the side of the board, it will appear on the opposite side
            Uuid::new_v4(), // Creates a new random id number
        )
    }

    pub fn new_bullet(moving_down: bool)->Self{
        let movement = if moving_down { 
            RelCords(1, 0)
        } else {
            RelCords(-1, 0)
        }; 
        Self::Bullet(
            ShipAI::new(
                10, // Time that needs to pass before moving the bullet down or up a square
                vec![(None, AIAction::RelativeMove(movement))], // Move the bullet, movement is a variable that either tells it to move up or down depending if a fly or the player shot the bullet
            ), 
            false, // Wrap is set to false, the bullet will not appear on the opposite side
            Uuid::new_v4(), // Creates ID number
        )
    }

    pub fn new_explosion()-> Self {
        Self::Explosion(
            ShipAI::new(
                10, // Time between actions
                vec![(None, AIAction::Remove)], // We simply remove the explosion after one frame of it existing
            ), 
            false, // No wrapping
            Uuid::new_v4(), // Creates ID number
        )
    }
}
  
pub struct ShipAI {
    pub timer: Timer,
    pub actions: Vec<(Option<Condition>, AIAction)>,
    pub action_index: usize,
}

impl ShipAI {
    pub fn new(action_interval: u64, actions: Vec<(Option<Condition>, AIAction)>) -> Self {
        ShipAI{
            timer: Timer::new(action_interval), 
            actions: actions,
            action_index: 0,
        }
    }

    pub fn get_ai_action(&mut self, cords: Cords, game_board: &HashMap<Cords, Ship>) -> AIAction { 
        if self.actions.is_empty() {
            return AIAction::Nothing; 
        }

        if self.timer.tick() {
            let (condition, action) = &self.actions[self.action_index];
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
            return action.clone();
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

    pub fn get_action(&mut self, cords: Cords, game_board: &HashMap<Cords, Ship>) -> ShipAction {
        self.get_ai_action(cords, game_board).to_ship_action(cords, game_board)
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
            AIAction::Move(move_cords) => { 
                return ShipAction::Move(move_cords, false); 
            },

            AIAction::MoveOrNothing(rel_cords) => {
                let condition = Condition::PositionAvailable(rel_cords.clone());
                match condition.evaluate(cords, game_board){
                    true => {
                        let (new_cords, wrap) = rel_cords.evaluate(cords);
                        ShipAction::Move(new_cords, wrap)
                    },
                    false => ShipAction::Nothing
                }
            }

            AIAction::RelativeMove(rel_cords) => {
                let (new_cords, wrap) = rel_cords.evaluate(cords);
                ShipAction::Move(new_cords, wrap)
            }

            AIAction::ShootOrNothing => {
                let condition = Condition::DontShootIfShipsAreBelow(RelCords(1, 0));
                match condition.evaluate(cords, game_board) { 
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
