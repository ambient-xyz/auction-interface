use crate::instructions::AuctionInstructionAccounts;
use ambient_auction_api::{InstructionAccounts, SubmitValidationAccounts, SubmitValidationArgs};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

#[repr(transparent)]
pub struct SubmitValidationInstructionAccounts<'a>(SubmitValidationAccounts<'a, AccountInfo>);

impl<'a> TryFrom<&'a [AccountInfo]> for SubmitValidationInstructionAccounts<'a> {
    type Error = ProgramError;
    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [
            bundle,
            vote_account,
            vote_program,
            vote_authority,
            job_request,
            ..,
        ] = accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        if !bundle.is_owned_by(&ambient_auction_api::ID) {
            return Err(ProgramError::IllegalOwner);
        }

        if !job_request.is_owned_by(&ambient_auction_api::ID) {
            return Err(ProgramError::IllegalOwner);
        }

        if !vote_authority.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        Ok(SubmitValidationInstructionAccounts(
            SubmitValidationAccounts {
                job_request,
                bundle,
                vote_account,
                vote_program,
                vote_authority,
            },
        ))
    }
}

impl<'a> AuctionInstructionAccounts<'a> for SubmitValidationInstructionAccounts<'a> {
    type Inner = SubmitValidationAccounts<'a, AccountInfo>;
    fn inner(&self) -> &Self::Inner {
        &self.0
    }
    fn to_account_metas(&'a self) -> impl Iterator<Item = AccountMeta<'a>> {
        self.inner().iter().map(AccountMeta::from)
    }
}

pub struct SubmitValidationInstruction<'a> {
    pub accounts: SubmitValidationInstructionAccounts<'a>,
    pub data: SubmitValidationArgs,
}

impl<'a> TryFrom<(&'a [AccountInfo], &'a [u8])> for SubmitValidationInstruction<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [AccountInfo], &'a [u8])) -> Result<Self, Self::Error> {
        let (accounts, data) = value;

        Ok(Self {
            accounts: self::SubmitValidationInstructionAccounts::try_from(accounts)?,
            data: self::SubmitValidationArgs::try_from(data)
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        })
    }
}
