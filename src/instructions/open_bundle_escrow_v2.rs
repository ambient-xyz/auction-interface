use crate::instructions::{to_program_error, AuctionInstructionAccounts};
use ambient_auction_api::{InstructionAccounts, OpenBundleEscrowV2Accounts, OpenBundleEscrowV2Args};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

#[repr(transparent)]
pub struct OpenBundleEscrowV2InstructionAccounts<'a>(OpenBundleEscrowV2Accounts<'a, AccountInfo>);

impl<'a> TryFrom<&'a [AccountInfo]> for OpenBundleEscrowV2InstructionAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let account_infos =
            OpenBundleEscrowV2Accounts::try_from(accounts).map_err(to_program_error)?;

        if !account_infos.payer.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        super::validate_config_policy_owner(account_infos.config_policy)?;

        Ok(Self(account_infos))
    }
}

impl<'a> AuctionInstructionAccounts<'a> for OpenBundleEscrowV2InstructionAccounts<'a> {
    type Inner = OpenBundleEscrowV2Accounts<'a, AccountInfo>;

    fn inner(&self) -> &Self::Inner {
        &self.0
    }

    fn to_account_metas(&'a self) -> impl Iterator<Item = AccountMeta<'a>> {
        self.inner().iter().map(AccountMeta::from)
    }
}

pub struct OpenBundleEscrowV2Instruction<'a> {
    pub accounts: OpenBundleEscrowV2InstructionAccounts<'a>,
    pub data: OpenBundleEscrowV2Args,
}

impl<'a> TryFrom<(&'a [AccountInfo], &'a [u8])> for OpenBundleEscrowV2Instruction<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [AccountInfo], &'a [u8])) -> Result<Self, Self::Error> {
        let (accounts, data) = value;

        Ok(Self {
            accounts: OpenBundleEscrowV2InstructionAccounts::try_from(accounts)?,
            data: OpenBundleEscrowV2Args::try_from(data)
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        })
    }
}
