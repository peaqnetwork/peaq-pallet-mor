//! Runtime mockup with this pallet for testing purposes.
use crate as peaq_pallet_mor;
use frame_support::{parameter_types, PalletId};
use frame_system as system;
use pallet_timestamp;
use pallet_balances;
use sp_core::{sr25519, Pair, H256};
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};

// system
type Block = frame_system::mocking::MockBlock<Test>;
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
// peaq-pallet-mor
pub type BalancesType = u128;
pub type MachineId = [u8; 32];

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
        Balances: pallet_balances::{Pallet, Call, Config<T>, Storage, Event<T>},
        PeaqMor: peaq_pallet_mor::{Pallet, Call, Storage, Event<T>},
    }
);

parameter_types! {
    // system
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

impl system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = sr25519::Public;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
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
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

impl peaq_pallet_mor::Config for Test {
    type Event = Event;
    type Currency = Balances;
    type PotId = PotId;
    type MachineId = MachineId;
    type WeightInfo = peaq_pallet_mor::weights::SubstrateWeight<Test>;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into()
}

pub fn account_key(s: &str) -> sr25519::Public {
    sr25519::Pair::from_string(&format!("//{}", s), None)
        .expect("static values are valid; qed")
        .public()
}
