//! Encapsules all error types and relevant methods of this pallet.

// use codec::{Decode, Encode};
// use frame_support::pallet_prelude::*;
// #[cfg(feature = "std")]
// use serde::{Deserialize, Serialize};
// use sp_std::vec::Vec;
// use peaq_pallet_did::did::DidError;
// use sp_runtime::DispatchError;


// /// Result definition for this pallet with unique error type to simplify coding.
// pub type Result<T> = core::result::Result<T, MorError>;


// /// This enum defines the all types of possible errors of this pallet. Have a
// /// look on the descriptions of each error state.
// #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
// #[derive(Debug, PartialEq, Eq, Encode, Decode)]
// pub enum MorErrorType {
//     /// Sent when authorization in Peaq-DID fails
//     AuthorizationFailed,
//     /// Sent when a machine description exceeds maximum length of 64 bytes.
//     MachineNameExceedMax64,
//     /// We don't expect other kinds of error in Peaq-DID
//     UnexpectedDidError
// }


// /// This struct encapsules the type of error with additional informations
// /// for the user, which are related to the source of this error.
// #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
// #[derive(Debug, PartialEq, Eq, Encode, Decode)]
// pub struct MorError {
//     /// Type of error, see definition above
//     pub typ: MorErrorType,
//     /// Additional information about the occured error
//     pub msg: Vec<u8>,
// }

// impl MorError {
//     /// Generates a new MorError including Result to simplify and shorten coding
//     pub fn err<T, Param: Parameter>(typ: MorErrorType, param: &Param) -> Result<T> {
//         // this transformation makes it possible to use RbacError without generic
//         let param = param.encode().as_slice().to_vec();
//         Err(MorError { typ, msg: param })
//     }
// }

// impl From<DidError> for MorError {
//     fn from(err: DidError) -> MorError {
//         let typ = match err {
//             DidError::AuthorizationFailed => MorErrorType::AuthorizationFailed,
//             DidError::NameExceedMaxChar => MorErrorType::MachineNameExceedMax64,
//             _ => MorErrorType::UnexpectedDidError,
//         };
//         MorError {
//             typ,
//             msg: Vec::new()
//         }
//     }
// }