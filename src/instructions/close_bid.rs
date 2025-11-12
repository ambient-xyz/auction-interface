use crate::instructions::AuctionInstructionAccounts;
use ambient_auction_api::{CloseBidAccounts, CloseBidArgs, InstructionAccounts};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

#[repr(transparent)]
pub struct CloseBidInstructionAccounts<'a>(CloseBidAccounts<'a, AccountInfo>);

impl<'a> TryFrom<&'a [AccountInfo]> for CloseBidInstructionAccounts<'a> {
    type Error = ProgramError;
    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [
            bid_authority,
            bid,
            auction_payer,
            auction,
            bundle,
            vote_account,
            vote_authority,
            vote_program,
            ..,
        ] = accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        if !bundle.is_owned_by(&ambient_auction_api::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        Ok(CloseBidInstructionAccounts(CloseBidAccounts {
            bid_authority,
            bid,
            auction_payer,
            bundle,
            vote_account,
            vote_authority,
            auction,
            vote_program,
        }))
    }
}

impl<'a> AuctionInstructionAccounts<'a> for CloseBidInstructionAccounts<'a> {
    type Inner = CloseBidAccounts<'a, AccountInfo>;
    fn inner(&self) -> &Self::Inner {
        &self.0
    }
    fn to_account_metas(&'a self) -> impl Iterator<Item = AccountMeta<'a>> {
        self.inner().iter().map(AccountMeta::from)
    }
}

pub struct CloseBidInstruction<'a> {
    pub accounts: CloseBidInstructionAccounts<'a>,
    pub data: CloseBidArgs,
}

impl<'a> TryFrom<(&'a [AccountInfo], &'a [u8])> for CloseBidInstruction<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [AccountInfo], &'a [u8])) -> Result<Self, Self::Error> {
        let (accounts, data) = value;

        Ok(Self {
            accounts: self::CloseBidInstructionAccounts::try_from(accounts)?,
            data: self::CloseBidArgs::try_from(data)
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        })
    }
}
