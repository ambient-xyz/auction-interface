use pinocchio::{
    account_info::{AccountInfo, MAX_PERMITTED_DATA_INCREASE},
    entrypoint::{NON_DUP_MARKER, deserialize},
    program_error::ProgramError,
};
use std::mem::MaybeUninit;

#[derive(Clone, Default)]
pub struct Account {
    pub key: [u8; 32],
    pub owner: [u8; 32],
    pub signer: bool,
    pub writable: bool,
    pub executable: bool,
    pub data: Vec<u8>,
    pub owner_error: Option<ProgramError>,
    pub key_error: Option<ProgramError>,
}

impl Account {
    pub fn writable(mut self) -> Self {
        self.writable = true;
        self
    }

    pub fn signed(mut self) -> Self {
        self.signer = true;
        self
    }

    pub fn owned(owner: [u8; 32], error: ProgramError) -> Self {
        Self {
            owner,
            owner_error: Some(error),
            ..Self::default()
        }
    }

    pub fn address(key: [u8; 32], error: ProgramError) -> Self {
        Self {
            key,
            key_error: Some(error),
            ..Self::default()
        }
    }
}

// Callbacks in these tests return only owned metadata or assertions.
pub fn with_accounts<T>(accounts: &[Account], f: impl FnOnce(&[AccountInfo]) -> T) -> T {
    let mut bytes = (accounts.len() as u64).to_le_bytes().to_vec();
    for account in accounts {
        bytes.extend_from_slice(&[
            NON_DUP_MARKER,
            u8::from(account.signer),
            u8::from(account.writable),
            u8::from(account.executable),
            0,
            0,
            0,
            0,
        ]);
        bytes.extend_from_slice(&account.key);
        bytes.extend_from_slice(&account.owner);
        bytes.extend_from_slice(&1_000_000u64.to_le_bytes());
        bytes.extend_from_slice(&(account.data.len() as u64).to_le_bytes());
        bytes.extend_from_slice(&account.data);
        bytes.resize(bytes.len() + MAX_PERMITTED_DATA_INCREASE, 0);
        bytes.resize(bytes.len().next_multiple_of(8), 0);
        bytes.extend_from_slice(&0u64.to_le_bytes()); // Rent epoch.
    }
    bytes.extend_from_slice(&0u64.to_le_bytes()); // Empty instruction data.
    bytes.extend_from_slice(&ambient_auction_api::ID);
    let mut aligned = vec![0u64; bytes.len().div_ceil(8)];
    bytemuck::cast_slice_mut::<u64, u8>(&mut aligned)[..bytes.len()].copy_from_slice(&bytes);
    let mut infos = [MaybeUninit::uninit(); 32];
    assert!(accounts.len() <= infos.len());
    // SAFETY: The owned, aligned buffer contains every runtime field, resize padding,
    // and trailing program ID. Only the initialized accounts reach the callback,
    // and neither their pointers nor the deserializer's static references escape it.
    unsafe {
        let (_, count, _) = deserialize(aligned.as_mut_ptr().cast(), &mut infos);
        let infos = std::slice::from_raw_parts(infos.as_ptr().cast::<AccountInfo>(), count);
        f(infos)
    }
}
