//! All pallet relevant structs are defined here

use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use frame_support::traits::{
    tokens::Balance as BalanceT, Currency,
};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::RuntimeDebug;
use sp_runtime::{
    Perbill,
    traits::Zero,
};

/// Short form type definition to simplify method definition.
pub type BalanceOf<T> =
    <<T as crate::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
/// Short form type definition to simplify method definition. This definition is neccessary
/// due to the tight coupling of another pallet (Peaq-DID).
pub type WeightOf<T> = <T as crate::Config>::WeightInfo;

/// This struct defines the configurable paramters of the Peaq-MOR pallet. All contained
/// parameters can be configured by a dispatchable function (extrinsic).
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct MorConfig<Balance> 
where Balance: BalanceT + MaxEncodedLen
{
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

// TO BE REMOVED WHEN MERGING PEAQ-FRAME-EXT AND THE CLAIM-MECHANISM!!!
// This is a copy-paste-solution, to do a quick-fix/workarround. Not to be meant to implement this
// here, it is part of the block-reward-pallet and shall be connected when merging all the changes
// due to the claim-mechanism in PEAQ.
#[derive(PartialEq, Eq, Clone, Encode, Default, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct DiscreteAverage<Balance>
where
	Balance: Zero + BalanceT,
{
	/// The average value.
	pub avg: Balance,
	/// Accumulator for building the next average value.
	pub(crate) accu: Balance,
	/// Number of blocks to averaged over.
	pub(crate) n_period: u32,
	/// Counter of blocks.
	pub(crate) cnt: u32,
}

impl<Balance> DiscreteAverage<Balance>
where
	Balance: Zero + BalanceT,
{
	/// New type pattern.
	pub fn new(avg: Balance, n_period: u32) -> DiscreteAverage<Balance> {
		assert!(avg > Balance::zero());
		DiscreteAverage { avg, accu: Balance::zero(), n_period, cnt: 0u32 }
	}

	/// Updates the average-value for a balance, shall be called each block.
	pub fn update(&mut self, next: &Balance) {
		self.accu += *next;
		self.cnt += 1u32;
		if self.cnt == self.n_period {
			self.avg = Perbill::from_rational(1u32, self.n_period) * self.accu;
			self.accu = Balance::zero();
			self.cnt = 0u32;
		}
	}
}

// Short form for the DiscreteAverage<BalanceOf<T>, Count>
pub(crate) type DiscAvg<T> = DiscreteAverage<BalanceOf<T>>;

