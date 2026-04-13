use crate::instructions::{AuctionInstructionAccounts, to_program_error};
use ambient_auction_api::{
    InitAuctionVerifiersAccounts, InitAuctionVerifiersArgs, InstructionAccounts,
};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

#[repr(transparent)]
pub struct InitAuctionVerifiersInstructionAccounts<'a>(
    InitAuctionVerifiersAccounts<'a, AccountInfo>,
);

impl<'a> TryFrom<&'a [AccountInfo]> for InitAuctionVerifiersInstructionAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let account_infos =
            InitAuctionVerifiersAccounts::try_from(accounts).map_err(to_program_error)?;

        Ok(InitAuctionVerifiersInstructionAccounts(account_infos))
    }
}

impl<'a> AuctionInstructionAccounts<'a> for InitAuctionVerifiersInstructionAccounts<'a> {
    type Inner = InitAuctionVerifiersAccounts<'a, AccountInfo>;

    fn inner(&self) -> &Self::Inner {
        &self.0
    }

    fn to_account_metas(&'a self) -> impl Iterator<Item = AccountMeta<'a>> {
        self.inner().iter().map(AccountMeta::from)
    }
}

pub struct InitAuctionVerifiersInstruction<'a> {
    pub accounts: InitAuctionVerifiersInstructionAccounts<'a>,
    pub data: InitAuctionVerifiersArgs,
}

impl<'a> TryFrom<(&'a [AccountInfo], &'a [u8])> for InitAuctionVerifiersInstruction<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [AccountInfo], &'a [u8])) -> Result<Self, Self::Error> {
        let (accounts, data) = value;

        Ok(Self {
            accounts: self::InitAuctionVerifiersInstructionAccounts::try_from(accounts)?,
            data: self::InitAuctionVerifiersArgs::try_from(data)
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        })
    }
}
