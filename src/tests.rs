//! Unit tests for this pallet, see spec definition

use frame_support::{assert_ok, assert_noop};
use sp_core::sr25519::Public;
// use sp_std::vec;
use crate::{
    mock::*,
    Error, types::CrtBalance,
};


// Defined in moch.rs:
// const O_ACCT: &'static str
// const U_ACCT: &'static str
// const M_ACCT: &'static str

const M_ATTR: &[u8] = b"Attribute";
const M_VAL: &[u8] = b"Value";


fn pt_register_machine(owner: Public, machine: Public) {
    // Register at least one attribute on Peaq-DID.
    // Expect no error.
    assert_ok!(PeaqDid::add_attribute(
        Origin::signed(owner),
        machine,
        M_ATTR.to_vec(),
        M_VAL.to_vec(),
        None
    ));

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
        // Expect error AuthorizationFailed.
        assert_noop!(
            PeaqMor::register_new_machine(
                Origin::signed(owner),
                machine),
            Error::<Test>::AuthorizationFailed
        );

        pt_register_machine(owner, machine);
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
            PeaqMor::get_online_rewards(
                Origin::signed(owner),
                machine),
            Error::<Test>::AuthorizationFailed
        );
        
        // Register new machine.
        pt_register_machine(owner, machine);

        // Now get the online rewards.
        // Expect no error.
        assert_ok!(PeaqMor::get_online_rewards(
            Origin::signed(owner),
            machine
        ));
    });
}


#[test]
fn pay_machine_usage_test() {
    new_test_ext().execute_with(|| {
        let owner = account_key(O_ACCT);
        let muser = account_key(U_ACCT);
        let machine = account_key(M_ACCT);
        let amount = CrtBalance::<Test>::from(1_000_000u32);

        // Try to pay a non-existing machine.
        // Expect error ???.
        assert_noop!(
            PeaqMor::pay_machine_usage(
                Origin::signed(muser),
                machine,
                amount),
            Error::<Test>::AuthorizationFailed
        );

        // Register new machine.
        pt_register_machine(owner, machine);

        // Now try to use it.
        // Expect no error.
        assert_ok!(PeaqMor::pay_machine_usage(
            Origin::signed(muser),
            machine,
            amount
        ));
    });
}