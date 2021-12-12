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
    fn add_record(&mut self, chat_id: i64, user_id: i64, value: f32) -> Result<&str, &str>;
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

    fn player_list(&mut self, chat_id: i64) -> String {

        let list_of_players : Vec<tally::UserTally> = match self.players.get_mut(&chat_id) {
            Some(v) => {
                if v.is_empty(){
                    return format!("No users have been registered yet")
                } else {
                    v.values().cloned().collect()
                }
            },
            None => return format!("No users have been initialized in this chat, try to register some users first"),
        };
        
        let mut return_string : String = "Players:".to_string();
        for player in &list_of_players {
            return_string = return_string + "\n" + &player.name;
        }

        format!("{}", return_string)
    }

    fn get_random(&mut self, chat_id: i64) -> String {

        let all_players = match self.players.get_mut(&chat_id) {
            Some(v) => {
                if v.is_empty(){
                    return format!("No users have been registered yet")
                } else {
                    v.values().cloned().collect()
                }
            },
            None => return format!("No users have been initialized in this chat, try to register some users first"),
        };
    
        let my_players :  Vec<tally::UserTally> = all_players;

        let rng = rand::thread_rng().gen_range(0..my_players.len());
        format!("{}", my_players[rng].name).to_string()
    }

    fn add_record(&mut self, chat_id: i64, user_id: i64, value: f32) -> Result<&str, &str> {
        if let Some(chat) = self.players.get_mut(&chat_id) {
            if chat.is_empty() {
                Err("No users have been initialized in this chat, try to register some users first")
            } else if let Some(user) = chat.get_mut(&user_id) {
                user.add_record(value)
            } else {
                // This should not happen
                Err("The user was not found")
            }
        } else {
            Err("No users have been registered yet")
        }
    }
}
