//! In this module are all traits of the pallet defined. These traits define the
//! behaviour of the pallet and its functionality. 
//! 
//! The main trait, which represents the top level use cases of this pallet, is ```Mor```.
//! Remaining traits will implement the functionality needed by the ```Mor``` trait.
//! These traits are ```PotAdm``` and ```MachineAdm```, where "Adm" is a short form for
//! administration. Have a look at each trait definition.

use frame_support::pallet_prelude::DispatchResult;

// use crate::types::AmountType;


pub trait MorBalance<AccountId, Balance> {
    fn mint_to_account(account: AccountId, amount: Balance) -> DispatchResult;

    fn transfer_from_pot(account: AccountId, amount: Balance) -> DispatchResult;
}