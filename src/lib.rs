pub mod instructions;

/// Vote Program Address to avoid `solana-vote-interface` crate dependency
const VOTE_ID: [u8; 32] =
    five8_const::decode_32_const("Vote111111111111111111111111111111111111111");

/// System Program Address to avoid an additional Solana SDK dependency
const SYSTEM_PROGRAM_ID: [u8; 32] =
    five8_const::decode_32_const("11111111111111111111111111111111");

/// Instructions Sysvar Address to avoid an additional Solana SDK dependency
const INSTRUCTIONS_SYSVAR_ID: [u8; 32] =
    five8_const::decode_32_const("Sysvar1nstructions1111111111111111111111111");
