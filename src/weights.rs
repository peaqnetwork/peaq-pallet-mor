//! This is only a template, the true file has to be generated

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn get_registration_reward() -> Weight;
	fn get_online_rewards() -> Weight;
	fn pay_machine_usage() -> Weight;
	fn set_configuration() -> Weight;
	fn fetch_pot_balance() -> Weight;
}


pub struct SubstrateWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: XXX (r:1 w:1)
	fn get_registration_reward() -> Weight {
		Weight::from_ref_time(20_419_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}

	fn get_online_rewards() -> Weight {
		Self::get_registration_reward()
	}

	fn pay_machine_usage() -> Weight {
		Self::get_registration_reward()
	}
	
	fn set_configuration() -> Weight {
		Self::get_registration_reward()
	}
	
	fn fetch_pot_balance() -> Weight {
		Self::get_registration_reward()
	}
	
}