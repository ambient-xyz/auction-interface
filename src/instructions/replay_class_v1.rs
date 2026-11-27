use crate::SYSTEM_PROGRAM_ID;
use crate::instructions::{AuctionInstructionAccounts, to_program_error};
use ambient_auction_api::{
    InstructionAccounts, JoinReplayClassV1Accounts, JoinReplayClassV1Args,
    LeaveReplayClassV1Accounts, LeaveReplayClassV1Args, RemoveReplayClassMemberV1Accounts,
    RemoveReplayClassMemberV1Args, SelectReplayVerifiersV1Accounts, SelectReplayVerifiersV1Args,
};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

fn owned_by_program(account: &AccountInfo) -> Result<(), ProgramError> {
    if !account.is_owned_by(&ambient_auction_api::ID) {
        return Err(ProgramError::InvalidAccountOwner);
    }
    Ok(())
}

macro_rules! replay_class_instruction {
    ($accounts:ident, $inner:ident, $instruction:ident, $args:ident, |$infos:ident| $checks:block) => {
        #[repr(transparent)]
        pub struct $accounts<'a>($inner<'a, AccountInfo>);

        impl<'a> TryFrom<&'a [AccountInfo]> for $accounts<'a> {
            type Error = ProgramError;

            fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
                let $infos = $inner::try_from(accounts).map_err(to_program_error)?;
                $checks
                Ok(Self($infos))
            }
        }

        impl<'a> AuctionInstructionAccounts<'a> for $accounts<'a> {
            type Inner = $inner<'a, AccountInfo>;

            fn inner(&self) -> &Self::Inner {
                &self.0
            }

            fn to_account_metas(&'a self) -> impl Iterator<Item = AccountMeta<'a>> {
                self.inner().iter().map(AccountMeta::from)
            }
        }

        pub struct $instruction<'a> {
            pub accounts: $accounts<'a>,
            pub data: $args,
        }

        impl<'a> TryFrom<(&'a [AccountInfo], &'a [u8])> for $instruction<'a> {
            type Error = ProgramError;

            fn try_from(value: (&'a [AccountInfo], &'a [u8])) -> Result<Self, Self::Error> {
                let (accounts, data) = value;
                Ok(Self {
                    accounts: $accounts::try_from(accounts)?,
                    data: $args::try_from(data).map_err(|_| ProgramError::InvalidInstructionData)?,
                })
            }
        }
    };
}

replay_class_instruction!(
    JoinReplayClassV1InstructionAccounts,
    JoinReplayClassV1Accounts,
    JoinReplayClassV1Instruction,
    JoinReplayClassV1Args,
    |infos| {
        if !infos.node.is_signer() || !infos.payer.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }
        if !infos.payer.is_writable() || !infos.replay_class.is_writable() {
            return Err(ProgramError::InvalidArgument);
        }
        super::validate_config_policy_owner(infos.config_policy)?;
        if infos.system_program.key() != &SYSTEM_PROGRAM_ID {
            return Err(ProgramError::IncorrectProgramId);
        }
    }
);

replay_class_instruction!(
    LeaveReplayClassV1InstructionAccounts,
    LeaveReplayClassV1Accounts,
    LeaveReplayClassV1Instruction,
    LeaveReplayClassV1Args,
    |infos| {
        if !infos.node.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }
        if !infos.replay_class.is_writable() {
            return Err(ProgramError::InvalidArgument);
        }
        owned_by_program(infos.replay_class)?;
    }
);

replay_class_instruction!(
    RemoveReplayClassMemberV1InstructionAccounts,
    RemoveReplayClassMemberV1Accounts,
    RemoveReplayClassMemberV1Instruction,
    RemoveReplayClassMemberV1Args,
    |infos| {
        if !infos.admin.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }
        if !infos.replay_class.is_writable() {
            return Err(ProgramError::InvalidArgument);
        }
        owned_by_program(infos.replay_class)?;
        super::validate_config_policy_owner(infos.config_policy)?;
    }
);

replay_class_instruction!(
    SelectReplayVerifiersV1InstructionAccounts,
    SelectReplayVerifiersV1Accounts,
    SelectReplayVerifiersV1Instruction,
    SelectReplayVerifiersV1Args,
    |infos| {
        if !infos.bundle_escrow.is_writable() {
            return Err(ProgramError::InvalidArgument);
        }
        owned_by_program(infos.bundle_escrow)?;
        super::validate_current_bundle_escrow(infos.bundle_escrow)?;
        super::validate_verifier_selection_accounts(infos.auction_verifiers, infos.slot_hashes)?;
        owned_by_program(infos.replay_class)?;
        super::validate_config_policy_owner(infos.config_policy)?;
        if let Some(dispute) = infos.bundle_verification_dispute {
            if !dispute.is_writable() {
                return Err(ProgramError::InvalidArgument);
            }
            owned_by_program(dispute)?;
        }
    }
);
