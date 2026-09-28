use crate::instructions::{
    AuctionInstructionAccounts, to_program_error, validate_config_policy_owner,
};
use ambient_auction_api::{InstructionAccounts, SetConfigPolicyV2Accounts, SetConfigPolicyV2Args, SetConfigPolicySmallV3Args, ConfigPolicyV2PatchKind};
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
    pub small_credit_enabled: Option<u8>,
}

impl<'a> TryFrom<(&'a [AccountInfo], &'a [u8])> for SetConfigPolicyV2Instruction<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [AccountInfo], &'a [u8])) -> Result<Self, Self::Error> {
        let (accounts, data) = value;

        let (data, small_credit_enabled) = if data.len() == std::mem::size_of::<SetConfigPolicySmallV3Args>() {
            let args = SetConfigPolicySmallV3Args::try_from(data)
                .map_err(|_| ProgramError::InvalidInstructionData)?;
            if args.patch_kind == ConfigPolicyV2PatchKind::DISPUTE_SETTINGS {
                return Err(ProgramError::InvalidInstructionData);
            }
            (SetConfigPolicyV2Args {
                patch_kind: args.patch_kind,
                authority_kind: args.authority_kind,
                authority_index: args.authority_index,
                v2_verifiers_per_auction: args.v2_verifiers_per_auction,
                v2_verifier_quorum: args.v2_verifier_quorum,
                _reserved0: [0; 3],
                tier: args.tier,
                policy_flags: args.policy_flags,
                max_auction_credits_per_update: args.max_auction_credits_per_update,
                missed_verification_dispute_window_slots: 0,
                dispute_verification_window_slots: 0,
                paid_verification_dispute_window_slots: 0,
                paid_verification_dispute_bond_lamports: 0,
                authority: args.authority,
                tier_config: args.tier_config,
            }, Some(args.small_credit_enabled))
        } else {
            let args = SetConfigPolicyV2Args::try_from(data)
                .map_err(|_| ProgramError::InvalidInstructionData)?;
            if matches!(args.patch_kind, ConfigPolicyV2PatchKind::SMALL_CREDIT_SETTINGS | ConfigPolicyV2PatchKind::SMALL_CREDIT_SLASH_AUTHORITY) {
                return Err(ProgramError::InvalidInstructionData);
            }
            (args, None)
        };
        Ok(Self {
            accounts: SetConfigPolicyV2InstructionAccounts::try_from(accounts)?,
            data,
            small_credit_enabled,
        })
    }
}
