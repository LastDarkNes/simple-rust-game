//! This contract implements simple counter backed by storage on blockchain.
//!
//! The contract provides methods to [increment] / [decrement] counter and
//! get it's current value [get_num] or [reset].
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen};
use near_sdk::env::{block_timestamp, predecessor_account_id};
use std::collections::{HashMap};
// use near_sdk::collections::{LookupMap};
    




#[derive(BorshDeserialize, BorshSerialize)]
pub struct Game {
    balance: u64,
    entity_stack: [u8; 12],
    last_click: u64,
    current_click: u64,
}
    

impl Game {
    pub fn new() -> Game {
        Game {
            balance: 1000u64,
            entity_stack: [0; 12],
            last_click: 0u64,
            current_click: 0u64,
        }
    }
}


#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Main {
    max_entity_level: u8,
    max_user_balance: u64,
    users: HashMap<String, Game>,
}

#[near_bindgen]
impl Main {
    // self.max_user_balance = 20_000;
    // self.max_entity_level = 10;

    /// Support methods:

    pub fn validate_user(&mut self, user: String) {
        if !self.users.contains_key(&user) {
            self.users.insert(user, Game::new());
        }
    }

    // Returns players balance
    pub fn get_balance(&self) -> u64 {
        let user = predecessor_account_id().to_string();
        self.validate_user(user);
        let mut user_data = self.users.get(&user);

        return user_data.balance;
    }

    // Returns players entity_stack
    pub fn get_entity_stack(&self) -> [u8; 12]  {
        let user = predecessor_account_id().to_string();
        self.validate_user(user);
        let mut user_data = self.users.get(&user);

        return user_data.entity_stack;
    }

    // Returns max entity tier from players stack
    pub fn get_max_tier_entity(&self) -> u8{
        let user = predecessor_account_id().to_string();
        self.validate_user(user);
        let mut user_data = self.users.get(&user);

        let mut max = 0u8;

        for i in user_data.entity_stack {

            if i > max {
                max = i;
            }
        }

        return max;
    }

    // Adds new entity to players stack
    // !!!PRIVATE METHOD!!!
    fn add_entity(&mut self, tier: u8, slot: usize) {
        let user = predecessor_account_id().to_string();
        self.validate_user(user);
        let mut user_data = self.users.get(&user);

        user_data.entity_stack[slot] = tier; 
    }


    // Players methods:

    //Buyes new entities by players balance
    pub fn buy_entity(&mut self, tier: u8) {
        let user = predecessor_account_id().to_string();
        self.validate_user(user);
        let mut user_data = self.users.get(&user);

        let mut index = 0usize;
        let max_tier_entity = self.get_max_tier_entity();
        let cost = u64::from(tier) * 1000;

        if max_tier_entity < tier {
            panic!("Your max entity tier is {}, you need at least one {} tier entity", max_tier_entity, tier);

        } else if self.balance < cost {
            panic!("You are not enough money");

        } else if tier > self.max_entity_level {
            panic!("Requested entity tier greater than maximum");
        }


        for i in user_data.entity_stack {
            index += 1;

            if i == 0 {
                self.add_entity(tier, index - 1);
                user_data.balance -= cost;
                break;

            }
       
        }
        panic!("You are hasn't empety slots");
    }

    // Merging two same tier entities from players stack
    pub fn merge_entities(&mut self, st_slot: usize, nd_slot: usize) {
        let user = predecessor_account_id().to_string();
        self.validate_user(user);
        let mut user_data = self.users.get(&user);

        let st_entity = user_data.entity_stack[st_slot]; 
        let nd_entity = user_data.entity_stack[nd_slot];

        if st_entity != nd_entity {

            panic!("You can merge only same tier entities");

        } else {

            user_data.entity_stack[nd_slot] = nd_entity + 1;
            user_data.entity_stack[st_slot] = 0;
        }
    }


    // By clicking gives to player 100*sum_all_tiers scores and returns new balance
    // P.S Work every 5 min
    pub fn click(&mut self) {
        let user = predecessor_account_id().to_string();
        self.validate_user(user);
        let mut user_data = self.users.get(&user);

        let mut summ = 0u64;
        user_data.current_click = block_timestamp();

        
        if user_data.current_click - user_data.last_click >= 60000000000 || user_data.last_click == 0 {

            for i in user_data.entity_stack{
                summ += i;
            }
            user_data.balance += summ * 100;
            user_data.last_click = user_data.current_click;
            
        } else {
            panic!("You can't click now");    
        }
        
    }


    
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be: `cargo test`
 * Note: 'rust-counter-tutorial' comes from cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_balance() {
        let game = Game {
            balance: 1000,
            entity_stack: [0; 12],
            last_click: 0,
            current_click: 0,
        };

        assert_eq!(1000, game.balance);
    }

    #[test]
    fn get_entity_stack() {
        let game = Game {
            balance: 1000,
            entity_stack: [0; 12],
            last_click: 0,
            current_click: 0,
        };
        
        assert_eq!([0; 12], game.entity_stack);
    }


    #[test]
    fn buy_entity() {
        let mut game = Game {
            balance: 3000,
            entity_stack: [0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            last_click: 0,
            current_click: 0,
        };

        game.buy_entity(2);
        assert_eq!(2, game.entity_stack[0]);

    }


    #[test]
    #[should_panic]
    fn buy_entity2() {
        let mut game = Game {
            balance: 2000,
            entity_stack: [1, 1, 2, 2, 3, 4, 5, 1, 1, 2, 3, 4],
            last_click: 0,
            current_click: 0,
        };

        
        game.buy_entity(1)

    }

 
}


