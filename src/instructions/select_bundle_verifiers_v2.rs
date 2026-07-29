use crate::instructions::{AuctionInstructionAccounts, to_program_error};
use ambient_auction_api::{
    InstructionAccounts, SelectBundleVerifiersV2Accounts, SelectBundleVerifiersV2Args,
};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

#[repr(transparent)]
pub struct SelectBundleVerifiersV2InstructionAccounts<'a>(
    SelectBundleVerifiersV2Accounts<'a, AccountInfo>,
);

impl<'a> TryFrom<&'a [AccountInfo]> for SelectBundleVerifiersV2InstructionAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let account_infos =
            SelectBundleVerifiersV2Accounts::try_from(accounts).map_err(to_program_error)?;

        if !account_infos.bundle_escrow.is_writable() {
            return Err(ProgramError::InvalidArgument);
        }
        if !account_infos
            .bundle_escrow
            .is_owned_by(&ambient_auction_api::ID)
        {
            return Err(ProgramError::InvalidAccountOwner);
        }

        if let Some(bundle_verification_dispute) = account_infos.bundle_verification_dispute {
            if !bundle_verification_dispute.is_writable() {
                return Err(ProgramError::InvalidArgument);
            }
            if !bundle_verification_dispute.is_owned_by(&ambient_auction_api::ID) {
                return Err(ProgramError::InvalidAccountOwner);
            }
        }

        Ok(Self(account_infos))
    }
}

impl<'a> AuctionInstructionAccounts<'a> for SelectBundleVerifiersV2InstructionAccounts<'a> {
    type Inner = SelectBundleVerifiersV2Accounts<'a, AccountInfo>;

    fn inner(&self) -> &Self::Inner {
        &self.0
    }

    fn to_account_metas(&'a self) -> impl Iterator<Item = AccountMeta<'a>> {
        self.inner().iter().map(AccountMeta::from)
    }
}

pub struct SelectBundleVerifiersV2Instruction<'a> {
    pub accounts: SelectBundleVerifiersV2InstructionAccounts<'a>,
    pub data: SelectBundleVerifiersV2Args,
}

impl<'a> TryFrom<(&'a [AccountInfo], &'a [u8])> for SelectBundleVerifiersV2Instruction<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [AccountInfo], &'a [u8])) -> Result<Self, Self::Error> {
        let (accounts, data) = value;

        Ok(Self {
            accounts: SelectBundleVerifiersV2InstructionAccounts::try_from(accounts)?,
            data: SelectBundleVerifiersV2Args::try_from(data)
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        })
    }
}
