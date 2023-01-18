//! In this module are the traits of the pallet defined.

use sp_std::vec::Vec;

use crate::structs::*;

pub use crate::error::Result;


/// Trait defines the basic behaviour of Machine Owners Rewards (MOR)
pub trait Mor<MachineId, AccountId> {
    /// This method registers a new machine on the network and rewards the owner once
    fn register_machine(owner: &AccountId, machine: &MachineId) -> Result<()>;
    
    /// This method rewards machine owners for their machines beeing online on
    /// the network for a certain period of time
    fn get_rewarded(owner: &AccountId, machine: &MachineId) -> Result<()>;
}


/// Trait defines internal behaviour in relation to a certain machine
pub trait MachineAdm<MachineId, AccountId> {
    /// Creates a new machine entry and adds to storage
    fn add_machine(owner: &AccountId, machine: &MachineId, desc: &MachineDesc) -> Result<()>;

    /// Updates the owner's account of a registered machine
    fn update_account(owner: &AccountId, new_owner: &AccountId, machine: &MachineId) -> Result<()>;

    /// Disables an existing machine
    fn disable_machine(owner: &AccountId, machine: &MachineId) -> Result<()>;

    /// Getter method for machines in storage
    fn get_machine(owner: &AccountId, machine: &MachineId) -> Result<Machine>;

    /// Getter method for all machines in storage related to one account (owner)
    fn get_machines(owner: &AccountId) -> Result<Vec<Machine>>;
}