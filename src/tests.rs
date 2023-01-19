//! Unit tests for this pallet, spec definition
//! 
use frame_support::{assert_ok, assert_noop};
use crate::{
    mock::*,
    Error,
    structs::MachineDesc,
};


#[test]
fn register_new_machine_test() {
    new_test_ext().execute_with(|| {
        let acct = "Micha";
        let origin = account_key(acct);
        let machine = *b"21676474666576474646673646376637";
        let desc = MachineDesc::from_terms(
            "owner",
            "location",
            "typ",
            1
        );

        // Register new machine on the network.
        // Expect no error.
        assert_ok!(PeaqPalletMor::register_new_machine(
            Origin::signed(origin),
            origin,
            machine,
            desc
        ));
    });
}


#[test]
fn fetch_machine_test() {
    new_test_ext().execute_with(|| {
        let acct = "Micha";
        let origin = account_key(acct);
        let machine = *b"21676474666576474646673646376637";
        let machine_err = *b"21676474666576474646673646376638";
        let desc = MachineDesc::from_terms(
            "owner",
            "location",
            "typ",
            1
        );

        // Fetch a machine, but no machines are registered.
        // Expect OwnerDoesNotExist error
        assert_noop!(
            PeaqPalletMor::fetch_machine(
                Origin::signed(origin),
                origin,
                machine
            ),
            Error::<Test>::OwnerDoesNotExist
        );

        // Register new machine for further testing.
        assert_ok!(PeaqPalletMor::register_new_machine(
            Origin::signed(origin),
            origin,
            machine,
            desc
        ));

        // Fetch registered machine.
        // Expect to be able to fetch this registered machine.
        assert_ok!(PeaqPalletMor::fetch_machine(
            Origin::signed(origin),
            origin,
            machine
        ));

        // Request wrong machine-ID from owner, which is registered.
        // Expect error MachineDoesNotExist
        assert_noop!(
            PeaqPalletMor::fetch_machine(
                Origin::signed(origin),
                origin,
                machine_err
            ),
            Error::<Test>::MachineDoesNotExist
        );
    });
}


#[test]
fn get_online_rewards_test() {
    new_test_ext().execute_with(|| {
        let acct = "Micha";
        let origin = account_key(acct);
        let machine = *b"21676474666576474646673646376637";
        let machine_err = *b"21676474666576474646673646376638";
        let desc = MachineDesc::from_terms(
            "owner",
            "location",
            "typ",
            1
        );

        // Try to collect rewards. No machines registered.
        // Expect error OwnerDoesNotExist.
        assert_noop!(
            PeaqPalletMor::get_online_rewards(
                Origin::signed(origin),
                origin,
                machine),
            Error::<Test>::OwnerDoesNotExist
        );

        // Register new machine for further testing.
        assert_ok!(PeaqPalletMor::register_new_machine(
            Origin::signed(origin),
            origin,
            machine,
            desc
        ));
        
        // Try to get rewarded for wrong machine.
        // Expect error MachineDoesNotExist.
        assert_noop!(
            PeaqPalletMor::get_online_rewards(
                Origin::signed(origin),
                origin,
                machine_err),
            Error::<Test>::MachineDoesNotExist
        );

        // Get rewards for properly registered machine.
        // Expect no errors.
        assert_ok!(PeaqPalletMor::get_online_rewards(
            Origin::signed(origin),
            origin,
            machine
        ));
    });
}

#[test]
fn desc_test() {
    let desc = MachineDesc::from_terms(
        "owner",
        "location",
        "typ",
        1
    );

    let result = String::from("owner_location_typ_00001")
        .as_bytes()
        .to_vec();

    assert_eq!(desc.as_bytes(), Ok(result));
}