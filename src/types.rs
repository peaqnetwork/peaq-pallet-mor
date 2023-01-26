//! All pallet relevant structs are defined here

use frame_support::traits::Currency;


/// Short form type definition to simplify method definition.
pub type CrtBalance<T> = <<T as crate::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
// /// Short form type definition to simplify method definition.
// pub type CrtPosImbalance<T> = <<T as crate::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::PositiveImbalance;
/// Short form type definition to simplify method definition.
pub type CrtWeight<T> = <T as crate::Config>::WeightInfo;
