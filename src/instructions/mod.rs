mod append_data;
mod authorize_bundle_dispute_evidence_v4;
mod cancel_bundle;
mod claim_small_credits_v4;
mod claim_verifier_lstake_v2;
mod claim_winner_lstake_v2;
mod close_bid;
mod close_bundle_verifier_page_v4;
mod close_request;
mod commit_auction_settlement_v2;
mod commit_auction_settlement_v3;
mod dispute_bundle_verification_v2;
mod end_auction;
mod expire_bundle_escrow_v2;
mod finalize_bundle_verification_v2;
mod init_bundle;
mod init_bundle_verifier_page_v2;
mod init_config;
mod init_config_policy_v2;
mod open_bundle_escrow_v2;
mod place_bid;
mod post_bundle_pricing;
mod post_bundle_result_v2;
mod request_job;
mod reveal_bid;
mod seal_bundle_pricing;
mod select_bundle_verifiers_v2;
mod set_config_policy_v2;
mod slash_small_credits;
mod submit_job;
mod submit_validation;

pub use append_data::*;
pub use authorize_bundle_dispute_evidence_v4::*;
pub use cancel_bundle::*;
pub use claim_small_credits_v4::*;
pub use claim_verifier_lstake_v2::*;
pub use claim_winner_lstake_v2::*;
pub use close_bid::*;
pub use close_bundle_verifier_page_v4::*;
pub use close_request::*;
pub use commit_auction_settlement_v2::*;
pub use commit_auction_settlement_v3::*;
pub use dispute_bundle_verification_v2::*;
pub use end_auction::*;
pub use expire_bundle_escrow_v2::*;
pub use finalize_bundle_verification_v2::*;
pub use init_bundle::*;
pub use init_bundle_verifier_page_v2::*;
pub use init_config::*;
pub use init_config_policy_v2::*;
pub use open_bundle_escrow_v2::*;
pub use place_bid::*;
pub use post_bundle_pricing::*;
pub use post_bundle_result_v2::*;
pub use request_job::*;
pub use reveal_bid::*;
pub use seal_bundle_pricing::*;
pub use select_bundle_verifiers_v2::*;
pub use set_config_policy_v2::*;
pub use slash_small_credits::*;
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

fn validate_bundle_escrow(
    account: &AccountInfo,
    require_pricing: bool,
) -> Result<(), ProgramError> {
    if !account.is_owned_by(&ambient_auction_api::ID) {
        return Err(ProgramError::InvalidAccountOwner);
    }
    if !account.is_writable() {
        return Err(ProgramError::InvalidArgument);
    }
    let data = account.try_borrow_data()?;
    let state = ambient_auction_api::BundleEscrowV2::from_bytes(&data)
        .ok_or_else(|| to_program_error(AuctionError::InvalidBundleEscrowV2State))?;
    let lifecycle = state
        .lifecycle()
        .ok_or_else(|| to_program_error(AuctionError::InvalidAccountLayoutVersion))?;
    let small = state.reward_tier == ambient_auction_api::RequestTier::Small as u64;
    if (small && lifecycle.small_credit_mint == ambient_auction_api::Pubkey::default())
        || (!small
            && (lifecycle.small_credit_mint != ambient_auction_api::Pubkey::default()
                || lifecycle.small_credit_amount != 0))
    {
        return Err(to_program_error(AuctionError::InvalidBundleEscrowV2State));
    }
    if !(1..=ambient_auction_api::MAX_BUNDLE_VERIFIER_PAGES)
        .contains(&lifecycle.expected_page_count)
    {
        return Err(to_program_error(AuctionError::InvalidVerifierPageV2Input));
    }

    if require_pricing && state.pricing().is_none() {
        return Err(to_program_error(AuctionError::InvalidAccountLayoutVersion));
    }

    Ok(())
}

fn validate_current_bundle_escrow(account: &AccountInfo) -> Result<(), ProgramError> {
    validate_bundle_escrow(account, false)
}

fn validate_settlement_bundle_escrow(
    account: &AccountInfo,
    writable: bool,
) -> Result<(), ProgramError> {
    if !account.is_owned_by(&ambient_auction_api::ID) {
        return Err(ProgramError::InvalidAccountOwner);
    }
    if writable && !account.is_writable() {
        return Err(ProgramError::InvalidArgument);
    }
    let data = account.try_borrow_data()?;
    let state = ambient_auction_api::BundleEscrowV2::from_bytes(&data)
        .ok_or_else(|| to_program_error(AuctionError::InvalidBundleEscrowV2State))?;
    if state.layout().version == ambient_auction_api::AccountLayoutVersion::V3 {
        if state.reward_tier != ambient_auction_api::RequestTier::Small as u64
            || state
                .small_v3()
                .is_none_or(|small| small.mint == ambient_auction_api::Pubkey::default())
            || state.escrow_lamports != 0
            || state.clearing_price_per_output_token != 0
        {
            return Err(to_program_error(AuctionError::InvalidBundleEscrowV2State));
        }
        return Ok(());
    }
    drop(data);
    validate_current_bundle_escrow(account)
}

fn validate_pricing_bundle_escrow(account: &AccountInfo) -> Result<(), ProgramError> {
    validate_bundle_escrow(account, true)
}

fn validate_pricing_bundle_verifier_page(account: &AccountInfo) -> Result<(), ProgramError> {
    if !account.is_owned_by(&ambient_auction_api::ID) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    let data = account.try_borrow_data()?;
    let page = ambient_auction_api::BundleVerifierPageV2::from_bytes(&data)
        .ok_or_else(|| to_program_error(AuctionError::InvalidBundleVerifierPageV2State))?;

    if page.pricing().is_none() {
        return Err(to_program_error(AuctionError::InvalidAccountLayoutVersion));
    }

    Ok(())
}

fn validate_writable_pricing_bundle_verifier_page(
    account: &AccountInfo,
) -> Result<(), ProgramError> {
    if !account.is_writable() {
        return Err(ProgramError::InvalidArgument);
    }

    validate_pricing_bundle_verifier_page(account)
}
