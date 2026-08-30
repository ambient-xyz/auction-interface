use crate::instructions::{to_program_error, AuctionInstructionAccounts};
use ambient_auction_api::error::AuctionError;
use ambient_auction_api::{CloseRequestAccounts, CloseRequestArgs, InstructionAccounts};
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::AccountMeta;
use pinocchio::program_error::ProgramError;

#[repr(transparent)]
pub struct CloseRequestInstructionAccounts<'a>(CloseRequestAccounts<'a, AccountInfo>);

impl<'a> TryFrom<&'a [AccountInfo]> for CloseRequestInstructionAccounts<'a> {
    type Error = ProgramError;
    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let account_infos = CloseRequestAccounts::try_from(accounts).map_err(to_program_error)?;

        let CloseRequestAccounts {
            request_authority,
            job_request,
            bundle_payer: _,
            bundle,
            registry,
            auction,
            auction_payer: _,
            child_bundle: _,
            child_auction: _,
            child_bundle_payer: _,
        } = account_infos;

        if !bundle.is_owned_by(&ambient_auction_api::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        if !job_request.is_owned_by(&ambient_auction_api::ID) {
            return Err(ProgramError::IllegalOwner);
        }

        if !registry.is_owned_by(&ambient_auction_api::ID) {
            return Err(ProgramError::Custom(AuctionError::InvalidRegistry.code()));
        }

        if !request_authority.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // The polarity of this check was inverted: it rejected an auction that *is* owned by the
        // auction program and accepted one that is not. That is the exact opposite of the intent and
        // of the idiom used by every other account here and in every sibling instruction
        // (`if !x.is_owned_by(&ID) { return Err(..) }`), and the chosen error — `IncorrectAuction` —
        // only makes sense as "this account is not one of our auctions".
        //
        // Consequences of the old form: a legitimate, program-owned auction could never be passed, so
        // the guard could only ever be satisfied by an account this program does not own — including
        // an account fabricated by an attacker under a program they control. Any state the handler
        // subsequently reads from `auction` would then be attacker-chosen.
        //
        // The client always supplies an existing auction here: `close_request` in
        // `auction-client/src/sdk/instructions.rs` derives it with `find_auction(bundle_key)` and marks
        // it writable, so requiring program ownership does not reject any call the SDK can build.
        if !auction.is_owned_by(&ambient_auction_api::ID) {
            return Err(ProgramError::Custom(AuctionError::IncorrectAuction.code()));
        }

        Ok(CloseRequestInstructionAccounts(account_infos))
    }
}

impl<'a> AuctionInstructionAccounts<'a> for CloseRequestInstructionAccounts<'a> {
    type Inner = CloseRequestAccounts<'a, AccountInfo>;
    fn inner(&self) -> &Self::Inner {
        &self.0
    }
    fn to_account_metas(&'a self) -> impl Iterator<Item = AccountMeta<'a>> {
        self.inner().iter().map(AccountMeta::from)
    }
}

pub struct CloseRequestInstruction<'a> {
    pub accounts: CloseRequestInstructionAccounts<'a>,
    pub data: CloseRequestArgs,
}

impl<'a> TryFrom<(&'a [AccountInfo], &'a [u8])> for CloseRequestInstruction<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [AccountInfo], &'a [u8])) -> Result<Self, Self::Error> {
        let (accounts, data) = value;

        Ok(Self {
            accounts: self::CloseRequestInstructionAccounts::try_from(accounts)?,
            data: self::CloseRequestArgs::try_from(data)
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        })
    }
}
