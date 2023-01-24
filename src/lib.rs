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
//! - `fetch_machine_info` - Fetches a registered machine's description and its enable-state.
//! 
//! - `enable_machine` - Enables a machine after it has been disabled.
//! 
//! - `disable_machine` - Disables a machine on the network. Note, at the moment a machine 
//!     cannot be deleted, so this is the way to remove a machine from the network.
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

    // use codec::MaxEncodedLen;
    use frame_system::pallet_prelude::*;
    use frame_support::{
        pallet_prelude::*,
        PalletId,
        traits::{
            Currency, ExistenceRequirement, Get, LockableCurrency, ReservableCurrency, //, Time as MomentTime
            Imbalance
        }
    };
    use sp_runtime::traits::{AccountIdConversion};
    // use sp_std::vec::Vec;

    use peaq_pallet_did as PeaqDid;
    use peaq_pallet_did::{
        Pallet as DidPallet,
        Error as DidPalletErr,
        did::{Did, DidError}
    };

    use crate::{
        // error::{
        //     MorError,
        //     MorErrorType::{
        //         AuthorizationFailed, MachineNameExceedMax64, UnexpectedDidError
        //     },
        //     Result
        // },
        types::*,
        mor::*,
        weights::WeightInfo,
    };


    // macro_rules! dpatch {
    //     ($res:expr) => {
    //         match $res {
    //             Ok(_d) => {
    //                 Ok(())
    //             }
    //             Err(e) => Error::<T>::dispatch_error(e),
    //         }
    //     };
    // }

    // macro_rules! dpatch_dposit {
    //     ($res:expr, $event:expr) => {
    //         match $res {
    //             Ok(d) => {
    //                 Self::deposit_event($event(d));
    //                 Ok(())
    //             }
    //             Err(e) => Error::<T>::dispatch_error(e),
    //         }
    //     };
    // }

    macro_rules! dpatch_dposit_par {
        ($res:expr, $event:expr) => {
            match $res {
                Ok(_d) => {
                    Self::deposit_event($event);
                    Ok(())
                }
                Err(e) => did_dispatch_error(e),
            }
        };
    }


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
			+ Eq
            + From<u128>;

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
    pub type MachineList<T: Config> = StorageMap<_,
            Blake2_128Concat,
            T::AccountId,
            bool,
            ValueQuery
    >;

    /// This storage keeps the current configuration of rewards, which are given to
    /// machines and their owners. E.g. when registering a new machine, x token will
    /// be given to the owner, this amount can be configured over here.

    
    /// Possible Event types of this pallet.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Machine has been rewarded
        MintedRewards(T::AccountId, Balance<T>),
        /// Machine owner has been rewarded
        PayedFromPot(T::AccountId, Balance<T>),
    }


    // /// For description of error types, please have a look into module error.
    // #[pallet::error]
    // pub enum Error<T> {
    //     AuthorizationFailed,
    //     MachineNameExceedMax64,
    //     UnexpectedDidError
    // }

    // impl<T: Config> Error<T> {
    //     fn dispatch_error(err: MorError) -> DispatchResult {
    //         match err.typ {
    //             AuthorizationFailed => Err(Error::<T>::AuthorizationFailed.into()),
    //             MachineNameExceedMax64 => Err(Error::<T>::MachineNameExceedMax64.into()),
    //             UnexpectedDidError => Err(Error::<T>::UnexpectedDidError.into()),
    //         }
    //     }
    // }
    fn did_dispatch_error<T: PeaqDid::Config>(err: DidError) -> DispatchResult {
        match err {
            DidError::NotFound => Err(DidPalletErr::<T>::AttributeNotFound.into()),
            DidError::AlreadyExist => Err(DidPalletErr::<T>::AttributeAlreadyExist.into()),
            DidError::NameExceedMaxChar => {
                Err(DidPalletErr::<T>::AttributeNameExceedMax64.into())
            }
            DidError::FailedCreate => Err(DidPalletErr::<T>::AttributeCreationFailed.into()),
            DidError::FailedUpdate => Err(DidPalletErr::<T>::AttributeCreationFailed.into()),
            DidError::AuthorizationFailed => {
                Err(DidPalletErr::<T>::AttributeAuthorizationFailed.into())
            }
            DidError::MaxBlockNumberExceeded => {
                Err(DidPalletErr::<T>::MaxBlockNumberExceeded.into())
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
            machine: T::AccountId
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            DidPallet::<T>::is_owner(&sender, &machine)
                .map_err(|e| did_dispatch_error::<T>(e));

            let amount = <Balance<T>>::from(100000000000000000u128);
            Self::mint_to_account(sender, amount)
        }

        /// In this early version one can collect rewards for a machine, which has been online
        /// on the network for a certain period of time.
        #[pallet::weight(T::WeightInfo::some_extrinsic())]
        pub fn get_online_rewards(
            origin: OriginFor<T>,
            machine: T::AccountId
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            DidPallet::<T>::is_owner(&sender, &machine)
                .map_err(|e| did_dispatch_error::<T>(e));

            // 1 AGNG = 1_000_000_000_000_000_000
            let reward = Balance::<T>::from(100_000_000_000_000_000u128);
            todo!();

            Self::transfer_from_pot(sender, reward)
        }

        /// In this early version one can collect rewards for a machine, which has been online
        /// on the network for a certain period of time.
        #[pallet::weight(T::WeightInfo::some_extrinsic())]
        pub fn pay_machine_usage(
            origin: OriginFor<T>,
            owner: T::AccountId,
            machine: T::AccountId,
            amount: Balance<T>
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            DidPallet::<T>::is_owner(&sender, &machine)
                .map_err(|e| did_dispatch_error::<T>(e));

            Self::mint_to_account(sender, amount)
        }
    }

    impl<T: Config> MorBalance<T::AccountId, Balance<T>> for Pallet<T> {
        fn mint_to_account(account: T::AccountId, amount: Balance<T>) -> DispatchResult {
            let mut total_imbalance = <PositiveImbalance<T>>::zero();

            // See https://substrate.recipes/currency-imbalances.html
            dpatch_dposit_par!(
                T::Currency::deposit_into_existing(&account, amount),
                Event::MintedRewards(account.clone(), amount)
            )
        }

        fn transfer_from_pot(account: T::AccountId, amount: Balance<T>) -> DispatchResult {
            let pot: T::AccountId = T::PotId::get().into_account_truncating();
            T::Currency::transfer(pot, &account, amount, ExistenceRequirement::KeepAlive)
        }
    }

}