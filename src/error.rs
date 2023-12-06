//! Encapsules all error types and relevant methods of this pallet.

use parity_scale_codec::{Decode, Encode};
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
    /// Sent when authorization fails in Peaq-DID, when registering the machine
    /// in Peaq-MOR, or when someone trys to get the online rewards for a machine,
    /// who does not own it.
    DidAuthorizationFailed,
    /// Sent when there are not enough tokens to withdrawel from the pot.
    InsufficientTokensInPot,
    /// Sent when given machine ID is already registered in Peaq-MOR.
    MachineAlreadyRegistered,
    /// Sent when a machine is not registered in Peaq-MOR.
    MachineNotRegistered,
    /// Sent when the amount of a machine usage payment is out of the configured range.
    MachinePaymentOutOfRange,
    /// Sent when authorization fails in Peaq-MOR. This can happen, if the owner
    /// of a machine gets updated in Peaq-DID, but not in Peaq-MOR.
    MorAuthorizationFailed,
    /// Sent when tried to update pallet's config, but failed because config
    /// is not consistent.
    MorConfigIsNotConsistent,
    /// Sent when Tokens could not be transfered, e.g. when minted or taken from pot.
    TokensCouldNotBeTransfered,
    /// Sent when an unexpected Peaq-DID error occurs. This means, return
    /// to developer of the Peaq-MOR pallet.
    UnexpectedDidError,
}

impl From<DidError> for MorError {
    fn from(err: DidError) -> Self {
        match err {
            DidError::AuthorizationFailed => MorError::DidAuthorizationFailed,
            _ => MorError::UnexpectedDidError,
        }
    }
}
