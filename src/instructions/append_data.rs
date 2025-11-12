use crate::instructions::AuctionInstructionAccounts;
use ambient_auction_api::{
    AppendDataAccounts as AppendDataAccountsInner, AppendDataArgs, InstructionAccounts,
};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

#[repr(transparent)]
pub struct AppendDataInstructionAccounts<'a>(AppendDataAccountsInner<'a, AccountInfo>);

impl<'a> TryFrom<&'a [AccountInfo]> for AppendDataInstructionAccounts<'a> {
    type Error = ProgramError;
    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [data_authority, data_account, system_program, ..] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        if !data_account.is_owned_by(&ambient_auction_api::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        Ok(AppendDataInstructionAccounts(AppendDataAccountsInner {
            data_authority,
            system_program,
            data_account,
        }))
    }
}

impl<'a> AuctionInstructionAccounts<'a> for AppendDataInstructionAccounts<'a> {
    type Inner = AppendDataAccountsInner<'a, AccountInfo>;
    fn inner(&self) -> &Self::Inner {
        &self.0
    }
    fn to_account_metas(&'a self) -> impl Iterator<Item = AccountMeta<'a>> {
        self.inner().iter().map(AccountMeta::from)
    }
}

pub struct AppendDataInstruction<'a> {
    pub accounts: AppendDataInstructionAccounts<'a>,
    pub data: AppendDataArgs,
    pub write_data: &'a [u8],
}

impl<'a> TryFrom<(&'a [AccountInfo], &'a [u8])> for AppendDataInstruction<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [AccountInfo], &'a [u8])) -> Result<Self, Self::Error> {
        let (accounts, data) = value;

        let (header, write_data) = data.split_at(std::mem::size_of::<AppendDataArgs>());

        Ok(Self {
            accounts: self::AppendDataInstructionAccounts::try_from(accounts)?,
            data: self::AppendDataArgs::try_from(header)
                .map_err(|_| ProgramError::InvalidInstructionData)?,
            write_data,
        })
    }
}
