//! All pallet relevant structs are defined here
use codec::{Decode, Encode};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::RuntimeDebug;
use sp_std::vec::Vec;


/// Machine struct stores information about one registered machine. The owner can setup
/// one account for getting rewarded. A machine cannot be deleted, but be disabled instead.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(
    Clone, PartialEq, Eq, PartialOrd, Ord, Default, TypeInfo, Decode, Encode, RuntimeDebug,
)]
pub struct Machine {
    /// Name of the registered machine
    pub name: Vec<u8>,
    /// Enabled flag
    pub enabled: bool,
}


/// Method provides a name suggestion based on the type of machine, it's location,
/// name of the institution/company/owner and a counting number.
pub fn suggest_machine_name(location: &str, typ: &str, owner: &str, count: usize) -> Vec<u8> {
    todo!();
    Vec::new()
}