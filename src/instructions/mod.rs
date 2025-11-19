mod append_data;
mod cancel_bundle;
mod close_bid;
mod close_request;
mod end_auction;
mod init_bundle;
mod init_config;
mod place_bid;
mod request_job;
mod reveal_bid;
mod submit_job;
mod submit_validation;

pub use append_data::*;
pub use cancel_bundle::*;
pub use close_bid::*;
pub use close_request::*;
pub use end_auction::*;
pub use init_bundle::*;
pub use init_config::*;
pub use place_bid::*;
pub use request_job::*;
pub use reveal_bid::*;
pub use submit_job::*;
pub use submit_validation::*;

use ambient_auction_api::{InstructionAccounts, InstructionData};
use pinocchio::ProgramResult;
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;

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
