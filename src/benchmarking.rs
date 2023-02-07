//! Benchmarking setup for peaq-pallet-mor.

use super::*;

#[allow(unused)]
use crate::{
    Pallet as PeaqMor,
    types::{BalanceOf, MorConfig},
};
use frame_benchmarking::{
    account, benchmarks, impl_benchmark_test_suite
};
use frame_system::{Pallet as System, RawOrigin};
use sp_runtime::traits::Zero;

/// Assert that the last event equals the provided one.
fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
    System::<T>::assert_last_event(generic_event.into());
}

const CALLER_ACC_STR: &str = "Alice";
const MACHINE_ACC_STR: &str = "Charlie";
const REG_FEE: u128 = 100_000_000_000_000_000u128;

benchmarks! {
    where_clause { where
        BalanceOf<T>: From<u128> + Zero
    }

    get_registration_reward {
        let caller: T::AccountId = account(CALLER_ACC_STR, 0, 0);
        let machine: T::AccountId = account(MACHINE_ACC_STR, 0, 0);
    }: _(RawOrigin::Signed(caller.clone()), machine)
    verify {
        assert_last_event::<T>(Event::<T>::RegistrationRewardPayed(
            caller.clone(), BalanceOf::<T>::from(REG_FEE)
        ).into());
    }

    get_online_rewards {
        let caller: T::AccountId = account(CALLER_ACC_STR, 0, 0);
        let machine: T::AccountId = account(MACHINE_ACC_STR, 0, 0);
    }: _(RawOrigin::Signed(caller.clone()), machine)
    verify {
        assert_last_event::<T>(Event::<T>::OnlineRewardsPayed(
            caller.clone(), BalanceOf::<T>::zero()
        ).into());
    }

    pay_machine_usage {
        let caller: T::AccountId = account(CALLER_ACC_STR, 0, 0);
        let machine: T::AccountId = account(MACHINE_ACC_STR, 0, 0);
    }: _(RawOrigin::Signed(caller.clone()), machine.clone(), BalanceOf::<T>::from(REG_FEE))
    verify {
        assert_last_event::<T>(Event::<T>::MachineUsagePayed(
            machine, BalanceOf::<T>::from(REG_FEE)
        ).into());
    }

    set_configuration {
        let caller: T::AccountId = account(CALLER_ACC_STR, 0, 0);
        let config: MorConfig<BalanceOf<T>> = MorConfig::<BalanceOf<T>>::default();
    }: _(RawOrigin::Signed(caller.clone()), config.clone())
    verify {
        assert_last_event::<T>(Event::<T>::MorConfigChanged(
            config
        ).into());
    }

    fetch_pot_balance {
        let caller: T::AccountId = account(CALLER_ACC_STR, 0, 0);
    }: _(RawOrigin::Signed(caller.clone()))
    verify {
        assert_last_event::<T>(Event::<T>::FetchedPotBalance(
            BalanceOf::<T>::zero()
        ).into());
    }
}

impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
