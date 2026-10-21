use crate::instructions::{AuctionInstructionAccounts, to_program_error};
use ambient_auction_api::{
    CloseBundleEscrowV4Accounts, CloseBundleEscrowV4Args, InstructionAccounts,
};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

/// Account checks for `CloseBundleEscrowV4`: the payer signs and is writable (it receives the rent); the escrow is a
/// writable, program-owned V4 `BundleEscrowV2`. The program checks that the payer is the one the escrow was opened by.
#[repr(transparent)]
pub struct CloseBundleEscrowV4InstructionAccounts<'a>(CloseBundleEscrowV4Accounts<'a, AccountInfo>);

impl<'a> TryFrom<&'a [AccountInfo]> for CloseBundleEscrowV4InstructionAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let account_infos =
            CloseBundleEscrowV4Accounts::try_from(accounts).map_err(to_program_error)?;

        if !account_infos.payer.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if !account_infos.payer.is_writable() {
            return Err(ProgramError::InvalidArgument);
        }

        if account_infos.payer.key() == account_infos.bundle_escrow.key() {
            return Err(ProgramError::InvalidArgument);
        }

        super::validate_current_bundle_escrow(account_infos.bundle_escrow)?;

        Ok(Self(account_infos))
    }
}

impl<'a> AuctionInstructionAccounts<'a> for CloseBundleEscrowV4InstructionAccounts<'a> {
    type Inner = CloseBundleEscrowV4Accounts<'a, AccountInfo>;

    fn inner(&self) -> &Self::Inner {
        &self.0
    }

    fn to_account_metas(&'a self) -> impl Iterator<Item = AccountMeta<'a>> {
        self.inner().iter().map(AccountMeta::from)
    }
}

pub struct CloseBundleEscrowV4Instruction<'a> {
    pub accounts: CloseBundleEscrowV4InstructionAccounts<'a>,
    pub data: CloseBundleEscrowV4Args,
}

impl<'a> TryFrom<(&'a [AccountInfo], &'a [u8])> for CloseBundleEscrowV4Instruction<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [AccountInfo], &'a [u8])) -> Result<Self, Self::Error> {
        let (accounts, data) = value;

        Ok(Self {
            accounts: CloseBundleEscrowV4InstructionAccounts::try_from(accounts)?,
            data: CloseBundleEscrowV4Args::try_from(data)
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        })
    }
}
