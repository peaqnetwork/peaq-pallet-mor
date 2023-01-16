//! This is only a template, the true file has to be generated
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn some_extrinsic() -> Weight;
}


pub struct SubstrateWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: XXX (r:1 w:1)
	fn some_extrinsic() -> Weight {
		Weight::from_ref_time(20_419_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}