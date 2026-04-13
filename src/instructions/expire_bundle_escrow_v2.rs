use crate::instructions::{to_program_error, AuctionInstructionAccounts};
use ambient_auction_api::{ExpireBundleEscrowV2Accounts, ExpireBundleEscrowV2Args, InstructionAccounts};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

#[repr(transparent)]
pub struct ExpireBundleEscrowV2InstructionAccounts<'a>(
    ExpireBundleEscrowV2Accounts<'a, AccountInfo>,
);

impl<'a> TryFrom<&'a [AccountInfo]> for ExpireBundleEscrowV2InstructionAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let account_infos =
            ExpireBundleEscrowV2Accounts::try_from(accounts).map_err(to_program_error)?;

        if !account_infos.bundle_escrow.is_owned_by(&ambient_auction_api::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        super::validate_config_policy_owner(account_infos.config_policy)?;

        Ok(Self(account_infos))
    }
}

impl<'a> AuctionInstructionAccounts<'a> for ExpireBundleEscrowV2InstructionAccounts<'a> {
    type Inner = ExpireBundleEscrowV2Accounts<'a, AccountInfo>;

    fn inner(&self) -> &Self::Inner {
        &self.0
    }

    fn to_account_metas(&'a self) -> impl Iterator<Item = AccountMeta<'a>> {
        self.inner().iter().map(AccountMeta::from)
    }
}

pub struct ExpireBundleEscrowV2Instruction<'a> {
    pub accounts: ExpireBundleEscrowV2InstructionAccounts<'a>,
    pub data: ExpireBundleEscrowV2Args,
}

impl<'a> TryFrom<(&'a [AccountInfo], &'a [u8])> for ExpireBundleEscrowV2Instruction<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [AccountInfo], &'a [u8])) -> Result<Self, Self::Error> {
        let (accounts, data) = value;

        Ok(Self {
            accounts: ExpireBundleEscrowV2InstructionAccounts::try_from(accounts)?,
            data: ExpireBundleEscrowV2Args::try_from(data)
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        })
    }
}
