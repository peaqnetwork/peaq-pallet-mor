//! Pallet template
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
                MachineIsDisabled, MachineDoesNotExist, MachineDescIoError,
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


    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        // TODO define dependencies on other pallets...

        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        
        /// Currency...
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

    
    // The pallet's runtime storage items.
    // https://docs.substrate.io/main-docs/build/runtime-storage/
    /// This storage keeps all registered machines, their descriptions and states
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
    /// registered only once (prevents registering same machine on different accounts)
    #[pallet::storage]
    pub type MachineIds<T: Config> = StorageMap<_,
        Blake2_128Concat,
        T::MachineId,
        (),
        ValueQuery>;

    
    // Pallets use events to inform users when important changes are made.
    // https://docs.substrate.io/main-docs/build/events-errors/
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new machine was registered on the network
        NewMachineRegistered(T::AccountId, T::MachineId),
        /// Machine has been rewarded for beeing online on the network
        OwnerGotRewarded(T::AccountId, Balance<T>),
        /// Machine entry was fetched
        FetchedMachineDescription(Machine),
    }


    // Pallets have errors to inform users when one occured
    // https://docs.substrate.io/main-docs/build/events-errors/
    /// For description of error types, please have a look into module error
    #[pallet::error]
    pub enum Error<T> {
        OwnerDoesNotExist,
        MachineNameExceedMax64,
        MachineAlreadyExists,
        MachineIsDisabled,
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

        fn disable_machine(owner: &T::AccountId, machine: &T::MachineId) -> Result<()> {
            let mut ms = Self::get_machine(owner, machine)?;
            ms.enabled = false;
            <Machines<T>>::set(owner, machine, ms);
            Ok(())
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