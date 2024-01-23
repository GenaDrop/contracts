use std::collections::{HashSet, HashMap};

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, LookupMap};
use near_sdk::json_types::{U128, U64};
use near_sdk::serde::Serialize;
use near_sdk::{BorshStorageKey, AccountId, PanicOnDefault};
use serde::Deserialize;

use crate::TokenId;

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    Contest,
    Entry,
    VoteMap(String),
    Submission(String),
    VoteRecord(String),
    Winners(String),
}

/// nomination struct
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Art {
    pub title: String,
    pub contract_id: AccountId,
    /// timestamp in ms
    pub timestamp: u64,
    /// sum of received upvotes
    pub votes: u32,
    pub token_id: TokenId,
    pub image_url: String
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PayOutInfo {
    pub amount: f64,
    pub proposal_id: Option<u64>,
}



#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct ContestSession {
    pub submission_start_time: u64, // 20 days in seconds
    pub submission_end_time: u64,
    pub submissions: UnorderedMap<AccountId, Art>,
    pub voting_start_time: u64,    // 10 days in seconds
    pub voting_end_time: u64,
    pub title: String,
    pub description: String,
    pub dao_id: AccountId,
    pub logo_url: String,
    pub art_voters_mapping: LookupMap<AccountId, Vec<AccountId>>, // map artist to a vec of their voters
    pub vote_record: Vec<AccountId>,
    pub winners: UnorderedMap<AccountId, PayOutInfo>,
    pub blacklist: Vec<AccountId>,
    pub prize: f64,
    pub places: i16,
    pub quorum: i16,
    // minimum number of votes an art must have to be considered for a winning slot
    pub min_art_vote: i16,
    pub creator: AccountId,
}


#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct SerializeableContestSession {
    pub submission_start_time: u64, // 20 days in seconds
    pub submission_end_time: u64,
    pub voting_start_time: u64,    // 10 days in seconds
    pub voting_end_time: u64,
    pub title: String,
    pub description: String,
    pub dao_id: AccountId,
    pub prize: f64,
    pub places: i16,
    pub logo_url: String,
    pub submissions: u64,
    pub winners: Vec<AccountId>,
    pub creator: AccountId

}

/// How the voting policy votes get weigthed.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
#[serde(crate = "near_sdk::serde")]
pub enum WeightKind {
    /// Using token amounts and total delegated at the moment.
    TokenWeight,
    /// Weight of the group role. Roles that don't have scoped group are not supported.
    RoleWeight,
}


#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq))]
#[serde(crate = "near_sdk::serde")]
#[serde(untagged)]
pub enum WeightOrRatio {
    Weight(U128),
    Ratio(u64, u64),
}


/// Defines configuration of the vote.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq))]
#[serde(crate = "near_sdk::serde")]
pub struct VotePolicy {
    /// Kind of weight to use for votes.
    pub weight_kind: WeightKind,
    /// Minimum number required for vote to finalize.
    /// If weight kind is TokenWeight - this is minimum number of tokens required.
    ///     This allows to avoid situation where the number of staked tokens from total supply is too small.
    /// If RoleWeight - this is minimum number of votes.
    ///     This allows to avoid situation where the role is got too small but policy kept at 1/2, for example.
    pub quorum: U128,
    /// How many votes to pass this vote.
    pub threshold: WeightOrRatio,
}


#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq))]
#[serde(crate = "near_sdk::serde")]
pub enum RoleKind {
    /// Matches everyone, who is not matched by other roles.
    Everyone,
    /// Member greater or equal than given balance. Can use `1` as non-zero balance.
    Member(U128),
    /// Set of accounts.
    Group(HashSet<AccountId>),
}


#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq))]
#[serde(crate = "near_sdk::serde")]
pub struct RolePermission {
    /// Name of the role to display to the user.
    pub name: String,
    /// Kind of the role: defines which users this permissions apply.
    pub kind: RoleKind,
    /// Set of actions on which proposals that this role is allowed to execute.
    /// <proposal_kind>:<action>
    pub permissions: HashSet<String>,
    /// For each proposal kind, defines voting policy.
    pub vote_policy: HashMap<String, VotePolicy>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq))]
#[serde(crate = "near_sdk::serde")]
pub struct Policy {
    /// List of roles and permissions for them in the current policy.
    pub roles: Vec<RolePermission>,
    /// Default vote policy. Used when given proposal kind doesn't have special policy.
    pub default_vote_policy: VotePolicy,
    /// Proposal bond.
    pub proposal_bond: U128,
    /// Expiration period for proposals.
    pub proposal_period: U64,
    /// Bond for claiming a bounty.
    pub bounty_bond: U128,
    /// Period in which giving up on bounty is not punished.
    pub bounty_forgiveness_period: U64,
}