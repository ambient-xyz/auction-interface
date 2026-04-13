use crate::instructions::{AuctionInstructionAccounts, to_program_error};
use ambient_auction_api::{
    InitConfigPolicyV2Accounts, InitConfigPolicyV2Args, InstructionAccounts,
};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

#[repr(transparent)]
pub struct InitConfigPolicyV2InstructionAccounts<'a>(InitConfigPolicyV2Accounts<'a, AccountInfo>);

impl<'a> TryFrom<&'a [AccountInfo]> for InitConfigPolicyV2InstructionAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let account_infos =
            InitConfigPolicyV2Accounts::try_from(accounts).map_err(to_program_error)?;

        if !account_infos.authority.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        Ok(Self(account_infos))
    }
}

impl<'a> AuctionInstructionAccounts<'a> for InitConfigPolicyV2InstructionAccounts<'a> {
    type Inner = InitConfigPolicyV2Accounts<'a, AccountInfo>;

    fn inner(&self) -> &Self::Inner {
        &self.0
    }

    fn to_account_metas(&'a self) -> impl Iterator<Item = AccountMeta<'a>> {
        self.inner().iter().map(AccountMeta::from)
    }
}

pub struct InitConfigPolicyV2Instruction<'a> {
    pub accounts: InitConfigPolicyV2InstructionAccounts<'a>,
    pub data: InitConfigPolicyV2Args,
}

impl<'a> TryFrom<(&'a [AccountInfo], &'a [u8])> for InitConfigPolicyV2Instruction<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [AccountInfo], &'a [u8])) -> Result<Self, Self::Error> {
        let (accounts, data) = value;

        Ok(Self {
            accounts: InitConfigPolicyV2InstructionAccounts::try_from(accounts)?,
            data: InitConfigPolicyV2Args::try_from(data)
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        })
    }
}
