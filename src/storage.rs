use rand::Rng;
use super::tally;

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
    fn player_list(&mut self, chat_id: i64) -> String;
    fn get_random(&mut self, chat_id: i64) -> String;
    fn add_record(&mut self, chat_id: i64, user_id: i64, value: f32) -> String;
    fn get_players_vect(&mut self, chat_id: i64) -> Result<Vec<tally::UserTally>, &str>;
    fn get_player(&mut self, chat_id: i64, user_id: i64) -> Result<&mut tally::UserTally, &str>;
}

use std::collections::HashMap;

pub struct HashMapBeerTally {
    /// Player names for a given chat id and user id.
    /// In other words, the same user can participate in multiple chats.
    players: HashMap<i64, HashMap<i64, tally::UserTally>>,
}

impl HashMapBeerTally {
    pub fn new() -> HashMapBeerTally {
        HashMapBeerTally {
            players: HashMap::new(),
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
                
                usernames.insert(user_id, tally::UserTally::new(username));
                self.players.insert(chat_id, usernames);
                RegisterPlayerResult::Registered
            }
            Some(usernames) => {
                if let Some(user_tally) = usernames.get(&user_id) {
                    return RegisterPlayerResult::AlreadyRegistered(
                        user_tally.name.clone(),
                    );
                } else if usernames.values().any(|x| x.name == username) {
                    RegisterPlayerResult::UsernameTaken
                } else {
                    usernames.insert(user_id, tally::UserTally::new(username));
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

    fn get_players_vect(&mut self, chat_id: i64) -> Result<Vec<tally::UserTally>, &str> { 
        if let Some(chat) = self.players.get_mut(&chat_id) {
            if chat.is_empty() {
                // Found the chat, but no users are registered
                // Someone probably unregistered
                Err("No users have been initialized in this chat, try to register some users first")
            } else {
                Ok(chat.values().cloned().collect())
            }
        } else {
            // The chat could not be found (no one has registered yet)
            Err("No users have been registered yet")
        }
    }

    fn get_player(&mut self, chat_id: i64, user_id: i64) -> Result<&mut tally::UserTally, &str> { 
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

    fn player_list(&mut self, chat_id: i64) -> String {
        match self.get_players_vect(chat_id) {
            Ok(list_of_players) => {
                let mut return_string : String = "Players:".to_string();
                for player in &list_of_players {
                    return_string = return_string + "\n" + &player.name;
                }
                format!("{}", return_string)
            },
            Err(msg) =>  format!("{}", msg)
        }
    }

    fn get_random(&mut self, chat_id: i64) -> String {
        match self.get_players_vect(chat_id) {
            Ok(list_of_players) => {
                let rng = rand::thread_rng().gen_range(0..list_of_players.len());
                format!("{}", list_of_players[rng].name)
            },
            Err(msg) =>  format!("{}", msg)
        }
    }

    fn add_record(&mut self, chat_id: i64, user_id: i64, value: f32) -> String {
        match self.get_player(chat_id, user_id) {
            Ok(user) => user.add_record(value),
            Err(msg) => format!("{}", msg)
        }
    }
}
