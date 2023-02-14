//! Benchmarking setup for peaq-pallet-mor.

use super::*;

use crate::{
    mock_const::*,
    Pallet as PeaqMor,
    types::{BalanceOf, MorConfig},
};
use peaq_pallet_did::Pallet as PeaqDid;
use frame_benchmarking::{
    account, benchmarks, impl_benchmark_test_suite
};
use frame_system::{Pallet as System, RawOrigin};
use sp_runtime::traits::Zero;

/// Assert that the last event equals the provided one.
fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
    System::<T>::assert_last_event(generic_event.into());
}


benchmarks! {
    where_clause { where
        BalanceOf<T>: From<u128> + Zero
    }

    get_registration_reward {
        let owner: T::AccountId = account(O_ACCT, 0, 0);
        let machine: T::AccountId = account(M_ACCT, 0, 0);
        PeaqDid::<T>::add_attribute(
            RawOrigin::Signed(owner.clone()).into(),
            machine.clone(),
            M_ATTR.to_vec(),
            M_VAL.to_vec(),
            None
        ).expect("check unit-tests");
    }: _(RawOrigin::Signed(owner.clone()), machine.clone())
    verify {
        assert_last_event::<T>(Event::<T>::RegistrationRewardPayed(
            owner.clone(), BalanceOf::<T>::from(REG_FEE)
        ).into());
    }

    get_online_rewards {
        let owner: T::AccountId = account(O_ACCT, 0, 0);
        let machine: T::AccountId = account(M_ACCT, 0, 0);
        PeaqDid::<T>::add_attribute(
            RawOrigin::Signed(owner.clone()).into(),
            machine.clone(),
            M_ATTR.to_vec(),
            M_VAL.to_vec(),
            None
        ).expect("check unit-tests");
        PeaqMor::<T>::get_registration_reward(
            RawOrigin::Signed(owner.clone()).into(),
            machine.clone()
        ).expect("check unit-tests");
    }: _(RawOrigin::Signed(owner.clone()), machine.clone())
    verify {
        assert_last_event::<T>(Event::<T>::OnlineRewardsPayed(
            owner.clone(), BalanceOf::<T>::zero()
        ).into());
    }

    pay_machine_usage {
        let user: T::AccountId = account(U_ACCT, 0, 0);
        let machine: T::AccountId = account(M_ACCT, 0, 0);
    }: _(RawOrigin::Signed(user.clone()), machine.clone(), BalanceOf::<T>::from(REG_FEE))
    verify {
        assert_last_event::<T>(Event::<T>::MachineUsagePayed(
            machine, BalanceOf::<T>::from(REG_FEE)
        ).into());
    }

    set_configuration {
        let config: MorConfig<BalanceOf<T>> = MorConfig::<BalanceOf<T>>{ 
            registration_reward: BalanceOf::<T>::from(REG_FEE),
            machine_usage_fee_min: BalanceOf::<T>::from(100_000_000_000_000_000u128),
            machine_usage_fee_max: BalanceOf::<T>::from(3_000_000_000_000_000_000u128),
            track_n_block_rewards: 10u8
        };
    }: _(RawOrigin::Root, config.clone())
    verify {
        assert_last_event::<T>(Event::<T>::MorConfigChanged(
            config
        ).into());
    }

    fetch_pot_balance {
    }: _(RawOrigin::Root)
    verify {
        // assert_last_event::<T>(Event::<T>::FetchedPotBalance(
        //     BalanceOf::<T>::from(10_000_000_000_000_000_000u128)
        // ).into());
    }
}

impl_benchmark_test_suite!(PeaqMor, crate::mock::new_test_ext(), crate::mock::Test);
