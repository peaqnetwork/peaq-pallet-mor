//! This module defines the pallet's error and result types

use codec::{Decode, Encode};
use frame_support::pallet_prelude::*;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_std::vec::Vec;


/// Result definition for this pallet with unique error type
pub type Result<T> = core::result::Result<T, MorError>;


/// This enum defines the basic type of error for this pallet
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Debug, Encode, Decode)]
pub enum MorErrorType {
    /// Sent when an owner does not exist
    OwnerDoesNotExist,
    /// Sent when a machine description exceeds maximum length of 64 bytes
    MachineNameExceedMax64,
    /// Sent when a machine is already registered (double registration)
    MachineAlreadyExists,
    /// Sent when a machine is disabled and tried to be accessed
    MachineIsDisabled,
    /// Sent when a machine does not exist (could not be found)
    MachineDoesNotExist,
}


/// This struct...
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Debug, Encode, Decode)]
pub struct MorError {
    /// Type of error, see definition above
    pub typ: MorErrorType,
    /// Additional information about the occured error
    pub msg: Vec<u8>,
}

impl MorError {
    /// generates a new RbacError including Result
    pub fn err<T, Param: Parameter>(typ: MorErrorType, param: &Param) -> Result<T> {
        // this transformation makes it possible to use RbacError without generic
        let param = param.encode().as_slice().to_vec();
        Err(MorError { typ, msg: param })
    }
}