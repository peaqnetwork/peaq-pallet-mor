//! In this module are all traits of the pallet defined. These traits define the
//! behaviour of the pallet and its functionality. 
//! 
//! The main trait, which represents the top level use cases of this pallet, is ```Mor```.
//! Remaining traits will implement the functionality needed by the ```Mor``` trait.
//! These traits are ```PotAdm``` and ```MachineAdm```, where "Adm" is a short form for
//! administration. Have a look at each trait definition.

use frame_support::traits::Currency;
use sp_std::vec::Vec;

use crate::structs::*;

pub use crate::error::Result;


/// Short form type definition to simplify method definition.
pub type Balance<T> = <<T as crate::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;


/// Trait defines the top level behaviour of Machine Owners Rewards Pallet (Pallet-MOR). Users
/// will find here the use cases of this pallet. Every of these methods will be mirrored on 
/// dispatchable funtions (extrinsics). Some may be callable via RPC.
/// 
/// These methods will be called internally either by dispatchable functions or via RPC interface.
pub trait Mor<AccountId, MachineId> {
    /// This method registers a new machine on the network and rewards the owner once.
    /// Errors will occur, if the owner does not own this machine, the machine already
    /// exists.
    fn register_new_machine(owner: &AccountId, machine: &MachineId, name: &Vec<u8>) -> Result<()>;
    
    /// This method rewards machine owners for their machines beeing online on
    /// the network for a certain period of time.
    fn get_online_rewards(owner: &AccountId, machine: &MachineId) -> Result<()>;

    // Implemented in MachineAdm
    // fn disable_machine(owner: &AccountId, machine: &MachineId) -> Result<()>;

    // Implemented in MachineAdm
    // fn enable_machine(owner: &AccountId, machine: &MachineId) -> Result<()>;
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


/// Trait defines internal behaviour in relation to a certain machine, to be able
/// to administrate machines, e.g. adding one, update its owner account, enable and
/// disable it.
/// 
/// First of all: a machine can be registered only once. If a machine ID already
/// exists, an error will occur. Modifications on a machine can be done if the
/// owner exists, the machine exists and if it is enabled, otherwise an error will
/// be raised.
pub trait MachineAdm<AccountId, MachineId> {
    /// Creates a new machine entry and adds to storage. Error can occur
    /// if the machine is already registered. This will also happen when
    /// another user has already registered this machine ID.
    fn add_machine(owner: &AccountId, machine: &MachineId, desc: &Vec<u8>) -> Result<()>;

    /// Updates the owner's account of a registered machine. Errors will
    /// be raised if the machine or the owner doesn't exist.
    fn update_account(owner: &AccountId, new_owner: &AccountId, machine: &MachineId) -> Result<()>;

    /// Enables a disabled machine. An error will be raised if the machine
    /// is already enabled or if it doesn't exist.
    fn enable_machine(owner: &AccountId, machine: &MachineId) -> Result<()>;

    /// Disables an existing machine. An error will be raised if the machine
    /// is already disabled or if it doesn't exist.
    fn disable_machine(owner: &AccountId, machine: &MachineId) -> Result<()>;

    /// Getter method for machines in storage. An error will occur if the
    /// machine or the owner doesn't exist, or if the machine is disabled.
    fn get_machine(owner: &AccountId, machine: &MachineId) -> Result<Machine>;

    /// Getter method for machines, but this ones forces it. Will return
    /// disabled machines too.
    fn get_machine_force(owner: &AccountId, machine: &MachineId) -> Result<Machine>;

    /// Getter method for all machines in storage related to one account (owner).
    fn get_machines(owner: &AccountId) -> Result<Vec<Machine>>;
}