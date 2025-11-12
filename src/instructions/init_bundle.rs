use crate::instructions::AuctionInstructionAccounts;
use ambient_auction_api::{InitBundleAccounts, InitBundleArgs, InstructionAccounts};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

#[repr(transparent)]
pub struct InitBundleInstructionAccounts<'a>(InitBundleAccounts<'a, AccountInfo>);

impl<'a> TryFrom<&'a [AccountInfo]> for InitBundleInstructionAccounts<'a> {
    type Error = ProgramError;
    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [payer, bundle, registry, system_program, ..] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        Ok(InitBundleInstructionAccounts(InitBundleAccounts {
            bundle,
            payer,
            registry,
            system_program,
        }))
    }
}

impl<'a> AuctionInstructionAccounts<'a> for InitBundleInstructionAccounts<'a> {
    type Inner = InitBundleAccounts<'a, AccountInfo>;
    fn inner(&self) -> &Self::Inner {
        &self.0
    }
    fn to_account_metas(&'a self) -> impl Iterator<Item = AccountMeta<'a>> {
        self.inner().iter().map(AccountMeta::from)
    }
}

pub struct InitBundleInstruction<'a> {
    pub accounts: InitBundleInstructionAccounts<'a>,
    pub data: InitBundleArgs,
}

impl<'a> TryFrom<(&'a [AccountInfo], &'a [u8])> for InitBundleInstruction<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [AccountInfo], &'a [u8])) -> Result<Self, Self::Error> {
        let (accounts, data) = value;

        Ok(Self {
            accounts: self::InitBundleInstructionAccounts::try_from(accounts)?,
            data: self::InitBundleArgs::try_from(data)
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        })
    }
}
