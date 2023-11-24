//! The trait definition for the weights of extrinsics.

use frame_support::weights::Weight;

pub trait WeightInfo {
    fn get_registration_reward() -> Weight;
    fn get_online_rewards() -> Weight;
    fn pay_machine_usage() -> Weight;
    fn set_configuration() -> Weight;
    fn fetch_pot_balance() -> Weight;
}
