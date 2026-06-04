use crate::SYSTEM_PROGRAM_ID;
use crate::instructions::{AuctionInstructionAccounts, to_program_error};
use ambient_auction_api::{
    InitBundleVerifierPageV2Accounts, InitBundleVerifierPageV2Args, InstructionAccounts,
};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

#[repr(transparent)]
pub struct InitBundleVerifierPageV2InstructionAccounts<'a>(
    InitBundleVerifierPageV2Accounts<'a, AccountInfo>,
);

impl<'a> TryFrom<&'a [AccountInfo]> for InitBundleVerifierPageV2InstructionAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let account_infos =
            InitBundleVerifierPageV2Accounts::try_from(accounts).map_err(to_program_error)?;

        if !account_infos.payer.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if !account_infos.payer.is_writable() || !account_infos.bundle_verifier_page.is_writable() {
            return Err(ProgramError::InvalidArgument);
        }

        if !account_infos
            .bundle_escrow
            .is_owned_by(&ambient_auction_api::ID)
        {
            return Err(ProgramError::InvalidAccountOwner);
        }

        if account_infos.system_program.key() != &SYSTEM_PROGRAM_ID {
            return Err(ProgramError::IncorrectProgramId);
        }

        Ok(Self(account_infos))
    }
}

impl<'a> AuctionInstructionAccounts<'a> for InitBundleVerifierPageV2InstructionAccounts<'a> {
    type Inner = InitBundleVerifierPageV2Accounts<'a, AccountInfo>;

    fn inner(&self) -> &Self::Inner {
        &self.0
    }

    fn to_account_metas(&'a self) -> impl Iterator<Item = AccountMeta<'a>> {
        self.inner().iter().map(AccountMeta::from)
    }
}

pub struct InitBundleVerifierPageV2Instruction<'a> {
    pub accounts: InitBundleVerifierPageV2InstructionAccounts<'a>,
    pub data: InitBundleVerifierPageV2Args,
}

impl<'a> TryFrom<(&'a [AccountInfo], &'a [u8])> for InitBundleVerifierPageV2Instruction<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [AccountInfo], &'a [u8])) -> Result<Self, Self::Error> {
        let (accounts, data) = value;

        Ok(Self {
            accounts: InitBundleVerifierPageV2InstructionAccounts::try_from(accounts)?,
            data: InitBundleVerifierPageV2Args::try_from(data)
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        })
    }
}
