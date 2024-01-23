use near_sdk::env::panic_str;
use near_sdk::FunctionError;

use crate::TokenId;

/// Contract errors
#[cfg_attr(not(target_arch = "wasm32"), derive(PartialEq, Debug))]
pub enum SubmissionError {
    WrongIssuer,
    NoSBTs,
    DuplicateCandidate,
    DoubleVote(TokenId),
    MinBond(u128, u128),
    Blacklisted,
    NoBond
}

impl FunctionError for SubmissionError {
    fn panic(&self) -> ! {
        match self {
            SubmissionError::WrongIssuer => {
                panic_str("expected human SBTs proof from the human issuer only")
            }
            SubmissionError::NoSBTs => panic_str("voter is not a verified human, expected IAH SBTs proof from the IAH issuer only"),
            SubmissionError::DuplicateCandidate => panic_str("double vote for the same candidate"),
            SubmissionError::DoubleVote(sbt) => {
                panic_str(&format!("user already voted with sbt={}", sbt))
            },
            SubmissionError::MinBond(req, amt) => panic_str(&format!("required bond amount={}, deposited={}", req, amt)),
            SubmissionError::Blacklisted => panic_str("user is blacklisted/no longer owns nft"),
            SubmissionError::NoBond => panic_str("Voter didn't bond")
        }
    }
}

/// Contract errors
#[cfg_attr(not(target_arch = "wasm32"), derive(PartialEq, Debug))]
pub enum RevokeSubmissionError {
    NotActive,
    NotVoted,
    NotBlacklisted,
}

impl FunctionError for RevokeSubmissionError {
    fn panic(&self) -> ! {
        match self {
            RevokeSubmissionError::NotActive => {
                panic_str("can only revoke votes between proposal start and (end time + cooldown)")
            }
            RevokeSubmissionError::NotVoted => panic_str(
                "voter did not vote on this proposal or the vote has been already revoked",
            ),
            RevokeSubmissionError::NotBlacklisted => {
                panic_str("can not revoke a not blacklisted voter")
            }
        }
    }
}
