use crate::instructions::{to_program_error, AuctionInstructionAccounts};
use ambient_auction_api::{CancelBundleAccounts, CancelBundleArgs, InstructionAccounts};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

#[repr(transparent)]
pub struct CancelBundleInstructionAccounts<'a>(CancelBundleAccounts<'a, AccountInfo>);

impl<'a> TryFrom<&'a [AccountInfo]> for CancelBundleInstructionAccounts<'a> {
    type Error = ProgramError;
    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let account_infos = CancelBundleAccounts::try_from(accounts).map_err(to_program_error)?;

        let CancelBundleAccounts {
            payer: _,
            bundle,
            child_bundle: _,
            registry,
            system_program: _,
        } = account_infos;

        if !bundle.is_owned_by(&ambient_auction_api::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        if !registry.is_owned_by(&ambient_auction_api::ID) {
            return Err(ProgramError::IllegalOwner);
        }

        Ok(CancelBundleInstructionAccounts(account_infos))
    }
}

impl<'a> AuctionInstructionAccounts<'a> for CancelBundleInstructionAccounts<'a> {
    type Inner = CancelBundleAccounts<'a, AccountInfo>;
    fn inner(&self) -> &Self::Inner {
        &self.0
    }
    fn to_account_metas(&'a self) -> impl Iterator<Item = AccountMeta<'a>> {
        self.inner().iter().map(AccountMeta::from)
    }
}

pub struct CancelBundleInstruction<'a> {
    pub accounts: CancelBundleInstructionAccounts<'a>,
    pub data: CancelBundleArgs,
}

impl<'a> TryFrom<(&'a [AccountInfo], &'a [u8])> for CancelBundleInstruction<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [AccountInfo], &'a [u8])) -> Result<Self, Self::Error> {
        let (accounts, data) = value;

        Ok(Self {
            accounts: self::CancelBundleInstructionAccounts::try_from(accounts)?,
            data: self::CancelBundleArgs::try_from(data)
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        })
    }
}
