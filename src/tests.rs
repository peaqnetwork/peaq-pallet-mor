//! Unit tests for this pallet, spec definition

use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn extrinsic_test() {
    new_test_ext().execute_with(|| {
        let acct = "Micha";
        let origin = account_key(acct);

        assert_ok!(PeaqPallet::some_extrinsic(
            Origin::signed(origin)
        ));

        // assert_noop!(
        //     PeaqPallet::some_extrinsic(Origin::signed(origin)),
        //     Error::<Test>::SomeError
        // );
    });
}