//! This contract implements simple counter backed by storage on blockchain.
//!
//! The contract provides methods to [increment] / [decrement] counter and
//! get it's current value [get_num] or [reset].
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen};
use near_sdk::env::{block_timestamp};
    


#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Game {
    balance: u64,
    entity_stack: [u8; 12],
    last_click: u64,
    current_click: u64,
}

#[near_bindgen]
impl  Game {

    /// Support methods:

    // Returns players balance
    pub fn get_balance(&self) -> u64 {
        return self.balance;
    }

    // Returns players entity_stack
    pub fn get_entity_stack(&self) -> [u8; 12]  {
        return self.entity_stack;
    }

    // Returns max entity tier from players stack
    pub fn get_max_tier_entity(&self) -> u8{
        let mut max = 0u8;

        for i in self.entity_stack {

            if i > max {
                max = i;
            }
        }

        return max;
    }

    // Adds new entity to players stack
    // !!!PRIVATE METHOD!!!
    fn add_entity(&mut self, tier: u8, slot: usize) {
        self.entity_stack[slot] = tier; 
    }


    // Players methods:

    //Buyes new entities by players balance
    pub fn buy_entity(&mut self, tier: u8) {

        let mut index = 0usize;
        let max_tier_entity = self.get_max_tier_entity();
        let cost = u64::from(tier) * 1000;

        for i in self.entity_stack {
            index += 1;

            if max_tier_entity < tier {
                panic!("Your max entity tier is {}, you need at least one {} tier entity", max_tier_entity, tier);

            } else if self.balance < cost{

                panic!("You are not enough money");

            } else if i == 0 {
                self.add_entity(tier, index - 1);
                self.balance -= cost;
                break;

            } else {
                panic!("You are hasn't empety slots");

            }       
        }
    }

    // Merging two same tier entities from players stack
    pub fn merge_entities(&mut self, st_slot: usize, nd_slot: usize) {
        let st_entity = self.entity_stack[st_slot]; 
        let nd_entity = self.entity_stack[nd_slot];

        if st_entity != nd_entity {

            panic!("You can merge only same tier entities");

        } else {

            self.entity_stack[nd_slot] = nd_entity + 1;
            self.entity_stack[st_slot] = 0;
        }
    }


    // By clicking gives to player 100*sum_all_tiers scores and returns new balance
    // P.S Work every 5 min
    pub fn click(&mut self) -> u64   {

        let mut summ = 0u32;
        self.current_click = block_timestamp();

        
        if self.current_click - self.last_click >= 60000000000 || self.last_click == 0 {

            log!("{}", self.last_click);

            for i in self.entity_stack{
                summ += u32::from(i);
            }
            self.balance += u64::from(summ) * 1000;
            self.last_click = self.current_click;
            
        } else {
            panic!("You can't click now");    
        }
        
        return self.balance;
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
    fn buy_entity2() {
        let mut game = Game {
            balance: 2000,
            entity_stack: [1, 1, 2, 2, 3, 4, 5, 1, 1, 2, 3, 4],
            last_click: 0,
            current_click: 0,
        };

        
        assert_eq!(panic!("You are not enough money"), game.buy_entity(3));

    }

 
}


