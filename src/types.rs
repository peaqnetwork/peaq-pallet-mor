//! All pallet relevant structs are defined here
use frame_support::traits::Currency;


/// Short form type definition to simplify method definition.
pub type CrtBalance<T> = <<T as crate::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
// /// Short form type definition to simplify method definition.
// pub type CrtPosImbalance<T> = <<T as crate::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::PositiveImbalance;
/// Short form type definition to simplify method definition.
pub type CrtWeight<T> = <T as crate::Config>::WeightInfo;


// pub struct RewardConfig<T: > {
//     /// How much tokens a machine owner gets rewarded, when registering a new machine on the network.
//     registration: Balance<T>,
//     /// How much tokens a machine owner gets transfered, when keeping his machine online for a certain
//     /// period of time, e.g. 20 minutes.
//     online_period: Balance<T>,
//     // /// Defines how much time is one period (to be online to get rewarded).
//     // online_time: Time,
// }