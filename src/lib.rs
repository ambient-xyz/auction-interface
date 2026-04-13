use pinocchio::pubkey::Pubkey;

pub mod instructions;

/// Vote Program Address to avoid `solana-vote-interface` crate dependency
const VOTE_ID: [u8; 32] =
    five8_const::decode_32_const("Vote111111111111111111111111111111111111111");

/// Instructions Sysvar Address to avoid an additional Solana SDK dependency
const INSTRUCTIONS_SYSVAR_ID: [u8; 32] =
    five8_const::decode_32_const("Sysvar1nstructions1111111111111111111111111");

pub const ID: Pubkey = pinocchio_pubkey::from_str("Auction111111111111111111111111111111111111");
