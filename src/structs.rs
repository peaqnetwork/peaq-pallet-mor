//! All pallet relevant structs are defined here
use codec::{Decode, Encode};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::RuntimeDebug;
// use sp_std::vec::Vec;

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(
    Clone, PartialEq, Eq, PartialOrd, Ord, Default, TypeInfo, Decode, Encode, RuntimeDebug,
)]
pub struct SomeData {
    //// data description goes here
    pub data: u8,
}