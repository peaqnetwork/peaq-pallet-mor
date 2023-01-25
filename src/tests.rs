//! Unit tests for this pallet, see spec definition

use frame_support::{assert_ok, assert_noop};
// use sp_std::vec;
use crate::{
    mock::*,
    Error,
};


const OWN_ACCT: &'static str = "Alice";
// const USR_ACCT: &'static str = "SomeUser";
const MAC_ACCT: &'static str = "Dave";

const M_ATTR: &[u8] = b"Attribute";
const M_VAL: &[u8] = b"Value";


#[test]
fn register_new_machine_test() {
    new_test_ext().execute_with(|| {
        let owner = account_key(OWN_ACCT);
        let machine = account_key(MAC_ACCT);

        // Try to register new machine on Peaq-MOR, which is not registered in Peaq-DID.
        // Expect error AuthorizationFailed.
        assert_noop!(PeaqMor::register_new_machine(
            Origin::signed(owner),
            machine),
            Error::<Test>::AuthorizationFailed
        );

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
    });
}


// #[test]
// fn get_online_rewards_test() {
//     new_test_ext().execute_with(|| {
//         let origin = account_key(ACCT);
//         let name: Vec<u8> = Vec::from(MACHINE_DESC);

//         // Try to collect rewards. No machines registered.
//         // Expect error OwnerDoesNotExist.
//         assert_noop!(
//             PeaqMor::get_online_rewards(
//                 Origin::signed(origin),
//                 origin,
//                 MACHINE),
//             Error::<Test>::OwnerDoesNotExist
//         );

//         // Register new machine for further testing.
//         assert_ok!(PeaqMor::register_new_machine(
//             Origin::signed(origin),
//             origin,
//             MACHINE,
//             name
//         ));
        
//         // Try to get rewarded for wrong machine.
//         // Expect error MachineDoesNotExist.
//         assert_noop!(
//             PeaqMor::get_online_rewards(
//                 Origin::signed(origin),
//                 origin,
//                 MACHINE_NONE),
//             Error::<Test>::MachineDoesNotExist
//         );

//         // Get rewards for properly registered machine.
//         // Expect no errors.
//         assert_ok!(PeaqMor::get_online_rewards(
//             Origin::signed(origin),
//             origin,
//             MACHINE
//         ));
//     });
// }
