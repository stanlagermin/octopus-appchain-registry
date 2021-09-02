use std::convert::TryInto;

use near_sdk::Timestamp;

use crate::types::{AppchainMetadata, AppchainState, AppchainStatus};
use crate::*;

/// Appchain basedata
#[derive(BorshDeserialize, BorshSerialize)]
pub struct AppchainBasedata {
    appchain_id: AppchainId,
    appchain_metadata: AppchainMetadata,
    appchain_anchor: AccountId,
    appchain_owner: AccountId,
    register_deposit: Balance,
    appchain_state: AppchainState,
    upvote_deposit: Balance,
    downvote_deposit: Balance,
    registered_time: Timestamp,
    go_live_time: Timestamp,
}

impl AppchainBasedata {
    /// Return a new instance of AppchainBasedata with the given parameters
    pub fn new(
        appchain_id: AppchainId,
        appchain_metadata: AppchainMetadata,
        appchain_owner: AccountId,
        register_deposit: Balance,
    ) -> Self {
        Self {
            appchain_id: appchain_id.clone(),
            appchain_metadata,
            appchain_anchor: String::new(),
            appchain_owner,
            register_deposit,
            appchain_state: AppchainState::Registered,
            upvote_deposit: 0,
            downvote_deposit: 0,
            registered_time: env::block_timestamp(),
            go_live_time: 0,
        }
    }
    /// Get appchain id
    pub fn id(&self) -> &AppchainId {
        &self.appchain_id
    }
    /// Get mutable metadata
    pub fn metadata(&mut self) -> &mut AppchainMetadata {
        &mut self.appchain_metadata
    }
    /// Get acount id of anchor
    pub fn anchor(&self) -> &AccountId {
        &self.appchain_anchor
    }
    /// Get account id of owner
    pub fn owner(&self) -> &AccountId {
        &self.appchain_owner
    }
    /// Get initial deposit
    pub fn register_deposit(&self) -> Balance {
        self.register_deposit
    }
    /// Get state
    pub fn state(&self) -> AppchainState {
        self.appchain_state.clone()
    }
    /// Get upvote deposit
    pub fn upvote_deposit(&self) -> Balance {
        self.upvote_deposit
    }
    /// Get downvote deposit
    pub fn downvote_deposit(&self) -> Balance {
        self.downvote_deposit
    }
    /// Get voting score
    pub fn voting_score(&self) -> i128 {
        if let Some(bytes) = env::storage_read(
            &StorageKey::AppchainVotingScore(self.appchain_id.clone()).into_bytes(),
        ) {
            i128::from_be_bytes(bytes.try_into().expect(&format!(
                "Invalid storage data for voting score of appchain {}",
                self.appchain_id
            )))
        } else {
            0
        }
    }
    /// Get full status
    pub fn status(&self) -> AppchainStatus {
        AppchainStatus {
            appchain_id: self.appchain_id.clone(),
            appchain_metadata: self.appchain_metadata.clone(),
            appchain_anchor: self.appchain_anchor.clone(),
            appchain_owner: self.appchain_owner.clone(),
            register_deposit: self.register_deposit.into(),
            appchain_state: self.appchain_state.clone(),
            upvote_deposit: self.upvote_deposit.into(),
            downvote_deposit: self.downvote_deposit.into(),
            voting_score: self.voting_score().into(),
            registered_time: self.registered_time.into(),
            go_live_time: self.go_live_time.into(),
        }
    }
    /// Change owner
    pub fn change_owner(&mut self, new_owner: &AccountId) {
        assert_ne!(
            self.appchain_owner,
            new_owner.clone(),
            "The owner not changed."
        );
        self.appchain_owner.clear();
        self.appchain_owner.push_str(new_owner);
    }
    /// Set initial deposit
    pub fn set_initial_deposit(&mut self, deposit: Balance) {
        self.register_deposit = deposit;
    }
    /// Set anchor account
    pub fn set_anchor_account(&mut self, anchor_account: &AccountId) {
        self.appchain_anchor.clear();
        self.appchain_anchor.push_str(anchor_account);
    }
    /// Change state
    pub fn change_state(&mut self, new_state: AppchainState) {
        assert_ne!(self.appchain_state, new_state, "The state not changed.");
        self.appchain_state = new_state;
    }
    /// Increase upvote deposit
    pub fn increase_upvote_deposit(&mut self, value: Balance) {
        self.upvote_deposit += value;
    }
    /// Decrease upvote deposit
    pub fn decrease_upvote_deposit(&mut self, value: Balance) {
        self.upvote_deposit = self
            .upvote_deposit
            .checked_sub(value)
            .expect("Upvote deposit is not big enough to decrease.");
    }
    /// Increase upvote deposit
    pub fn increase_downvote_deposit(&mut self, value: Balance) {
        self.downvote_deposit += value;
    }
    /// Decrease upvote deposit
    pub fn decrease_downvote_deposit(&mut self, value: Balance) {
        self.downvote_deposit = self
            .downvote_deposit
            .checked_sub(value)
            .expect("Downvote deposit is not big enough to decrease.");
    }
    /// Count voting score
    pub fn count_voting_score(&self) {
        let voting_score =
            self.voting_score() + self.upvote_deposit as i128 - self.downvote_deposit as i128;
        env::storage_write(
            &StorageKey::AppchainVotingScore(self.appchain_id.clone()).into_bytes(),
            &voting_score.to_be_bytes(),
        );
    }
    /// Reduce voting score by the given percent
    pub fn reduce_voting_score_by_percent(&self, percent: u16) {
        assert!(percent <= 100, "Invalid percent value.");
        let mut voting_score = self.voting_score();
        voting_score -= voting_score * percent as i128 / 100;
        env::storage_write(
            &StorageKey::AppchainVotingScore(self.appchain_id.clone()).into_bytes(),
            &voting_score.to_be_bytes(),
        );
    }
}
