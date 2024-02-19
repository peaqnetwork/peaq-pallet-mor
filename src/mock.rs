//! Runtime mockup with this pallet for testing purposes.
use crate as peaq_pallet_mor;
pub use crate::{
    mock_const::*,
    types::{BalanceOf, MorConfig},
};

use frame_benchmarking::account;
use frame_support::{construct_runtime, parameter_types, PalletId};
use frame_system;
use pallet_balances;
use pallet_timestamp;
use sp_core::{sr25519, H256};
use sp_io;
use sp_runtime::traits::{AccountIdConversion, BlakeTwo256, IdentityLookup};
use sp_runtime::BuildStorage;
use sp_std::{boxed::Box, vec};

// system
pub type Block = frame_system::mocking::MockBlock<Test>;
// pallet-balances
pub type BalancesType = u128;

// Configure a mock runtime to test the pallet.
construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        Sudo: pallet_sudo,
        Timestamp: pallet_timestamp,
        Balances: pallet_balances,
        PeaqDid: peaq_pallet_did,
        PeaqMor: peaq_pallet_mor,
    }
);

parameter_types! {
    // frame_system
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
    // pallet_timestamp
    pub const MinimumPeriod: u64 = 5;
    // peaq-pallet-mor
    pub const PotId: PalletId = PalletId(*b"PotMchOw");
    // pallet_balances
    pub const ExistentialDeposit: u128 = 500;
    pub const MaxLocks: u32 = 50;
}

impl frame_system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type Nonce = u64;
    type Block = Block;
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = sr25519::Public;
    type Lookup = IdentityLookup<Self::AccountId>;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<BalancesType>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_sudo::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type WeightInfo = ();
}

impl pallet_timestamp::Config for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

impl pallet_balances::Config for Test {
    type MaxLocks = MaxLocks;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type Balance = BalancesType;
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type FreezeIdentifier = ();
    type MaxHolds = ();
    type MaxFreezes = ();
    type RuntimeHoldReason = ();
}

impl peaq_pallet_did::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Time = pallet_timestamp::Pallet<Test>;
    type WeightInfo = peaq_pallet_did::weights::WeightInfo<Test>;
}

impl peaq_pallet_mor::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type ExistentialDeposit = ExistentialDeposit;
    type Currency = Balances;
    type PotId = PotId;
    type WeightInfo = peaq_pallet_mor::weights::WeightInfo<Test>;
}

// Build genesis storage according to the mock runtime.
#[allow(dead_code)]
pub fn new_test_ext() -> sp_io::TestExternalities {
    //  creates a default balance for the owner account
    let owner = account_key(O_ACCT);
    let user = account_key(U_ACCT);
    let machine = account_key(M_ACCT);
    let mor_pot = PotId::get().into_account_truncating();

    // setup genesis configuration details
    let mut test_ext = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();

    pallet_sudo::GenesisConfig::<Test> {
        key: Some(owner.clone()),
    }
    .assimilate_storage(&mut test_ext)
    .unwrap();

    pallet_balances::GenesisConfig::<Test> {
        balances: vec![
            (owner, 10_000_000_000_000_000_000),
            (user, 10_000_000_000_000_000_000),
            (machine, 1_000_000_000_000_000_000),
            (mor_pot, 10_000_000_000_000_000_000),
        ],
    }
    .assimilate_storage(&mut test_ext)
    .unwrap();

    peaq_pallet_mor::GenesisConfig::<Test> {
        mor_config: MorConfig {
            registration_reward: BalanceOf::<Test>::from(REG_FEE),
            machine_usage_fee_min: BalanceOf::<Test>::from(100_000_000_000_000_000u128),
            machine_usage_fee_max: BalanceOf::<Test>::from(3_000_000_000_000_000_000u128),
            track_n_block_rewards: 10u8,
        },
    }
    .assimilate_storage(&mut test_ext)
    .unwrap();

    test_ext.into()
}

#[allow(dead_code)]
pub fn account_key(s: &'static str) -> <Test as frame_system::Config>::AccountId {
    account(s, 0, 0)
}
