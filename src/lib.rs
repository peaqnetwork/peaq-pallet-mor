//! # Peaq-Pallet-Mor
//!
//! Peaq-Pallet-Mor is a substrate based custom pallet of EoTLabs GmbH. For more
//! general informations about this pallet's elements, have a look into the following
//! links:
//! - <https://docs.substrate.io/build/custom-pallets/>
//! - <https://docs.substrate.io/build/events-and-errors/>
//! - <https://docs.substrate.io/build/runtime-storage/>
//! - <https://docs.substrate.io/build/tx-weights-fees/>
//!
//! ## Overview
//!  
//! In this crate, the pallet is defined which implements the functionality to
//! distribute Machine Owner Rewards (MOR). About possible rewards have a look
//! into paragraph Rewarding. For further informations about the functional
//! architecture have a look in the description of module ```traits```.
//!
//! Please have also a look into the README of the GitHub repository.
//!
//! ### Terminology
//!
//! - **Machine:** By machine a true device in real world is meant, e.g. a charging station
//!     or electrical car. For demonstration purpose this can be a Raspberry Pi. A machine
//!     has its own account and will be identified by the Peaq-DID pallet.
//!
//! - **Machine owner:** In abstract here we talk about a person who owns that machine and
//!     will administrate it. In a blockchain's point of view we talk about an account.
//!
//! - **Reward:** Rewards are fungible tokens, which will be transfered either to the machine
//!     owner's account or the machine's account directly. Rewarding means the transfer of
//!     fungible tokens to an account (from the machine or its owner).
//!
//! - **Pot:** This pallet has a seperate account to administrate collected block-rewards and
//!     to be able to distribute them to machines and machine owners.
//!
//! - **Defined time period:** You will read several times the term "defined time period".
//!     When machines are online, they will not be rewarded by each block finalization. Instead
//!     they will get rewarded after a time period, e.g. 20 minutes. This time period is interally
//!     defined and machines will be tracked if they have been online on the network for that time
//!     period. After that time period machines were online, they can be rewarded.
//!  
//! ### Rewarding
//!
//! At Peaq we reward machine owners for registering new machines to the network,
//! and for their machines continously beeing online on the network. This pallet
//! will implement those reward mechanisms.
//!
//! Rewards will be given either in that moment, when a new machine will be registerd,
//! or after beeing online a certain period of time.
//!
//! ## Technical Informations
//!
//! ### Integration on Runtime
//!
//! How to generally add a pallet to the runtime, have a look at the substrate's
//! documentation. Basic steps are:
//!
//! - Define a Pot-Account for it, for example:
//!     ```ignore
//!     parameter_types! {
//!         pub const PotMorId: PalletId = PalletId(*b"PotMchOw");
//!         pub const ExistentialDeposit: u128 = 500;
//!     }
//!     ```
//!
//! - Configure the pallet within the runtime by defining:
//!     ```ignore
//!     impl peaq_pallet_mor::Config for Runtime {
//!         type Event = Event;
//!         type ExistentialDeposit = ExistentialDeposit;
//!         type Currency = Balances;
//!         type PotId = PotMorId;
//!         type WeightInfo = peaq_pallet_mor::weights::SubstrateWeight<Runtime>;
//!     }
//!     ```
//!
//! - Add pallet on list of pallets within `construct_runtime!` macro:
//!     ```ignore
//!     construct_runtime! {
//!         pub enum Runtime where
//!             Block = Block,
//!             NodeBlock = Block,
//!             UncheckedExtrinsic = UncheckedExtrinsic,
//!         {
//!             System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
//!             // ...
//!             PeaqMor: peaq_pallet_mor::{Pallet, Call, Config<T>, Storage, Event<T>}
//!         }
//!     }
//!     ```
//!
//! - Add/configure genesis configuration `GenesisConfig<T>`:
//!     ```ignore
//!     GenesisConfig {
//!     system: SystemConfig {
//!         // Add Wasm runtime to storage.
//!         code: wasm_binary.to_vec(),
//!     },
//!     // ...
//!     peaq_mor: PeaqMorConfig {
//!         mor_config: MorConfig {
//!             registration_reward: 100_000_000_000_000_000u128,
//!             machine_usage_fee_min: 1_000u128,
//!             machine_usage_fee_max: 3_000_000_000_000_000_000u128,
//!             track_n_block_rewards: 200,
//!         },
//!     },
//!     ```
//!
//! - Implement a mechanism to fill that Pot-account `PotMorId`
//!
//! ### Dispatchable Functions (Extrinsics)
//!
//! - `get_registration_reward` - As it says, after registering a new machine with to
//!     Peaq-DID, a reward can be collected once per machine (identified by the machine's
//!     account-ID). Tokens will be minted.
//!
//! - `get_online_rewards` - Machine owners can be rewarded for having their machines
//!     continiously online on the network. Tokens will be transfered from the pallet's
//!     pot to the account of the machine owner.
//!
//! - `pay_machine_usage` - Simulates the payment of a used machine. Tokens will be
//!     minted, because currently users have no tokens on their accounts.
//!
//! - `set_configuration` - Setting a new pallet configuration. This can only be done
//!     by a sudo-user. For details about configuration have a look at the definition
//!     of `MorConfig`.
//!
//! - Remaining methods are temporary for development and debug purpose.
//!

#![cfg_attr(not(feature = "std"), no_std)]
// Fix benchmarking failure
#![recursion_limit = "256"]

pub use pallet::*;

#[cfg(any(test, feature = "runtime-benchmarks"))]
mod mock_const;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod error;
pub mod mor;
pub mod types;

pub mod weights;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {

    use core::cmp::Ordering;
    use frame_support::{
        pallet_prelude::*,
        traits::{
            Currency, ExistenceRequirement, Get, Imbalance, LockableCurrency, ReservableCurrency,
        },
        PalletId,
    };
    use frame_system::pallet_prelude::*;
    use sp_io::hashing::blake2_256;
    use sp_runtime::traits::{AccountIdConversion, One, Zero};
    use sp_std::{vec, vec::Vec};

    use peaq_pallet_did::{did::Did, Pallet as DidPallet};

    use super::WeightInfo;
    use crate::{
        error::{
            MorError,
            MorError::{
                DidAuthorizationFailed, InsufficientTokensInPot, MachineAlreadyRegistered,
                MachineNotRegistered, MachinePaymentOutOfRange, MorAuthorizationFailed,
                MorConfigIsNotConsistent, TokensCouldNotBeTransfered, UnexpectedDidError,
            },
            MorResult,
        },
        mor::*,
        types::*,
    };

    macro_rules! dpatch_dposit_par {
        ($res:expr, $event:expr) => {
            match $res {
                Ok(_d) => {
                    Self::deposit_event($event);
                    Ok(())
                }
                Err(e) => Err(e),
            }
        };
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    /// Configuration trait of this pallet.
    #[pallet::config]
    pub trait Config: frame_system::Config + peaq_pallet_did::Config
    where
        BalanceOf<Self>: Zero + One + PartialOrd + Eq,
    {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// The minimum amount required to keep an account open.
        #[pallet::constant]
        type ExistentialDeposit: Get<BalanceOf<Self>>;

        /// The currency type.
        type Currency: Currency<Self::AccountId>
            + ReservableCurrency<Self::AccountId>
            + LockableCurrency<Self::AccountId>
            + Eq;

        /// Account Identifier from which the internal Pot is generated.
        #[pallet::constant]
        type PotId: Get<PalletId>;

        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
    }

    /// This storage is only a lookup table, to make sure, that each machine will be
    /// registered only once (prevents registering same machine on different accounts).
    /// Its purpose is not designed for interacting with machines on the network.
    /// Key of the StorageMap will be the machine's account, value the owner's account.
    #[pallet::storage]
    #[pallet::getter(fn machine_register_of)]
    pub(super) type MachineRegister<T: Config> =
        StorageMap<_, Blake2_128Concat, [u8; 32], [u8; 32], ValueQuery>;

    /// Storage for recording incoming block-rewards. Its purpose is to be able to
    /// calculate the amount (sum) of all collected block-rewards within the defined
    /// time period.
    /// u8 stores the next Vec-index to be written over.
    /// Vec of balances of collected block-rewards.
    #[pallet::storage]
    #[pallet::getter(fn rewards_record_of)]
    pub(super) type RewardsRecordStorage<T: Config> =
        StorageValue<_, (u8, Vec<BalanceOf<T>>), ValueQuery>;

    /// This storage is for the sum over collected block-rewards. This amount will be
    /// transfered to an owner's account, when he requests the online-reward for his
    /// macine.
    #[pallet::storage]
    #[pallet::getter(fn period_reward_of)]
    pub(super) type PeriodRewardStorage<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    /// This storage hols the configuration of this pallet. About configurable
    /// parameters have a look at the MorConfig definition/description.
    #[pallet::storage]
    #[pallet::getter(fn mor_config_of)]
    pub(super) type MorConfigStorage<T: Config> =
        StorageValue<_, MorConfig<BalanceOf<T>>, ValueQuery>;

    /// Possible Event types of this pallet.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Machine has been rewarded by minting tokens.
        MintedTokens(BalanceOf<T>),
        /// The pallet's configuration has been updated.
        MorConfigChanged(MorConfig<BalanceOf<T>>),
        /// Fetches the pallet's configuration.
        FetchedMorConfig(MorConfig<BalanceOf<T>>),
        /// Temporary for development. Fetched balance of MOR pot.
        FetchedPotBalance(BalanceOf<T>),
        /// Temporary for development. Fetched current amount of rewarding.
        FetchedCurrentRewarding(BalanceOf<T>),
        /// Sent when machine usage has been payed.
        MachineUsagePayed(T::AccountId, BalanceOf<T>),
        /// Sent when the online rewards have been transfered.
        OnlineRewardsPayed(T::AccountId, BalanceOf<T>),
        /// Sent when a registration rewards have been transfered.
        RegistrationRewardPayed(T::AccountId, BalanceOf<T>),
    }

    /// For description of error types, please have a look into module error for
    /// further informations about error types.
    #[pallet::error]
    pub enum Error<T> {
        DidAuthorizationFailed,
        InsufficientTokensInPot,
        MachineAlreadyRegistered,
        MachineNotRegistered,
        MachinePaymentOutOfRange,
        MorAuthorizationFailed,
        MorConfigIsNotConsistent,
        TokensCouldNotBeTransfered,
        UnexpectedDidError,
    }

    impl<T: Config> Error<T> {
        fn from_mor(err: MorError) -> DispatchError {
            match err {
                DidAuthorizationFailed => Error::<T>::DidAuthorizationFailed.into(),
                InsufficientTokensInPot => Error::<T>::InsufficientTokensInPot.into(),
                MachineAlreadyRegistered => Error::<T>::MachineAlreadyRegistered.into(),
                MachineNotRegistered => Error::<T>::MachineNotRegistered.into(),
                MachinePaymentOutOfRange => Error::<T>::MachinePaymentOutOfRange.into(),
                MorAuthorizationFailed => Error::<T>::MorAuthorizationFailed.into(),
                MorConfigIsNotConsistent => Error::<T>::MorConfigIsNotConsistent.into(),
                TokensCouldNotBeTransfered => Error::<T>::TokensCouldNotBeTransfered.into(),
                UnexpectedDidError => Error::<T>::UnexpectedDidError.into(),
            }
        }
    }

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub mor_config: MorConfig<BalanceOf<T>>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                mor_config: MorConfig::<BalanceOf<T>>::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            assert!(self.mor_config.is_consistent(T::ExistentialDeposit::get()));
            Pallet::<T>::init_storages(&self.mor_config);
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_runtime_upgrade() -> frame_support::weights::Weight {
            let mor_config = MorConfigStorage::<T>::get();
            let reward_rec = RewardsRecordStorage::<T>::get();
            if mor_config.track_n_block_rewards as usize != reward_rec.1.len() {
                let mor_config = MorConfig::<BalanceOf<T>>::default();
                Self::init_storages(&mor_config);
                T::DbWeight::get().reads_writes(2, 3)
            } else {
                T::DbWeight::get().reads_writes(2, 0)
            }
        }
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Registers a new machine on the network by given account-ID and machine-ID. This
        /// method will raise errors if the machine is already registered, or if the
        /// authorization in Peaq-DID fails.
        #[pallet::call_index(0)]
        #[pallet::weight(WeightOf::<T>::get_registration_reward())]
        pub fn get_registration_reward(
            origin: OriginFor<T>,
            machine: T::AccountId,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let reward = Self::register_machine(&sender, &machine).map_err(Error::<T>::from_mor)?;

            dpatch_dposit_par!(
                Self::mint_to_account(&sender, reward),
                Event::<T>::RegistrationRewardPayed(sender, reward)
            )
        }

        /// In this early version one can collect rewards for a machine, which has been online
        /// on the network for a defined time period, see MorConfig. This method will raise
        /// errors if the authorization in Peaq-DID fails or if the machine is not registered
        /// in Peaq-MOR.
        #[pallet::call_index(1)]
        #[pallet::weight(WeightOf::<T>::get_online_rewards())]
        pub fn get_online_rewards(origin: OriginFor<T>, machine: T::AccountId) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let reward = Self::reward_machine(&sender, &machine).map_err(Error::<T>::from_mor)?;

            dpatch_dposit_par!(
                Self::transfer_from_pot(&sender, reward),
                Event::<T>::OnlineRewardsPayed(sender, reward)
            )
        }

        /// When using a machine, this extrinsic is about to pay the fee for the machine usage.
        /// Assumption is, that the origin is the user, which used the machine and he will pay
        /// the fee for machine usage.
        #[pallet::call_index(2)]
        #[pallet::weight(WeightOf::<T>::pay_machine_usage())]
        pub fn pay_machine_usage(
            origin: OriginFor<T>,
            machine: T::AccountId,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            ensure_signed(origin)?;

            let config = MorConfigStorage::<T>::get();

            // MachineUsagePayed
            if config.machine_usage_fee_min > amount || amount > config.machine_usage_fee_max {
                Err(Error::<T>::from_mor(MachinePaymentOutOfRange))
            } else {
                dpatch_dposit_par!(
                    Self::mint_to_account(&machine, amount),
                    Event::<T>::MachineUsagePayed(machine, amount)
                )
            }
        }

        /// Updates the pallet's configuration parameters by passing a MorConfig-struct.
        #[pallet::call_index(3)]
        #[pallet::weight(WeightOf::<T>::set_configuration())]
        pub fn set_configuration(
            origin: OriginFor<T>,
            config: MorConfig<BalanceOf<T>>,
        ) -> DispatchResult {
            ensure_root(origin)?;

            if config.is_consistent(T::ExistentialDeposit::get()) {
                Self::resize_track_storage(config.track_n_block_rewards);
                MorConfigStorage::<T>::put(config.clone());

                Self::deposit_event(Event::<T>::MorConfigChanged(config));
                Ok(())
            } else {
                Err(Error::<T>::from_mor(MorConfigIsNotConsistent))
            }
        }

        /// This is temporary for debug and development
        #[pallet::call_index(4)]
        #[pallet::weight(WeightOf::<T>::fetch_pot_balance())]
        pub fn fetch_pot_balance(origin: OriginFor<T>) -> DispatchResult {
            ensure_root(origin)?;

            let pot: T::AccountId = T::PotId::get().into_account_truncating();
            let amount = T::Currency::free_balance(&pot);

            Self::deposit_event(Event::<T>::FetchedPotBalance(amount));
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// This method internally initialises the pallet's storages in dependency of the given MorConfig.
        pub(crate) fn init_storages(mor_config: &MorConfig<BalanceOf<T>>) {
            let reward_record = (
                0u8,
                vec![BalanceOf::<T>::zero(); mor_config.track_n_block_rewards as usize],
            );

            MorConfigStorage::<T>::put(mor_config.clone());
            RewardsRecordStorage::<T>::put(reward_record);
            PeriodRewardStorage::<T>::put(BalanceOf::<T>::zero());
        }
    }

    // See MorBalance trait definition for further details
    impl<T: Config> MorBalance<T::AccountId, BalanceOf<T>> for Pallet<T> {
        fn mint_to_account(account: &T::AccountId, amount: BalanceOf<T>) -> DispatchResult {
            let imbalance = T::Currency::issue(amount);

            let amount = imbalance.peek();

            let imbalance = T::Currency::deposit_creating(account, amount);
            Self::deposit_event(Event::<T>::MintedTokens(imbalance.peek()));
            Ok(())
        }

        fn transfer_from_pot(account: &T::AccountId, amount: BalanceOf<T>) -> DispatchResult {
            let pot: T::AccountId = T::PotId::get().into_account_truncating();

            if T::Currency::free_balance(&pot) >= amount {
                T::Currency::transfer(&pot, account, amount, ExistenceRequirement::KeepAlive)?;
                Ok(())
            } else {
                Err(Error::<T>::from_mor(InsufficientTokensInPot))
            }
        }

        fn log_block_rewards(amount: BalanceOf<T>) {
            let mor_config = MorConfigStorage::<T>::get();
            let n_blocks = mor_config.track_n_block_rewards;

            // RewardsRecordStorage: (u8, Vec<BalanceOf<T>>)
            // (Next array-slot to write in, vector of imbalances)
            let (mut slot_cnt, mut balances) = RewardsRecordStorage::<T>::get();
            balances[slot_cnt as usize] = amount;
            slot_cnt += 1;
            if slot_cnt >= n_blocks {
                slot_cnt = 0;
            }

            // PeriodRewardStorage: BalanceOf<T>
            // Sum of last n_blocks block-rewards
            let mut period_reward = BalanceOf::<T>::zero();
            balances.iter().for_each(|&b| period_reward += b);

            RewardsRecordStorage::<T>::set((slot_cnt, balances));
            PeriodRewardStorage::<T>::set(period_reward);
        }

        fn resize_track_storage(new_size: u8) {
            let new_size = new_size as usize;
            let (_slot_cnt, mut balances) = RewardsRecordStorage::<T>::get();

            let cur_size = balances.len();
            match cur_size.cmp(&new_size) {
                Ordering::Less => {
                    balances.resize(new_size, BalanceOf::<T>::zero());
                    let slot_cnt = cur_size as u8;
                    assert!(balances.len() == new_size);
                    RewardsRecordStorage::<T>::put((slot_cnt, balances));
                }
                Ordering::Greater => {
                    let slot_cnt = 0u8;
                    let balances = balances.split_off(cur_size - new_size);
                    assert!(balances.len() == new_size);
                    RewardsRecordStorage::<T>::put((slot_cnt, balances));
                }
                _ => {}
            }
        }
    }

    // See MorMachine trait description for further details
    impl<T: Config> MorMachine<T::AccountId, BalanceOf<T>> for Pallet<T> {
        fn register_machine(
            owner: &T::AccountId,
            machine: &T::AccountId,
        ) -> MorResult<BalanceOf<T>> {
            // Registered in Peaq-DID and is this the owner?
            DidPallet::<T>::is_owner(owner, machine).map_err(MorError::from)?;

            let machine_hash = (machine).using_encoded(blake2_256);
            if MachineRegister::<T>::contains_key(machine_hash) {
                Err(MorError::MachineAlreadyRegistered)
            } else {
                let owner_hash = (owner).using_encoded(blake2_256);
                let config = MorConfigStorage::<T>::get();
                MachineRegister::<T>::insert(machine_hash, owner_hash);
                // 1 AGNG = 1_000_000_000_000_000_000
                Ok(config.registration_reward)
            }
        }

        fn reward_machine(owner: &T::AccountId, machine: &T::AccountId) -> MorResult<BalanceOf<T>> {
            // Is still registered in Peaq-DID and is this the owner?
            DidPallet::<T>::is_owner(owner, machine).map_err(MorError::from)?;
            // Is machine registered in Peaq-MOR?
            let machine_hash = (machine).using_encoded(blake2_256);
            if !MachineRegister::<T>::contains_key(machine_hash) {
                return Err(MorError::MachineNotRegistered);
            }
            let owner_hash = MachineRegister::<T>::get(machine_hash);
            if owner_hash != (owner).using_encoded(blake2_256) {
                return Err(MorError::MorAuthorizationFailed);
            }

            Ok(PeriodRewardStorage::<T>::get())
        }
    }
}
