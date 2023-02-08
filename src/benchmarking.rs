//! Benchmarking setup for peaq-pallet-mor.

use super::*;

use crate::{
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

pub(crate) const O_ACCT: &'static str = "Alice"; // Owner
pub(crate) const U_ACCT: &'static str = "Bob"; // User
pub(crate) const M_ACCT: &'static str = "Charlie"; // Machine
pub(crate) const M_ATTR: &[u8] = b"Attribute";
pub(crate) const M_VAL: &[u8] = b"Value";
const REG_FEE: u128 = 100_000_000_000_000_000u128;


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
        let owner: T::AccountId = account(O_ACCT, 0, 0);
        let config: MorConfig<BalanceOf<T>> = MorConfig::<BalanceOf<T>>::default();
    }: _(RawOrigin::Signed(owner.clone()), config.clone())
    verify {
        assert_last_event::<T>(Event::<T>::MorConfigChanged(
            config
        ).into());
    }

    fetch_pot_balance {
        let owner: T::AccountId = account(O_ACCT, 0, 0);
    }: _(RawOrigin::Signed(owner.clone()))
    verify {
        assert_last_event::<T>(Event::<T>::FetchedPotBalance(
            BalanceOf::<T>::zero()
        ).into());
    }
}

impl_benchmark_test_suite!(PeaqMor, crate::mock::new_test_ext(), crate::mock::Test);
