use crate::instructions::{
    AuctionInstructionAccounts, to_program_error, validate_config_policy_owner,
};
use ambient_auction_api::{InstructionAccounts, SetConfigPolicyV2Accounts, SetConfigPolicyV2Args};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

#[repr(transparent)]
pub struct SetConfigPolicyV2InstructionAccounts<'a>(SetConfigPolicyV2Accounts<'a, AccountInfo>);

impl<'a> TryFrom<&'a [AccountInfo]> for SetConfigPolicyV2InstructionAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let account_infos =
            SetConfigPolicyV2Accounts::try_from(accounts).map_err(to_program_error)?;

        if !account_infos.authority.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        validate_config_policy_owner(account_infos.config_policy)?;

        Ok(Self(account_infos))
    }
}

impl<'a> AuctionInstructionAccounts<'a> for SetConfigPolicyV2InstructionAccounts<'a> {
    type Inner = SetConfigPolicyV2Accounts<'a, AccountInfo>;

    fn inner(&self) -> &Self::Inner {
        &self.0
    }

    fn to_account_metas(&'a self) -> impl Iterator<Item = AccountMeta<'a>> {
        self.inner().iter().map(AccountMeta::from)
    }
}

pub struct SetConfigPolicyV2Instruction<'a> {
    pub accounts: SetConfigPolicyV2InstructionAccounts<'a>,
    pub data: SetConfigPolicyV2Args,
}

impl<'a> TryFrom<(&'a [AccountInfo], &'a [u8])> for SetConfigPolicyV2Instruction<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [AccountInfo], &'a [u8])) -> Result<Self, Self::Error> {
        let (accounts, data) = value;

        Ok(Self {
            accounts: SetConfigPolicyV2InstructionAccounts::try_from(accounts)?,
            data: SetConfigPolicyV2Args::try_from(data)
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        })
    }
}
