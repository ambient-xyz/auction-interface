use super::{AuctionInstructionAccounts, to_program_error};
use ambient_auction_api::{
    AuthorizeBundleDisputeEvidenceV5Accounts, AuthorizeBundleDisputeEvidenceV5Args,
    InstructionAccounts,
};
use pinocchio::{account_info::AccountInfo, instruction::AccountMeta, program_error::ProgramError};

pub struct AuthorizeBundleDisputeEvidenceV5InstructionAccounts<'a>(
    AuthorizeBundleDisputeEvidenceV5Accounts<'a, AccountInfo>,
);

impl<'a> TryFrom<&'a [AccountInfo]> for AuthorizeBundleDisputeEvidenceV5InstructionAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let accounts = AuthorizeBundleDisputeEvidenceV5Accounts::try_from(accounts)
            .map_err(to_program_error)?;
        if !accounts.submitter.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }
        if !accounts.bundle_verification_dispute.is_writable() {
            return Err(ProgramError::InvalidArgument);
        }
        if !accounts.bundle_escrow.is_owned_by(&ambient_auction_api::ID)
            || !accounts
                .bundle_verification_dispute
                .is_owned_by(&ambient_auction_api::ID)
        {
            return Err(ProgramError::InvalidAccountOwner);
        }
        if accounts.instructions_sysvar.key() != &crate::INSTRUCTIONS_SYSVAR_ID {
            return Err(ProgramError::UnsupportedSysvar);
        }
        Ok(Self(accounts))
    }
}

impl<'a> AuctionInstructionAccounts<'a>
    for AuthorizeBundleDisputeEvidenceV5InstructionAccounts<'a>
{
    type Inner = AuthorizeBundleDisputeEvidenceV5Accounts<'a, AccountInfo>;
    fn inner(&self) -> &Self::Inner {
        &self.0
    }
    fn to_account_metas(&'a self) -> impl Iterator<Item = AccountMeta<'a>> {
        self.inner().iter().map(AccountMeta::from)
    }
}

pub struct AuthorizeBundleDisputeEvidenceV5Instruction<'a> {
    pub accounts: AuthorizeBundleDisputeEvidenceV5InstructionAccounts<'a>,
    pub data: AuthorizeBundleDisputeEvidenceV5Args,
}

impl<'a> TryFrom<(&'a [AccountInfo], &'a [u8])>
    for AuthorizeBundleDisputeEvidenceV5Instruction<'a>
{
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
