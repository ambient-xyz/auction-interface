use super::{AuctionInstructionAccounts, to_program_error};
use ambient_auction_api::{
    CloseBundleVerifierPageV4Accounts, CloseBundleVerifierPageV4Args, InstructionAccounts,
};
use pinocchio::{account_info::AccountInfo, instruction::AccountMeta, program_error::ProgramError};

pub struct CloseBundleVerifierPageV4InstructionAccounts<'a>(
    CloseBundleVerifierPageV4Accounts<'a, AccountInfo>,
);

impl<'a> TryFrom<&'a [AccountInfo]> for CloseBundleVerifierPageV4InstructionAccounts<'a> {
    type Error = ProgramError;
    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let accounts =
            CloseBundleVerifierPageV4Accounts::try_from(accounts).map_err(to_program_error)?;
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
impl<'a> AuctionInstructionAccounts<'a> for CloseBundleVerifierPageV4InstructionAccounts<'a> {
    type Inner = CloseBundleVerifierPageV4Accounts<'a, AccountInfo>;
    fn inner(&self) -> &Self::Inner {
        &self.0
    }
    fn to_account_metas(&'a self) -> impl Iterator<Item = AccountMeta<'a>> {
        self.inner().iter().map(AccountMeta::from)
    }
}
pub struct CloseBundleVerifierPageV4Instruction<'a> {
    pub accounts: CloseBundleVerifierPageV4InstructionAccounts<'a>,
    pub data: CloseBundleVerifierPageV4Args,
}
impl<'a> TryFrom<(&'a [AccountInfo], &'a [u8])> for CloseBundleVerifierPageV4Instruction<'a> {
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
