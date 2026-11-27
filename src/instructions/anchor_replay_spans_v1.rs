use crate::SYSTEM_PROGRAM_ID;
use crate::instructions::{AuctionInstructionAccounts, to_program_error};
use ambient_auction_api::{
    AnchorReplaySpansV1Accounts, AnchorReplaySpansV1Args, InstructionAccounts,
};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

#[repr(transparent)]
pub struct AnchorReplaySpansV1InstructionAccounts<'a>(AnchorReplaySpansV1Accounts<'a, AccountInfo>);

impl<'a> TryFrom<&'a [AccountInfo]> for AnchorReplaySpansV1InstructionAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let account_infos =
            AnchorReplaySpansV1Accounts::try_from(accounts).map_err(to_program_error)?;

        if !account_infos.miner_node.is_signer() || !account_infos.payer.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if !account_infos.payer.is_writable() || !account_infos.replay_anchor.is_writable() {
            return Err(ProgramError::InvalidArgument);
        }

        if account_infos.system_program.key() != &SYSTEM_PROGRAM_ID {
            return Err(ProgramError::IncorrectProgramId);
        }

        Ok(Self(account_infos))
    }
}

impl<'a> AuctionInstructionAccounts<'a> for AnchorReplaySpansV1InstructionAccounts<'a> {
    type Inner = AnchorReplaySpansV1Accounts<'a, AccountInfo>;

    fn inner(&self) -> &Self::Inner {
        &self.0
    }

    fn to_account_metas(&'a self) -> impl Iterator<Item = AccountMeta<'a>> {
        self.inner().iter().map(AccountMeta::from)
    }
}

pub struct AnchorReplaySpansV1Instruction<'a> {
    pub accounts: AnchorReplaySpansV1InstructionAccounts<'a>,
    pub data: AnchorReplaySpansV1Args,
}

impl<'a> TryFrom<(&'a [AccountInfo], &'a [u8])> for AnchorReplaySpansV1Instruction<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [AccountInfo], &'a [u8])) -> Result<Self, Self::Error> {
        let (accounts, data) = value;

        Ok(Self {
            accounts: AnchorReplaySpansV1InstructionAccounts::try_from(accounts)?,
            data: AnchorReplaySpansV1Args::try_from(data)
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        })
    }
}
