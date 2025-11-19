use crate::instructions::AuctionInstructionAccounts;
use ambient_auction_api::error::AuctionError;
use ambient_auction_api::{InstructionAccounts, RequestJobAccounts, RequestJobArgs};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

#[repr(transparent)]
pub struct RequestJobInstructionAccounts<'a>(
    RequestJobAccounts<'a, AccountInfo, &'a [AccountInfo]>,
);

impl<'a> TryFrom<&'a [AccountInfo]> for RequestJobInstructionAccounts<'a> {
    type Error = ProgramError;
    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [
            payer,
            job_request,
            registry,
            input_data,
            system_program,
            config,
            bundle_auction_account_pairs @ ..,
        ] = accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        if !registry.is_owned_by(&ambient_auction_api::ID) {
            return Err(ProgramError::IllegalOwner);
        }

        let (last_bundle, bundle_auction_account_pairs) = bundle_auction_account_pairs
            .split_last()
            .ok_or(ProgramError::Custom(
                AuctionError::NotEnoughBundleAuctionAccounts.code(),
            ))?;

        if !config.is_owned_by(&ambient_auction_api::ID) {
            return Err(ProgramError::Custom(
                AuctionError::IllegalConfigOwner.code(),
            ));
        }

        Ok(RequestJobInstructionAccounts(RequestJobAccounts {
            payer,
            job_request,
            system_program,
            input_data,
            last_bundle,
            config,
            bundle_auction_account_pairs,
            registry,
        }))
    }
}

impl<'a> AuctionInstructionAccounts<'a> for RequestJobInstructionAccounts<'a> {
    type Inner = RequestJobAccounts<'a, AccountInfo, &'a [AccountInfo]>;
    fn inner(&self) -> &Self::Inner {
        &self.0
    }
    fn to_account_metas(&'a self) -> impl Iterator<Item = AccountMeta<'a>> {
        self.inner().iter().map(AccountMeta::from)
    }
}

pub struct RequestJobInstruction<'a> {
    pub accounts: RequestJobInstructionAccounts<'a>,
    pub data: RequestJobArgs,
}

impl<'a> TryFrom<(&'a [AccountInfo], &'a [u8])> for RequestJobInstruction<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [AccountInfo], &'a [u8])) -> Result<Self, Self::Error> {
        let (accounts, data) = value;

        Ok(Self {
            accounts: self::RequestJobInstructionAccounts::try_from(accounts)?,
            data: self::RequestJobArgs::try_from(data)
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        })
    }
}
