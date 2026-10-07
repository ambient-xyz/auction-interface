use crate::instructions::{
    AuctionInstructionAccounts, CommitAuctionSettlementV2InstructionAccounts,
};
use ambient_auction_api::{
    CommitAuctionSettlementV3Accounts, CommitAuctionSettlementV3Args, InstructionAccounts,
};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

#[repr(transparent)]
pub struct CommitAuctionSettlementV3InstructionAccounts<'a>(
    CommitAuctionSettlementV2InstructionAccounts<'a>,
);

impl<'a> TryFrom<&'a [AccountInfo]> for CommitAuctionSettlementV3InstructionAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let accounts = CommitAuctionSettlementV2InstructionAccounts::try_from(accounts)?;
        super::validate_pricing_bundle_escrow(accounts.inner().bundle_escrow)?;
        Ok(Self(accounts))
    }
}

impl<'a> AuctionInstructionAccounts<'a> for CommitAuctionSettlementV3InstructionAccounts<'a> {
    type Inner = CommitAuctionSettlementV3Accounts<'a, AccountInfo>;

    fn inner(&self) -> &Self::Inner {
        self.0.inner()
    }

    fn to_account_metas(&'a self) -> impl Iterator<Item = AccountMeta<'a>> {
        self.inner().iter().map(AccountMeta::from)
    }
}

pub struct CommitAuctionSettlementV3Instruction<'a> {
    pub accounts: CommitAuctionSettlementV3InstructionAccounts<'a>,
    pub data: CommitAuctionSettlementV3Args,
}

impl<'a> TryFrom<(&'a [AccountInfo], &'a [u8])> for CommitAuctionSettlementV3Instruction<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [AccountInfo], &'a [u8])) -> Result<Self, Self::Error> {
        let (accounts, data) = value;

        Ok(Self {
            accounts: CommitAuctionSettlementV3InstructionAccounts::try_from(accounts)?,
            data: CommitAuctionSettlementV3Args::try_from(data)
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        })
    }
}
