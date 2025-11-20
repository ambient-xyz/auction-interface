pub mod instructions;

/// Vote Program Address to avoid `solana-vote-interface` crate dependency
const VOTE_ID: [u8; 32] =
    five8_const::decode_32_const("Vote111111111111111111111111111111111111111");
