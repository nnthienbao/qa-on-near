/*
 * This is an example of a Rust smart contract with two simple, symmetric functions:
 *
 * 1. set_greeting: accepts a greeting, such as "howdy", and records it for the user (account_id)
 *    who sent the request
 * 2. get_greeting: accepts an account_id and returns the greeting saved for it, defaulting to
 *    "Hello"
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://github.com/near/near-sdk-rs
 *
 */

// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, LookupSet};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, setup_alloc};

setup_alloc!();

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Question {
  question_id: String,
  title: String,
  content: String,
  total_vote: i32,
  total_answer: i32,
  created_time: i64,
  creator_id: String
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Answer {
  answer_id: String,
  question_id: String,
  content: String,
  total_vote: i32,
  total_amount_donate: i64,
  created_time: i64,
  creator_id: String
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct DonateInfo {
  donate_info_id: String,
  answer_id: String,
  donate_creator_id: String,
  created_time: i64,
  amount: i64
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct QuestionCreateDto {
  title: String,
  content: String,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AnswerCreateDto {
  question_id: String,
  content: String,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct DonationCreateDto {
  answer_id: String,
  amount: i64
}

// Structs in Rust are similar to other languages, and may include impl keyword as shown below
// Note: the names of the structs are not important when calling the smart contract, but the function names are
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct QAndANear {
  map_question: UnorderedMap<String, Question>,
  map_answer: LookupMap<String, Answer>,
  map_donation_info: LookupMap<String, DonateInfo>,
  map_question_answer: LookupMap<String, LookupSet<String>>, // question_id => list answers id
  map_answer_donation: LookupMap<String, LookupSet<String>> // answer_id => list donation id info
}

impl Default for QAndANear {
  fn default() -> Self {
    Self {
      map_question: UnorderedMap::new(b"mq".to_vec()),
      map_answer: LookupMap::new(b"ma".to_vec()),
      map_donation_info: LookupMap::new(b"md".to_vec()),
      map_question_answer: LookupMap::new(b"mqa".to_vec()),
      map_answer_donation: LookupMap::new(b"mad".to_vec())
    }
  }
}

#[near_bindgen]
impl QAndANear {
  pub fn create_question(question: QuestionCreateDto) -> Option<Question> {
    None
  }

  pub fn create_answer(answer: AnswerCreateDto) -> Option<Answer> {
    None
  }

  pub fn donate(donation: DonationCreateDto) -> Option<DonateInfo> {
    None
  }

  pub fn get_list_question() -> Vec<Question> {
    return Vec::new();
  }

  pub fn get_question_detail(question_id: String) -> Option<Question> {
    None
  }

  pub fn get_list_answer_for_question(question_id: String) -> Vec<Answer> {
    return Vec::new();
  }

  pub fn get_donate_history(answer_id: String) -> Vec<DonateInfo> {
    return Vec::new();
  }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 *
 * To run from contract directory:
 * cargo test -- --nocapture
 *
 * From project root, to run in combination with frontend tests:
 * yarn test
 *
 */
#[cfg(test)]
mod tests {
  use super::*;
  use near_sdk::MockedBlockchain;
  use near_sdk::{testing_env, VMContext};

  // mock the context for testing, notice "signer_account_id" that was accessed above from env::
  fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
    VMContext {
      current_account_id: "alice_near".to_string(),
      signer_account_id: "bob_near".to_string(),
      signer_account_pk: vec![0, 1, 2],
      predecessor_account_id: "carol_near".to_string(),
      input,
      block_index: 0,
      block_timestamp: 0,
      account_balance: 0,
      account_locked_balance: 0,
      storage_usage: 0,
      attached_deposit: 0,
      prepaid_gas: 10u64.pow(18),
      random_seed: vec![0, 1, 2],
      is_view,
      output_data_receivers: vec![],
      epoch_height: 19,
    }
  }

  #[test]
  fn test_default() {
    let context = get_context(vec![], false);
    testing_env!(context);
    QAndANear::default();
  }
}
