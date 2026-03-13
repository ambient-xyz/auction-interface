use crate::instructions::{to_program_error, AuctionInstructionAccounts};
use ambient_auction_api::{InstructionAccounts, PostBundleResultV2Accounts, PostBundleResultV2Args};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

#[repr(transparent)]
pub struct PostBundleResultV2InstructionAccounts<'a>(PostBundleResultV2Accounts<'a, AccountInfo>);

impl<'a> TryFrom<&'a [AccountInfo]> for PostBundleResultV2InstructionAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let account_infos =
            PostBundleResultV2Accounts::try_from(accounts).map_err(to_program_error)?;

        if !account_infos.authority.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if !account_infos.bundle_escrow.is_owned_by(&ambient_auction_api::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        Ok(Self(account_infos))
    }
}

impl<'a> AuctionInstructionAccounts<'a> for PostBundleResultV2InstructionAccounts<'a> {
    type Inner = PostBundleResultV2Accounts<'a, AccountInfo>;

    fn inner(&self) -> &Self::Inner {
        &self.0
    }

    fn to_account_metas(&'a self) -> impl Iterator<Item = AccountMeta<'a>> {
        self.inner().iter().map(AccountMeta::from)
    }
}

pub struct PostBundleResultV2Instruction<'a> {
    pub accounts: PostBundleResultV2InstructionAccounts<'a>,
    pub data: PostBundleResultV2Args,
}

impl<'a> TryFrom<(&'a [AccountInfo], &'a [u8])> for PostBundleResultV2Instruction<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [AccountInfo], &'a [u8])) -> Result<Self, Self::Error> {
        let (accounts, data) = value;

        Ok(Self {
            accounts: PostBundleResultV2InstructionAccounts::try_from(accounts)?,
            data: PostBundleResultV2Args::try_from(data)
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        })
    }
}
