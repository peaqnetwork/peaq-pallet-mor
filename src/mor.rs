//! In this module are all traits of the pallet defined. These traits define the
//! behaviour of the pallet and its functionality. 
//! 
//! The main trait, which represents the top level use cases of this pallet, is ```Mor```.
//! Remaining traits will implement the functionality needed by the ```Mor``` trait.
//! These traits are ```PotAdm``` and ```MachineAdm```, where "Adm" is a short form for
//! administration. Have a look at each trait definition.

use frame_support::pallet_prelude::DispatchResult;

use crate::error::MorResult;


/// TODO
pub trait MorBalance<AccountId, Balance> {
    /// TODO
    fn mint_to_account(account: AccountId, amount: Balance) -> DispatchResult;

    /// TODO
    fn transfer_from_pot(account: AccountId, amount: Balance) -> DispatchResult;

    /// TODO
    fn log_block_rewards(amount: Balance);
}


/// TODO
pub trait MorMachine<AccountId> {
    /// TODO
    fn register_machine(account: &AccountId) -> MorResult<()>;
}