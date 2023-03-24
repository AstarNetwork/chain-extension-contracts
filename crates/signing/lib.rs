#![cfg_attr(not(feature = "std"), no_std)]

use ink::prelude::vec::Vec;

use scale::{
    Decode,
    Encode,
};

pub struct Signing;

impl Signing {
    pub fn verify(sig_type: SigType, signature: Vec<u8>, message: Vec<u8>, pubkey: Vec<u8>) -> Result<bool, SigningError> {
        ink::env::debug_println!("HERE 1!!");
        let res = ::ink::env::chain_extension::ChainExtensionMethod::build(0x00010001u32)
            .input::<(SigType, Vec<u8>, Vec<u8>, Vec<u8>)>()
            .output::<bool, false>()
            .handle_error_code::<SigningError>()
            .call(&(sig_type, signature, message, pubkey));
        ink::env::debug_println!("HERE 2!!");
        res
    }
}

#[derive(PartialEq, Debug, Eq, Clone, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum SigType {
    Sr25519,
    Ed25519,
    Ecdsa,
}

#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
#[derive(PartialEq, Eq, Copy, Clone, Encode, Decode, Debug)]
pub enum SigningError {
    InvalidSignature = 1,
    InvalidPubkey = 2,
    /// Unknown error
    UnknownError = 99,
}

impl ink::env::chain_extension::FromStatusCode for SigningError {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::InvalidSignature),
            2 => Err(Self::InvalidPubkey),
            99 => Err(Self::UnknownError),
            _ => panic!("encountered unknown status code"),
        }
    }
}
