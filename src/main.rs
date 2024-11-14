use rustyline::error::ReadlineError;
use rustyline::{Editor, history::DefaultHistory};
use std::collections::HashMap;

#[derive(Debug)]
struct Room {
    description: String,
    exits: HashMap<String, String>,
}

#[derive(Debug)]
struct Player {
    current_room: String,
}

#[derive(Debug)]
struct Game {
    rooms: HashMap<String, Room>,
    player: Player,
}

impl Game {
    fn new() -> Self {
        let mut rooms = HashMap::new();

        rooms.insert(
            "Entrance".to_string(),
            Room {
                description: "You are at the entrance of a mysterious cave.".to_string(),
                exits: HashMap::from([("north".to_string(), "Hall".to_string())]),
            },
        );

        rooms.insert(
            "Hall".to_string(),
            Room {
                description: "You are in a grand hall with a high ceiling.".to_string(),
                exits: HashMap::from([
                    ("south".to_string(), "Entrance".to_string()),
                    ("east".to_string(), "Armory".to_string()),
                ]),
            },
        );

        rooms.insert(
            "Armory".to_string(),
            Room {
                description: "You are in the Armory filled with old weapons".to_string(),
                exits: HashMap::from([
                    ("west".to_string(), "Hall".to_string()),
                    ("north".to_string(), "Treasure Room".to_string()),
                ]),
            },
        );

        rooms.insert(
            "Treasure Room".to_string(),
            Room {
                description: "You've found the treasure room filled with gold and jewels!".to_string(),
                exits: HashMap::from([
                    ("south".to_string(), "Armory".to_string()),
                ]),
            },
        );

        let player = Player {
            current_room: "Entrance".to_string(),
        };

        

        Game { rooms, player }
    }

    fn describe_current_room(&self) {
        if let Some(room) = self.rooms.get(&self.player.current_room) {
            println!("{}", room.description);
            println!("Exits: {:?}", room.exits.keys().collect::<Vec<_>>());
        }
    }

    fn move_player(&mut self, direction: &str) {
        let current_room = &self.player.current_room;
        if let Some(room) = self.rooms.get(current_room) {
            if let Some(next_room) = room.exits.get(direction) {
                self.player.current_room = next_room.clone();
                println!("You move {} and enter {}.", direction, next_room);
            } else {
                println!("You can't go that way!");
            }
        }
    }
}

fn main() {
    let mut game = Game::new();
    let mut rl = Editor::<(), DefaultHistory>::new().unwrap();

    println!("Welcome to the Adventure Game!");
    game.describe_current_room();

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let command = line.trim();
                match command {
                    "quit" => break,
                    "look" => game.describe_current_room(),
                    _ if command.starts_with("go ") => {
                        let direction = command.split_at(3).1.trim();
                        game.move_player(direction);
                    }
                    _ => println!("Unknown command. Try 'look' or 'go <direction>'."),
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!("Goodbye!");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}