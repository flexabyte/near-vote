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
}

#[near_bindgen]
impl Vote {

    pub fn initialize(&mut self) {
        self.allowedVotes = vec![
            "Beyond".to_string(), 
            "Impossible".to_string(), 
            "Fry's".to_string(), 
            "Squeaky Bean".to_string()
        ];
        
    }

    #[payable]
    pub fn add_vote(&mut self, vote: String) {
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

    pub fn get_total_votes(&self, option: String) -> Option::<u128> {
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
            .build()
    }

    #[test]
    fn set_get_vote() {
        let context = get_context(false);
        testing_env!(context);
        let mut contract = Vote::default();
        contract.initialize();
        contract.add_vote("Beyond".to_string());
        assert_eq!(get_logs(), vec!["bob_near voting for Beyond."]);
        let context = get_context(true);
        testing_env!(context);
        let vote = contract.get_vote();
        assert_eq!("Beyond".to_string(), vote.unwrap());
        assert_eq!(get_logs(), vec!["get_vote for account_id bob_near"]);
        let total_votes = contract.get_total_votes("Beyond".to_string());
        assert_eq!(1, total_votes.unwrap());
        assert_eq!(get_logs(), vec!["get_vote for account_id bob_near"]);
    }

    #[test]
    fn add_invalid_vote() {
        let context = get_context(false);
        testing_env!(context);
        let mut contract = Vote::default();
        contract.initialize();
        contract.add_vote("FutureFarm".to_string());
        assert_eq!(get_logs(), vec!["bob_near cannot vote for FutureFarm. Not a valid voting option."]);
    }
}
