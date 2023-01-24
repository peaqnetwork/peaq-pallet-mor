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

    use frame_system::pallet_prelude::*;
    use frame_support::{
        pallet_prelude::*,
        PalletId,
        traits::{
            Currency, ExistenceRequirement, Get, LockableCurrency, ReservableCurrency
            // OnTimestampSet, tokens::Balance
        },
    };
    use sp_std::vec::Vec;
    use sp_runtime::traits::AccountIdConversion;

    // use peaq_pallet_did as PeaqDid;
    use peaq_pallet_did::{
        Pallet as DidPallet,
        // Error as DidPalletErr,
        did::Did, //{, DidError}
    };

    use crate::{
        error::{
            MorError, MorResult,
            MorError::{
                AuthorizationFailed, MachineAlreadyRegistered, NameExceedMaxChar, UnexpectedDidError
            },
        },
        types::*,
        mor::*,
        weights::WeightInfo,
    };

    // Temporary constant, which defines how much blocks are generated to define
    // the period of time, which a machine has to be online to get rewarded
    const N_BLOCKS: usize = 200;



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
    #[pallet::storage]
    #[pallet::getter(fn machines_of)]
    pub type Machines<T: Config> = StorageMap<_,
        Blake2_128Concat,
        T::AccountId,
        bool,
        ValueQuery
    >;

    /// TODO
    #[pallet::storage]
    #[pallet::getter(fn rewards_record_of)]
    pub type RewardsRecord<T: Config> = StorageValue<_,
        (u8, Vec<CrtBalance<T>>),
        ValueQuery
    >;

    // TODO 
    #[pallet::storage]
    #[pallet::getter(fn period_reward_of)]
    pub type PeriodReward<T: Config> = StorageValue<_,
        CrtBalance<T>,
        ValueQuery
    >;
    

    
    /// Possible Event types of this pallet.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Machine has been rewarded
        MintedRewards(T::AccountId, CrtBalance<T>),
        /// Machine owner has been rewarded
        PayedFromPot(T::AccountId, CrtBalance<T>),
    }


    /// For description of error types, please have a look into module error.
    #[pallet::error]
    pub enum Error<T> {
        AuthorizationFailed,
        MachineAlreadyRegistered,
        NameExceedMaxChar,
        UnexpectedDidError,
    }
    
    impl<T: Config> Error<T> {
        fn dispatch_error(err: MorError) -> DispatchError {
            match err {
                AuthorizationFailed => Error::<T>::AuthorizationFailed.into(),
                MachineAlreadyRegistered => Error::<T>::MachineAlreadyRegistered.into(),
                NameExceedMaxChar => Error::<T>::NameExceedMaxChar.into(),
                UnexpectedDidError => Error::<T>::UnexpectedDidError.into(),
            }
        }
    }


    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> 
    where
        CrtBalance<T>: From<u128>
    {
        /// Registers a new machine on the network by given account-ID and machine-ID.
        #[pallet::weight(CrtWeight::<T>::some_extrinsic())]
        pub fn register_new_machine(
            origin: OriginFor<T>,
            machine: T::AccountId
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            DidPallet::<T>::is_owner(&sender, &machine).
                map_err(|e| Error::<T>::dispatch_error(MorError::from(e))
            )?;

            Self::register_machine(&machine).map_err(
                Error::<T>::dispatch_error
            )?;

            // 1 AGNG = 1_000_000_000_000_000_000
            let amount = <CrtBalance<T>>::from(100_000_000_000_000_000_u128);
            Self::mint_to_account(sender, amount)
        }

        /// In this early version one can collect rewards for a machine, which has been online
        /// on the network for a certain period of time.
        #[pallet::weight(CrtWeight::<T>::some_extrinsic())]
        pub fn get_online_rewards(
            origin: OriginFor<T>,
            machine: T::AccountId
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            DidPallet::<T>::is_owner(&sender, &machine).
                map_err(|e| Error::<T>::dispatch_error(MorError::from(e))
            )?;

            let reward = <PeriodReward<T>>::get();

            Self::transfer_from_pot(sender, reward)
        }

        /// In this early version one can collect rewards for a machine, which has been online
        /// on the network for a certain period of time.
        #[pallet::weight(CrtWeight::<T>::some_extrinsic())]
        pub fn pay_machine_usage(
            origin: OriginFor<T>,
            machine: T::AccountId,
            amount: CrtBalance<T>
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            DidPallet::<T>::is_owner(&sender, &machine).
                map_err(|e| Error::<T>::dispatch_error(MorError::from(e))
            )?;

            Self::mint_to_account(sender, amount)
        }
    }

    // See trait definition for further details
    impl<T: Config> MorBalance<T::AccountId, CrtBalance<T>> for Pallet<T> {
        fn mint_to_account(
            account: T::AccountId,
            amount: CrtBalance<T>
        ) -> DispatchResult {
            // let mut total_imbalance = <CrtPosImbalance<T>>::zero();

            // See https://substrate.recipes/currency-imbalances.html
            T::Currency::deposit_into_existing(&account, amount)?;
            Self::deposit_event(Event::<T>::MintedRewards(account, amount));
            // total_imbalance.maybe_subsume(r);
            // T::Reward::on_unbalanced(total_imbalance);
            
            Ok(())
        }

        fn transfer_from_pot(
            account: T::AccountId,
            amount: CrtBalance<T>
        ) -> DispatchResult {
            let pot: T::AccountId = T::PotId::get().into_account_truncating();
            
            T::Currency::transfer(&pot, &account, amount, ExistenceRequirement::KeepAlive)?;
            Self::deposit_event(Event::<T>::PayedFromPot(account, amount));

            Ok(())
        }

        fn log_block_rewards(
            amount: CrtBalance<T>
        ) {
            if !<RewardsRecord<T>>::exists() {
                // Do initial setup - Genesis??
                <RewardsRecord<T>>::set((1u8, vec![<CrtBalance<T>>::from(0u32); N_BLOCKS]));
            }

            // RewardsRecord: (u8, [CrtBalance<T>; N_BLOCKS])
            // Next array-slot to write in, Array of imbalances
            let (mut slot_cnt, mut balances) = <RewardsRecord<T>>::get();
            balances[slot_cnt as usize] = amount;
            slot_cnt += 1;
            if slot_cnt as usize >= N_BLOCKS {
                slot_cnt = 0;
            }

            // PeriodReward: CrtBalance<T>
            // Sum of last N_BLOCKS block-rewards
            let mut period_reward = <CrtBalance<T>>::from(0u32);
            balances.iter().for_each(|&b| period_reward += b);

            <RewardsRecord<T>>::set((slot_cnt, balances));
            <PeriodReward<T>>::set(period_reward);
        }
    }

    // See trait definition for further details
    impl<T: Config> MorMachine<T::AccountId> for Pallet<T> {
        fn register_machine(
            account: &T::AccountId
        ) -> MorResult<()> {
            if <Machines<T>>::contains_key(account) {
                Err(MorError::MachineAlreadyRegistered)
            } else {
                <Machines<T>>::insert(account, true);
                Ok(())
            }
        }
    }

}