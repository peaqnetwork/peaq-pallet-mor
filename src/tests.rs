//! Unit tests for this pallet, see spec definition

use crate::{
    mock::*,
    mor::MorBalance,
    types::{BalanceOf, MorConfig},
    Error,
};
use frame_support::{assert_noop, assert_ok};
use sp_core::sr25519::Public;
use sp_runtime::traits::BadOrigin;


fn register_machine_did(owner: Public, machine: Public) {
    // Register at least one attribute on Peaq-DID.
    // Expect no error.
    assert_ok!(PeaqDid::add_attribute(
        Origin::signed(owner),
        machine,
        M_ATTR.to_vec(),
        M_VAL.to_vec(),
        None
    ));
}

fn get_registration_reward_mor(owner: Public, machine: Public) {
    // Request rewards for new machine on Peaq-MOR.
    // Expect no error.
    assert_ok!(PeaqMor::get_registration_reward(
        Origin::signed(owner),
        machine
    ));
}

fn def_config(
    registration_reward: BalanceOf<Test>,
    machine_usage_fee_min: BalanceOf<Test>,
    machine_usage_fee_max: BalanceOf<Test>,
    track_n_block_rewards: u8
) -> MorConfig<BalanceOf<Test>> {
    MorConfig {
        registration_reward,
        machine_usage_fee_min,
        machine_usage_fee_max,
        track_n_block_rewards,
    }
}

#[test]
fn register_new_machine_test() {
    new_test_ext().execute_with(|| {
        let owner = account_key(O_ACCT);
        let machine = account_key(M_ACCT);

        // Try to register new machine on Peaq-MOR, which is not registered in Peaq-DID.
        // Expect error DidAuthorizationFailed.
        assert_noop!(
            PeaqMor::get_registration_reward(Origin::signed(owner), machine),
            Error::<Test>::DidAuthorizationFailed
        );

        register_machine_did(owner, machine);
        get_registration_reward_mor(owner, machine);
    });
}

#[test]
fn get_online_rewards_test() {
    new_test_ext().execute_with(|| {
        let owner = account_key(O_ACCT);
        let machine = account_key(M_ACCT);

        // Try to collect rewards. No machines registered.
        // Expect error AuthorizationFailed.
        assert_noop!(
            PeaqMor::get_online_rewards(Origin::signed(owner), machine),
            Error::<Test>::DidAuthorizationFailed
        );

        // Register new machine only in Peaq-DID.
        register_machine_did(owner, machine);

        // Try to register new machine on Peaq-MOR, which is only registered in Peaq-DID.
        // Expect error MorAuthorizationFailed.
        assert_noop!(
            PeaqMor::get_online_rewards(Origin::signed(owner), machine),
            Error::<Test>::MachineNotRegistered
        );

        // Now register machine in Peaq-MOR too.
        get_registration_reward_mor(owner, machine);

        // Now get the online rewards.
        // Expect no error.
        assert_ok!(PeaqMor::get_online_rewards(Origin::signed(owner), machine));
    });
}

#[test]
fn pay_machine_usage_test() {
    new_test_ext().execute_with(|| {
        let muser = account_key(U_ACCT);
        let machine = account_key(M_ACCT);
        let amount = BalanceOf::<Test>::from(500_000_000_000_000_000u128);
        let amount_oor = BalanceOf::<Test>::from(5_000_000_000_000_000_000u128);

        // Try to pay for machine usage.
        // Expect no error.
        assert_ok!(PeaqMor::pay_machine_usage(
            Origin::signed(muser),
            machine,
            amount
        ));

        // Try to pay out of range.
        // Expect error MachinePaymentOutOfRange.
        assert_noop!(
            PeaqMor::pay_machine_usage(
                Origin::signed(muser),
                machine,
                amount_oor
            ),
            Error::<Test>::MachinePaymentOutOfRange
        );
    });
}

#[test]
fn set_configuration_test() {
    new_test_ext().execute_with(|| {
        let b_low = BalanceOf::<Test>::from(100_000_000_000_000u128);
        let b_med = BalanceOf::<Test>::from(500_000_000_000_000u128);
        let b_max = BalanceOf::<Test>::from(2_500_000_000_000_000u128);

        // Try to set invalid configuration (range of balances).
        // Expect error MorConfigIsNotConsistent.
        let config = def_config(b_low, b_med, b_med, 50);
        assert_noop!(
            PeaqMor::set_configuration(Origin::root(), config),
            Error::<Test>::MorConfigIsNotConsistent
        );

        // Try to set invalid configuration (number of blocks).
        // Expect error MorConfigIsNotConsistent.
        let config = def_config(b_low, b_med, b_med, 0);
        assert_noop!(
            PeaqMor::set_configuration(Origin::root(), config),
            Error::<Test>::MorConfigIsNotConsistent
        );

        // Set valid configuration.
        // Expect no error.
        let config = def_config(b_low, b_med, b_max, 50);
        assert_ok!(PeaqMor::set_configuration(Origin::root(), config));
    });
}

#[test]
fn fetch_pot_balance_test() {
    new_test_ext().execute_with(|| {
        let muser = account_key(U_ACCT);

        // Try to fetch configuration details as regular user.
        // Expect error BadOrigin.
        assert_noop!(
            PeaqMor::fetch_pot_balance(Origin::signed(muser)),
            BadOrigin
        );

        // Try to fetch current pot-balance of the pallet.
        // Expect no error.
        assert_ok!(PeaqMor::fetch_pot_balance(Origin::root()));
    });
}

#[test]
fn log_block_rewards_test() {
    new_test_ext().execute_with(|| {
        let balance = BalanceOf::<Test>::from(100_000_000_000_000_000u128);
        // Try to log new block-reward in default genesis configuration.
        // Expect no error.
        PeaqMor::log_block_rewards(balance);
    });
}