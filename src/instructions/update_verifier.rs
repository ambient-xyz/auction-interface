use crate::VOTE_ID;
use crate::instructions::{AuctionInstructionAccounts, to_program_error};
use ambient_auction_api::{InstructionAccounts, UpdateVerifierAccounts, UpdateVerifierArgs};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

#[repr(transparent)]
pub struct UpdateVerifierInstructionAccounts<'a>(UpdateVerifierAccounts<'a, AccountInfo>);

impl<'a> TryFrom<&'a [AccountInfo]> for UpdateVerifierInstructionAccounts<'a> {
    type Error = ProgramError;
    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let account_infos = UpdateVerifierAccounts::try_from(accounts).map_err(to_program_error)?;

        let UpdateVerifierAccounts {
            vote_account,
            vote_authority,
        } = account_infos;

        if !vote_authority.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if !vote_account.is_owned_by(&VOTE_ID) {
            return Err(ProgramError::IllegalOwner);
        }

        Ok(UpdateVerifierInstructionAccounts(account_infos))
    }
}

impl<'a> AuctionInstructionAccounts<'a> for UpdateVerifierInstructionAccounts<'a> {
    type Inner = UpdateVerifierAccounts<'a, AccountInfo>;
    fn inner(&self) -> &Self::Inner {
        &self.0
    }
    fn to_account_metas(&'a self) -> impl Iterator<Item = AccountMeta<'a>> {
        self.inner().iter().map(AccountMeta::from)
    }
}

pub struct UpdateVerifierInstruction<'a> {
    pub accounts: UpdateVerifierInstructionAccounts<'a>,
    pub data: UpdateVerifierArgs,
}

impl<'a> TryFrom<(&'a [AccountInfo], &'a [u8])> for UpdateVerifierInstruction<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [AccountInfo], &'a [u8])) -> Result<Self, Self::Error> {
        let (accounts, data) = value;

        Ok(Self {
            accounts: self::UpdateVerifierInstructionAccounts::try_from(accounts)?,
            data: self::UpdateVerifierArgs::try_from(data)
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        })
    }
}
