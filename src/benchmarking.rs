//! Benchmarking setup for peaq-pallet-mor.

use super::*;

#[allow(unused)]
use crate::Pallet as Pallet;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite};
use frame_system::{Pallet as System, RawOrigin};

/// Assert that the last event equals the provided one.
fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
    System::<T>::assert_last_event(generic_event.into());
}

const CALLER_ACCOUNT_STR: &str = "Peaq";

benchmarks! {
    where_clause { where
    }

    some_extrinsic {
        let caller : T::AccountId = account(CALLER_ACCOUNT_STR, 0, 0);
    }: _(RawOrigin::Signed(caller.clone()))
    verify {
        assert_last_event::<T>(Event::<T>::SomeEvent(
            caller.clone()
        ).into());
    }
}

impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
