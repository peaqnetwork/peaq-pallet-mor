//! All pallet relevant structs are defined here

use codec::{Decode, Encode};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::RuntimeDebug;
use frame_support::traits::Currency;


/// Short form type definition to simplify method definition.
pub type CrtBalance<T> = <<T as crate::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
/// Short form type definition to simplify method definition.
pub type CrtWeight<T> = <T as crate::Config>::WeightInfo;


/// This struct defines the configurable paramters of the Peaq-MOR pallet. All contained
/// parameters can be configured by a dispatchable function (extrinsic).
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(
    PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo //, MaxEncodedLen
)]
pub struct MorConfig<Balance> {
    /// How much tokens a machine owner gets rewarded, when registering a new machine on the network.
    registration_reward: Balance,
    /// Defines how much time is one period (to be online to get rewarded).
    time_period_blocks: u8,
}
