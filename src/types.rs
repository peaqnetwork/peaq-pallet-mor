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
    /// Minimum balance limit for machine usage payments
    #[codec(compact)]
    pub machine_usage_fee_min: Balance,
    /// Maximum balance limit for machine usage payments
    #[codec(compact)]
    pub machine_usage_fee_max: Balance,
    /// Defines how much how much block rewards will be tracked in the past (to build a sum of them)
    #[codec(compact)]
    pub track_n_block_rewards: u8,
}

impl<Balance: Zero> Default for MorConfig<Balance> {
    fn default() -> Self {
        MorConfig {
            // Because Balance can only be set to zero to keep the pallet
            // as generic as possible - set every parameter to zero!
            // -> an initial configuration has to be done in Genesis or
            //    after deployment...
            registration_reward: Balance::zero(),
            machine_usage_fee_min: Balance::zero(),
            machine_usage_fee_max: Balance::zero(),
            track_n_block_rewards: 0,
        }
    }
}
