#![cfg(feature = "global-config")]
use crate::instructions::{AuctionInstructionAccounts, to_program_error};
use ambient_auction_api::{InitConfigAccounts, InitConfigArgs, InstructionAccounts};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

#[repr(transparent)]
pub struct InitConfigInstructionAccounts<'a>(InitConfigAccounts<'a, AccountInfo>);

impl<'a> TryFrom<&'a [AccountInfo]> for InitConfigInstructionAccounts<'a> {
    type Error = ProgramError;
    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let account_infos = InitConfigAccounts::try_from(accounts).map_err(to_program_error)?;

        Ok(InitConfigInstructionAccounts(account_infos))
    }
}

impl<'a> AuctionInstructionAccounts<'a> for InitConfigInstructionAccounts<'a> {
    type Inner = InitConfigAccounts<'a, AccountInfo>;
    fn inner(&self) -> &Self::Inner {
        &self.0
    }
    fn to_account_metas(&'a self) -> impl Iterator<Item = AccountMeta<'a>> {
        self.inner().iter().map(AccountMeta::from)
    }
}

pub struct InitConfigInstruction<'a> {
    pub accounts: InitConfigInstructionAccounts<'a>,
    pub data: InitConfigArgs,
}

impl<'a> TryFrom<(&'a [AccountInfo], &'a [u8])> for InitConfigInstruction<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [AccountInfo], &'a [u8])) -> Result<Self, Self::Error> {
        let (accounts, data) = value;

        Ok(Self {
            accounts: self::InitConfigInstructionAccounts::try_from(accounts)?,
            data: self::InitConfigArgs::try_from(data)
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        })
    }
}
