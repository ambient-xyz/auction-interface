use crate::instructions::AuctionInstructionAccounts;
use ambient_auction_api::{EndAuctionAccounts, EndAuctionArgs, InstructionAccounts};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

#[repr(transparent)]
pub struct EndAuctionInstructionAccounts<'a>(EndAuctionAccounts<'a, AccountInfo>);

impl<'a> TryFrom<&'a [AccountInfo]> for EndAuctionInstructionAccounts<'a> {
    type Error = ProgramError;
    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [auction, bundle, vote_account, payer, ..] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        if !auction.is_owned_by(&ambient_auction_api::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        if !bundle.is_owned_by(&ambient_auction_api::ID) {
            return Err(ProgramError::IllegalOwner);
        }

        Ok(EndAuctionInstructionAccounts(EndAuctionAccounts {
            bundle,
            vote_account,
            auction,
            payer,
        }))
    }
}

impl<'a> AuctionInstructionAccounts<'a> for EndAuctionInstructionAccounts<'a> {
    type Inner = EndAuctionAccounts<'a, AccountInfo>;
    fn inner(&self) -> &Self::Inner {
        &self.0
    }
    fn to_account_metas(&'a self) -> impl Iterator<Item = AccountMeta<'a>> {
        self.inner().iter().map(AccountMeta::from)
    }
}

pub struct EndAuctionInstruction<'a> {
    pub accounts: EndAuctionInstructionAccounts<'a>,
    pub data: EndAuctionArgs,
}

impl<'a> TryFrom<(&'a [AccountInfo], &'a [u8])> for EndAuctionInstruction<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [AccountInfo], &'a [u8])) -> Result<Self, Self::Error> {
        let (accounts, data) = value;

        Ok(Self {
            accounts: self::EndAuctionInstructionAccounts::try_from(accounts)?,
            data: self::EndAuctionArgs::try_from(data)
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        })
    }
}
