//! All pallet relevant structs are defined here

use codec::{Decode, Encode};
use frame_support::traits::Currency;
use frame_support::traits::tokens::Balance as BalanceT;
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::RuntimeDebug;
// use sp_runtime::traits::{Zero, One};


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
pub struct MorConfig<Balance> {
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

impl<Balance: BalanceT> MorConfig<Balance> {
    /// Method checks whether configuration is cons
    pub fn is_consistent(&self, existential_deposit: Balance) -> bool {
        // this parameter affects resulting vector size, therefor not allowed to be zero!
        let blocks = self.track_n_block_rewards > 0;
        let range_usage = self.machine_usage_fee_max > self.machine_usage_fee_min;
        let range_min = self.registration_reward > existential_deposit 
            && self.machine_usage_fee_min > existential_deposit;

        blocks && range_usage && range_min
    }
}

impl<Balance: BalanceT> Default for MorConfig<Balance> {
    fn default() -> Self {
        MorConfig {
            // Because Balance can only be set to zero to keep the pallet as generic 
            // as possible - set every parameter to zero! Except for track_n_block_rewards!
            // -> an initial configuration has to be done in Genesis or after deployment...
            registration_reward: Balance::zero(),
            machine_usage_fee_min: Balance::zero(),
            machine_usage_fee_max: Balance::one(),
            track_n_block_rewards: 1,
        }
    }
}