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
//! into paragraph Rewarding. For informations about the technical architecture
//! have a look in the description of module ```traits```.
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
//!     ```
//!     parameter_types! {
//!	        pub const PotMorId: PalletId = PalletId(*b"PotMchOw");
//!     }
//!     ```
//! 
//! - Configure the pallet within the runtime by defining:
//!     ```
//!     impl peaq_pallet_mor::Config for Runtime {
//!         type Event = Event;
//!         type Currency = Balances;
//!         type PotId = PotMorId;
//!         type MachineId = MachineId;
//!         type WeightInfo = peaq_pallet_mor::weights::SubstrateWeight<Runtime>;
//!     }
//!     ```
//! - Add pallet on list of pallets within `construct_runtime!` macro:
//!     ```PeaqMor: peaq_pallet_mor::{Pallet, Call, Storage, Event<T>}```
//! 
//! - TODO
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
//! - `fetch_machine` - Fetches a registered machine's description and its enable-state.
//! 
//! - TODO
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
pub mod traits;
pub mod structs;

pub mod weights;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {

    use codec::MaxEncodedLen;
    use frame_system::pallet_prelude::*;
    use frame_support::{
        pallet_prelude::*,
        PalletId,
        traits::{
            Currency, ExistenceRequirement, Get, LockableCurrency, ReservableCurrency
        }
    };
    use sp_runtime::traits::{AccountIdConversion, CheckedSub, Zero};
    use sp_std::{
        fmt::Debug,
        vec::Vec,
    };

    use crate::{
        error::{
            MorError,
            MorErrorType::{
                OwnerDoesNotExist, MachineNameExceedMax64, MachineAlreadyExists, 
                MachineIsDisabled, MachineIsAlreadyEnabled, MachineDoesNotExist, 
                MachineDescIoError,
            },
            Result
        },
        structs::*,
        traits::*,
        weights::WeightInfo,
    };


    macro_rules! dpatch {
        ($res:expr) => {
            match $res {
                Ok(_d) => {
                    Ok(())
                }
                Err(e) => Error::<T>::dispatch_error(e),
            }
        };
    }

    macro_rules! dpatch_dposit {
        ($res:expr, $event:expr) => {
            match $res {
                Ok(d) => {
                    Self::deposit_event($event(d));
                    Ok(())
                }
                Err(e) => Error::<T>::dispatch_error(e),
            }
        };
    }

    macro_rules! dpatch_dposit_par {
        ($res:expr, $event:expr) => {
            match $res {
                Ok(_d) => {
                    Self::deposit_event($event);
                    Ok(())
                }
                Err(e) => Error::<T>::dispatch_error(e),
            }
        };
    }


    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);


    /// Configuration trait of this pallet.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        // TODO define dependencies on other pallets...
        // + pallet_balances::Config ???

        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        
        /// Currency TODO
        type Currency: Currency<Self::AccountId>
			+ ReservableCurrency<Self::AccountId>
			+ LockableCurrency<Self::AccountId>
			+ Eq;

        /// Account Identifier from which the internal Pot is generated.
		#[pallet::constant]
		type PotId: Get<PalletId>;
        
        /// Machines are getting identified by an ID type
        type MachineId: Parameter
            + Member
            + MaybeSerializeDeserialize
            + Debug
            + Ord
            + Clone
            + Copy
            + MaxEncodedLen
            + Default;
        
        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
    }

    
    /// This storage keeps all registered machines, their owners, descriptions 
    /// and enabled-states. This is designed for active interactions with machines
    /// on the network.
    #[pallet::storage]
    // #[pallet::getter(fn machines_of)]
    pub type Machines<T: Config> = StorageDoubleMap<_,
            Blake2_128Concat,
            T::AccountId,
            Blake2_128Concat,
            T::MachineId,
            Machine,
            ValueQuery>;

    /// This storage is only a lookup table, to make sure, that each machine will be
    /// registered only once (prevents registering same machine on different accounts).
    /// Its purpose is not designed for interacting with machines on the network.
    #[pallet::storage]
    pub type MachineIds<T: Config> = StorageMap<_,
        Blake2_128Concat,
        T::MachineId,
        (),
        ValueQuery>;

    
    /// TODO
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // Example: A new staking round has started.
		// Example \[owner, machine\]
        /// A new machine was registered on the network
        NewMachineRegistered(T::AccountId, T::MachineId),
        /// Machine has been rewarded for beeing online on the network
        OwnerGotRewarded(T::AccountId, Balance<T>),
        /// Machine entry was fetched
        FetchedMachineDescription(Machine),
    }


    /// TODO For description of error types, please have a look into module error
    #[pallet::error]
    pub enum Error<T> {
        OwnerDoesNotExist,
        MachineNameExceedMax64,
        MachineAlreadyExists,
        MachineIsDisabled,
        MachineIsAlreadyEnabled,
        MachineDoesNotExist,
        MachineDescIoError,
    }

    impl<T: Config> Error<T> {
        fn dispatch_error(err: MorError) -> DispatchResult {
            match err.typ {
                OwnerDoesNotExist => Err(Error::<T>::OwnerDoesNotExist.into()),
                MachineNameExceedMax64 => Err(Error::<T>::MachineNameExceedMax64.into()),
                MachineAlreadyExists => Err(Error::<T>::MachineAlreadyExists.into()),
                MachineIsDisabled => Err(Error::<T>::MachineIsDisabled.into()),
                MachineIsAlreadyEnabled => Err(Error::<T>::MachineIsAlreadyEnabled.into()),
                MachineDoesNotExist => Err(Error::<T>::MachineDoesNotExist.into()),
                MachineDescIoError => Err(Error::<T>::MachineDescIoError.into()),
            }
        }
    }


    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Registers a new machine on the network by given account-ID and machine-ID.
        #[pallet::weight(T::WeightInfo::some_extrinsic())]
        pub fn register_new_machine(
            origin: OriginFor<T>,
            owner: T::AccountId,
            machine: T::MachineId,
            name: Vec<u8>
        ) -> DispatchResult {
            ensure_signed(origin)?;

            dpatch_dposit_par!(
                <Self as Mor<T::AccountId, T::MachineId>>::register_new_machine(&owner, &machine, &name),
                Event::NewMachineRegistered(owner, machine)
            )
        }

        /// In this early version one can collect rewards for a machine, which has been online
        /// on the network for a certain period of time.
        #[pallet::weight(T::WeightInfo::some_extrinsic())]
        pub fn get_online_rewards(
            origin: OriginFor<T>,
            owner: T::AccountId,
            machine: T::MachineId
        ) -> DispatchResult {
            ensure_signed(origin)?;

            dpatch!(
                <Self as Mor<T::AccountId, T::MachineId>>::get_online_rewards(&owner, &machine)
            )
        }

        /// Fetch a machine's description.
        #[pallet::weight(T::WeightInfo::some_extrinsic())]
        pub fn fetch_machine(
            origin: OriginFor<T>,
            owner: T::AccountId,
            machine: T::MachineId
        ) -> DispatchResult {
            ensure_signed(origin)?;

            dpatch_dposit!(
                Self::get_machine(&owner, &machine),
                Event::FetchedMachineDescription
            )
        }
    }

    // See description about crate::traits::Mor
    impl<T: Config> Mor<T::AccountId, T::MachineId> for Pallet<T> {
        fn register_new_machine(
            owner: &T::AccountId,
            machine: &T::MachineId,
            name: &Vec<u8>
        ) -> Result<()> {
            Self::add_machine(owner, machine, name)?;
            Self::get_registration_reward(owner);
            Ok(())
        }

        fn get_online_rewards(
            owner: &T::AccountId,
            machine: &T::MachineId
        ) -> Result<()> {
            Self::get_machine(owner, machine)?;
            Self::get_available_rewards(owner);
            Ok(())
        }

        fn disable_machine(
            owner: &T::AccountId,
            machine: &T::MachineId
        ) -> Result<()> {
            <Self as MachineAdm<T::AccountId, T::MachineId>>::disable_machine(owner, machine)
        }

        fn enable_machine(
            owner: &T::AccountId,
            machine: &T::MachineId
        ) -> Result<()> {
            <Self as MachineAdm<T::AccountId, T::MachineId>>::enable_machine(owner, machine)
        }
    }

    // See description about crate::traits::PotAdm
    impl<T: Config> PotAdm<T::AccountId, Balance<T>> for Pallet<T> {
        fn account_id() -> T::AccountId {
			T::PotId::get().into_account_truncating()
		}

        fn do_reward(
            pot: &T::AccountId,
            who: &T::AccountId,
            reward: Balance<T>
        ) {
            // Copied from parachain_staking::Pallet::do_reward()
            if let Ok(_success) = T::Currency::transfer(pot, who, reward, ExistenceRequirement::KeepAlive) {
				Self::deposit_event(Event::OwnerGotRewarded(who.clone(), reward));
			}
        }

        fn get_available_rewards(owner: &T::AccountId) {
            let pot = Self::account_id();
            let issue_number = T::Currency::free_balance(&pot)
                .checked_sub(&T::Currency::minimum_balance())
                .unwrap_or_else(Zero::zero);
            Self::do_reward(&pot, &owner, issue_number);
        }

        fn get_registration_reward(owner: &T::AccountId) {
            Self::get_available_rewards(owner);
        }
    }

    // For method's description have a look at crate::traits::MachineAdm
    impl<T: Config> MachineAdm<T::AccountId, T::MachineId> for Pallet<T> {
        fn add_machine(
            owner: &T::AccountId,
            machine: &T::MachineId,
            name: &Vec<u8>
        ) -> Result<()> {
            // First we check if this machine ID already exists in MachineIds storage,
            // to prevent that one machine will be registered in multiple accounts.
            if <MachineIds<T>>::contains_key(machine) {
                return MorError::err(MachineAlreadyExists, machine)
            }

            <Machines<T>>::insert(owner, machine, Machine::new(name));
            <MachineIds<T>>::insert(machine, ());

            Ok(())
        }

        fn update_account(
            owner: &T::AccountId,
            new_owner: &T::AccountId,
            machine: &T::MachineId
        ) -> Result<()> {
            let ms = Self::get_machine(owner, machine)?;
            <Machines<T>>::remove(owner, machine);
            <Machines<T>>::insert(new_owner, machine, ms);
            Ok(())
        }

        fn enable_machine(owner: &T::AccountId, machine: &T::MachineId) -> Result<()> {
            let mut ms = Self::get_machine(owner, machine)?;
            if ms.enabled {
                MorError::err(MachineIsAlreadyEnabled, machine)
            } else {
                ms.enabled = true;
                <Machines<T>>::set(owner, machine, ms);
                Ok(())
            }
        }

        fn disable_machine(owner: &T::AccountId, machine: &T::MachineId) -> Result<()> {
            let mut ms = Self::get_machine(owner, machine)?;
            if !ms.enabled {
                MorError::err(MachineIsDisabled, machine)
            } else {
                ms.enabled = false;
                <Machines<T>>::set(owner, machine, ms);
                Ok(())
            }
        }

        fn get_machine(owner: &T::AccountId, machine: &T::MachineId) -> Result<Machine> {
            if !<Machines<T>>::contains_key(owner, machine) {
                if <Machines<T>>::iter_prefix_values(owner).next().is_none() {
                    MorError::err(OwnerDoesNotExist, owner)
                } else {
                    MorError::err(MachineDoesNotExist, machine)
                }
            } else {
                let ms = <Machines<T>>::get(owner, machine);
                if ms.enabled {
                    Ok(ms)
                } else {
                    MorError::err(MachineIsDisabled, machine)
                }
            }
        }

        fn get_machines(
            owner: &T::AccountId
        ) -> Result<Vec<Machine>> {
            let owned_machines = <Machines<T>>::iter_prefix_values(owner);
            let mut machines = Vec::new();
            owned_machines.for_each(|m| {
                if m.enabled {
                    machines.push(m.clone())
                }
            });
            if machines.is_empty() {
                MorError::err(OwnerDoesNotExist, owner)
            } else {
                Ok(machines)
            }
        }
    }

}