use std::collections::HashMap;

use near_sdk::json_types::Base64VecU8;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{ext_contract, AccountId};

use crate::storage::Policy;

/// sbt_tokens_by owner interface for cross-contract calls
#[ext_contract(ext_sbtreg)]
pub trait ExtSbtRegistry {
    fn sbt_tokens_by_owner(
        &self,
        account: AccountId,
        issuer: Option<AccountId>,
        from_class: Option<u64>,
        limit: Option<u32>,
        with_expired: Option<bool>,
    ) -> Vec<(AccountId, Vec<OwnedToken>)>;

    fn is_human(&self, account: AccountId) -> Vec<(AccountId, Vec<TokenId>)>;
}

#[ext_contract(ext_contract)]
pub trait ExtNftContract{
    fn nft_token(
        &self,
        token_id: String
    ) -> Token;

}

#[ext_contract(ext_proposal_contract)]
pub trait ExtProposalContract{
    fn get_policy(
        &self,
    ) -> Policy;

}

// TODO: use SBT crate once it is published

/// token data for sbt_tokens_by_owner response
#[derive(Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct OwnedToken {
    pub token: u64,
    pub metadata: TokenMetadata,
}

/// TokenMetadata defines attributes for each SBT token.
#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenMetadata {
    pub class: u64,
    pub issued_at: Option<u64>,
    pub expires_at: Option<u64>,
    pub reference: Option<String>,
    pub reference_hash: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
#[serde(crate = "near_sdk::serde")]
pub struct TokenMetadataCompliant {
    /// the Title for this token. ex. "Arch Nemesis: Mail Carrier" or "Parcel 5055"
    pub title: Option<String>,
    /// free-form description of this token.
    pub description: Option<String>,
    /// URL to associated media, preferably to decentralized, content-addressed storage
    pub media: Option<String>,
    /// Base64-encoded sha256 hash of content referenced by the `media` field.
    /// Required if `media` is included.
    pub media_hash: Option<Base64VecU8>,
    /// number of copies of this set of metadata in existence when token was minted.
    pub copies: Option<u16>,
    /// When token was issued or minted, Unix epoch in milliseconds
    pub issued_at: Option<String>,
    /// ISO 8601 datetime when token expires.
    pub expires_at: Option<String>,
    /// ISO 8601 datetime when token starts being valid.
    pub starts_at: Option<String>,
    /// When token was last updated, Unix epoch in milliseconds
    pub updated_at: Option<String>,
    /// Brief description of what this thing is. Used by the mintbase indexer as "memo".
    pub extra: Option<String>,
    /// URL to an off-chain JSON file with more info. The Mintbase Indexer refers
    /// to this field as `thing_id` or sometimes, `meta_id`.
    pub reference: Option<String>,
    /// Base64-encoded sha256 hash of JSON from reference field. Required if
    /// `reference` is included.
    pub reference_hash: Option<Base64VecU8>,
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Token {
    pub token_id: String,
    /// The current owner of this token. Either an account_id or a token_id (if composed).
    pub owner_id: AccountId,
    pub metadata: TokenMetadataCompliant,
    pub approved_account_ids: HashMap<AccountId, u64>,
    pub royalty: HashMap<AccountId, u32>,
}
pub type TokenId = String;
pub type HumanSBTs = Vec<(AccountId, Vec<TokenId>)>;
