mod append_data;
mod cancel_bundle;
mod claim_verifier_lstake_v2;
mod claim_winner_lstake_v2;
mod close_bid;
mod close_request;
mod commit_auction_settlement_v2;
mod end_auction;
mod expire_bundle_escrow_v2;
mod finalize_bundle_verification_v2;
mod init_bundle;
mod init_config;
mod init_config_policy_v2;
mod open_bundle_escrow_v2;
mod place_bid;
mod post_bundle_result_v2;
mod request_job;
mod reveal_bid;
mod set_config_policy_v2;
mod submit_job;
mod submit_validation;

pub use append_data::*;
pub use cancel_bundle::*;
pub use claim_verifier_lstake_v2::*;
pub use claim_winner_lstake_v2::*;
pub use close_bid::*;
pub use close_request::*;
pub use commit_auction_settlement_v2::*;
pub use end_auction::*;
pub use expire_bundle_escrow_v2::*;
pub use finalize_bundle_verification_v2::*;
pub use init_bundle::*;
pub use init_config::*;
pub use init_config_policy_v2::*;
pub use open_bundle_escrow_v2::*;
pub use place_bid::*;
pub use post_bundle_result_v2::*;
pub use request_job::*;
pub use reveal_bid::*;
pub use set_config_policy_v2::*;
pub use submit_job::*;
pub use submit_validation::*;

use ambient_auction_api::error::AuctionError;
use ambient_auction_api::{InstructionAccounts, InstructionData};
use pinocchio::ProgramResult;
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

pub trait ProcessInstruction<'a>: TryFrom<(&'a [AccountInfo], &'a [u8])> {
    type Accounts: AuctionInstructionAccounts<'a>;
    type Data: InstructionData<'a>;
    fn accounts(&self) -> &Self::Accounts;
    fn data(&self) -> Self::Data;
    fn process(&self) -> ProgramResult;
    fn validate(&self) -> ProgramResult;
}

pub trait AuctionInstructionAccounts<'a>: TryFrom<&'a [AccountInfo]> {
    type Inner: InstructionAccounts<'a, AccountInfo>;
    fn inner(&self) -> &Self::Inner;
    fn to_account_metas(&'a self) -> impl Iterator<Item = AccountMeta<'a>>;
}

fn to_program_error(e: AuctionError) -> ProgramError {
    ProgramError::Custom(e.code())
}

fn validate_config_policy_owner(config_policy: &AccountInfo) -> Result<(), ProgramError> {
    if !config_policy.is_owned_by(&ambient_auction_api::ID) {
        return Err(to_program_error(AuctionError::IllegalConfigPolicyV2Owner));
    }

    Ok(())
}
