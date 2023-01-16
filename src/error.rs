//! This module defines the pallet's error and result types

use codec::{Decode, Encode};
// use frame_support::pallet_prelude::*;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};


/// Result definition for this pallet with unique error type
pub type Result<T> = core::result::Result<T, PalletError>;

/// This enum defines the basic type of error for this pallet
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Debug, Encode, Decode)]
pub enum PalletErrorType {
    /// Sent when something happens
    SomeError
}


/// This struct...
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Debug, Encode, Decode)]
pub struct PalletError {
    /// Type of error
    pub typ: PalletErrorType,
}