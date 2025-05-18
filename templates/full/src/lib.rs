#![no_std]

pub mod constants;
pub mod error;
pub mod instruction;
pub mod state;

#[cfg(feature = "std")]
extern crate std;

#[cfg(not(feature = "no-bpf-entrypoint"))]
mod entrypoint;

pinocchio_pubkey::declare_id!("__PROGRAM_ID__");
