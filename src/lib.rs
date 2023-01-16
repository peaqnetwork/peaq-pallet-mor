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

    // use codec::Encode;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    // use sp_io::hashing::blake2_256;
    // use sp_std::fmt::Debug;

    use crate::{
        error::{
            PalletError,
            PalletErrorType::{
                SomeError,
            },
            Result
        },
        structs::*,
        traits::FunctionalDescription,
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

    // macro_rules! dpatch_dposit_par {
    //     ($res:expr, $event:expr) => {
    //         match $res {
    //             Ok(_d) => {
    //                 Self::deposit_event($event);
    //                 Ok(())
    //             }
    //             Err(e) => Error::<T>::dispatch_error(e),
    //         }
    //     };
    // }


    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);


    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
    }

    
    // The pallet's runtime storage items.
    // https://docs.substrate.io/main-docs/build/runtime-storage/
    #[pallet::storage]
    #[pallet::getter(fn data_of)]
    pub type SomeStorage<T: Config> =
        StorageMap<_, Blake2_128Concat, u8, SomeData, ValueQuery>;

    
    // Pallets use events to inform users when important changes are made.
    // https://docs.substrate.io/main-docs/build/events-errors/
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Some event description
        SomeEvent(u8),
    }


    // Pallets have errors to inform users when one occured
    // https://docs.substrate.io/main-docs/build/events-errors/
    #[pallet::error]
    pub enum Error<T> {
        /// Some error description
        SomeError,
    }

    impl<T: Config> Error<T> {
        fn dispatch_error(err: PalletError) -> DispatchResult {
            match err.typ {
                SomeError => Err(Error::<T>::SomeError.into()),
            }
        }
    }


    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(T::WeightInfo::some_extrinsic())]
        pub fn some_extrinsic(
            origin: OriginFor<T>
        ) -> DispatchResult {
            ensure_signed(origin)?;

            dpatch_dposit!(Self::get_something(&SomeData{data: 5}), Event::SomeEvent)
        }
    }


    impl<T: Config> FunctionalDescription for Pallet<T> {
        fn get_something(data: &SomeData) -> Result<u8> {
            Ok(data.data)
        }
    }

}