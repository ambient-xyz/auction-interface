use crate::SYSTEM_PROGRAM_ID;
use crate::instructions::{AuctionInstructionAccounts, to_program_error};
use ambient_auction_api::{
    DisputeBundleVerificationV2Accounts, DisputeBundleVerificationV2Args, InstructionAccounts,
};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

#[repr(transparent)]
pub struct DisputeBundleVerificationV2InstructionAccounts<'a>(
    DisputeBundleVerificationV2Accounts<'a, AccountInfo>,
);

impl<'a> TryFrom<&'a [AccountInfo]> for DisputeBundleVerificationV2InstructionAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let account_infos =
            DisputeBundleVerificationV2Accounts::try_from(accounts).map_err(to_program_error)?;

        if !account_infos.dispute_payer.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if !account_infos.dispute_payer.is_writable()
            || !account_infos.bundle_escrow.is_writable()
            || !account_infos.bundle_verification_dispute.is_writable()
        {
            return Err(ProgramError::InvalidArgument);
        }

        if !account_infos
            .bundle_escrow
            .is_owned_by(&ambient_auction_api::ID)
        {
            return Err(ProgramError::InvalidAccountOwner);
        }

        super::validate_config_policy_owner(account_infos.config_policy)?;

        if account_infos.system_program.key() != &SYSTEM_PROGRAM_ID {
            return Err(ProgramError::IncorrectProgramId);
        }

        Ok(Self(account_infos))
    }
}

impl<'a> AuctionInstructionAccounts<'a> for DisputeBundleVerificationV2InstructionAccounts<'a> {
    type Inner = DisputeBundleVerificationV2Accounts<'a, AccountInfo>;

    fn inner(&self) -> &Self::Inner {
        &self.0
    }

    fn to_account_metas(&'a self) -> impl Iterator<Item = AccountMeta<'a>> {
        self.inner().iter().map(AccountMeta::from)
    }
}

pub struct DisputeBundleVerificationV2Instruction<'a> {
    pub accounts: DisputeBundleVerificationV2InstructionAccounts<'a>,
    pub data: DisputeBundleVerificationV2Args,
}

impl<'a> TryFrom<(&'a [AccountInfo], &'a [u8])> for DisputeBundleVerificationV2Instruction<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [AccountInfo], &'a [u8])) -> Result<Self, Self::Error> {
        let (accounts, data) = value;

        Ok(Self {
            accounts: DisputeBundleVerificationV2InstructionAccounts::try_from(accounts)?,
            data: DisputeBundleVerificationV2Args::try_from(data)
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        })
    }
}
