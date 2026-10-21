use crate::instructions::{AuctionInstructionAccounts, to_program_error};
use ambient_auction_api::{
    CommitSmallAuctionSettlementV4Accounts, CommitSmallAuctionSettlementV4Args, InstructionAccounts,
};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

#[repr(transparent)]
pub struct CommitSmallAuctionSettlementV4InstructionAccounts<'a>(
    CommitSmallAuctionSettlementV4Accounts<'a, AccountInfo>,
);

impl<'a> TryFrom<&'a [AccountInfo]> for CommitSmallAuctionSettlementV4InstructionAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let account_infos =
            CommitSmallAuctionSettlementV4Accounts::try_from(accounts).map_err(to_program_error)?;

        if !account_infos.coordinator.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        super::validate_config_policy_owner(account_infos.config_policy)?;

        super::validate_current_bundle_escrow(account_infos.bundle_escrow)?;

        Ok(Self(account_infos))
    }
}

impl<'a> AuctionInstructionAccounts<'a> for CommitSmallAuctionSettlementV4InstructionAccounts<'a> {
    type Inner = CommitSmallAuctionSettlementV4Accounts<'a, AccountInfo>;

    fn inner(&self) -> &Self::Inner {
        &self.0
    }

    fn to_account_metas(&'a self) -> impl Iterator<Item = AccountMeta<'a>> {
        self.inner().iter().map(AccountMeta::from)
    }
}

pub struct CommitSmallAuctionSettlementV4Instruction<'a> {
    pub accounts: CommitSmallAuctionSettlementV4InstructionAccounts<'a>,
    pub data: CommitSmallAuctionSettlementV4Args,
}

impl<'a> TryFrom<(&'a [AccountInfo], &'a [u8])> for CommitSmallAuctionSettlementV4Instruction<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [AccountInfo], &'a [u8])) -> Result<Self, Self::Error> {
        let (accounts, data) = value;

        Ok(Self {
            accounts: CommitSmallAuctionSettlementV4InstructionAccounts::try_from(accounts)?,
            data: CommitSmallAuctionSettlementV4Args::try_from(data)
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        })
    }
}
