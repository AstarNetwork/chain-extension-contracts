#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
pub mod staking_example {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use signing_extension::*;

    #[ink(storage)]
    pub struct SigningSample {}

    impl SigningSample {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn verify(&self, data: String, signature: Vec<u8>, pubkey: Vec<u8>) -> bool {
            let mut message_hash = <ink::env::hash::Blake2x256 as ink::env::hash::HashOutput>::Type::default();
            ink::env::hash_bytes::<ink::env::hash::Blake2x256>(&data.as_bytes(), &mut message_hash);

            ink::env::debug_println!("message_hash {:?}", message_hash);

            let result = Signing::verify(SigType::Ecdsa, signature, message_hash.to_vec(), pubkey);
            
            ink::env::debug_println!("result {:?}", result);

            result.unwrap_or(false)
        }
    }
}
