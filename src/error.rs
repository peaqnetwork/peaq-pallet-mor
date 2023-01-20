//! Encapsules all error types and relevant methods of this pallet.

use codec::{Decode, Encode};
use frame_support::pallet_prelude::*;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_std::vec::Vec;


/// Result definition for this pallet with unique error type to simplify coding.
pub type Result<T> = core::result::Result<T, MorError>;


/// This enum defines the all types of possible errors of this pallet. Have a
/// look on the descriptions of each error state.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, Encode, Decode)]
pub enum MorErrorType {
    /// Sent when the requested owner does not exist.
    OwnerDoesNotExist,
    /// Sent when a machine description exceeds maximum length of 64 bytes.
    MachineNameExceedMax64,
    /// Sent when a machine is already registered (double registration).
    MachineAlreadyExists,
    /// Sent when tried to access somehow a disabled machine.
    MachineIsDisabled,
    /// Sent when tried to enable a machine which is already enabled.
    MachineIsEnabled,
    /// Sent when a machine does not exist under this owner.
    MachineDoesNotExist,
    /// Sent when an I/O-error occurs during MachineDesc-into-bytes transformation
    MachineDescIoError,
}


/// This struct encapsules the type of error with additional informations
/// for the user, which are related to the source of this error.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, Encode, Decode)]
pub struct MorError {
    /// Type of error, see definition above
    pub typ: MorErrorType,
    /// Additional information about the occured error
    pub msg: Vec<u8>,
}

impl MorError {
    /// Generates a new MorError including Result to simplify and shorten coding
    pub fn err<T, Param: Parameter>(typ: MorErrorType, param: &Param) -> Result<T> {
        // this transformation makes it possible to use RbacError without generic
        let param = param.encode().as_slice().to_vec();
        Err(MorError { typ, msg: param })
    }
}

// #[cfg(feature = "std")]
// impl From<std::io::Error> for MorError {
//     fn from(_value: std::io::Error) -> MorError {
//         MorError{ 
//             typ: MorErrorType::MachineDescIoError,
//             msg: Vec::new()
//         }
//     }
// }