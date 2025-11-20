use crate::instructions::{to_program_error, AuctionInstructionAccounts};
use crate::VOTE_ID;
use ambient_auction_api::error::AuctionError;
use ambient_auction_api::{InstructionAccounts, RevealBidAccounts, RevealBidArgs};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

#[repr(transparent)]
pub struct RevealBidInstructionAccounts<'a>(RevealBidAccounts<'a, AccountInfo>);

impl<'a> TryFrom<&'a [AccountInfo]> for RevealBidInstructionAccounts<'a> {
    type Error = ProgramError;
    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let account_infos = RevealBidAccounts::try_from(accounts).map_err(to_program_error)?;
        let RevealBidAccounts {
            bid_authority,
            bid,
            auction,
            bundle,
            vote_account,
            vote_authority,
        } = account_infos;

        if !auction.is_owned_by(&ambient_auction_api::ID) {
            return Err(to_program_error(AuctionError::IncorrectAuction));
        }

        if !bid.is_owned_by(&ambient_auction_api::ID) {
            return Err(ProgramError::IllegalOwner);
        }

        if !bundle.is_owned_by(&ambient_auction_api::ID) {
            return Err(ProgramError::IllegalOwner);
        }

        if !bid_authority.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if !vote_authority.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if !vote_account.is_owned_by(&VOTE_ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        Ok(RevealBidInstructionAccounts(account_infos))
    }
}

impl<'a> AuctionInstructionAccounts<'a> for RevealBidInstructionAccounts<'a> {
    type Inner = RevealBidAccounts<'a, AccountInfo>;
    fn inner(&self) -> &Self::Inner {
        &self.0
    }
    fn to_account_metas(&'a self) -> impl Iterator<Item = AccountMeta<'a>> {
        self.inner().iter().map(AccountMeta::from)
    }
}

pub struct RevealBidInstruction<'a> {
    pub accounts: RevealBidInstructionAccounts<'a>,
    pub data: RevealBidArgs,
}

impl<'a> TryFrom<(&'a [AccountInfo], &'a [u8])> for RevealBidInstruction<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [AccountInfo], &'a [u8])) -> Result<Self, Self::Error> {
        let (accounts, data) = value;

        Ok(Self {
            accounts: self::RevealBidInstructionAccounts::try_from(accounts)?,
            data: self::RevealBidArgs::try_from(data)
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        })
    }
}
