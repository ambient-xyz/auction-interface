use crate::instructions::{AuctionInstructionAccounts, to_program_error};
use ambient_auction_api::{
    ClaimSmallCreditsV5Accounts, ClaimSmallCreditsV5Args, InstructionAccounts,
};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

#[repr(transparent)]
pub struct ClaimSmallCreditsV5InstructionAccounts<'a>(ClaimSmallCreditsV5Accounts<'a, AccountInfo>);

impl<'a> TryFrom<&'a [AccountInfo]> for ClaimSmallCreditsV5InstructionAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let account_infos =
            ClaimSmallCreditsV5Accounts::try_from(accounts).map_err(to_program_error)?;

        super::validate_config_policy_owner(account_infos.config_policy)?;
        if !account_infos.mint.is_writable() || !account_infos.token_account.is_writable() {
            return Err(ProgramError::InvalidArgument);
        }
        if !account_infos.token_program.executable() {
            return Err(ProgramError::IncorrectProgramId);
        }

        super::validate_current_bundle_escrow(account_infos.bundle_escrow)?;

        Ok(Self(account_infos))
    }
}

impl<'a> AuctionInstructionAccounts<'a> for ClaimSmallCreditsV5InstructionAccounts<'a> {
    type Inner = ClaimSmallCreditsV5Accounts<'a, AccountInfo>;

    fn inner(&self) -> &Self::Inner {
        &self.0
    }

    fn to_account_metas(&'a self) -> impl Iterator<Item = AccountMeta<'a>> {
        self.inner().iter().map(AccountMeta::from)
    }
}

pub struct ClaimSmallCreditsV5Instruction<'a> {
    pub accounts: ClaimSmallCreditsV5InstructionAccounts<'a>,
    pub data: ClaimSmallCreditsV5Args,
}

impl<'a> TryFrom<(&'a [AccountInfo], &'a [u8])> for ClaimSmallCreditsV5Instruction<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [AccountInfo], &'a [u8])) -> Result<Self, Self::Error> {
        let (accounts, data) = value;

        Ok(Self {
            accounts: ClaimSmallCreditsV5InstructionAccounts::try_from(accounts)?,
            data: ClaimSmallCreditsV5Args::try_from(data)
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        })
    }
}
