//! Unit tests for this pallet, see spec definition

use frame_support::{assert_ok, assert_noop};
use crate::{
    mock::*,
    Error,
    // structs::MachineDesc,
};


const ACCT: &'static str = "Micha";
const MACHINE: [u8; 32] = *b"21676474666576474646673646376637";
const MACHINE_NONE: [u8; 32] = *b"21676474666576474646673646376638";
const MACHINE_DESC: &'static str = "owner_location_type_00001";


#[test]
fn register_new_machine_test() {
    new_test_ext().execute_with(|| {
        let origin = account_key(ACCT);
        let name: Vec<u8> = Vec::from(MACHINE_DESC);

        // Register new machine on the network.
        // Expect no error.
        assert_ok!(PeaqMor::register_new_machine(
            Origin::signed(origin),
            origin,
            MACHINE,
            name
        ));
    });
}


#[test]
fn fetch_machine_info_test() {
    new_test_ext().execute_with(|| {
        let origin = account_key(ACCT);
        let name: Vec<u8> = Vec::from(MACHINE_DESC);

        // Fetch a machine, but no machines are registered.
        // Expect OwnerDoesNotExist error
        assert_noop!(
            PeaqMor::fetch_machine_info(
                Origin::signed(origin),
                origin,
                MACHINE
            ),
            Error::<Test>::OwnerDoesNotExist
        );

        // Register new machine for further testing.
        assert_ok!(PeaqMor::register_new_machine(
            Origin::signed(origin),
            origin,
            MACHINE,
            name
        ));

        // Fetch registered machine.
        // Expect to be able to fetch this registered machine.
        assert_ok!(PeaqMor::fetch_machine_info(
            Origin::signed(origin),
            origin,
            MACHINE
        ));

        // Request wrong machine-ID from owner, which is registered.
        // Expect error MachineDoesNotExist
        assert_noop!(
            PeaqMor::fetch_machine_info(
                Origin::signed(origin),
                origin,
                MACHINE_NONE
            ),
            Error::<Test>::MachineDoesNotExist
        );
    });
}


#[test]
fn get_online_rewards_test() {
    new_test_ext().execute_with(|| {
        let origin = account_key(ACCT);
        let name: Vec<u8> = Vec::from(MACHINE_DESC);

        // Try to collect rewards. No machines registered.
        // Expect error OwnerDoesNotExist.
        assert_noop!(
            PeaqMor::get_online_rewards(
                Origin::signed(origin),
                origin,
                MACHINE),
            Error::<Test>::OwnerDoesNotExist
        );

        // Register new machine for further testing.
        assert_ok!(PeaqMor::register_new_machine(
            Origin::signed(origin),
            origin,
            MACHINE,
            name
        ));
        
        // Try to get rewarded for wrong machine.
        // Expect error MachineDoesNotExist.
        assert_noop!(
            PeaqMor::get_online_rewards(
                Origin::signed(origin),
                origin,
                MACHINE_NONE),
            Error::<Test>::MachineDoesNotExist
        );

        // Get rewards for properly registered machine.
        // Expect no errors.
        assert_ok!(PeaqMor::get_online_rewards(
            Origin::signed(origin),
            origin,
            MACHINE
        ));
    });
}


#[test]
fn enable_disable_machine_test() {
    new_test_ext().execute_with(|| {
        let origin = account_key(ACCT);
        let name: Vec<u8> = Vec::from(MACHINE_DESC);

        // Register new machine for further testing.
        assert_ok!(PeaqMor::register_new_machine(
            Origin::signed(origin),
            origin,
            MACHINE,
            name
        ));

        // Try to enable this machine.
        // Expect error MachineIsAlreadyEnabled.
        assert_noop!(
            PeaqMor::enable_machine(
                Origin::signed(origin),
                origin,
                MACHINE),
            Error::<Test>::MachineIsEnabled
        );

        // Disable machine.
        // Expect no error.
        assert_ok!(PeaqMor::disable_machine(
            Origin::signed(origin),
            origin,
            MACHINE
        ));

        // Try to disable machine again.
        // Expect error MachineIsDisabled.
        assert_noop!(
            PeaqMor::disable_machine(
                Origin::signed(origin),
                origin,
                MACHINE),
            Error::<Test>::MachineIsDisabled
        );

        // Now finally enable the machine again.
        // Expect no error.
        assert_ok!(PeaqMor::enable_machine(
            Origin::signed(origin),
            origin,
            MACHINE
        ));
    });
}


#[test]
fn disable_machine_test() {

}

// #[test]
// fn desc_test() {
//     let desc = MACHINEDesc::from_terms(
//         "owner",
//         "location",
//         "typ",
//         1
//     );

//     let result = String::from("owner_location_typ_00001")
//         .as_bytes()
//         .to_vec();

//     assert_eq!(desc.as_bytes(), Ok(result));
// }