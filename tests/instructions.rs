mod support;

use ambient_auction_api::{error::AuctionError, *};
use ambient_auction_interface::instructions::*;
use bytemuck::Zeroable;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError};
use support::{Account, with_accounts};

const VOTE: [u8; 32] = five8_const::decode_32_const("Vote111111111111111111111111111111111111111");
const SYSVAR: [u8; 32] =
    five8_const::decode_32_const("Sysvar1111111111111111111111111111111111111");
const HASHES: [u8; 32] =
    five8_const::decode_32_const("SysvarS1otHashes111111111111111111111111111");
const INSTRUCTIONS: [u8; 32] =
    five8_const::decode_32_const("Sysvar1nstructions1111111111111111111111111");

type Meta = ([u8; 32], bool, bool);
type Parse = fn(&[AccountInfo], &[u8]) -> Result<Vec<Meta>, ProgramError>;

struct Case {
    accounts: Vec<Account>,
    data: Vec<u8>,
    parse: Parse,
    minimum: usize,
}

impl Case {
    fn parse(&self) -> Result<Vec<Meta>, ProgramError> {
        with_accounts(&self.accounts, |accounts| {
            (self.parse)(accounts, &self.data)
        })
    }

    fn assert_valid(&self) {
        let expected: Vec<_> = self
            .accounts
            .iter()
            .map(|account| (account.key, account.writable, account.signer))
            .collect();
        assert_eq!(self.parse(), Ok(expected), "account order and privileges");
    }
}

fn policy() -> Account {
    Account::owned(
        ID,
        ProgramError::Custom(AuctionError::IllegalConfigPolicyV2Owner.code()),
    )
}

fn owned() -> Account {
    Account::owned(ID, ProgramError::InvalidAccountOwner)
}

fn escrow() -> Account {
    let mut account = owned().writable();
    account.data = vec![0; BundleEscrowV2::LEN_V4];
    let state = BundleEscrowV2 {
        reward_tier: RequestTier::Standard as u64,
        ..BundleEscrowV2::default()
    };
    assert!(state.write_bytes_with_layout(&mut account.data, AccountLayoutVersion::V4));
    BundleEscrowV2::from_bytes_mut(&mut account.data)
        .unwrap()
        .lifecycle_mut()
        .unwrap()
        .expected_page_count = 1;
    account
}

fn page() -> Account {
    let mut account = owned();
    account.data = vec![0; BundleVerifierPageV2::LEN_V4];
    assert!(
        BundleVerifierPageV2::default()
            .write_bytes_with_layout(&mut account.data, AccountLayoutVersion::V4)
    );
    account
}

fn system() -> Account {
    Account::address([0; 32], ProgramError::IncorrectProgramId)
}

fn selection_sysvar(key: [u8; 32]) -> Account {
    Account {
        owner: SYSVAR,
        owner_error: Some(ProgramError::InvalidAccountOwner),
        ..Account::address(key, ProgramError::InvalidArgument)
    }
}

fn vote() -> Account {
    Account::owned(VOTE, ProgramError::InvalidAccountOwner)
}

fn vote_program() -> Account {
    Account::address(VOTE, ProgramError::IncorrectProgramId)
}

fn instructions_sysvar() -> Account {
    Account::address(INSTRUCTIONS, ProgramError::UnsupportedSysvar)
}

fn token_program() -> Account {
    Account {
        executable: true,
        ..Account::default()
    }
}

fn opening() -> Vec<Account> {
    vec![
        Account::default().signed().writable(),
        Account::default().writable(),
        policy(),
        system(),
    ]
}

fn settlement() -> Vec<Account> {
    vec![Account::default().signed(), escrow(), policy(), vote()]
}

// This match intentionally has no wildcard. An API addition must supply its parser
// and a passing case here, or be explicitly identified as disabled by the program.
fn case(instruction: AuctionInstruction) -> Option<Case> {
    macro_rules! case {
        ($instruction:ident, $args:ident, $accounts:expr, $minimum:expr) => {
            Case {
                accounts: $accounts,
                data: bytemuck::bytes_of(&$args::zeroed()).to_vec(),
                minimum: $minimum,
                parse: |accounts, data| {
                    let instruction = $instruction::try_from((accounts, data))?;
                    Ok(instruction
                        .accounts
                        .to_account_metas()
                        .map(|meta| (*meta.pubkey, meta.is_writable, meta.is_signer))
                        .collect())
                },
            }
        };
    }
    use AuctionInstruction::*;
    let mut case = match instruction {
        RequestJob | PlaceBid | EndAuction | CloseBid | SubmitJobOutput | CancelBundle
        | InitBundle | SubmitValidation | RevealBid | CloseRequest | AppendData | InitConfig
        | OpenBundleEscrowV2 => return None,
        AnchorReplaySpansV1 => case!(
            AnchorReplaySpansV1Instruction,
            AnchorReplaySpansV1Args,
            vec![
                Account::default().signed(),
                Account::default().signed().writable(),
                Account::default().writable(),
                system()
            ],
            4
        ),
        OpenBundleEscrowV4 => case!(
            OpenBundleEscrowV4Instruction,
            OpenBundleEscrowV4Args,
            opening(),
            4
        ),
        OpenPricedBundleEscrowV4 => case!(
            OpenPricedBundleEscrowV4Instruction,
            OpenPricedBundleEscrowV4Args,
            opening(),
            4
        ),
        OpenSmallBundleEscrowV4 => case!(
            OpenSmallBundleEscrowV4Instruction,
            OpenSmallBundleEscrowV4Args,
            opening(),
            4
        ),
        CommitAuctionSettlementV2 => case!(
            CommitAuctionSettlementV2Instruction,
            CommitAuctionSettlementV2Args,
            settlement(),
            4
        ),
        CommitAuctionSettlementV3 => case!(
            CommitAuctionSettlementV3Instruction,
            CommitAuctionSettlementV3Args,
            settlement(),
            4
        ),
        CommitSmallAuctionSettlementV4 => case!(
            CommitSmallAuctionSettlementV4Instruction,
            CommitSmallAuctionSettlementV4Args,
            vec![Account::default().signed(), escrow(), policy()],
            3
        ),
        PostBundleResultV2 => case!(
            PostBundleResultV2Instruction,
            PostBundleResultV2Args,
            vec![Account::default().signed(), escrow(), policy()],
            3
        ),
        FinalizeBundleVerificationV2 => case!(
            FinalizeBundleVerificationV2Instruction,
            FinalizeBundleVerificationV2Args,
            vec![
                Account::default().signed(),
                escrow(),
                Account::default(),
                Account::default(),
                instructions_sysvar(),
                policy()
            ],
            6
        ),
        ClaimWinnerLstakeV2 => case!(
            ClaimWinnerLstakeV2Instruction,
            ClaimWinnerLstakeV2Args,
            vec![
                escrow(),
                vote().writable(),
                vote_program(),
                Account::default().signed(),
                policy()
            ],
            5
        ),
        ClaimVerifierLstakeV2 => case!(
            ClaimVerifierLstakeV2Instruction,
            ClaimVerifierLstakeV2Args,
            vec![
                escrow(),
                vote().writable(),
                vote_program(),
                Account::default().signed(),
                policy(),
                page().writable()
            ],
            5
        ),
        ExpireBundleEscrowV2 => case!(
            ExpireBundleEscrowV2Instruction,
            ExpireBundleEscrowV2Args,
            vec![escrow(), Account::default(), policy()],
            3
        ),
        InitConfigPolicyV2 => case!(
            InitConfigPolicyV2Instruction,
            InitConfigPolicyV2Args,
            vec![
                Account::default().signed().writable(),
                Account::default().writable(),
                system()
            ],
            3
        ),
        SetConfigPolicyV2 => case!(
            SetConfigPolicyV2Instruction,
            SetConfigPolicyV2Args,
            vec![Account::default().signed(), policy().writable()],
            2
        ),
        InitBundleVerifierPageV2 => case!(
            InitBundleVerifierPageV2Instruction,
            InitBundleVerifierPageV2Args,
            vec![
                Account::default().signed().writable(),
                escrow(),
                Account::default().writable(),
                system()
            ],
            4
        ),
        DisputeBundleVerificationV2 => case!(
            DisputeBundleVerificationV2Instruction,
            DisputeBundleVerificationV2Args,
            vec![
                Account::default().signed().writable(),
                escrow(),
                Account::default().writable(),
                Account::default(),
                policy(),
                system()
            ],
            6
        ),
        SelectBundleVerifiersV2 => case!(
            SelectBundleVerifiersV2Instruction,
            SelectBundleVerifiersV2Args,
            vec![
                escrow(),
                selection_sysvar(AUCTION_VERIFIERS_HISTORY_ID),
                selection_sysvar(HASHES)
            ],
            3
        ),
        SlashSmallCredits => case!(
            SlashSmallCreditsInstruction,
            SlashSmallCreditsArgs,
            vec![
                Account::default().signed(),
                policy().writable(),
                Account::default().writable(),
                Account::default().writable(),
                token_program()
            ],
            5
        ),
        CloseBundleVerifierPageV4 => case!(
            CloseBundleVerifierPageV4Instruction,
            CloseBundleVerifierPageV4Args,
            vec![Account::default().writable(), escrow(), page().writable()],
            3
        ),
        AuthorizeBundleDisputeEvidenceV4 => case!(
            AuthorizeBundleDisputeEvidenceV4Instruction,
            AuthorizeBundleDisputeEvidenceV4Args,
            vec![
                Account::default().signed(),
                owned(),
                owned().writable(),
                instructions_sysvar()
            ],
            4
        ),
        ClaimSmallCreditsV4 => case!(
            ClaimSmallCreditsV4Instruction,
            ClaimSmallCreditsV4Args,
            vec![
                escrow(),
                policy(),
                Account::default().writable(),
                Account::default().writable(),
                token_program()
            ],
            5
        ),
        PostBundlePricing => case!(
            PostBundlePricingInstruction,
            PostBundlePricingArgs,
            vec![Account::default().signed(), escrow(), page().writable()],
            3
        ),
        SealBundlePricing => case!(
            SealBundlePricingInstruction,
            SealBundlePricingArgs,
            vec![Account::default().signed(), escrow(), page()],
            3
        ),
    };
    for (index, account) in case.accounts.iter_mut().enumerate() {
        if account.key_error.is_none() {
            account.key = [index as u8 + 10; 32];
        }
    }
    Some(case)
}

fn cases() -> impl Iterator<Item = (AuctionInstruction, Case)> {
    (0..=u8::MAX).filter_map(|tag| {
        let instruction = AuctionInstruction::try_from(tag).ok()?;
        Some((instruction, case(instruction)?))
    })
}

fn assert_accounts(case: &Case) {
    case.assert_valid();
    for (index, account) in case.accounts.iter().enumerate() {
        let mut mutations: Vec<(&str, Account, ProgramError)> = Vec::new();
        if account.signer {
            let mut changed = account.clone();
            changed.signer = false;
            mutations.push(("signature", changed, ProgramError::MissingRequiredSignature));
        }
        if account.writable {
            let mut changed = account.clone();
            changed.writable = false;
            mutations.push(("writable", changed, ProgramError::InvalidArgument));
        }
        if let Some(error) = &account.owner_error {
            let mut changed = account.clone();
            changed.owner = [99; 32];
            mutations.push(("owner", changed, *error));
        }
        if let Some(error) = &account.key_error {
            let mut changed = account.clone();
            changed.key = [99; 32];
            mutations.push(("address", changed, *error));
        }
        if account.executable {
            let mut changed = account.clone();
            changed.executable = false;
            mutations.push(("executable", changed, ProgramError::IncorrectProgramId));
        }
        for (field, changed, error) in mutations {
            let mut accounts = case.accounts.clone();
            accounts[index] = changed;
            assert_eq!(
                with_accounts(&accounts, |accounts| (case.parse)(accounts, &case.data)),
                Err(error),
                "account {index}: {field}"
            );
        }
    }
}

#[test]
fn every_usable_instruction_preserves_account_order_and_rejects_invalid_accounts() {
    for (instruction, case) in cases() {
        eprintln!("{instruction:?}");
        assert_accounts(&case);
        for count in 0..case.minimum {
            assert_eq!(
                with_accounts(&case.accounts[..count], |accounts| (case.parse)(
                    accounts, &case.data
                )),
                Err(ProgramError::Custom(AuctionError::NotEnoughAccounts.code())),
                "{instruction:?}: {count} accounts"
            );
        }
    }
}

#[test]
fn every_usable_instruction_rejects_invalid_payload_lengths() {
    for (instruction, mut case) in cases() {
        case.assert_valid();
        let original = case.data.clone();
        if !original.is_empty() {
            case.data.pop();
            assert_eq!(
                case.parse(),
                Err(ProgramError::InvalidInstructionData),
                "{instruction:?}: short payload"
            );
        }
        case.data = original;
        case.data.push(0);
        assert_eq!(
            case.parse(),
            Err(ProgramError::InvalidInstructionData),
            "{instruction:?}: long payload"
        );
    }
}

#[test]
fn optional_accounts_keep_their_order_and_validation() {
    use AuctionInstruction::*;
    let mut commit = case(CommitAuctionSettlementV2).unwrap();
    commit
        .accounts
        .push(selection_sysvar(AUCTION_VERIFIERS_HISTORY_ID));
    assert_eq!(commit.parse(), Err(ProgramError::NotEnoughAccountKeys));
    commit.accounts.push(selection_sysvar(HASHES));
    assert_accounts(&commit);
    commit.accounts.push(Account::default());
    assert_eq!(commit.parse(), Err(ProgramError::InvalidArgument));

    let mut post = case(PostBundleResultV2).unwrap();
    post.accounts.push(page().writable());
    assert_accounts(&post);
    post.accounts.push(owned());
    assert_accounts(&post);

    let mut dispute = case(DisputeBundleVerificationV2).unwrap();
    dispute
        .accounts
        .push(selection_sysvar(AUCTION_VERIFIERS_HISTORY_ID));
    assert_accounts(&dispute);

    let mut selection = case(SelectBundleVerifiersV2).unwrap();
    selection.accounts.push(Account::default().writable());
    assert_accounts(&selection);
    selection.accounts.push(Account::default().writable());
    assert_accounts(&selection);
    selection.accounts[4].executable = true;
    assert_eq!(selection.parse(), Err(ProgramError::InvalidArgument));
    selection.accounts[4].executable = false;
    selection.accounts.push(Account::default());
    assert_eq!(
        selection.parse(),
        Err(ProgramError::Custom(
            AuctionError::InvalidVerifierSelectionAccounts.code()
        ))
    );

    for instruction in [FinalizeBundleVerificationV2, ExpireBundleEscrowV2] {
        let mut case = case(instruction).unwrap();
        case.accounts.extend([
            Account {
                key: [71; 32],
                ..Account::default()
            },
            Account {
                key: [72; 32],
                ..Account::default()
            },
        ]);
        case.assert_valid();
    }
    for instruction in [ClaimVerifierLstakeV2, SealBundlePricing] {
        let mut case = case(instruction).unwrap();
        let mut extra = page();
        extra.key = [77; 32];
        extra.writable = instruction == ClaimVerifierLstakeV2;
        case.accounts.push(extra);
        assert_accounts(&case);
    }
}

#[test]
fn selection_sysvars_must_be_readonly_and_unsigned() {
    for instruction in [
        AuctionInstruction::SelectBundleVerifiersV2,
        AuctionInstruction::CommitAuctionSettlementV2,
        AuctionInstruction::DisputeBundleVerificationV2,
    ] {
        let mut case = case(instruction).unwrap();
        if instruction == AuctionInstruction::CommitAuctionSettlementV2 {
            case.accounts.extend([
                selection_sysvar(AUCTION_VERIFIERS_HISTORY_ID),
                selection_sysvar(HASHES),
            ]);
        } else if instruction == AuctionInstruction::DisputeBundleVerificationV2 {
            case.accounts
                .push(selection_sysvar(AUCTION_VERIFIERS_HISTORY_ID));
        }
        case.assert_valid();
        for index in 0..case.accounts.len() {
            if case.accounts[index].owner == SYSVAR {
                case.accounts[index].signer = true;
                assert_eq!(case.parse(), Err(ProgramError::InvalidArgument));
                case.accounts[index].signer = false;
                case.accounts[index].writable = true;
                assert_eq!(case.parse(), Err(ProgramError::InvalidArgument));
                case.accounts[index].writable = false;
            }
        }
    }
}

#[test]
fn small_payloads_keep_their_fields_and_reject_other_forms() {
    let mut init = case(AuctionInstruction::InitConfigPolicyV2).unwrap();
    let mut args = InitConfigPolicySmallV3Args::zeroed();
    args.initial_admin_authority = [41; 32].into();
    init.data = bytemuck::bytes_of(&args).to_vec();
    assert_accounts(&init);
    with_accounts(&init.accounts, |accounts| {
        let instruction =
            InitConfigPolicyV2Instruction::try_from((accounts, init.data.as_slice())).unwrap();
        assert_eq!(
            instruction.data.initial_admin_authority,
            args.initial_admin_authority
        );
        assert_eq!(instruction.data.paid_verification_dispute_window_slots, 0);
    });

    let mut set = case(AuctionInstruction::SetConfigPolicyV2).unwrap();
    let mut args = SetConfigPolicySmallV3Args::zeroed();
    args.patch_kind = ConfigPolicyV2PatchKind::SMALL_CREDIT_SETTINGS;
    args.small_credit_enabled = 1;
    args.authority = [42; 32].into();
    set.data = bytemuck::bytes_of(&args).to_vec();
    assert_accounts(&set);
    with_accounts(&set.accounts, |accounts| {
        let instruction =
            SetConfigPolicyV2Instruction::try_from((accounts, set.data.as_slice())).unwrap();
        assert_eq!(instruction.small_credit_enabled, Some(1));
        assert_eq!(instruction.data.authority, args.authority);
    });
    args.patch_kind = ConfigPolicyV2PatchKind::DISPUTE_SETTINGS;
    set.data = bytemuck::bytes_of(&args).to_vec();
    assert_eq!(set.parse(), Err(ProgramError::InvalidInstructionData));
    let mut full_args = SetConfigPolicyV2Args::zeroed();
    for kind in [
        ConfigPolicyV2PatchKind::SMALL_CREDIT_SETTINGS,
        ConfigPolicyV2PatchKind::SMALL_CREDIT_SLASH_AUTHORITY,
    ] {
        full_args.patch_kind = kind;
        set.data = bytemuck::bytes_of(&full_args).to_vec();
        assert_eq!(set.parse(), Err(ProgramError::InvalidInstructionData));
    }

    let mut post = case(AuctionInstruction::PostBundleResultV2).unwrap();
    let mut args = PostBundleResultV3Args::zeroed();
    args.input_tokens[0] = 123;
    post.data = bytemuck::bytes_of(&args).to_vec();
    assert_eq!(post.parse(), Err(ProgramError::InvalidInstructionData));
    {
        let mut state = BundleEscrowV2::from_bytes_mut(&mut post.accounts[1].data).unwrap();
        state.reward_tier = RequestTier::Small as u64;
        state.lifecycle_mut().unwrap().small_credit_mint = [43; 32].into();
    }
    assert_accounts(&post);
    with_accounts(&post.accounts, |accounts| {
        let instruction =
            PostBundleResultV2Instruction::try_from((accounts, post.data.as_slice())).unwrap();
        assert_eq!(instruction.input_tokens, Some(args.input_tokens));
    });
    post.data = bytemuck::bytes_of(&args.post).to_vec();
    assert_eq!(post.parse(), Err(ProgramError::InvalidInstructionData));
}

#[test]
fn supported_account_layouts_remain_usable() {
    use AuctionInstruction::*;
    for instruction in [
        CommitAuctionSettlementV2,
        PostBundleResultV2,
        FinalizeBundleVerificationV2,
        InitBundleVerifierPageV2,
        ExpireBundleEscrowV2,
    ] {
        let mut case = case(instruction).unwrap();
        let index = usize::from(instruction != ExpireBundleEscrowV2);
        let mut data = vec![0; BundleEscrowV2::LEN_V3];
        let state = BundleEscrowV2 {
            reward_tier: RequestTier::Small as u64,
            ..BundleEscrowV2::default()
        };
        assert!(state.write_bytes_with_layout(&mut data, AccountLayoutVersion::V3));
        BundleEscrowV2::from_bytes_mut(&mut data)
            .unwrap()
            .small_v3_mut()
            .unwrap()
            .mint = [44; 32].into();
        case.accounts[index].data = data;
        if instruction == PostBundleResultV2 {
            case.data = bytemuck::bytes_of(&PostBundleResultV3Args::zeroed()).to_vec();
        }
        // V3 page creation reads the escrow without updating its V4 lifecycle fields.
        if instruction == InitBundleVerifierPageV2 {
            case.accounts[index].writable = false;
        }
        case.assert_valid();
    }
    for (instruction, mut case) in cases() {
        for index in 0..case.accounts.len() {
            if case.accounts[index].data.len() == BundleEscrowV2::LEN_V4 {
                let original = case.accounts[index].data.clone();
                case.accounts[index].data.pop();
                assert_eq!(
                    case.parse(),
                    Err(ProgramError::Custom(
                        AuctionError::InvalidBundleEscrowV2State.code()
                    )),
                    "{instruction:?}: truncated escrow"
                );
                case.accounts[index].data = original;
                BundleEscrowV2::from_bytes_mut(&mut case.accounts[index].data)
                    .unwrap()
                    .lifecycle_mut()
                    .unwrap()
                    .expected_page_count = 0;
                assert_eq!(
                    case.parse(),
                    Err(ProgramError::Custom(
                        AuctionError::InvalidVerifierPageV2Input.code()
                    )),
                    "{instruction:?}: missing page count"
                );
            }
        }
    }
}
