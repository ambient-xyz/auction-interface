use crate::INSTRUCTIONS_SYSVAR_ID;
use crate::instructions::{AuctionInstructionAccounts, to_program_error};
use ambient_auction_api::{
    FinalizeBundleVerificationV2Accounts, FinalizeBundleVerificationV2Args, InstructionAccounts,
};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

#[repr(transparent)]
pub struct FinalizeBundleVerificationV2InstructionAccounts<'a>(
    FinalizeBundleVerificationV2Accounts<'a, AccountInfo>,
);

impl<'a> TryFrom<&'a [AccountInfo]> for FinalizeBundleVerificationV2InstructionAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let account_infos =
            FinalizeBundleVerificationV2Accounts::try_from(accounts).map_err(to_program_error)?;

        let FinalizeBundleVerificationV2Accounts {
            coordinator,
            bundle_escrow,
            instructions_sysvar,
            ..
        } = account_infos;

        if !coordinator.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if !bundle_escrow.is_owned_by(&ambient_auction_api::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        if instructions_sysvar.key() != &INSTRUCTIONS_SYSVAR_ID {
            return Err(ProgramError::UnsupportedSysvar);
        }

        Ok(Self(account_infos))
    }
}

impl<'a> AuctionInstructionAccounts<'a> for FinalizeBundleVerificationV2InstructionAccounts<'a> {
    type Inner = FinalizeBundleVerificationV2Accounts<'a, AccountInfo>;

    fn inner(&self) -> &Self::Inner {
        &self.0
    }

    fn to_account_metas(&'a self) -> impl Iterator<Item = AccountMeta<'a>> {
        self.inner().iter().map(AccountMeta::from)
    }
}

pub struct FinalizeBundleVerificationV2Instruction<'a> {
    pub accounts: FinalizeBundleVerificationV2InstructionAccounts<'a>,
    pub data: FinalizeBundleVerificationV2Args,
}

impl<'a> TryFrom<(&'a [AccountInfo], &'a [u8])> for FinalizeBundleVerificationV2Instruction<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [AccountInfo], &'a [u8])) -> Result<Self, Self::Error> {
        let (accounts, data) = value;

        Ok(Self {
            accounts: FinalizeBundleVerificationV2InstructionAccounts::try_from(accounts)?,
            data: FinalizeBundleVerificationV2Args::try_from(data)
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        })
    }
}
