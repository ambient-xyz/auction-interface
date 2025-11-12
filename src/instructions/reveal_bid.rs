use crate::instructions::AuctionInstructionAccounts;
use ambient_auction_api::{InstructionAccounts, RevealBidAccounts, RevealBidArgs};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

#[repr(transparent)]
pub struct RevealBidInstructionAccounts<'a>(RevealBidAccounts<'a, AccountInfo>);

impl<'a> TryFrom<&'a [AccountInfo]> for RevealBidInstructionAccounts<'a> {
    type Error = ProgramError;
    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [
            bid_authority,
            bid,
            auction,
            bundle,
            vote_account,
            vote_authority,
            ..,
        ] = accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        if !auction.is_owned_by(&ambient_auction_api::ID) {
            return Err(ProgramError::InvalidAccountOwner);
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

        Ok(RevealBidInstructionAccounts(RevealBidAccounts {
            auction,
            bid,
            bid_authority,
            bundle,
            vote_account,
            vote_authority,
        }))
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
