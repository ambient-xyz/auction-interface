use crate::instructions::{AuctionInstructionAccounts, to_program_error};
use ambient_auction_api::{InstructionAccounts, SealBundlePricingAccounts, SealBundlePricingArgs};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

#[repr(transparent)]
pub struct SealBundlePricingInstructionAccounts<'a>(SealBundlePricingAccounts<'a, AccountInfo>);

impl<'a> TryFrom<&'a [AccountInfo]> for SealBundlePricingInstructionAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let account_infos =
            SealBundlePricingAccounts::try_from(accounts).map_err(to_program_error)?;

        if !account_infos.coordinator.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        super::validate_v6_bundle_escrow(account_infos.bundle_escrow)?;

        for page in account_infos.bundle_verifier_pages {
            super::validate_v6_bundle_verifier_page(page)?;
        }

        Ok(Self(account_infos))
    }
}

impl<'a> AuctionInstructionAccounts<'a> for SealBundlePricingInstructionAccounts<'a> {
    type Inner = SealBundlePricingAccounts<'a, AccountInfo>;

    fn inner(&self) -> &Self::Inner {
        &self.0
    }

    fn to_account_metas(&'a self) -> impl Iterator<Item = AccountMeta<'a>> {
        self.inner().iter().map(AccountMeta::from)
    }
}

pub struct SealBundlePricingInstruction<'a> {
    pub accounts: SealBundlePricingInstructionAccounts<'a>,
    pub data: SealBundlePricingArgs,
}

impl<'a> TryFrom<(&'a [AccountInfo], &'a [u8])> for SealBundlePricingInstruction<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [AccountInfo], &'a [u8])) -> Result<Self, Self::Error> {
        let (accounts, data) = value;

        Ok(Self {
            accounts: SealBundlePricingInstructionAccounts::try_from(accounts)?,
            data: SealBundlePricingArgs::try_from(data)
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        })
    }
}
