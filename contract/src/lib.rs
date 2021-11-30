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
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, setup_alloc};

use nanoid::nanoid;

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
  creator_id: String,
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
  creator_id: String,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct DonateInfo {
  donate_info_id: String,
  answer_id: String,
  donate_creator_id: String,
  created_time: i64,
  amount: i64,
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
  amount: i64,
}

// Structs in Rust are similar to other languages, and may include impl keyword as shown below
// Note: the names of the structs are not important when calling the smart contract, but the function names are
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct QAndANear {
  map_question: UnorderedMap<String, Question>,
  map_answer: LookupMap<String, Answer>,
  map_donation_info: LookupMap<String, DonateInfo>,
  map_question_answer: LookupMap<String, UnorderedSet<String>>, // question_id => list answers id
  map_answer_donation: LookupMap<String, UnorderedSet<String>>, // answer_id => list donation id info
}

impl Default for QAndANear {
  fn default() -> Self {
    Self {
      map_question: UnorderedMap::new(b"mq".to_vec()),
      map_answer: LookupMap::new(b"ma".to_vec()),
      map_donation_info: LookupMap::new(b"md".to_vec()),
      map_question_answer: LookupMap::new(b"mqa".to_vec()),
      map_answer_donation: LookupMap::new(b"mad".to_vec()),
    }
  }
}

#[near_bindgen]
impl QAndANear {
  pub fn create_question(&mut self, question: QuestionCreateDto) -> Option<Question> {
    let creator_id = env::signer_account_id();
    let question_save = Question {
      question_id: self.generate_id(),
      title: question.title,
      content: question.content,
      total_vote: 0,
      total_answer: 0,
      created_time: self.get_current_timestamp_in_millis(),
      creator_id: creator_id,
    };
    self.map_question_answer.insert(
      &question_save.question_id,
      &UnorderedSet::new(env::sha256(&question_save.question_id.as_bytes())),
    );
    return self
      .map_question
      .insert(&question_save.question_id, &question_save);
  }

  pub fn create_answer(&mut self, answer: AnswerCreateDto) -> Option<Answer> {
    let creator_id = env::signer_account_id();
    let answer_save = Answer {
      answer_id: self.generate_id(),
      question_id: answer.question_id,
      content: answer.content,
      total_vote: 0,
      total_amount_donate: 0,
      created_time: self.get_current_timestamp_in_millis(),
      creator_id: creator_id,
    };
    self.map_answer_donation.insert(
      &answer_save.answer_id,
      &UnorderedSet::new(env::sha256(&answer_save.answer_id.as_bytes())),
    );
    return self.map_answer.insert(&answer_save.answer_id, &answer_save);
  }

  pub fn donate(&mut self, donation: DonationCreateDto) -> Option<DonateInfo> {
    let creator_id = env::signer_account_id();
    let answer_id = donation.answer_id.clone();
    let donation_save = DonateInfo {
      donate_info_id: self.generate_id(),
      answer_id: answer_id.clone(),
      donate_creator_id: creator_id,
      created_time: self.get_current_timestamp_in_millis(),
      amount: donation.amount,
    };
    // update answer total donate
    match self.map_answer.get(&answer_id) {
      Some(mut answer) => {
        // update map map_answer_donation
        match self.map_answer_donation.get(&answer_id) {
          Some(mut set_donation) => {
            if !(set_donation.insert(&donation_save.donate_info_id)) {
              env::panic(
                format!(
                  "Set donation has contain for donate_info_id {}",
                  donation_save.donate_info_id
                )
                .as_bytes(),
              );
            }
            answer.total_amount_donate += donation.amount;
            self.map_answer.insert(&answer.answer_id, &answer);
            self.map_answer_donation.insert(&answer_id, &set_donation);
            return self
              .map_donation_info
              .insert(&donation_save.donate_info_id, &donation_save);
          }
          None => {
            env::panic(
              format!("Map answer donation for answer_id {} not found", answer_id).as_bytes(),
            );
          }
        }
      }
      None => {
        env::panic(format!("Map answer for answer_id {} not found", answer_id).as_bytes());
      }
    }
  }

  pub fn get_list_question(&self) -> Vec<Question> {
    let mut vec_ret = <Vec<Question>>::new();
    for (_, question) in self.map_question.iter() {
      vec_ret.push(question);
    }
    return vec_ret;
  }

  pub fn get_question_detail(&self, question_id: String) -> Option<Question> {
    return self.map_question.get(&question_id);
  }

  pub fn get_list_answer_for_question(&self, question_id: String) -> Vec<Answer> {
    let mut vec_ret = <Vec<Answer>>::new();
    match self.map_question_answer.get(&question_id) {
      Some(set_answer) => {
        for answer_id in set_answer.iter() {
          match self.map_answer.get(&answer_id) {
            Some(answer) => {
              vec_ret.push(answer);
            }
            None => {}
          }
        }
        return vec_ret;
      }
      None => {
        env::panic(format!("Question with question_id {} not found", question_id).as_bytes());
      }
    }
  }

  pub fn get_donate_history(&self, answer_id: String) -> Vec<DonateInfo> {
    let mut vec_ret = <Vec<DonateInfo>>::new();
    match self.map_answer_donation.get(&answer_id) {
      Some(set_donation) => {
        for donation_id in set_donation.iter() {
          match self.map_donation_info.get(&donation_id) {
            Some(donation) => {
              vec_ret.push(donation);
            }
            None => {}
          }
        }
        return vec_ret;
      }
      None => {
        env::panic(format!("Answer with answer_id {} not found", answer_id).as_bytes());
      }
    }
  }

  #[private]
  pub fn generate_id(&self) -> String {
    return nanoid!();
  }

  #[private]
  pub fn get_current_timestamp_in_millis(&self) -> i64 {
    return ((env::block_timestamp() / (86400 * 1000000000)) as i64) * (86400 * 1000);
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

  #[test]
  fn test_generate_id() {
    let context = get_context(vec![], false);
    testing_env!(context);
    let contract = QAndANear::default();
    let id1 = contract.generate_id();
    println!("{}", id1);
    let id2 = contract.generate_id();
    println!("{}", id2);
    assert_ne!(id1, id2);
  }
}
