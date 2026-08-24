use crate::instructions::{
    AuctionInstructionAccounts, to_program_error, validate_config_policy_owner,
};
use ambient_auction_api::{InstructionAccounts, SlashSmallCreditsAccounts, SlashSmallCreditsArgs};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

#[repr(transparent)]
pub struct SlashSmallCreditsInstructionAccounts<'a>(SlashSmallCreditsAccounts<'a, AccountInfo>);

impl<'a> TryFrom<&'a [AccountInfo]> for SlashSmallCreditsInstructionAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let account_infos =
            SlashSmallCreditsAccounts::try_from(accounts).map_err(to_program_error)?;

        if !account_infos.slash_authority.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        validate_config_policy_owner(account_infos.config_policy)?;

        if !account_infos.mint.is_writable() || !account_infos.token_account.is_writable() {
            return Err(ProgramError::InvalidArgument);
        }

        if !account_infos.token_program.executable() {
            return Err(ProgramError::IncorrectProgramId);
        }

        Ok(Self(account_infos))
    }
}

impl<'a> AuctionInstructionAccounts<'a> for SlashSmallCreditsInstructionAccounts<'a> {
    type Inner = SlashSmallCreditsAccounts<'a, AccountInfo>;

    fn inner(&self) -> &Self::Inner {
        &self.0
    }

    fn to_account_metas(&'a self) -> impl Iterator<Item = AccountMeta<'a>> {
        self.inner().iter().map(AccountMeta::from)
    }
}

pub struct SlashSmallCreditsInstruction<'a> {
    pub accounts: SlashSmallCreditsInstructionAccounts<'a>,
    pub data: SlashSmallCreditsArgs,
}

impl<'a> TryFrom<(&'a [AccountInfo], &'a [u8])> for SlashSmallCreditsInstruction<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [AccountInfo], &'a [u8])) -> Result<Self, Self::Error> {
        let (accounts, data) = value;

        Ok(Self {
            accounts: SlashSmallCreditsInstructionAccounts::try_from(accounts)?,
            data: SlashSmallCreditsArgs::try_from(data)
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        })
    }
}
