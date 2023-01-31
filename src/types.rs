//! All pallet relevant structs are defined here

use codec::{Decode, Encode};
use frame_support::traits::Currency;
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::RuntimeDebug;
use sp_runtime::traits::Zero;

/// Short form type definition to simplify method definition.
pub type BalanceOf<T> =
    <<T as crate::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
/// Short form type definition to simplify method definition. This definition is neccessary
/// due to the tight coupling of another pallet (Peaq-DID).
pub type WeightOf<T> = <T as crate::Config>::WeightInfo;

/// This struct defines the configurable paramters of the Peaq-MOR pallet. All contained
/// parameters can be configured by a dispatchable function (extrinsic).
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct MorConfig<Balance: Zero> {
    /// How much tokens a machine owner gets rewarded, when registering a new machine on the network.
    #[codec(compact)]
    pub registration_reward: Balance,
    /// Defines how much time is one period (to be online to get rewarded).
    #[codec(compact)]
    pub time_period_blocks: u8,
}

impl<Balance: Zero> Default for MorConfig<Balance> {
    fn default() -> Self {
        MorConfig {
            // This should be 100_000_000_000_000_000, but requires to many trait bounds
            registration_reward: Balance::zero(),
            // Equals 20 minutes
            time_period_blocks: 200,
        }
    }
}
