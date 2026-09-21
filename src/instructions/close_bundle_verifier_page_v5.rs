use super::{AuctionInstructionAccounts, to_program_error};
use ambient_auction_api::{
    CloseBundleVerifierPageV5Accounts, CloseBundleVerifierPageV5Args, InstructionAccounts,
};
use pinocchio::{account_info::AccountInfo, instruction::AccountMeta, program_error::ProgramError};

pub struct CloseBundleVerifierPageV5InstructionAccounts<'a>(
    CloseBundleVerifierPageV5Accounts<'a, AccountInfo>,
);

impl<'a> TryFrom<&'a [AccountInfo]> for CloseBundleVerifierPageV5InstructionAccounts<'a> {
    type Error = ProgramError;
    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let accounts =
            CloseBundleVerifierPageV5Accounts::try_from(accounts).map_err(to_program_error)?;
        super::validate_current_bundle_escrow(accounts.bundle_escrow)?;
        if !accounts.funder.is_writable() || !accounts.bundle_verifier_page.is_writable() {
            return Err(ProgramError::InvalidArgument);
        }
        if !accounts
            .bundle_verifier_page
            .is_owned_by(&ambient_auction_api::ID)
        {
            return Err(ProgramError::InvalidAccountOwner);
        }
        Ok(Self(accounts))
    }
}
impl<'a> AuctionInstructionAccounts<'a> for CloseBundleVerifierPageV5InstructionAccounts<'a> {
    type Inner = CloseBundleVerifierPageV5Accounts<'a, AccountInfo>;
    fn inner(&self) -> &Self::Inner {
        &self.0
    }
    fn to_account_metas(&'a self) -> impl Iterator<Item = AccountMeta<'a>> {
        self.inner().iter().map(AccountMeta::from)
    }
}
pub struct CloseBundleVerifierPageV5Instruction<'a> {
    pub accounts: CloseBundleVerifierPageV5InstructionAccounts<'a>,
    pub data: CloseBundleVerifierPageV5Args,
}
impl<'a> TryFrom<(&'a [AccountInfo], &'a [u8])> for CloseBundleVerifierPageV5Instruction<'a> {
    type Error = ProgramError;
    fn try_from((accounts, data): (&'a [AccountInfo], &'a [u8])) -> Result<Self, Self::Error> {
        Ok(Self {
            accounts: accounts.try_into()?,
            data: data
                .try_into()
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        })
    }
}
