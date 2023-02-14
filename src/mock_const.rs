//! Seperate constants from mock, to prevent dependency-errors, when compiling
//! the network-node with feature runtime-benchmarks enabled.

/// Generic owner of some machine
pub const O_ACCT: &'static str = "Alice";
/// Generic user of some machine
pub const U_ACCT: &'static str = "Bob"; // User
/// Generic machine
pub const M_ACCT: &'static str = "RPi001"; // Machine
/// One generic attribute for the machine (needed by Peaq-Did)
pub const M_ATTR: &[u8] = b"Type";
/// One value to the attribute for the machine
pub const M_VAL: &[u8] = b"RPiMachine";
/// Typical registration reward
pub const REG_FEE: u128 = 100_000_000_000_000_000u128;