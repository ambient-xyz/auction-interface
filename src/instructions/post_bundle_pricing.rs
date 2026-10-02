use crate::instructions::{AuctionInstructionAccounts, to_program_error};
use ambient_auction_api::{InstructionAccounts, PostBundlePricingAccounts, PostBundlePricingArgs};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

#[repr(transparent)]
pub struct PostBundlePricingInstructionAccounts<'a>(PostBundlePricingAccounts<'a, AccountInfo>);

impl<'a> TryFrom<&'a [AccountInfo]> for PostBundlePricingInstructionAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let account_infos =
            PostBundlePricingAccounts::try_from(accounts).map_err(to_program_error)?;

        if !account_infos.coordinator.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        super::validate_v6_bundle_escrow(account_infos.bundle_escrow)?;
        super::validate_writable_v6_bundle_verifier_page(account_infos.bundle_verifier_page)?;

        Ok(Self(account_infos))
    }
}

impl<'a> AuctionInstructionAccounts<'a> for PostBundlePricingInstructionAccounts<'a> {
    type Inner = PostBundlePricingAccounts<'a, AccountInfo>;

    fn inner(&self) -> &Self::Inner {
        &self.0
    }

    fn to_account_metas(&'a self) -> impl Iterator<Item = AccountMeta<'a>> {
        self.inner().iter().map(AccountMeta::from)
    }
}

pub struct PostBundlePricingInstruction<'a> {
    pub accounts: PostBundlePricingInstructionAccounts<'a>,
    pub data: PostBundlePricingArgs,
}

impl<'a> TryFrom<(&'a [AccountInfo], &'a [u8])> for PostBundlePricingInstruction<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [AccountInfo], &'a [u8])) -> Result<Self, Self::Error> {
        let (accounts, data) = value;

        Ok(Self {
            accounts: PostBundlePricingInstructionAccounts::try_from(accounts)?,
            data: PostBundlePricingArgs::try_from(data)
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        })
    }
}
