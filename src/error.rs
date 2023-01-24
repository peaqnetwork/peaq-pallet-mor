//! Encapsules all error types and relevant methods of this pallet.

use codec::{Decode, Encode};
use peaq_pallet_did::did::DidError;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};


/// Result definition for this pallet with unique error type to simplify coding.
pub type MorResult<T> = core::result::Result<T, MorError>;


/// This enum defines the all types of possible errors of this pallet. Have a
/// look on the descriptions of each error state.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, Encode, Decode)]
pub enum MorError {
    /// Sent when given machine ID is already registered.
    MachineAlreadyRegistered,
    /// Sent when authorization fails in Peaq-DID.
    AuthorizationFailed,
    /// Sent when name exceeds maximum length.
    NameExceedMaxChar,
    /// Sent when an unexpected Peaq-DID error occurs.
    UnexpectedDidError,
}


impl From<DidError> for MorError {
    fn from(err: DidError) -> Self {
        match err {
            DidError::NameExceedMaxChar => MorError::NameExceedMaxChar,
            DidError::AuthorizationFailed => MorError::AuthorizationFailed,
            _ => MorError::UnexpectedDidError
        }
    }
}