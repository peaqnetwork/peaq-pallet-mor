//! All pallet relevant structs are defined here
use codec::{Decode, Encode};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::RuntimeDebug;
use sp_std::vec::Vec;
use std::io::Write;

use crate::error::{
    MorError, MorErrorType::MachineNameExceedMax64, Result
};


/// Machine struct stores information about one registered machine. The owner can setup
/// one account for getting rewarded. A machine cannot be deleted, but be disabled instead.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(
    Clone, PartialEq, Eq, PartialOrd, Ord, Default, TypeInfo, Decode, Encode, RuntimeDebug,
)]
pub struct Machine {
    /// Name of the registered machine
    pub name: Vec<u8>,
    /// Enabled flag
    pub enabled: bool,
}

impl Machine {
    /// Returns a new Machine struct with defaults and given name
    pub fn new(desc: &MachineDesc) -> Result<Machine> {
        Ok(Machine{ name: desc.as_bytes()?, enabled: true })
    }
}


/// Administrative struct to simplify a machine's description/naming. This implementation
/// uses the location and the type of the machine, the owner and a possible count of the
/// machine to generate a description for the machine in an unique pattern.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(
    Clone, PartialEq, Eq, PartialOrd, Ord, Default, TypeInfo, Decode, Encode, RuntimeDebug,
)]
pub struct MachineDesc {
    /// Name/Company owning the machine, e.g. "Meyer", "Volkswagen"
    owner: Vec<u8>,
    /// Place, where the machine is (usually) located, e.g. "Bamberg", "New-Jersey"
    location: Vec<u8>,
    /// Type of the machine, e.g. "car", "charger" etc.
    typ: Vec<u8>,
    /// Number/Index/Count to enable having multiple machines of same type & location
    count: u16
}

impl MachineDesc {
    /// Generates a new MachineDesc instance by using the &str-terms location, typ(e),
    /// owner and a given counting number.
    pub fn from_terms(owner: &str, location: &str, typ: &str, count: u16) -> MachineDesc {
        MachineDesc{
            owner: owner.as_bytes().to_vec(),
            location: location.as_bytes().to_vec(),
            typ: typ.as_bytes().to_vec(),
            count: count
        }
    }

    /// Generates the harmonized description for a machine out of the contained parameters.
    pub fn as_bytes(&self) -> Result<Vec<u8>> {
        // There must be a better solution...
        let mut bytes: Vec<u8> = self.owner.clone();
        bytes.push(b'_');
        bytes.extend_from_slice(&self.location);
        bytes.push(b'_');
        bytes.extend_from_slice(&self.typ);
        bytes.push(b'_');
        write!(bytes, "{:05}", self.count)?;
        if bytes.len() <= 64 {
            Ok(bytes)
        } else {
            MorError::err(MachineNameExceedMax64, &bytes)
        }
    }
}