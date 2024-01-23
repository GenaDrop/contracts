
# Creative DAO Art Contest Smart Contract

## Deployments

|   environment   |                           address |
| :-------------: | --------------------------------: |
|  mainnet prod   |          `contest.genadrop.near` |
| mainnet testing | `fund-beta.genadrop.near` |

## Requirements

- Only I Am Human verified accounts can submit art and vote.(currently disabled)
- Each account can submit art/Vote at most one time. Votes are not revocable, and can't be changed.
- Only an admin set while deploying the contract can create Contest sessions. Each Contest specifies:
- Artist must own the nft they're submitting for contest, if at anytime the nft leaves their wallet, they'll be disqualified.

  - `title`: A String serving as the name of the contest i.e 
  - `dao_id`: the dao responsible for payout proposals for the prospective contest winners
  - `submission_start_time`: submission start time as UNIX time (in epoch timestamp seconds)
  - `submission_end_time`: submission end time as UNIX time (in epoch timestamp seconds)
  - `voting_start_time`: voting start time as UNIX time (in epoch timestamp seconds)(will mostly be same value as submission_end_time, as voting is expected to start immediately submission ends.)
  - `voting_end_time`: voting end time as UNIX time (in epoch timestamp seconds)
  - `description`: A String parameter serving as the description of the contest i.e 
  - `quorum`: minimum amount of legit accounts to vote to legitimize the contest.(currently ignored).
  - `prize`: f64 floating integer parameter which serves as the pool prize to be shared amongst winners per places.
  - `places`: i64 integer parameter which serves as the number of places(not winners) possible in a contest, for example if a contest has a value of 3 for its places it means that, the pool prize will be shared into 3 equal places and distributed to users who fall in a particular place based on their number of votes, if a contest has 6 contestants and 3 places, if there is not ie in vote among the top 3, then the prize pool is shared equally amongst the top 3, however if there is a tie between the 3rd and the 4th contestant, it means that they both would hsare the prize lot for the third place.
  - `min_art_vote`: minimum amount of votes a candidate needs to be considered a winner/amongst the winner.

## Flow

- genadrop deploys the Contest smart contract and sets admin for creating new contest sessions.
- contract admin creates new contest, with a submission start and end date, users then have to submit their art entry within this submission period.
- we can query submitted arts directly from the contest contract.
- Once the submission period ends voting starts (`now >= contest.submission_end_time`).
- Anyone can query the proposal and the ongoing result at any time.
- Voting is active until the `contest.voting_end_time` time.
- After voting ends, the contest is finalized by calling the (`finalize_contest`) function, this function picks the winners and computes the prize per winner based on what was explained in `places`.
- A Transfer proposal is then created on the dao that was chosen when creating the contest, 



# create proposal
# note: all time values must be in epoch timestamp seconds
`near call fund-beta.genadrop.near create_contest '{"title": "New Year Art contest", "description": "New Year vibes, New Year inspiration, New Year Arts!", "dao_id": "wazes-dao.sputnik-dao.near", "logo_url": "https://picsum.photos/200/300.jpg", "submission_start_time": 1704794499,"submission_end_time": 1704812269,"voting_start_time": 1704812269, "voting_end_time": 1704826669,"prize": 1000, "places": 4, "quorum": 2, "min_art_vote": 2}' --accountId acct.near --gas 75000000000000`

# submit Art 
`near call fund-beta.genadrop.near submit_art '{"nft_contract_id": "acct.near","token_id": 1687132812125,"contest_id": 1}' --accountId acct.near --gas 75000000000000`

# vote 
`near call fund-beta.genadrop.near vote '{"submission_owner": "acct.near","contest_id": 2}' --accountId acct.near --gas 75000000000000`

# Get contest detail
`near view fund-beta.genadrop.near get_contest_detail '{"contest_id": 2}'`