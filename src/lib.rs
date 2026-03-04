use ambient_auction_api::PROGRAM_ID;
use pinocchio::pubkey::Pubkey;

pub mod instructions;

/// Vote Program Address to avoid `solana-vote-interface` crate dependency
const VOTE_ID: [u8; 32] =
    pinocchio_pubkey::decode_32_const("Vote111111111111111111111111111111111111111");

pub const ID: Pubkey = pinocchio_pubkey::from_str(PROGRAM_ID);
