use crate::instructions::AuctionInstructionAccounts;
use ambient_auction_api::{CloseRequestAccounts, CloseRequestArgs, InstructionAccounts};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

#[repr(transparent)]
pub struct CloseRequestInstructionAccounts<'a>(CloseRequestAccounts<'a, AccountInfo>);

impl<'a> TryFrom<&'a [AccountInfo]> for CloseRequestInstructionAccounts<'a> {
    type Error = ProgramError;
    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [request_authority, job_request, bundle_payer, bundle, ..] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        if !bundle.is_owned_by(&ambient_auction_api::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        if !job_request.is_owned_by(&ambient_auction_api::ID) {
            return Err(ProgramError::IllegalOwner);
        }

        if !request_authority.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        Ok(CloseRequestInstructionAccounts(CloseRequestAccounts {
            bundle,
            request_authority,
            job_request,
            bundle_payer,
        }))
    }
}

impl<'a> AuctionInstructionAccounts<'a> for CloseRequestInstructionAccounts<'a> {
    type Inner = CloseRequestAccounts<'a, AccountInfo>;
    fn inner(&self) -> &Self::Inner {
        &self.0
    }
    fn to_account_metas(&'a self) -> impl Iterator<Item = AccountMeta<'a>> {
        self.inner().iter().map(AccountMeta::from)
    }
}

pub struct CloseRequestInstruction<'a> {
    pub accounts: CloseRequestInstructionAccounts<'a>,
    pub data: CloseRequestArgs,
}

impl<'a> TryFrom<(&'a [AccountInfo], &'a [u8])> for CloseRequestInstruction<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [AccountInfo], &'a [u8])) -> Result<Self, Self::Error> {
        let (accounts, data) = value;

        Ok(Self {
            accounts: self::CloseRequestInstructionAccounts::try_from(accounts)?,
            data: self::CloseRequestArgs::try_from(data)
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        })
    }
}
