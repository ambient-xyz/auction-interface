use crate::instructions::{to_program_error, AuctionInstructionAccounts};
use crate::VOTE_ID;
use ambient_auction_api::{
    CommitAuctionSettlementV2Accounts, CommitAuctionSettlementV2Args, InstructionAccounts,
};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

#[repr(transparent)]
pub struct CommitAuctionSettlementV2InstructionAccounts<'a>(
    CommitAuctionSettlementV2Accounts<'a, AccountInfo>,
);

impl<'a> TryFrom<&'a [AccountInfo]> for CommitAuctionSettlementV2InstructionAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let account_infos =
            CommitAuctionSettlementV2Accounts::try_from(accounts).map_err(to_program_error)?;

        let CommitAuctionSettlementV2Accounts {
            coordinator,
            bundle_escrow,
            config_policy: _,
            winner_vote_account,
        } = account_infos;

        if !coordinator.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if !bundle_escrow.is_owned_by(&ambient_auction_api::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        super::validate_config_policy_owner(account_infos.config_policy)?;

        if !winner_vote_account.is_owned_by(&VOTE_ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        Ok(Self(account_infos))
    }
}

impl<'a> AuctionInstructionAccounts<'a> for CommitAuctionSettlementV2InstructionAccounts<'a> {
    type Inner = CommitAuctionSettlementV2Accounts<'a, AccountInfo>;

    fn inner(&self) -> &Self::Inner {
        &self.0
    }

    fn to_account_metas(&'a self) -> impl Iterator<Item = AccountMeta<'a>> {
        self.inner().iter().map(AccountMeta::from)
    }
}

pub struct CommitAuctionSettlementV2Instruction<'a> {
    pub accounts: CommitAuctionSettlementV2InstructionAccounts<'a>,
    pub data: CommitAuctionSettlementV2Args,
}

impl<'a> TryFrom<(&'a [AccountInfo], &'a [u8])> for CommitAuctionSettlementV2Instruction<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [AccountInfo], &'a [u8])) -> Result<Self, Self::Error> {
        let (accounts, data) = value;

        Ok(Self {
            accounts: CommitAuctionSettlementV2InstructionAccounts::try_from(accounts)?,
            data: CommitAuctionSettlementV2Args::try_from(data)
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        })
    }
}
