use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, metadata, near_bindgen, AccountId};

use std::collections::HashMap;

metadata! {
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Vote {
    allowedVotes: Vec<String>,
    userVotes: HashMap<AccountId, String>,
    totalVotes: HashMap<String, u128>,
    endTimestamp: u64,
    voteStarted: bool,
}

#[near_bindgen]
impl Vote {

    // These are hardcoded to prevent any option tampering.
    pub fn initialize(&mut self, allowedOptions: Vec<String>, endTimestamp: u64) {
        assert!(!self.voteStarted, "Vote is already ongoing.");
        self.allowed_votes = allowedOptions;
        self.endTimestamp = endTimestamp;
        self.voteStarted = true;
    }

    pub fn get_options(&self) -> Vec::<String> {
        self.allowedVotes.clone()
    }

    /// Allows a user to vote on a specific option.
    #[payable]
    pub fn add_vote(&mut self, vote: String) {
        assert!(env::block_timestamp() <= self.endTimestamp, "Voting has ended.");
        let account_id = env::signer_account_id();
        if self.userVotes.get(&account_id).is_some() {
            log!("{} already voted! Cannot vote again!", account_id);
        } else {
            if self.allowedVotes.contains(&vote) {
                log!("{} voting for {}.", account_id, vote);
                self.userVotes.insert(account_id, vote.clone());
                let currentVoteCount = self.totalVotes.get(&vote);
                // First vote ?
                if currentVoteCount.is_none() {
                    self.totalVotes.insert(vote, 1);
                } else {
                    self.totalVotes.insert(vote, currentVoteCount.unwrap()+1);
                }
            } else {
                log!("{} cannot vote for {}. Not a valid voting option.", 
                        account_id, vote);
            }
        }
    }

    #[payable]
    pub fn get_vote(&mut self) -> Option::<String> {
        let account_id = env::signer_account_id();
        log!("get_vote for account_id {}", account_id);
        self.userVotes.get(&account_id).cloned()
    }

    // Get total votes can only be called once voting has ended.
    pub fn get_total_votes(&self, option: String) -> Option::<u128> {
        assert!(env::block_timestamp() > self.endTimestamp, "Voting hasn't ended yet.");
        self.totalVotes.get(&option).cloned()
    }

}
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, VMContext};

    fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id("bob_near".parse().unwrap())
            .is_view(is_view)
            .block_timestamp(1653229512000000000) // lower timestamp
            .build()
    }

    fn get_final_context(is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id("bob_near".parse().unwrap())
            .is_view(is_view)
            .block_timestamp(1653429512000000000) // higher timestamp
            .build()
    }

    fn get_new_user_context(username: String) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id(username.parse().unwrap())
            .block_timestamp(1653229512000000000) // lower timestamp during voting
            .build()
    }

    #[test]
    fn set_get_vote() {
        let context = get_context(false);
        testing_env!(context);
        let mut contract = Vote::default();
        let allowed_votes = vec![
            "Beyond".to_string(), 
            "Impossible".to_string(), 
            "Fry's".to_string(), 
            "Squeaky Bean".to_string()
        ];
        contract.initialize(allowed_votes, 1653239512000000000);
        contract.add_vote("Beyond".to_string());
        let context = get_context(true);
        let vote = contract.get_vote();
        assert_eq!("Beyond".to_string(), vote.unwrap());
    }

    #[test]
    fn get_options() {
        let context = get_context(false);
        testing_env!(context);
        let mut contract = Vote::default();
        let allowed_votes = vec![
            "Beyond".to_string(), 
            "Impossible".to_string(), 
            "Fry's".to_string(), 
            "Squeaky Bean".to_string()
        ];
        contract.initialize(allowed_votes, 1653239512000000000);
        let options = contract.get_options();
        assert_eq!(options[0], "Beyond");
        assert_eq!(options[1], "Impossible");
        assert_eq!(options[2], "Fry's");
        assert_eq!(options[3], "Squeaky Bean");
    }

    #[test]
    fn add_invalid_vote() {
        let context = get_context(false);
        testing_env!(context);
        let mut contract = Vote::default();
        let allowed_votes = vec![
            "Beyond".to_string(), 
            "Impossible".to_string(), 
            "Fry's".to_string(), 
            "Squeaky Bean".to_string()
        ];
        contract.initialize(allowed_votes, 1653239512000000000);
        contract.add_vote("FutureFarm".to_string());
        assert_eq!(get_logs(), vec!["bob_near cannot vote for FutureFarm. Not a valid voting option."]);
    }

    #[test]
    #[should_panic]
    fn double_initialize() {
        let context = get_context(false);
        testing_env!(context);
        let mut contract = Vote::default();
        let allowed_votes = vec![
            "Beyond".to_string(), 
            "Impossible".to_string(), 
            "Fry's".to_string(), 
            "Squeaky Bean".to_string()
        ];
        contract.initialize(allowed_votes.clone(), 1653239512000000000);
        contract.initialize(allowed_votes, 9999999999999999999);
    }

    #[test]
    fn voting_ended() {
        let context = get_context(false);
        testing_env!(context);
        let mut contract = Vote::default();
        let allowed_votes = vec![
            "Beyond".to_string(), 
            "Impossible".to_string(), 
            "Fry's".to_string(), 
            "Squeaky Bean".to_string()
        ];
        contract.initialize(allowed_votes, 1653239512000000000);
        contract.add_vote("Beyond".to_string());
        let context = get_final_context(false);
        testing_env!(context);
        let total_votes = contract.get_total_votes("Beyond".to_string());
        assert_eq!(1, total_votes.unwrap());
    }

    #[test]
    fn end_to_end_test() {
        let context = get_context(false);
        testing_env!(context);
        let mut contract = Vote::default();
        let allowed_votes = vec![
            "Beyond".to_string(), 
            "Impossible".to_string(), 
            "Fry's".to_string(), 
            "Squeaky Bean".to_string()
        ];
        contract.initialize(allowed_votes, 1653239512000000000);
        let options = contract.get_options();
        // 100 users, 25 votes for each option.
        for i in 0..100 {
            let context = get_new_user_context(format!("user{}", i));
            testing_env!(context);
            contract.add_vote(options[i%4].clone());
        }
        let context = get_final_context(false);
        testing_env!(context);
        for option in options.iter() {
            let total_votes = contract.get_total_votes(option.to_string());
            println!("{} got {:?} votes.", option, total_votes);
            assert_eq!(25, total_votes.unwrap());
        }
    }
}
