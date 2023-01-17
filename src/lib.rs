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
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    // use sp_io::hashing::blake2_256;
    use sp_std::{
        fmt::Debug,
        vec::Vec,
    };

    use crate::{
        error::{
            MorError,
            MorErrorType::{
                OwnerDoesNotExist, MachineAlreadyExists, MachineIsDisabled, MachineDoesNotExist,
            },
            Result
        },
        structs::Machine,
        traits::{Mor, MachineAdm},
        weights::WeightInfo,
    };


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
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
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
    #[pallet::storage]
    // #[pallet::getter(fn machines_of)]
    pub type Machines<T: Config> = StorageDoubleMap<_,
            Blake2_128Concat,
            T::AccountId,
            Blake2_128Concat,
            T::MachineId,
            Machine,
            ValueQuery>;

    #[pallet::storage]
    pub type MachineIds<T: Config> = StorageValue<_, T::MachineId, ValueQuery>;

    
    // Pallets use events to inform users when important changes are made.
    // https://docs.substrate.io/main-docs/build/events-errors/
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Some event description
        NewMachineRegistered(T::AccountId, T::MachineId),
        /// Machine entry was fetched
        FetchedMachine(Machine),
    }


    // Pallets have errors to inform users when one occured
    // https://docs.substrate.io/main-docs/build/events-errors/
    /// For description of error types, please have a look into module error
    #[pallet::error]
    pub enum Error<T> {
        OwnerDoesNotExist,
        MachineAlreadyExists,
        MachineIsDisabled,
        MachineDoesNotExist,
    }

    impl<T: Config> Error<T> {
        fn dispatch_error(err: MorError) -> DispatchResult {
            match err.typ {
                OwnerDoesNotExist => Err(Error::<T>::OwnerDoesNotExist.into()),
                MachineAlreadyExists => Err(Error::<T>::MachineAlreadyExists.into()),
                MachineIsDisabled => Err(Error::<T>::MachineIsDisabled.into()),
                MachineDoesNotExist => Err(Error::<T>::MachineDoesNotExist.into()),
            }
        }
    }


    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(T::WeightInfo::some_extrinsic())]
        pub fn register_new_machine(
            origin: OriginFor<T>,
            machine: T::MachineId,
            owner: T::AccountId,
        ) -> DispatchResult {
            ensure_signed(origin)?;

            dpatch_dposit_par!(
                Self::register_machine(&owner, &machine),
                Event::NewMachineRegistered(owner, machine)
            )
        }

        #[pallet::weight(T::WeightInfo::some_extrinsic())]
        pub fn fetch_machine(
            origin: OriginFor<T>,
            owner: T::AccountId,
            machine: T::MachineId
        ) -> DispatchResult {
            ensure_signed(origin)?;

            dpatch_dposit!(
                Self::get_machine(&owner, &machine),
                Event::FetchedMachine
            )
        }
    }

    // See description about crate::traits::Mor
    impl<T: Config> Mor<T::MachineId, T::AccountId> for Pallet<T> {
        fn register_machine(owner: &T::AccountId, machine: &T::MachineId) -> Result<()> {
            Ok(())
        }

        fn get_rewarded(owner: &T::AccountId, machine: &T::MachineId) -> Result<()> {
            Ok(())
        }
    }

    // See description about crate::traits::MachineAdm
    impl<T: Config> MachineAdm<T::MachineId, T::AccountId> for Pallet<T> {
        fn add_machine(
            owner: &T::AccountId,
            machine: &T::MachineId
        ) -> Result<()> {
            Ok(())
        }

        fn update_account(
            owner: &T::AccountId,
            new_owner: &T::AccountId,
            machine: &T::MachineId
        ) -> Result<()> {
            Ok(())
        }

        fn disable_machine(owner: &T::AccountId, machine: &T::MachineId) -> Result<()> {
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
                Ok(<Machines<T>>::get(owner, machine))
            }
        }

        fn get_machines(
            owner: &T::AccountId
        ) -> Result<Vec<Machine>> {
            let owned_machines = <Machines<T>>::iter_prefix_values(owner);
            let mut machines = Vec::new();
            owned_machines.for_each(|m| machines.push(m.clone()));
            if machines.is_empty() {
                MorError::err(OwnerDoesNotExist, owner)
            } else {
                Ok(machines)
            }
        }
    }

}