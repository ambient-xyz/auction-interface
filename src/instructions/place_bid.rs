use crate::instructions::AuctionInstructionAccounts;
use ambient_auction_api::{InstructionAccounts, PlaceBidAccounts, PlaceBidArgs};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

#[repr(transparent)]
pub struct PlaceBidInstructionAccounts<'a>(PlaceBidAccounts<'a, AccountInfo>);

impl<'a> TryFrom<&'a [AccountInfo]> for PlaceBidInstructionAccounts<'a> {
    type Error = ProgramError;
    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [payer, bid, auction, system_program, ..] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        if !auction.is_owned_by(&ambient_auction_api::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        Ok(PlaceBidInstructionAccounts(PlaceBidAccounts {
            auction,
            payer,
            bid,
            system_program,
        }))
    }
}

impl<'a> AuctionInstructionAccounts<'a> for PlaceBidInstructionAccounts<'a> {
    type Inner = PlaceBidAccounts<'a, AccountInfo>;
    fn inner(&self) -> &Self::Inner {
        &self.0
    }
    fn to_account_metas(&'a self) -> impl Iterator<Item = AccountMeta<'a>> {
        self.inner().iter().map(AccountMeta::from)
    }
}

pub struct PlaceBidInstruction<'a> {
    pub accounts: PlaceBidInstructionAccounts<'a>,
    pub data: PlaceBidArgs,
}

impl<'a> TryFrom<(&'a [AccountInfo], &'a [u8])> for PlaceBidInstruction<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [AccountInfo], &'a [u8])) -> Result<Self, Self::Error> {
        let (accounts, data) = value;

        Ok(Self {
            accounts: self::PlaceBidInstructionAccounts::try_from(accounts)?,
            data: self::PlaceBidArgs::try_from(data)
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        })
    }
}
