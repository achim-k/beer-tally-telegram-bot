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
    fn player_list(&mut self, chat_id: i64) -> String;
    fn get_random(&mut self, chat_id: i64) -> String;
}

use std::collections::HashMap;

pub struct HashMapBeerTally {
    /// Player names for a given chat id and user id.
    /// In other words, the same user can participate in multiple chats.
    players: HashMap<i64, HashMap<i64, String>>,
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
                usernames.insert(user_id, username.to_string());
                self.players.insert(chat_id, usernames);
                RegisterPlayerResult::Registered
            }
            Some(usernames) => {
                if let Some(registered_username) = usernames.get(&user_id) {
                    return RegisterPlayerResult::AlreadyRegistered(
                        registered_username.to_string(),
                    );
                } else if usernames.values().any(|x| x == username) {
                    return RegisterPlayerResult::UsernameTaken;
                }
                usernames.insert(user_id, username.to_string());
                RegisterPlayerResult::Registered
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

    fn player_list(&mut self, chat_id: i64) -> String {

        let list_of_players : Vec<String> = match self.players.get_mut(&chat_id) {
            Some(v) => v.values().cloned().collect(),
            None => return format!("No users have been registered yet"),
        };
        
        format!("{:?}", list_of_players)
    }

    fn get_random(&mut self, chat_id: i64) -> String {

        let all_players = match self.players.get_mut(&chat_id) {
            Some(v) => v,
            None => return format!("No users have been registered yet"),
        };
    
        let my_players : Vec<String> = all_players.values().cloned().collect();

        let rng = rand::thread_rng().gen_range(0..my_players.len());
        format!("{}", my_players[rng]).to_string()
    }
}
