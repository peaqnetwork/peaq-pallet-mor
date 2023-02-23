//! In this module are all traits of the pallet defined. These traits define the core
//! behaviour of the pallet and its functionality.

use frame_support::pallet_prelude::DispatchResult;

use crate::error::MorResult;

/// The trait `MorBalance` describes relevant functionality related to tokens. If
/// tokens will be minted or transfered from the pot is implemented here. Also a
/// method to track the collected block-rewards is listed here.
pub trait MorBalance<AccountId, Balance> {
    /// Core function to mint new tokens and transfer them to a given account.
    fn mint_to_account(account: &AccountId, amount: Balance) -> DispatchResult;

    /// Core function to transfer tokens from the pallet's pot to a given account.
    fn transfer_from_pot(account: &AccountId, amount: Balance) -> DispatchResult;

    /// The pallet shall keep track of the last N block-rewards, which have been collected.
    /// When a machine owner requests the online-reward he shall be rewarded in the same
    /// amount, that has been collected in the last time period.
    fn log_block_rewards(amount: Balance);

    /// When the configuration of the pallet will be changed, the storage size changes
    /// too. This method will reorganize the storage of the pallet and adapt its content.
    fn resize_track_storage(new_size: u8);
}

/// The trait `MorMachine` encapsules adminstrative methods related to machines.
pub trait MorMachine<AccountId, Balance> {
    /// Internal registration of a machine, to track which machines have been registered
    /// or not. The purpose is to prevent registering the same machine multiple times or
    /// registering a machine, which is not listed in Peaq-DID.
    fn register_machine(owner: &AccountId, machine: &AccountId) -> MorResult<Balance>;

    /// Internal functionality to be used by the dispatchable method.
    fn reward_machine(owner: &AccountId, machine: &AccountId) -> MorResult<Balance>;
}
