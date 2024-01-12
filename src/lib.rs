// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Initializes a custom, global allocator for Rust programs compiled to WASM.
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

/// Import the Stylus SDK along with alloy primitive types for use in our program.
use stylus_sdk::{prelude::*};

// Define the entrypoint as a Solidity storage object, in this case a struct
// called `Counter` with a single uint256 value called `number`. The sol_storage! macro
// will generate Rust-equivalent structs with all fields mapped to Solidity-equivalent
// storage slots and types.
sol_storage! {
    #[entrypoint]
    pub struct AiAnswers {
        // Important: these are Solidity types not Rust!
        string prompt;
        string answer;
    }
}

/// Define an implementation of the generated Counter struct, defining a set_number
/// and increment method using the features of the Stylus SDK.
#[external]
impl AiAnswers {

    pub fn get_prompt(&self) -> Result<String, Vec<u8>> {
        Ok(self.prompt.get_string())
    }

    pub fn set_prompt(&mut self, new_prompt: String) -> Result<(), Vec<u8>> {
        self.prompt.set_str(new_prompt);
        Ok(())
    }

    pub fn get_answer(&self) -> Result<String, Vec<u8>> {
        Ok(self.answer.get_string())
    }

    pub fn set_answer(&mut self, new_answer: String) -> Result<(), Vec<u8>> {
        self.answer.set_str(new_answer);
        Ok(())
    }
}
