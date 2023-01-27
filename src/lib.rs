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
//! - **Machine:** TODO.
//!
//! - **Owner:** TODO
//!
//! - **Machine-Description:** TODO
//!
//! - **Reward:** TODO
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
//! - Define a Pot-Account for it, e.g.:
//!     ```ignore
//!     parameter_types! {
//!	        pub const PotMorId: PalletId = PalletId(*b"PotMchOw");
//!     }
//!     ```
//!
//! - Configure the pallet within the runtime by defining:
//!     ```ignore
//!     impl peaq_pallet_mor::Config for Runtime {
//!         type Event = Event;
//!         type Currency = Balances;
//!         type PotId = PotMorId;
//!         type MachineId = MachineId;
//!         type WeightInfo = peaq_pallet_mor::weights::SubstrateWeight<Runtime>;
//!     }
//!     ```
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
//!             PeaqMor: peaq_pallet_mor::{Pallet, Call, Storage, Event<T>}
//!         }
//!     }
//!     ```
//!
//! - Implement a mechanism to fill that Pot-account `PotMorId`
//!
//! ### Dispatchable Functions (Extrinsics)
//!
//! - `register_new_machine` - As it says, to register a new machine with an unique
//!     machine-ID. Machines can only be registered once, and not be deleted. If you
//!     want to remove a machine, you can disable it.
//!
//! - `get_online_rewards` - Machine owners can be rewarded for having their machines
//!     continiously online on the network.
//!
//! - `pay_machine_usage` - TODO
//!

#![cfg_attr(not(feature = "std"), no_std)]
// Fix benchmarking failure
#![recursion_limit = "256"]

pub use pallet::*;

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

    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, ExistenceRequirement, Get, LockableCurrency, ReservableCurrency},
        PalletId,
    };
    use frame_system::pallet_prelude::*;
    use sp_io::hashing::blake2_256;
    use sp_runtime::traits::AccountIdConversion;
    use sp_std::{vec, vec::Vec};

    use peaq_pallet_did::{did::Did, Pallet as DidPallet};

    use crate::{
        error::{
            MorError,
            MorError::{
                MachineAlreadyRegistered, MachineNotRegistered, DidAuthorizationFailed,
                MorAuthorizationFailed, UnexpectedDidError, InsufficientTokensInPot
            },
            MorResult,
        },
        mor::*,
        types::*,
        weights::WeightInfo,
    };
    

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    /// Configuration trait of this pallet.
    #[pallet::config]
    pub trait Config: frame_system::Config + peaq_pallet_did::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Currency description TODO
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
    pub(super) type RewardsRecord<T: Config> =
        StorageValue<_, (u8, Vec<CrtBalance<T>>), ValueQuery>;

    /// This storage is for the sum over collected block-rewards. This amount will be
    /// transfered to an owner's account, when he requests the online-reward for his
    /// macine.
    #[pallet::storage]
    #[pallet::getter(fn period_reward_of)]
    pub(super) type PeriodReward<T: Config> = StorageValue<_, CrtBalance<T>, ValueQuery>;

    /// This storage hols the configuration of this pallet. About configurable
    /// parameters have a look at the MorConfig definition/description.
    #[pallet::storage]
    #[pallet::getter(fn mor_config_of)]
    pub(super) type MorConfigStorage<T: Config> = StorageValue<_, MorConfig<T>, ValueQuery>;


    /// Possible Event types of this pallet.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Machine has been rewarded by minting tokens.
        RewardsMinted(T::AccountId, CrtBalance<T>),
        /// Machine owner has been rewarded by tokens from the pot.
        RewardsFromPot(T::AccountId, CrtBalance<T>),
        /// Temporary for development. Fetched balance of MOR pot.
        FetchedPotBalance(CrtBalance<T>),
        /// Temporary for development. Fetched current amount of rewarding.
        FetchedCurrentRewarding(CrtBalance<T>),
    }

    /// For description of error types, please have a look into module error for
    /// further informations about error types.
    #[pallet::error]
    pub enum Error<T> {
        MachineAlreadyRegistered,
        MachineNotRegistered,
        DidAuthorizationFailed,
        MorAuthorizationFailed,
        UnexpectedDidError,
        InsufficientTokensInPot
    }

    impl<T: Config> Error<T> {
        fn from_mor(err: MorError) -> DispatchError {
            match err {
                MachineAlreadyRegistered => Error::<T>::MachineAlreadyRegistered.into(),
                MachineNotRegistered => Error::<T>::MachineNotRegistered.into(),
                DidAuthorizationFailed => Error::<T>::DidAuthorizationFailed.into(),
                MorAuthorizationFailed => Error::<T>::MorAuthorizationFailed.into(),
                UnexpectedDidError => Error::<T>::UnexpectedDidError.into(),
                InsufficientTokensInPot => Error::<T>::InsufficientTokensInPot.into(),
            }
        }
    }


    #[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub mor_config: MorConfig<T>,
		// pub block_issue_reward: BalanceOf<T>,
		// pub hard_cap: BalanceOf<T>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				mor_config: MorConfig::<T>::default(),
				// block_issue_reward: Default::default(),
				// hard_cap: Default::default(),
			}
		}
	}

    #[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			MorConfigStorage::<T>::put(self.mor_config.clone());
			// BlockIssueReward::<T>::put(self.block_issue_reward);
			// HardCap::<T>::put(self.hard_cap);
		}
	}


    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T>
    where CrtBalance<T>: From<u64> + From<u128>
    {
        /// Registers a new machine on the network by given account-ID and machine-ID. This
        /// method will raise errors if the machine is already registered, or if the
        /// authorization in Peaq-DID fails.
        #[pallet::weight(CrtWeight::<T>::some_extrinsic())]
        pub fn register_new_machine(origin: OriginFor<T>, machine: T::AccountId) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let reward = Self::register_machine(&sender, &machine).map_err(Error::<T>::from_mor)?;

            Self::mint_to_account(&sender, reward)
        }

        /// In this early version one can collect rewards for a machine, which has been online
        /// on the network for a defined time period, see MorConfig. This method will raise
        /// errors if the authorization in Peaq-DID fails or if the machine is not registered
        /// in Peaq-MOR.
        #[pallet::weight(CrtWeight::<T>::some_extrinsic())]
        pub fn get_online_rewards(origin: OriginFor<T>, machine: T::AccountId) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let reward = Self::reward_machine(&sender, &machine).map_err(Error::<T>::from_mor)?;

            Self::transfer_from_pot(&sender, reward)
        }

        /// When using a machine, this extrinsic is about to pay the fee for the machine usage.
        /// Assumption is, that the origin is the user, which used the machine and he will pay
        /// the fee for machine usage.
        #[pallet::weight(CrtWeight::<T>::some_extrinsic())]
        pub fn pay_machine_usage(
            origin: OriginFor<T>,
            machine: T::AccountId,
            amount: CrtBalance<T>,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            T::Currency::transfer(&sender, &machine, amount, ExistenceRequirement::KeepAlive)
        }

        /// This is temporary for debug and development
        #[pallet::weight(CrtWeight::<T>::some_extrinsic())]
        pub fn fetch_pot_balance(origin: OriginFor<T>) -> DispatchResult {
            ensure_signed(origin)?;

            let pot: T::AccountId = T::PotId::get().into_account_truncating();
            let amount = T::Currency::free_balance(&pot);

            Self::deposit_event(Event::<T>::FetchedPotBalance(amount));
            Ok(())
        }

        /// This is temporary for debug and development
        #[pallet::weight(CrtWeight::<T>::some_extrinsic())]
        pub fn fetch_period_rewarding(origin: OriginFor<T>) -> DispatchResult {
            ensure_signed(origin)?;

            let amount = <PeriodReward<T>>::get();

            Self::deposit_event(Event::<T>::FetchedCurrentRewarding(amount));
            Ok(())
        }
    }

    // See MorBalance trait definition for further details
    impl<T: Config> MorBalance<T::AccountId, CrtBalance<T>> for Pallet<T>
    where CrtBalance<T>: From<u64> + From<u128>
    {
        fn mint_to_account(account: &T::AccountId, amount: CrtBalance<T>) -> DispatchResult {
            // let mut total_imbalance = <CrtPosImbalance<T>>::zero();

            // See https://substrate.recipes/currency-imbalances.html
            T::Currency::deposit_into_existing(account, amount)?;
            Self::deposit_event(Event::<T>::RewardsMinted(account.clone(), amount));
            // total_imbalance.maybe_subsume(r);
            // T::Reward::on_unbalanced(total_imbalance);

            Ok(())
        }

        fn transfer_from_pot(account: &T::AccountId, amount: CrtBalance<T>) -> DispatchResult {
            let pot: T::AccountId = T::PotId::get().into_account_truncating();

            if T::Currency::free_balance(&pot) >= amount {
                T::Currency::transfer(&pot, account, amount, ExistenceRequirement::KeepAlive)?;
                Self::deposit_event(Event::<T>::RewardsFromPot(account.clone(), amount));
                Ok(())
            } else {
                Err(Error::<T>::from_mor(InsufficientTokensInPot))
            }
        }

        fn log_block_rewards(amount: CrtBalance<T>) {
            let mor_config = <MorConfigStorage<T>>::get();
            let n_blocks = mor_config.time_period_blocks;
            if !<RewardsRecord<T>>::exists() {
                <RewardsRecord<T>>::set((1u8, vec![<CrtBalance<T>>::from(0u128); n_blocks]));
            }

            // RewardsRecord: (u8, Vec<CrtBalance<T>>)
            // Next array-slot to write in, Array of imbalances
            let (mut slot_cnt, mut balances) = <RewardsRecord<T>>::get();
            balances[slot_cnt as usize] = amount;
            slot_cnt += 1;
            if slot_cnt >= n_blocks {
                slot_cnt = 0;
            }

            // PeriodReward: CrtBalance<T>
            // Sum of last n_blocks block-rewards
            let mut period_reward = <CrtBalance<T>>::from(0u128);
            // Workarround, skip some block rewards to gain always positive balance
            balances.iter().skip(5).for_each(|&b| period_reward += b);

            <RewardsRecord<T>>::set((slot_cnt, balances));
            <PeriodReward<T>>::set(period_reward);
        }
    }

    // See MorMachine trait description for further details
    impl<T: Config> MorMachine<T::AccountId, CrtBalance<T>> for Pallet<T>
    where CrtBalance<T>: From<u64> + From<u128>
    {
        fn register_machine(
            owner: &T::AccountId,
            machine: &T::AccountId,
        ) -> MorResult<CrtBalance<T>> {
            // Registered in Peaq-DID and is this the owner?
            DidPallet::<T>::is_owner(&owner, &machine).map_err(MorError::from)?;

            let machine_hash = (machine).using_encoded(blake2_256);
            if <MachineRegister<T>>::contains_key(machine_hash) {
                Err(MorError::MachineAlreadyRegistered)
            } else {
                let owner_hash = (owner).using_encoded(blake2_256);
                <MachineRegister<T>>::insert(machine_hash, owner_hash);
                // 1 AGNG = 1_000_000_000_000_000_000
                Ok(<CrtBalance<T>>::from(100_000_000_000_000_000u128))
            }
        }

        fn reward_machine(
            owner: &T::AccountId,
            machine: &T::AccountId,
        ) -> MorResult<CrtBalance<T>> {
            // Is still registered in Peaq-DID and is this the owner?
            DidPallet::<T>::is_owner(owner, machine).map_err(MorError::from)?;
            // Is machine registered in Peaq-MOR?
            let machine_hash = (machine).using_encoded(blake2_256);
            if !<MachineRegister<T>>::contains_key(machine_hash) {
                return Err(MorError::MachineNotRegistered);
            }
            let owner_hash = <MachineRegister<T>>::get(machine_hash);
            if owner_hash != (owner).using_encoded(blake2_256) {
                return Err(MorError::MorAuthorizationFailed);
            }

            Ok(<PeriodReward<T>>::get())
        }
    }
}
