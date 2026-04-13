use crate::VOTE_ID;
use crate::instructions::{AuctionInstructionAccounts, to_program_error};
use ambient_auction_api::{
    ClaimWinnerLstakeV2Accounts, ClaimWinnerLstakeV2Args, InstructionAccounts,
};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

#[repr(transparent)]
pub struct ClaimWinnerLstakeV2InstructionAccounts<'a>(ClaimWinnerLstakeV2Accounts<'a, AccountInfo>);

impl<'a> TryFrom<&'a [AccountInfo]> for ClaimWinnerLstakeV2InstructionAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let account_infos =
            ClaimWinnerLstakeV2Accounts::try_from(accounts).map_err(to_program_error)?;

        let ClaimWinnerLstakeV2Accounts {
            bundle_escrow,
            winner_vote_account,
            vote_program,
            vote_authority,
        } = account_infos;

        if !bundle_escrow.is_owned_by(&ambient_auction_api::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        if vote_program.key() != &VOTE_ID {
            return Err(ProgramError::IncorrectProgramId);
        }

        if !winner_vote_account.is_owned_by(&VOTE_ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        if !vote_authority.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        Ok(Self(account_infos))
    }
}

impl<'a> AuctionInstructionAccounts<'a> for ClaimWinnerLstakeV2InstructionAccounts<'a> {
    type Inner = ClaimWinnerLstakeV2Accounts<'a, AccountInfo>;

    fn inner(&self) -> &Self::Inner {
        &self.0
    }

    fn to_account_metas(&'a self) -> impl Iterator<Item = AccountMeta<'a>> {
        self.inner().iter().map(AccountMeta::from)
    }
}

pub struct ClaimWinnerLstakeV2Instruction<'a> {
    pub accounts: ClaimWinnerLstakeV2InstructionAccounts<'a>,
    pub data: ClaimWinnerLstakeV2Args,
}

impl<'a> TryFrom<(&'a [AccountInfo], &'a [u8])> for ClaimWinnerLstakeV2Instruction<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [AccountInfo], &'a [u8])) -> Result<Self, Self::Error> {
        let (accounts, data) = value;

        Ok(Self {
            accounts: ClaimWinnerLstakeV2InstructionAccounts::try_from(accounts)?,
            data: ClaimWinnerLstakeV2Args::try_from(data)
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        })
    }
}
