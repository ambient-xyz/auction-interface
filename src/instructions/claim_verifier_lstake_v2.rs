use crate::VOTE_ID;
use crate::instructions::{AuctionInstructionAccounts, to_program_error};
use ambient_auction_api::{
    ClaimVerifierLstakeV2Accounts, ClaimVerifierLstakeV2Args, InstructionAccounts,
};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

#[repr(transparent)]
pub struct ClaimVerifierLstakeV2InstructionAccounts<'a>(
    ClaimVerifierLstakeV2Accounts<'a, AccountInfo>,
);

impl<'a> TryFrom<&'a [AccountInfo]> for ClaimVerifierLstakeV2InstructionAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let account_infos =
            ClaimVerifierLstakeV2Accounts::try_from(accounts).map_err(to_program_error)?;

        let ClaimVerifierLstakeV2Accounts {
            bundle_escrow,
            verifier_vote_account,
            vote_program,
            vote_authority,
            ..
        } = account_infos;

        if !bundle_escrow.is_owned_by(&ambient_auction_api::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        if vote_program.key() != &VOTE_ID {
            return Err(ProgramError::IncorrectProgramId);
        }

        if !verifier_vote_account.is_owned_by(&VOTE_ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        if !vote_authority.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        super::validate_config_policy_owner(account_infos.config_policy)?;

        for bundle_verifier_page in account_infos.bundle_verifier_pages {
            if !bundle_verifier_page.is_owned_by(&ambient_auction_api::ID) {
                return Err(ProgramError::InvalidAccountOwner);
            }
        }

        Ok(Self(account_infos))
    }
}

impl<'a> AuctionInstructionAccounts<'a> for ClaimVerifierLstakeV2InstructionAccounts<'a> {
    type Inner = ClaimVerifierLstakeV2Accounts<'a, AccountInfo>;

    fn inner(&self) -> &Self::Inner {
        &self.0
    }

    fn to_account_metas(&'a self) -> impl Iterator<Item = AccountMeta<'a>> {
        self.inner().iter().map(AccountMeta::from)
    }
}

pub struct ClaimVerifierLstakeV2Instruction<'a> {
    pub accounts: ClaimVerifierLstakeV2InstructionAccounts<'a>,
    pub data: ClaimVerifierLstakeV2Args,
}

impl<'a> TryFrom<(&'a [AccountInfo], &'a [u8])> for ClaimVerifierLstakeV2Instruction<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [AccountInfo], &'a [u8])) -> Result<Self, Self::Error> {
        let (accounts, data) = value;

        Ok(Self {
            accounts: ClaimVerifierLstakeV2InstructionAccounts::try_from(accounts)?,
            data: ClaimVerifierLstakeV2Args::try_from(data)
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        })
    }
}
