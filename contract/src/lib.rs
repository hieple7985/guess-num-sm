/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen};

// Define the default message
const DEFAULT_GUESSING: u32 = 0;
const DEFAULT_SECRET_NUMBER: u32 = 33;

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    guessing: u32,
    secret_number: u32,
}

// Define the default, which automatically initializes the contract
impl Default for Contract{
    fn default() -> Self{
        Self{
            guessing: DEFAULT_GUESSING,
            secret_number: DEFAULT_SECRET_NUMBER,
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    // Public method - returns the guessing saved, defaulting to DEFAULT_MESSAGE
    pub fn get_guessing(&self) -> u32 {
        return self.guessing.clone();
    }

    pub fn get_guessing_msg(&self) -> String {
        let guessing = self.guessing.clone();
        log!("You guessed number: {}", guessing);
        let result1: String = format!("You guessed number: {}", guessing);

        let secret_number = self.secret_number.clone();
        log!("You got secret_number: {}", secret_number);
        let result2: String = format!("You got secret_number: {}", secret_number);

        let result: String = format!("{result1}, {result2}", result1=result1, result2=result2);

        return result;
    }

    // Public method - accepts a guessing number, such as "from 1 to 100", and records it
    pub fn set_guessing(&mut self, guessing: u32) {
        // Use env::log to record logs permanently to the blockchain!
        log!("Saving guessing {}", guessing);
        self.guessing = guessing;
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_default_guessing() {
        let contract = Contract::default();
        // this test did not call set_guessing so should return the default "Hello" guessing
        assert_eq!(
            contract.get_guessing(),
            "Guess the number".to_string()
        );
    }

    // #[test]
    // fn set_then_get_guessing() {
    //     let mut contract = Contract::default();
    //     contract.set_guessing("Guess the number".to_string());
    //     assert_eq!(
    //         contract.get_guessing(),
    //         "Guess the number".to_string()
    //     );
    // }
}
