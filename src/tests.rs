//! Unit tests for this pallet, see spec definition

use crate::{
    mock::*,
    types::{BalanceOf, MorConfig},
    Error,
};
use frame_support::{assert_noop, assert_ok};
use sp_core::sr25519::Public;
use sp_runtime::traits::BadOrigin;

// Defined in moch.rs:
// const O_ACCT: &'static str
// const U_ACCT: &'static str
// const M_ACCT: &'static str

const M_ATTR: &[u8] = b"Attribute";
const M_VAL: &[u8] = b"Value";

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

fn register_machine_mor(owner: Public, machine: Public) {
    // Register new machine on Peaq-MOR.
    // Expect no error.
    assert_ok!(PeaqMor::register_new_machine(
        Origin::signed(owner),
        machine
    ));
}

#[test]
fn register_new_machine_test() {
    new_test_ext().execute_with(|| {
        let owner = account_key(O_ACCT);
        let machine = account_key(M_ACCT);

        // Try to register new machine on Peaq-MOR, which is not registered in Peaq-DID.
        // Expect error DidAuthorizationFailed.
        assert_noop!(
            PeaqMor::register_new_machine(Origin::signed(owner), machine),
            Error::<Test>::DidAuthorizationFailed
        );

        register_machine_did(owner, machine);
        register_machine_mor(owner, machine);
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
        register_machine_mor(owner, machine);

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
        let amount = BalanceOf::<Test>::from(1_000_000u32);

        // Try to pay for machine usage.
        // Expect no error.
        assert_ok!(PeaqMor::pay_machine_usage(
            Origin::signed(muser),
            machine,
            amount
        ));
    });
}

#[test]
fn set_configuration_test() {
    new_test_ext().execute_with(|| {
        let config = MorConfig {
            registration_reward: BalanceOf::<Test>::from(500_000_000_000_000_000u128),
            time_period_blocks: 100,
        };

        // Try to set new configuration for the pallet.
        // Expect no error.
        assert_ok!(PeaqMor::set_configuration(Origin::root(), config));
    });
}

#[test]
fn fetch_configuration_test() {
    new_test_ext().execute_with(|| {
        let muser = account_key(U_ACCT);

        // Try to fetch configuration details as regular user.
        // Expect error BadOrigin.
        assert_noop!(
            PeaqMor::fetch_configuration(Origin::signed(muser)),
            BadOrigin
        );

        // Try to fetch current configuration of the pallet.
        // Expect no error.
        assert_ok!(PeaqMor::fetch_configuration(Origin::root()));
    });
}

#[test]
fn fetch_pot_balance_test() {
    new_test_ext().execute_with(|| {
        let muser = account_key(U_ACCT);

        // Try to fetch configuration details as regular user.
        // Expect error BadOrigin.
        assert_noop!(
            PeaqMor::fetch_configuration(Origin::signed(muser)),
            BadOrigin
        );

        // Try to fetch current pot-balance of the pallet.
        // Expect no error.
        assert_ok!(PeaqMor::fetch_pot_balance(Origin::root()));
    });
}

#[test]
fn fetch_period_rewarding_test() {
    new_test_ext().execute_with(|| {
        let muser = account_key(U_ACCT);

        // Try to fetch configuration details as regular user.
        // Expect error BadOrigin.
        assert_noop!(
            PeaqMor::fetch_configuration(Origin::signed(muser)),
            BadOrigin
        );

        // Try to fetch current pot-balance of the pallet.
        // Expect no error.
        assert_ok!(PeaqMor::fetch_period_rewarding(Origin::root()));
    });
}
