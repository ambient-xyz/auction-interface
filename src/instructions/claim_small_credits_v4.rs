use crate::instructions::{AuctionInstructionAccounts, to_program_error};
use ambient_auction_api::{
    ClaimSmallCreditsV4Accounts, ClaimSmallCreditsV4Args, InstructionAccounts,
};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

#[repr(transparent)]
pub struct ClaimSmallCreditsV4InstructionAccounts<'a>(ClaimSmallCreditsV4Accounts<'a, AccountInfo>);

impl<'a> TryFrom<&'a [AccountInfo]> for ClaimSmallCreditsV4InstructionAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let account_infos =
            ClaimSmallCreditsV4Accounts::try_from(accounts).map_err(to_program_error)?;

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

impl<'a> AuctionInstructionAccounts<'a> for ClaimSmallCreditsV4InstructionAccounts<'a> {
    type Inner = ClaimSmallCreditsV4Accounts<'a, AccountInfo>;

    fn inner(&self) -> &Self::Inner {
        &self.0
    }

    fn to_account_metas(&'a self) -> impl Iterator<Item = AccountMeta<'a>> {
        self.inner().iter().map(AccountMeta::from)
    }
}

pub struct ClaimSmallCreditsV4Instruction<'a> {
    pub accounts: ClaimSmallCreditsV4InstructionAccounts<'a>,
    pub data: ClaimSmallCreditsV4Args,
}

impl<'a> TryFrom<(&'a [AccountInfo], &'a [u8])> for ClaimSmallCreditsV4Instruction<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [AccountInfo], &'a [u8])) -> Result<Self, Self::Error> {
        let (accounts, data) = value;

        Ok(Self {
            accounts: ClaimSmallCreditsV4InstructionAccounts::try_from(accounts)?,
            data: ClaimSmallCreditsV4Args::try_from(data)
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        })
    }
}
