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
    // git commit -m "nancy is so cool, she did so much work today. She should have some coffee"
    // git push

#[tokio::main]
async fn main()-> Result<(), String> { // Change this to return Result<(), String>
    execute!(std::io::stdout(), terminal::Clear(terminal::ClearType::All), cursor::MoveTo(0, 0));
    let mut game = GameState::new();
//done
    game.add_ship(Cords(2, 3), Ship::new_fly())?;
    game.add_ship(Cords(3, 4), Ship::new_fly())?;
    game.add_ship(Cords(2, 5), Ship::new_fly())?;
    game.add_ship(Cords(3, 6), Ship::new_fly())?;
    game.add_ship(Cords(2, 7), Ship::new_fly())?;
    game.add_ship(Cords(3, 8), Ship::new_fly())?;
    game.add_ship(Cords(2, 9), Ship::new_fly())?;
    game.add_ship(Cords(3, 10), Ship::new_fly())?;
    // Add 8 Fly Ships here
    // E.G: game.add_ship(Coords(2, 3), Ship::new_fly())?;

    // Here I've marked down each fly's coordinates

    // Fly 1: Coords(2, 3)
    // Fly 2: Coords(3, 4)
    // Fly 3: Coords(2, 5)
    // Fly 4: Coords(3, 6)
    // Fly 5: Coords(2, 7)
    // Fly 6: Coords(3, 8)
    // Fly 7: Coords(2, 9)
    // Fly 8: Coords(3, 10)

    game.start_game().await?;

    Ok(())// return Ok(())
}

pub struct GameState {
    pub game_board: HashMap<Cords, Ship>, // Change the value type from char to Ship
    pub tick_count: u32,// Add a field - tick_count: u32
    pub player: Player,
    pub gamelevel: GameLevel,// Add a field - gamelevel: GameLevel
}

impl GameState {
    pub fn new() -> GameState {
        let game_level: GameLevel::new(Level::Easy)// Create a variable called game_level set to GameLevel::new(Level::Easy)
        let level_status: game_level.get_level_status();// Create a variable called level_status set to game_level.get_level_status();
        
        GameState {
            game_board: HashMap::new(),
            tick_count: 0,// Add the tick_count field here and set it to 0
            player: Player::new(), // Pass level_status.1 into Player::new()
            game_level: game_level,// Add the gamelevel field here and set it to game_level
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
                } else if self.player.current_position == Some (current_position) { // change to Some(current_position)
                    print!("{}", self.player.display_char); // change from '^' to self.player.display_char
                } else if let Some(ship) = self.game_board.get(&current_position) {
                    print!("{}", ship.display_char()); // change from ship to ship.display_char()
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

    // Instead of taking in the ship's display character, we want to take in the whole Ship now (change ship's type from char to Ship)
    // Add a return type Result<(), String>     This is how we will handle the crashes when the player goes out of bounds
    pub fn add_ship(&mut self, cords: Cords, ship: Ship)-> Result<(), String>  {
        // Delete this, we want to change the logic of this function
        //self.game_board.insert(cords, ship); 
        if let Some(_existing_ship) = self.remove_ship(cords) {
            self.game_board.insert(cords, Ship::new_explosion());

        } else{
            self.game_board.insert(cords, ship);
        }     
        // Before we add a ship to the board, let's check if something already exists at that location.
        // If something is there, we want to cause an explosion.

        // Add an `if let` statement here to see if we can remove a ship at `cords`:
        // if let Some(_existing_ship) = self.remove_ship(cords) {
            // If a ship exists at that location, cause an explosion.
            // Insert a key-value pair into `self.game_board` where `cords` is the key and `Ship::new_explosion()` is the value.
            // self.game_board.insert(cords, Ship::new_explosion());
        // else
            // If there is no ship at the given location, we don't need an explosion.
            // Simply add the new ship.
            // self.game_board.insert(cords, ship);

        // We haven't added crash handling yet, so for now return Ok(())
        // We'll learn Ok(()), Err(()), and Result types later
        Ok(())
    }
   

    pub fn remove_ship(&mut self, Cords)-> Option<Ship>{
        self.game_board.remove(&cords)
    }
    // Create a function called remove_ship
    // This function should take in &mut self, and Cords
    // This function should return Option<Ship>
    // Remember game_board is a HashMap
    // So, we can use the method remove() on it
    // remove() returns an optional ship (Option<Ship>)
    // Some() if there is a value/ship that matches the given key
    // None if there is no value/ship that matches the given key
    // Use this command: self.game_board.remove(&cords)
    
    pub fn move_ship(&mut self, old_cords: Cords, new_cords: Cords) {
        if let Some(ship) = self.remove_ship(old_cords) {
            self.add_ship(new_cords, ship).ok();
        }
    }


    // Create a function called move_ship
    // This function will take in &mut self, old_cords: Cords, new_cords: Cords
    // This function won't return anything
    // This function will see if there is a ship to remove at the old cords
    // And then the ship back at the new cords

    // Use an if let statement on self.remove_ship(old_cords)
    // Inside the body of that if let statement
        // run this command to add a ship: self.add_ship(new_cords, ship).ok()
        // the ship variable passed into add_ship above, is the value that the if let stament will give you


    pub fn ship_actions(&mut self)-> Result<(), String>{
        let to_update: Vec<(Coords, Uuid)> = self
            .game_board
            .iter()
            .map(|(coords, ship)| (*coords, ship.get_id()))
            .collect::<Vec<(Coords, Uuid)>>();
        
        for (coords, ship_id) in to_update {          
            if let Some(mut current_ship) = self.game_board.remove(&coords) {
                match current_ship.get_action(coords, &mut self.game_board){
                    ShipAction::Remove => {

                    }
                    ShipAction::Shoot => {
                        let shoot_position = Coords(coords.0, coords.1 - 1);
                        self.current_ship(shoot_position, Ship::new_bullet(true));
                    }
                    ShipAction::Move(new_coords, wrapped) => {
                        if wrapped || (wrapped && current_ship.wrap())

                    }
                    Ok(())
                }
            }
                    
        }
    }   


    // Create a function called `ship_actions`.
    // This function will loop through all the ships on the board
    // and make them perform their intended actions (move left, right, up, down, or shoot).
    // The function should take in `&mut self` and return `Result<(), String>`.

        // Inside `ship_actions`, create a variable called `to_update` and set it to:
        /*
            self
            .game_board
            .iter()
            .map(|(coords, ship)| (*coords, ship.get_id()))
            .collect::<Vec<(Coords, Uuid)>>();
        */
        // This variable collects all the keys and values from the `game_board` HashMap and 
        // stores them in a vector of `(Coords, Uuid)` tuples.

        // Create a for loop to iterate through the `to_update` variable:
        // for (coords, ship_id) in to_update {

            // Inside the loop, create an `if let` statement to check if there is a ship at the current `coords`:
            // if let Some(mut current_ship) = self.game_board.remove(&coords) {

                // We need to match the result of `current_ship.get_action(coords, &mut self.game_board)`:

                    // If the action is `ShipAction::Remove`, do nothing.
                        // No action needed.

                    // If the action is `ShipAction::Shoot`, do the following:
                        // Create a variable called `shoot_position` and set it to `Coords(cords.0 + 1, cords.1)`.

                        // Add a ship to `self` at `coords` with `current_ship`.

                        // Add a bullet (Ship::new_bullet(true)) to `self` at `shoot_position`.

                    // If the action is `ShipAction::Move(new_coords, wrapped)`, do the following:
                        // Check if `wrapped` is `false`, or if `wrapped` is `true` and `current_ship.wrap()` is `true`.
                            // Add a ship to `self` at `new_coords` with `current_ship`.

                    // If the action is `ShipAction::Nothing`, do the following:
                        // Add a ship back to `self` at `coords` with `current_ship`.
 
    // Return `Ok(())` to indicate the function completed successfully.


// ship_actions handles all of the actions a ship can take.
// Now, let's handle all of the player's actions.
// Create an async function called `player_actions` that takes in &mut self.
    pub async fn player_actions(&mut self) {
        if let Some(player_pos) = self.player.current_position {
            self.game_board.get(&player_pos).is_some() {
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
        if self.game_board.get(&self.player.start_position).is_none() {
            self.player.respawn();
        }
        if let Some(bullet_position) = self.player.use_key().await {
            self.add_ship(bullet_position, Ship::new_bullet(false));
            
        }
    }    


    // This function will run if any of the following three things happen:
    // 1. The player was shot (the player has not moved, is still on the board, and no key has been pressed).
    // 2. The player needs to respawn (the player is not on the board).
    // 3. The player shot something (the player is on the board and a key was pressed).

    // Start by handling the case where the player is shot or runs into a fly.
    // Create an `if let` statement for `self.player.current_position` (We need to check if the player is currently on the board, i.e., not blown up).
        // If the player is on the board, check if there is something in the player's position (like a bullet or a fly).
        // You can do this by checking `self.game_board.get(&player_pos).is_some()`.
            // If something is present, call `remove_ship` on `self` using the value returned from the `if let` statement (use `player_pos` here).
            // Then, insert an explosion by calling: `self.game_board.insert(player_pos, Ship::new_explosion());`.

            // Next, add an `if let` statement to check if the player has run out of lives.
                // Use an `if let` on `self.player.handle_collision()`.
                    // If there are lives left, print: `"Oh no... lives left: {}", lives - 1`.
                    // If `self.player.handle_collision()` returns `None`, print `"Ouch, you died."` and call `exit(0)`.

    // Now, let's handle respawning the player.
    // Call `self.player.respawn()` if `self.game_board.get(&self.player.start_position).is_none()`.

    // Finally, handle the player shooting.
    // Move the `if let Some(bullet_pos)` statement from `run_game` to here.
    // Update the `add_ship` statement from `|` to `Ship::new_bullet(false)`.


    // Let's fix up this run_game function
    // Rename it to start_game
    // Add a 10 millisecond delay, so we're not moving to quickly: thread::sleep(Duration::from_millis(10));
    // Increase self.tick_count by 1
    // run self.ship_actions()
    // run player_actions().await
    // keep self.display_board() but make sure the if let statement is deleted
    pub async fn start_game(&mut self) -> Result<(), String> {
        loop {
            thread::sleep(Duration::from_millis(10))
            self.display_board();
            self.tick_count += 1;
            self.ship_actions();
            player.actions().await;
        }
    }
}


pub struct Player {
    pub display_char: char,
    pub lives: u8// Add a field called lives with type u8
    pub current_position: Option<Cords>, // change to Option<Cords>
    pub start_position: Cords// Add a field called start_position with type Cords
    pub death_timer: Timer, 
    pub key_reader: KeyReader,
}

impl Player {
    pub fn new(lives: u8) -> Self { // Fix: take in lives: u8 as a parameter
        let start_position = Cords(ROWS - 2, COLUMNS / 2);
        Player {
            display_char: '^', 
            lives,
            current_ position //Some(start_position): Cords(ROWS - 2, COLUMNS / 2), // Change to Some(start_position)
            start_position: start_position// Add the field start_position and set it to start_position,
            death_timer: Timer::new(200),
            key_reader: KeyReader::new(),
        }
    }

    // I made some changes here, don't worry abt them. I'll add notes later
    // Mainly just fixed the crash when going out of bounds
    pub async fn use_key(&mut self) -> Option<Coords> {
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
    // Create a function called handle_collision
    // This should take in &mut self and return Option<u8>
    // This function will be used to change values when the play was hit by a bullet or fly
    // The return value will contain lives left
    // Subtract 1 from self.lives
    // Now if self.lives is 0, return None (player is dead)
    // else, set self.current_position to None and return Some(self.lives)

    pub fn respawn(&mut self, can_respawn: bool) {
        if can_respawn && self.current_position.is_none() && self.death_timer.tick() {
            self.current position = Some(self.start_position);
        }
    }
    // Create a function called respawn
    // This function should take in &mut self, and can_respawn: bool
    // This function will wait until the death_timer is done running (death_timer is a lil delay so we don't respawn too fast)
    // Inside this function, 
        // Check if self.current_position.is_none() and self.death_timer.tick() is true
            // If so, set self.current position to Some(self.start_position)
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
//// Quick explanation for the fields here:
// ShipAI - The code that tells the ship when to move or shoot.
// bool - Determines if this ship will appear on the other side when it hits a wall (e.g., if it hits the left wall, will it reappear on the right side?).
// Uuid - Each ship has a unique ID number.

// Create an `impl` block for `Ship`.
impl Ship{
    pub fn display_char(&self)-> char{
        match self {
            Ship::Fly => 'F',
            Ship::Explosion => 'X',
            Ship::Bullet => '|',
        }
    }

    pub fn get_id(&self)-> Uuid {
        match self {
            Ship::Fly(_, _, uuid) => uuid,
            Ship::Explosion(_, _, uuid) => uuid,
            Ship::Bullet(_, _, uuid) => uuid,
        }
    }

    pub fn get_action(&mut self, cords: Cords, hashbrowns: &mut HashMap<Cords, Ship>)-> ShipActions {
        match self {
            Ship::Fly(ai, _, _) => ai,
            Ship::Explosion(ai, _, _) => ai,
            Ship::Bullet(ai, _, _) => ai,
        }
        ai.get_action(cords, hashbrowns)
    }

    pub fn wrap(&self)->bool {
        match self {
            Ship::Fly(_, fool, _) => fool,
            Ship::Explosion(_, fool, _) => fool,
            Ship::Bullet(_, fool, _) => fool,
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
            true,  // Wrap is enabled for Fly.
            Uuid::new_v4(),
        )
    }

    pub fn new_bullet(moving_down: bool)->Self{
        let movement: if moving_down {
            RelCords(1, 0)
        } 
        else {
            RelCords(-1, 0)
        }
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
    // Add a function called `display_char`:
    // This function should take `&self` and return a `char`.
    // Match on `self`:
    // - If `Ship::Fly`, return 'F'.
    // - If `Ship::Explosion`, return 'X'.
    // - If `Ship::Bullet`, return '|'.

    // Add a function called `get_id`:
    // This function should take `&self` and return a `Uuid`.
    // Use a match statement on `self` and return the ID.

    // Add a function called `get_action`:
    // This function will take `&mut self`, `Cords`, and `&mut HashMap<Cords, Ship>`.
    // It should return `ShipAction`.
    // Use a match statement on `self` to get the `ShipAI` field (use the variable name `ai`),
    // and then return `ai.get_action(cords, game_board)`.

    // Add a function called `wrap`:
    // This function will return the boolean field that decides if the ship will come back around when it hits a wall.
    // This function should take `&self` and return `bool`.
    // Use a match statement on `self` to return the `wrap` value.

    // Add a function called `new_fly`:
    // This function has no parameters and returns `Self`.
    // Paste the code below (I'll add notes later, but all it does is create a new `Fly` ship).
    // Return the following:
    /*
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
            true,  // Wrap is enabled for Fly.
            Uuid::new_v4(),
        )
    */

    // Add a function called new_bullet
    // This function should take in a bool called moving_down and return Self
    // Create a variable called movement
        // If moving_down, set the variable to RelCords(1, 0)
        // Else, set the variable to RelCords(-1, 0)
    // Then paste in this code, I'll explain it later (it creates a bullet)
    /*
    Self::Bullet(
        ShipAI::new(
            10, 
            vec![(None, AIAction::RelativeMove(movement))],
        ), 
        false, 
        Uuid::new_v4(),
    )
    */


    // Add a function called new_explosion that returns Self
    // Paste this code in, it cerates a new Explosion ship
    /*
    Self::Explosion(
        ShipAI::new(
            10, 
            vec![(None, AIAction::Remove)],
        ), 
        false, 
        Uuid::new_v4(),
    )
     */

    


pub struct ShipAI {
    pub timer: Timer,
    pub actions: Vec<(Option<Condition>, AIAction)>,
    pub action_index: usize,
}

impl ShipAI {
    pub fn new(action_interval: u64, actions: Vec<Option<Condition>, AIAction>)-> Self {
        timer: action_interval,
        actions: actions,
        action_index: 0,
    }

    pub fn get_ai_action(&mut self, cords: Cords, game_board: &HashMap<Cords, Ship>)-> AIaction {
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
// Create an `impl` block for `ShipAI`

// Add a function called `new` with two parameters:
// - `action_interval: u64`
// - `actions: Vec<Option<Condition>, AIAction>>` 

// This function should return `Self`
// Return an instance of `ShipAI` where:
// - `timer` is set to a new timer with the interval as the `action_interval` parameter.
// - `actions` is set to the `actions` parameter.
// - `action_index` is set to 0.

// Create a function called `get_ai_action`
// This function should take in `&mut self`, `Cords`, `&HashMap<Cords, Ship>`, and return `AIAction`.
// Remember, `ShipAI` has a field called `actions`. If this is empty, the ship isn't supposed to do anything.
// Create an `if` statement to check if `self.actions.is_empty()`. If true, return `AIAction::Nothing`.

// Create an `if` statement to check if `self.timer.tick()` (meaning if it's time to do the next action).
// If so:
// - We need to store the action in `self.actions` at the `self.action_index`.
// - Since `self.actions` contains a vector of tuples, setting it to `&self.actions[self.action_index]` will return a tuple.
// - Let `(condition, action) = &self.actions[self.action_index]` to separate the tuple values.
// - Now `condition` contains the `Option<Condition>`, and `action` contains the `AIAction`.

// Create another `if` statement to check if `condition` is `Some`.
// If so, create another `if` statement to check if `!condition.evaluate(cords, game_board)` (I'll explain this line later).
// - If so, run `self.next_action()` and return `self.get_ai_action(cords, game_board)`.

// Run `self.next_action()` and return `action.clone()`.

// Otherwise, return `AIAction::Nothing`.


// Add a function called `next_action` that takes in `&mut self`
// This function needs to check if we've run the last action in a sequence.
// Use `self.action_index == self.actions.len() - 1` to check if it's the last action.
// - If so, set `self.action_index` to 0.
// - If not, increment `self.action_index` by 1.

// Add a function called `get_action`
// This function should take in `&mut self`, `Cords`, and `&HashMap<Cords, Ship>`.
// It should return `ShipAction`.
// Paste this command below, I'll explain later: `self.get_ai_action(cords, game_board).to_ship_action(cords, game_board)`.


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
// Create an `impl` block for `Condition`
// Add a function called `evaluate` that takes in `&self`, `cords: Cords`, and `game_board: &HashMap<Cords, Ship>`.
// This has a lot of code that will be hard for me to explain right now, but I'll add explanations later.

// Match on `self` (which is a variant of `Condition`):
// - If `self` is `Condition::ShipExists(ref target_cords)`,
//   return `game_board.contains_key(target_cords)` to check if the target position exists on the board.

// - If `self` is `Condition::PositionAvailable(rel_cords)`,
//   return `game_board.get(&rel_cords.evaluate(cords).0).is_none()` to check if the position is available on the game board.

// - If `self` is `Condition::DontShootIfShipsAreBelow(_)`:
//   - Create a variable called `below_cords` and set it to the `cords` parameter.
//   - Create a loop that checks below the current position:
//     - If `!game_board.contains_key(&below_cords)`, break the loop, meaning no ship is below.
//     - Create an `if let` statement for `game_board.get(&below_cords)` (name the value `ship`).
//       - Inside that, add another `if let Ship::Fly(_, _, _) = ship`, and return `false` to prevent shooting if a fly ship is below.
//     - Add 1 to `below_cords.0` to move down the board (increasing the row position).
    
//   - After the loop, return `true` to indicate that it's safe to shoot if no fly ships were found below.


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

            AIAction::Move => {
                return ShipAction::Move(cords, false);
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
                let (new_cords, wrap) = rel_cords.evaluate(cords)
                ShipAction::Move(new_cords, wrap)
            }

            AIAction::ShootOrNothing => {
                let condition = Condition::DontShootIfShipsAreBelow(RelCords(1, 0));
                match condition.evaluate(cords, game_board) => {
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
// Create an impl block for AIAction
// Add a function called to_ship_action
// This function should take in self, cords: Cords, and game_board: &HashMap<Cords, Ship>
// It should return ShipAction

// Inside the function, match on self (the AIAction enum)
// For each variant of the AIAction, return the corresponding ShipAction

// Handle each variant as follows:

// 1. If self is AIAction::Remove
//    - Return ShipAction::Remove (remove the ship)

// 2. If self is AIAction::Shoot
//    - Return ShipAction::Shoot (the ship shoots)

// 3. If self is AIAction::Move(cords)
//    - Return ShipAction::Move(cords, false) (move the ship to the provided coordinates, without wrapping)

// 4. If self is AIAction::MoveOrNothing(rel_cords)
//    - Create a condition using Condition::PositionAvailable with rel_cords (check if the position is available)
//    - If the position is available, use rel_cords.evaluate(cords) to get the new coordinates and whether the ship will wrap around the board
//    - Return ShipAction::Move(new_cords, wrap) if the position is available, otherwise return ShipAction::Nothing

// 5. If self is AIAction::RelativeMove(rel_cords)
//    - Use rel_cords.evaluate(cords) to calculate the new coordinates and check if wrapping is needed
//    - Return ShipAction::Move(new_cords, wrapped) (move the ship to the new coordinates, applying wrap if necessary)

// 6. If self is AIAction::ShootOrNothing
//    - Create a condition using Condition::DontShootIfShipsAreBelow (check if there are ships below the current position)
//    - If the condition is true, return ShipAction::Shoot (the ship shoots)
//    - If not, return ShipAction::Nothing (the ship does nothing)

// 7. If self is AIAction::Nothing
//    - Return ShipAction::Nothing (the ship does nothing)

// The function should return the corresponding ShipAction based on the conditionals

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






// Create an impl block for GameLevel
// Add a function called new
// This function should take in a level of type Level and return Self (GameLevel)
// Inside the function, return a new instance of GameLevel with current_level set to the provided level

// Add a function called get_level_status
// This function should take in &self and return a tuple of (u64, u8)
// Inside the function, match on self.current_level (the Level enum)

// For each variant of the Level enum, call the corresponding function (easy(), medium(), or hard()) 
// that returns a tuple with speed and lives

// Create a private function called easy
// This function should return a tuple of (u64, u8) representing speed and lives for the Easy level
// Create variables speed at 500 and lives at 5
// Return the tuple (speed, lives)

// Create a private function called medium
// This function should return a tuple of (u64, u8) representing speed and lives for the Medium level
// Create variables speed at 300 and lives at 3
// Return the tuple (speed, lives)

// Create a private function called hard
// This function should return a tuple of (u64, u8) representing speed and lives for the Hard level
// Create variables speed at 100 and lives at 1
// Return the tuple (speed, lives)
//done




// This is a stupid complicated funciton that i'm not gonna have you write, but i will explain using notes later
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
