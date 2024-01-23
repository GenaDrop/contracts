use near_sdk::{serde::Serialize, AccountId};
use serde_json::json;

use near_sdk::env;

use crate::TokenId;

/// Helper struct to create Standard NEAR Event JSON.
/// Arguments:
/// * `standard`: name of standard e.g. nep171
/// * `version`: e.g. 1.0.0
/// * `event`: associate event data
#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct NearEvent<T: Serialize> {
    pub standard: &'static str,
    pub version: &'static str,

    // `flatten` to not have "event": {<EventVariant>} in the JSON, just have the contents of {<EventVariant>}.
    #[serde(flatten)]
    pub event: T,
}

impl<T: Serialize> NearEvent<T> {
    pub fn to_json_event_string(&self) -> String {
        let s = serde_json::to_string(&self)
            .ok()
            .unwrap_or_else(|| env::abort());
        format!("EVENT_JSON:{}", s)
    }

    pub fn emit(self) {
        env::log_str(&self.to_json_event_string());
    }
}

/// Helper struct to be used in `NearEvent.event` to construct NEAR Event compatible payload
#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct EventPayload<T: Serialize> {
    /// event name
    pub event: &'static str,
    /// event payload
    pub data: T,
}

impl<T: Serialize> EventPayload<T> {
    pub fn emit(self, standard: &'static str, version: &'static str) {
        NearEvent {
            standard,
            version,
            event: self,
        }
        .emit()
    }
}

fn emit_event<T: Serialize>(event: EventPayload<T>) {
    NearEvent {
        standard: "cdao",
        version: "1.0.0",
        event,
    }
    .emit();
}

// pub(crate) fn emit_bond(amount: Balance) {
//     emit_event(EventPayload {
//         event: "bond",
//         data: json!({ "amount": amount.to_string() }),
//     });
// }

pub(crate) fn emit_artist(artist: AccountId, contest_id: i16, nft_token_id: TokenId, contract_id: AccountId ) {
    emit_event(EventPayload {
        event: "vote",
        data: json!({ "artist": artist, "contest_id": contest_id, "nft_token_id": nft_token_id, "contract_id": contract_id}),
    });
}

pub(crate) fn emit_voter(artist: AccountId, contest_id: i16, voter: AccountId) {
    emit_event(EventPayload {
        event: "vote",
        data: json!({ "artist": artist, "contest_id": contest_id, "voter": voter }),
    });
}

pub(crate) fn emit_contest_created(title: String, submission_start_time: u64, submission_end_time: u64, voting_start_time: u64, voting_end_time: u64, prize: f64, places: i16, quorum: i16, min_art_vote: i16) {
    emit_event(EventPayload {
        event: "vote",
        data: json!({ "title": title, "submission_start_time": submission_start_time, "submission_end_time": submission_end_time, "voting_start_time": voting_start_time, "voting_end_time": voting_end_time, "prize": prize,  "places": places,  "quorum": quorum ,  "min_art_vote": min_art_vote }),
    });
}
