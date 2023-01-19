//! In this module are the traits of the pallet defined.

use frame_support::traits::Currency;
use sp_std::vec::Vec;

use crate::structs::*;

pub use crate::error::Result;


// Pallet specific type definitions
pub type Balance<T> = <<T as crate::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;


/// Trait defines the basic behaviour of Machine Owners Rewards (MOR)
pub trait Mor<AccountId, MachineId> {
    /// This method registers a new machine on the network and rewards the owner once.
    fn register_new_machine(owner: &AccountId, machine: &MachineId, desc: &MachineDesc) -> Result<()>;
    
    /// This method rewards machine owners for their machines beeing online on
    /// the network for a certain period of time.
    fn get_online_rewards(owner: &AccountId, machine: &MachineId) -> Result<()>;
}


/// Trait defines behaviour of Pot- and Reward-Mechanism
pub trait PotAdm<AccountId, Balance> {
    /// Get a unique, inaccessible account id from the `PotId`.
	fn account_id() -> AccountId;

    /// Pours balance from the pot to an account (rewards an owner).
    fn do_reward(pot: &AccountId, who: &AccountId, reward: Balance);

    // Distributes all available ballance from the pot to machine owners.
    // fn distribute_rewards();
    /// In this version one account gets all available rewards on request.
    fn get_available_rewards(owner: &AccountId);

    /// Rewards machine owner once for registering a new machine.
    fn get_registration_reward(owner: &AccountId);
}


/// Trait defines internal behaviour in relation to a certain machine
pub trait MachineAdm<AccountId, MachineId> {
    /// Creates a new machine entry and adds to storage.
    fn add_machine(owner: &AccountId, machine: &MachineId, desc: &MachineDesc) -> Result<()>;

    /// Updates the owner's account of a registered machine.
    fn update_account(owner: &AccountId, new_owner: &AccountId, machine: &MachineId) -> Result<()>;

    /// Disables an existing machine.
    fn disable_machine(owner: &AccountId, machine: &MachineId) -> Result<()>;

    /// Getter method for machines in storage.
    fn get_machine(owner: &AccountId, machine: &MachineId) -> Result<Machine>;

    /// Getter method for all machines in storage related to one account (owner).
    fn get_machines(owner: &AccountId) -> Result<Vec<Machine>>;
}