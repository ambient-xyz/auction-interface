use crate::instructions::{to_program_error, AuctionInstructionAccounts};
use ambient_auction_api::error::AuctionError;
use ambient_auction_api::{InstructionAccounts, SubmitJobOutputAccounts, SubmitJobOutputArgs};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

#[repr(transparent)]
pub struct SubmitJobOutputInstructionAccounts<'a>(SubmitJobOutputAccounts<'a, AccountInfo>);

impl<'a> TryFrom<&'a [AccountInfo]> for SubmitJobOutputInstructionAccounts<'a> {
    type Error = ProgramError;
    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let account_infos =
            SubmitJobOutputAccounts::try_from(accounts).map_err(to_program_error)?;

        let SubmitJobOutputAccounts {
            bid_authority,
            bundle,
            job_request,
            bid,
            auction,
            output_data_account: _,
        } = account_infos;

        if !bid_authority.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if !bundle.is_owned_by(&ambient_auction_api::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        if !job_request.is_owned_by(&ambient_auction_api::ID) {
            return Err(ProgramError::IllegalOwner);
        }

        if !bid.is_owned_by(&ambient_auction_api::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        if !auction.is_owned_by(&ambient_auction_api::ID) {
            return Err(to_program_error(AuctionError::IncorrectAuction));
        }

        Ok(SubmitJobOutputInstructionAccounts(account_infos))
    }
}

impl<'a> AuctionInstructionAccounts<'a> for SubmitJobOutputInstructionAccounts<'a> {
    type Inner = SubmitJobOutputAccounts<'a, AccountInfo>;
    fn inner(&self) -> &Self::Inner {
        &self.0
    }
    fn to_account_metas(&'a self) -> impl Iterator<Item = AccountMeta<'a>> {
        self.inner().iter().map(AccountMeta::from)
    }
}

pub struct SubmitJobOutputInstruction<'a> {
    pub accounts: SubmitJobOutputInstructionAccounts<'a>,
    pub data: SubmitJobOutputArgs,
}

impl<'a> TryFrom<(&'a [AccountInfo], &'a [u8])> for SubmitJobOutputInstruction<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [AccountInfo], &'a [u8])) -> Result<Self, Self::Error> {
        let (accounts, data) = value;

        Ok(Self {
            accounts: self::SubmitJobOutputInstructionAccounts::try_from(accounts)?,
            data: self::SubmitJobOutputArgs::try_from(data)
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        })
    }
}
