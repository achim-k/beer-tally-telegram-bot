use rand::Rng;

pub enum RegisterPlayerResult {
    InvalidUsername,
    AlreadyRegistered(String),
    UsernameTaken,
    Registered,
}

pub trait BeerTally {
    fn register_player(
        &mut self,
        chat_id: i64,
        user_id: i64,
        username: &str,
    ) -> RegisterPlayerResult;
    fn unregister_player(&mut self, chat_id: i64, user_id: i64) -> Result<(), ()>;
    fn print_player_list(&mut self, chat_id: i64) -> String;
    fn get_random(&self, chat_id: i64) -> String;
    fn change_name(&mut self, chat_id: i64, user_id: i64, username: &str) -> String;
}

use std::collections::HashMap;

pub struct HashMapBeerTally {
    /// Player names for a given chat id and user id.
    /// In other words, the same user can participate in multiple chats.
    players: HashMap<i64, HashMap<i64, UserTally>>,
}

impl HashMapBeerTally {
    pub fn new() -> HashMapBeerTally {
        HashMapBeerTally {
            players: HashMap::new(),
        }
    }

    fn get_players_vect(&self, chat_id: i64) -> Result<Vec<UserTally>, String> {
        if let Some(chat) = self.players.get(&chat_id) {
            if chat.is_empty() {
                // Found the chat, but no users are registered
                // Someone probably unregistered
                Err(
                    "No users have been initialized in this chat, try to register some users first"
                        .to_string(),
                )
            } else {
                Ok(chat.values().cloned().collect())
            }
        } else {
            // The chat could not be found (no one has registered yet)
            Err("No users have been registered yet".to_string())
        }
    }

    fn get_player_mut(&mut self, chat_id: i64, user_id: i64) -> Result<&mut UserTally, &str> {
        if let Some(chat) = self.players.get_mut(&chat_id) {
            if chat.is_empty() {
                // Found the chat, but no users are registered
                // Someone probably unregistered
                Err("No users have been initialized in this chat, try to register some users first")
            } else if let Some(user) = chat.get_mut(&user_id) {
                Ok(user)
            } else {
                // This should not happen
                Err("The user was not found")
            }
        } else {
            // The chat could not be found (no one has registered yet)
            Err("No users have been registered yet")
        }
    }
}

impl BeerTally for HashMapBeerTally {
    fn register_player(
        &mut self,
        chat_id: i64,
        user_id: i64,
        username: &str,
    ) -> RegisterPlayerResult {
        if username.is_empty() || !username.chars().all(char::is_alphanumeric) {
            return RegisterPlayerResult::InvalidUsername;
        }

        match self.players.get_mut(&chat_id) {
            None => {
                let mut usernames = HashMap::new();

                usernames.insert(user_id, UserTally::new(username));
                self.players.insert(chat_id, usernames);
                RegisterPlayerResult::Registered
            }
            Some(usernames) => {
                if let Some(user_tally) = usernames.get(&user_id) {
                    RegisterPlayerResult::AlreadyRegistered(user_tally.name.clone())
                } else if usernames.values().any(|x| x.name == username) {
                    RegisterPlayerResult::UsernameTaken
                } else {
                    usernames.insert(user_id, UserTally::new(username));
                    RegisterPlayerResult::Registered
                }
            }
        }
    }

    fn unregister_player(&mut self, chat_id: i64, user_id: i64) -> Result<(), ()> {
        match self.players.get_mut(&chat_id) {
            None => Err(()),
            Some(usernames) => match usernames.remove(&user_id) {
                Some(_) => Ok(()),
                None => Err(()),
            },
        }
    }

    fn print_player_list(&mut self, chat_id: i64) -> String {
        match self.get_players_vect(chat_id) {
            Ok(list_of_players) => {
                let mut return_string: String = "Players:".to_string();
                for player in &list_of_players {
                    return_string = return_string + "\n" + &player.name;
                }
                return_string
            }
            Err(msg) => msg,
        }
    }

    fn get_random(&self, chat_id: i64) -> String {
        match self.get_players_vect(chat_id) {
            Ok(list_of_players) => {
                let rng = rand::thread_rng().gen_range(0..list_of_players.len());
                list_of_players[rng].name.clone()
            }
            Err(msg) => msg,
        }
    }

    fn change_name(&mut self, chat_id: i64, user_id: i64, username: &str) -> String {
        if username.is_empty() || !username.chars().all(char::is_alphanumeric) {
            return String::from("Invalid username. Only alphanumeric characters are allowed.");
        }

        match self.get_player_mut(chat_id, user_id) {
            Ok(user_tally) => user_tally.change_name(username),
            Err(msg) => msg.to_string(),
        }
    }
}


#[derive(Clone, Debug)]
pub struct UserTally {
    pub name: String,
    pub records: Vec<f32>,
}

impl UserTally {
    // Constructor for UserTally
    pub fn new(username: &str) -> UserTally {
        UserTally {
            name: username.to_string(),
            records: Vec::new(),
        }
    }

    pub fn change_name(&mut self, username: &str) -> String {
        self.name = username.to_string();
        format!("Username has been changed to {}", username)
    }
}
