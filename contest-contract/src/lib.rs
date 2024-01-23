use std::collections::{HashMap, HashSet};
use std::ops::Div;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, LookupMap};
use near_sdk::{env, near_bindgen, require, AccountId, PanicOnDefault, Promise};
use events::{emit_contest_created, emit_voter, emit_artist};


mod constants;
pub mod storage;

pub use crate::constants::*;
use crate::storage::*;

pub mod ext;
pub use crate::ext::*;

mod events;


mod errors;
pub use crate::errors::*;


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct ContestContract {
    pub paused: bool,
    pub admin: AccountId,
    pub contests: UnorderedMap<i16, ContestSession>, // maps a session id to a session Contest o ject
    pub user_entries: UnorderedMap<AccountId, Vec<i16>>, // User to contest IDs mapping
    pub sbt_registry: AccountId,
    pub session_counter: i16,
}
// pub struct Contract {
//     pub paused: bool,
//     /// map account to art
//     pub arts: UnorderedMap<AccountId, Art>,
//     pub sbt_registry: AccountId,
//     /// OG token (issuer, class_id)
//     /// map (art, upvoter) -> timestamp_ms
//     pub upvotes: LookupMap<(Art, AccountId), u64>,
//     /// list of admins
//     pub admins: LazyOption<Vec<AccountId>>,
    
    
//     /// next comment id
//     pub next_comment_id: u64,
//     pub voting_date_set: bool,
// }



#[near_bindgen]
impl ContestContract {
    #[init]
    pub fn new(
        sbt_registry: AccountId,
        admin: AccountId,
    ) -> Self {
        Self {
            admin,
            sbt_registry,
            paused: false,
            contests: UnorderedMap::new(StorageKey::Contest),
            user_entries: UnorderedMap::new(StorageKey::Entry),
            session_counter: 0,
        }
    }

    #[payable]
    pub fn create_contest(
        &mut self,
        title: String,
        description: String,
        dao_id: AccountId,
        logo_url: String,
        submission_start_time: u64,
        submission_end_time: u64,
        voting_start_time: u64,
        voting_end_time: u64,
        prize: f64,
        places: i16,
        quorum: i16,
        min_art_vote: i16
    ) {
        // Ensure the admin is calling this function
        // assert_eq!(env::predecessor_account_id(), self.admin, "admin action");

        self.session_counter += 1;
        let session_id = self.session_counter;

        // Check if contest ID already exists
        assert!(!self.contests.get(&session_id).is_some(), "contest exists");

        // Create a new contest session
        let contest = ContestSession {
            title: title.clone(),
            description,
            dao_id,
            logo_url,
            submission_start_time,
            submission_end_time,
            voting_start_time,
            voting_end_time,
            submissions: UnorderedMap::new(StorageKey::Submission(submission_start_time.to_string() + &session_id.to_string())),
            art_voters_mapping: LookupMap::new(StorageKey::VoteMap(session_id.to_string())),
            vote_record: vec![],
            winners: UnorderedMap::new(StorageKey::Winners(voting_end_time.to_string() + &session_id.to_string())),
            blacklist: vec![],
            prize,
            places,
            quorum,
            min_art_vote,
            creator: env::signer_account_id()
        };

        self.contests.insert(&session_id, &contest);
        emit_contest_created(title, submission_start_time, submission_end_time, voting_start_time, voting_end_time, prize, places, quorum, min_art_vote);
    }

    pub fn get_contests(&self) -> Vec<(i16, SerializeableContestSession)> {
        let mut result: Vec<(i16, SerializeableContestSession)> = vec![];
        for (co_id, co_ses) in self.contests.iter() {
            result.push((co_id, SerializeableContestSession{
                submission_start_time: co_ses.submission_start_time,
                submission_end_time: co_ses.submission_end_time,
                voting_start_time: co_ses.voting_start_time,
                voting_end_time: co_ses.voting_end_time,
                title: co_ses.title,
                description: co_ses.description,
                dao_id: co_ses.dao_id,
                prize: co_ses.prize,
                places: co_ses.places,
                logo_url: co_ses.logo_url,
                submissions: co_ses.submissions.len(),
                winners: co_ses.winners.iter().map(|(key, _)| key.clone()).collect(),
                creator: co_ses.creator
            }));
        }
        result
    }

    pub fn get_contests_by_creator(&self, creator: AccountId) -> Vec<(i16, SerializeableContestSession)> {
        let mut result: Vec<(i16, SerializeableContestSession)> = vec![];
        for (co_id, co_ses) in self.contests.iter().filter(|x| x.1.creator.eq(&creator)) {

            result.push((co_id, SerializeableContestSession{
                submission_start_time: co_ses.submission_start_time,
                submission_end_time: co_ses.submission_end_time,
                voting_start_time: co_ses.voting_start_time,
                voting_end_time: co_ses.voting_end_time,
                title: co_ses.title,
                description: co_ses.description,
                dao_id: co_ses.dao_id,
                prize: co_ses.prize,
                places: co_ses.places,
                logo_url: co_ses.logo_url,
                submissions: co_ses.submissions.len(),
                winners: co_ses.winners.iter().map(|(key, _)| key.clone()).collect(),
                creator: co_ses.creator
            }));
        }
        result
    }

    pub fn get_contest_detail(&self, contest_id: i16) -> SerializeableContestSession {
        let contest = self.contests.get(&contest_id).expect("COntest not found");
        SerializeableContestSession{
            submission_start_time: contest.submission_start_time,
            submission_end_time: contest.submission_end_time,
            voting_start_time: contest.voting_start_time,
            voting_end_time: contest.voting_end_time,
            title: contest.title,
            description: contest.description,
            dao_id: contest.dao_id,
            prize: contest.prize,
            places: contest.places,
            logo_url: contest.logo_url,
            submissions: contest.submissions.len(),
            winners: contest.winners.iter().map(|(key, _)| key.clone()).collect(),
            creator: contest.creator
        }
    }

    pub fn get_contest_arts(&self, contest_id: i16) -> Vec<(AccountId, Art)> {
        let mut response = vec![];
        let contest = self.contests.get(&contest_id).expect("COntest not found");
        for (owner, art) in contest.submissions.iter() {
            response.push((owner, art));
        }
        response
    }

    pub fn get_artist_art_vote(&self, contest_id: i16, artist: AccountId) -> Art {
        let contest = self.contests.get(&contest_id).expect("COntest not found");
        contest.submissions.get(&artist).expect("No submission found for artist")
    }

    pub fn get_user_voted(&self, contest_id: i16, user: AccountId) -> bool {
        let contest = self.contests.get(&contest_id).expect("COntest not found");
        contest.vote_record.contains(&user)
    }
    pub fn get_specific_art_voters(&self, contest_id: i16, artist: AccountId) -> Vec<AccountId> {
        let contest = self.contests.get(&contest_id).expect("COntest not found");
        contest.art_voters_mapping.get(&artist).unwrap_or(vec![])
    }

    pub fn get_all_user_voted(&self, contest_id: i16) -> Vec<AccountId> {
        let contest = self.contests.get(&contest_id).expect("COntest not found");
        contest.vote_record
    }

    pub fn get_winner_payout_info(&self, contest_id: i16, winner: AccountId) -> PayOutInfo {
        let contest = self.contests.get(&contest_id).expect("Contest not found");
        require!(
            contest.winners.get(&winner).is_some(),
            "winner not found"
        );
        contest.winners.get(&winner).unwrap()

    }

    #[payable]
    pub fn admin_disqualify_artist(&mut self, contest_id: i16, artist: AccountId) {
        require!(env::predecessor_account_id() == self.admin, "admin action");
        let mut contest = self.contests.get(&contest_id).expect("COntest not found");
        contest.submissions.remove(&artist);
        contest.blacklist.push(artist);
        self.contests.insert(&contest_id, &contest);

    }


    // #[payable]
    // pub fn set_voting_time(&mut self, new_time: u64) {
    //     require!(self.admins.get().unwrap().contains(&env::signer_account_id()), "not an admin");
    //     require!(new_time > self.submission_end_time, "vote must start after submission ends");
    //     require!(!self.voting_date_set, "vote date already set");
    //     self.voting_start_time = new_time;
    //     self.voting_date_set = true;
    // }

    // #[payable]
    // pub fn set_voting_end_time(&mut self, new_time: u64) {
    //     require!(self.admins.get().unwrap().contains(&env::signer_account_id()), "not an admin");
    //     require!(new_time > self.voting_start_time && new_time > env::block_timestamp_ms(), "vote must start after submission ends");
    //     self.voting_start_time = new_time;
    // }

    #[payable]
    pub fn pause_contract(&mut self, pause: bool) {
        assert_eq!(env::predecessor_account_id(), self.admin, "admin action");
        self.paused = pause;
    }


    pub fn is_submission_active(&self, contest_id: i16) -> bool {
        let current_timestamp = env::block_timestamp().div(1000000000);
        let contest = self.contests.get(&contest_id).expect("Contest not found");
        return contest.submission_start_time < current_timestamp && current_timestamp < contest.submission_end_time;
    }

    pub fn is_voting_active(&self, contest_id: i16) -> bool {
        let current_timestamp = env::block_timestamp().div(1000000000);
        let contest = self.contests.get(&contest_id).expect("Contest not found");
        return contest.voting_start_time < current_timestamp && current_timestamp < contest.voting_end_time;
    }

    // #[payable]
    // pub fn change_submission_end(&mut self, new_time: u64, contest_id: i16) {
    //     require!(self.admins.get().unwrap().contains(&env::signer_account_id()), "not an admin");
    //     require!(new_time > env::block_timestamp_ms() && new_time > self.submission_start_time , "new time must surpass present");
    //     self.submission_end_time = new_time;
    // }


    /// Returns list of pairs:
    /// (artist, art).
    // pub fn arts(&self) -> Vec<(AccountId, Art)> {
    //     let mut results: Vec<(AccountId, Art)> = Vec::new();
    //     for n in self.arts.iter() {
    //         results.push((n.0, n.1));
    //     }
    //     results
    // }


    #[payable]
    pub fn submit_art(
        &mut self,
        nft_contract_id: AccountId,
        token_id: TokenId,
        contest_id: i16
    ) -> Promise {
        require!(!self.paused, "contract paused");
        let current_time = env::block_timestamp().div(1000000000);
        let contest = self.contests.get(&contest_id).expect("COntest not found");
        let artist = env::predecessor_account_id();

        require!(
            !contest.blacklist.contains(&artist),
            "you're blacklisted"
        );
        assert!(
            current_time >= contest.submission_start_time && current_time <= contest.submission_end_time,
            "Contest not open for submissions"
        );

        assert!(
            !contest.submissions.get(&artist).is_some(),
            "You've already submitted for this contest"
        );

        require!(
            env::prepaid_gas() >= GAS_SUBMISSION,
            format!("not enough gas, min: {:?}", GAS_SUBMISSION)
        );

        // self.on_submission_verified(token_id, nft_contract_id, contest_id);

        let promise = ext_contract::ext(nft_contract_id.clone())
            .nft_token(token_id.to_string());
        // let sbt_promise = ext_sbtreg::ext(self.sbt_registry.clone()).is_human(artist);
        promise.then(
            Self::ext(env::current_account_id())
                .with_static_gas(GAS_SUBMISSION)
                .on_submission_verified(token_id, nft_contract_id, contest_id)
        )
    }
    
    #[private]
    #[handle_result]
    pub fn on_submission_verified(
        &mut self,
        #[callback_unwrap] token: Token,
        token_id: TokenId,
        nft_contract_id: AccountId,
        contest_id: i16,
    ) -> Result<(), SubmissionError> {
        // require!(
        //     !tokens.is_empty(),
        //     "not a verified human member, or the tokens are expired"
        // );
        assert_eq!(env::signer_account_id(), token.owner_id, "not owner");

        let mut contest = self.contests.get(&contest_id).expect("Contest not found");

        // let mut submissions = contest.submissions;



        contest.submissions.insert(&token.owner_id,  &Art{ title: token.metadata.title.unwrap_or("UnTitled".to_string()), contract_id: nft_contract_id.clone(), timestamp: env::block_timestamp().div(1000000000), votes: 0, token_id: token_id.clone(), image_url: token.metadata.media.unwrap_or("".to_string()) });
        // self.arts.insert(&env::signer_account_id(),);

        self.contests.insert(&contest_id, &contest);

        emit_artist(env::signer_account_id(), contest_id, token_id, nft_contract_id);
        Ok(())
    }

    #[payable]
    pub fn vote(&mut self, contest_id: i16, submission_owner: AccountId) -> Promise {
        let current_time = env::block_timestamp().div(1000000000);
        let contest = self.contests.get(&contest_id).expect("Contest not found");

        // Check if voting has started
        assert!(
            contest.voting_start_time <= current_time && contest.voting_end_time >current_time ,
            "Voting for this contest has not started or has ended"
        );

        let user_account_id = env::predecessor_account_id();

        // Ensure the user hasn't already voted in this contest
        assert!(
            !contest.vote_record.contains(&user_account_id),
            "You've already voted in this contest"
        );
        let art = contest.submissions.get(&submission_owner).unwrap();
        let promise = ext_contract::ext(art.contract_id)
            .nft_token(art.token_id);
        // let sbt_promise = ext_sbtreg::ext(self.sbt_registry.clone()).is_human(user_account_id);
        promise.then(
            Self::ext(env::current_account_id())
                .with_static_gas(GAS_SUBMISSION)
                .on_vote_verified(submission_owner, contest_id)
        )
    }

    #[private]
    #[handle_result]
    pub fn on_vote_verified(
        &mut self,
        #[callback_unwrap] token: Token,
        // #[callback_unwrap] tokens: Vec<(AccountId, Vec<TokenId>)>,
        submission_owner: AccountId,
        contest_id: i16,
    ) -> Result<(), SubmissionError> {
        // require!(
        //     !tokens.is_empty(),
        //     "not a verified human member, or the tokens are expired"
        // );

        let mut contest = self.contests.get(&contest_id).expect("Contest not found");

        if submission_owner != token.owner_id {
            contest.submissions.remove(&submission_owner);
            self.contests.insert(&contest_id, &contest);
            env::panic_str("user disqualified, no longer owns nft");
        }
        
        contest.vote_record.push(env::signer_account_id());
        let mut art = contest.submissions.get(&submission_owner).unwrap();
        art.votes += 1;
        contest.submissions.insert(&submission_owner, &art);
        let mut voters = contest.art_voters_mapping.get(&submission_owner).unwrap_or_else(
            || vec![]
        );
        voters.push(env::signer_account_id());
        contest.art_voters_mapping.insert(&submission_owner, &voters);
        self.contests.insert(&contest_id, &contest);

        emit_voter(submission_owner, contest_id, env::signer_account_id());
        Ok(())
    }

    #[payable]
    pub fn finalise_contest(&mut self, contest_id: i16) {
        let current_time = env::block_timestamp().div(1000000000);
        let mut contest = self.contests.get(&contest_id).expect("Contest not found");
        assert!(
            contest.voting_end_time <= current_time,
            "voting ongoing"
        );
        assert!(
            contest.winners.is_empty(),
            "Contest already finalised"
        );
        let mut arts_and_votes: Vec<(AccountId, u32)> = contest.submissions.iter().filter(|x| x.1.votes >= contest.min_art_vote as u32).map(|(k,v)| (k, v.votes)).collect();
        arts_and_votes.sort_by(|a, b| b.1.cmp(&a.1));
        let mut highest_vote = arts_and_votes[0].1;

        let mut top_winners: Vec<&(AccountId, u32)> = arts_and_votes
            .iter()
            .enumerate()
            .filter(|(_, (_, votes))| *votes == highest_vote)
            .map(|(_, z)| z)
            .collect();

            let mut places = if arts_and_votes.len() < contest.places as usize {arts_and_votes.len() as i16} else {contest.places};
            let mut prize = contest.prize;
    
            while places > 0 {
                if top_winners.len() >= places as usize {
                println!("will it ever reach here....");
                    for x in top_winners.iter() {
                        let winner_prize = prize / top_winners.len() as f64;
                        let pay_info = PayOutInfo {
                            amount: winner_prize,
                            proposal_id: None,
                        };
                        contest.winners.insert(&x.0, &pay_info);
                        // Assuming `self` is an instance of a struct containing contests
                        // and `contest_id` is the identifier for the current contest
                    }
                    self.contests.insert(&contest_id, &contest);
                    break;
                } else {
                    for x in top_winners.iter() {
                        let winner_prize = prize / places as f64;
                        let pay_info = PayOutInfo {
                            amount: winner_prize,
                            proposal_id: None,
                        };
                        contest.winners.insert(&x.0, &pay_info);
                        places -= 1;
                        prize -= winner_prize;
                        // Assuming `self` is an instance of a struct containing contests
                        // and `contest_id` is the identifier for the current contest
                        // self.contests.insert(contest_id, self.clone());
                    }
    
                    
                    
                    
    
                    let mut vote_iterator = arts_and_votes
                        .iter()
                        .filter(|(_, votes)| *votes < highest_vote);    
                    highest_vote = vote_iterator.next().map(|&(_, votes)| votes).unwrap_or(0);
    
                    top_winners = arts_and_votes
                        .iter()
                        .enumerate()
                        .filter(|(_, (_, votes))| *votes == highest_vote)
                        .map(|(_, z)| z)
                        .collect();
                    
                    println!("places mutated....{}, {}, {:?}", places, highest_vote, top_winners)
                }
            }

        // let prize_per_winner = contest.prize/winners.len() as i64;
        // for winner_acct in winners.iter() {
        //     let pay_info = PayOutInfo{amount: prize_per_winner, proposal_id: 0, fulfilled: false};
        //     contest.winners.insert(*winner_acct, &pay_info);
        // }
        
    }

    pub fn set_payout_proposal_id(&mut self, contest_id: i16, winner: AccountId, proposal_id: u64) {
        let contest = self.contests.get(&contest_id).expect("Contest not found");
        require!(
            contest.winners.get(&winner).is_some(),
            "winner not found"
        );
        let promise = ext_proposal_contract::ext(contest.dao_id)
            .get_policy();
        // let sbt_promise = ext_sbtreg::ext(self.sbt_registry.clone()).is_human(user_account_id);
        promise.then(
            Self::ext(env::current_account_id())
                .with_static_gas(GAS_SUBMISSION)
                .on_dao_policy_verified(winner, contest_id, proposal_id)
        );
        
    }

    fn match_user(&self, role_kind: RoleKind, user: &AccountId) -> bool {
        match role_kind {
            RoleKind::Everyone => true,
            RoleKind::Member(amount) => amount.0 > 1,
            RoleKind::Group(accounts) => accounts.contains(user),
        }
    }

    fn get_user_roles(&self, user: &AccountId, policy: Policy) -> HashMap<String, HashSet<String>> {
        let mut roles = HashMap::default();
        for role in policy.roles.iter() {
            if self.match_user(role.kind.clone(), user) {
                roles.insert(role.name.clone(), role.permissions.clone());
            }
        }
        roles
    }

    

    #[private]
    #[handle_result]
    pub fn on_dao_policy_verified(
        &mut self,
        #[callback_unwrap] policy: Policy,
        // #[callback_unwrap] tokens: Vec<(AccountId, Vec<TokenId>)>,
        winner: AccountId,
        contest_id: i16,
        proposal_id: u64
    ) -> Result<(), SubmissionError> {
        // require!(
        //     !tokens.is_empty(),
        //     "not a verified human member, or the tokens are expired"
        // );

        let roles = self.get_user_roles(&env::signer_account_id(), policy);
        let mut allowed = false;
        let _allowed_roles: Vec<String>= roles
            .into_iter()
            .filter_map(|(role, permissions)| {
                let allowed_role = permissions.contains(&format!(
                    "{}:{}",
                    "transfer",
                    "AddProposal".to_string()
                )) || permissions
                    .contains(&format!("{}:*", "transfer"))
                    || permissions.contains(&format!("*:{}", "AddProposal".to_string()))
                    || permissions.contains("*:*");
                allowed = allowed || allowed_role;
                if allowed_role {
                    Some(role)
                } else {
                    None
                }
            }).collect();
        
        require!(
            allowed,
            "Unauthorized to set id"
        );
        let mut contest = self.contests.get(&contest_id).expect("Contest not found");
        let mut winner_pay_info = contest.winners.get(&winner).unwrap();
        winner_pay_info.proposal_id = proposal_id.into();
        contest.winners.insert(&winner , &winner_pay_info);
        self.contests.insert(&contest_id, &contest);
        Ok(())
    }



    // fn assert_submission_active(&self, contest_id: i16) {
    //     let current_timestamp = env::block_timestamp_ms();
    //     require!(
    //         self.submission_start_time < current_timestamp && current_timestamp < self.submission_end_time,
    //         "submission period over"
    //     );
    // }
}


// #[test]
// fn create_contest() {
//     let alice: AccountId = "alice.near".parse().unwrap();
//     let registry: AccountId = "genadrop.near".parse().unwrap();
//     let mut contract = ContestContract {
//         admin: alice,
//         sbt_registry: registry,
//         paused: false,
//         contests: UnorderedMap::new(StorageKey::Contest),
//         user_entries: UnorderedMap::new(StorageKey::Entry),
//         session_counter: 0,
//     };
//     let cf = contract.create_contest("EMEA Art contest".to_string(), 1696167848, 1696596739, 1696683139,  1697374339);
//     println!("go on soun");
// }

