// use std::vec::{self, Vec};

// use super::constants::U256;
// use hashbrown::HashMap;

// #[derive(Debug, Clone)]
// pub struct Account {
//     nonce: u64,
//     balance: U256,
//     code: Vec<u8>,
//     storage: HashMap<U256, u64>,
// }

// impl Default for Account {
//     /// This denotes an empty account.
//     fn default() -> Self {
//         Self {
//             nonce: 0,
//             balance: U256::from(0),
//             code: Vec::new(),
//             storage: HashMap::new(),
//         }
//     }
// }

// #[derive(Debug, Clone)]
// pub struct Transaction {
//     to: u64,
//     value: u64,
//     data: Vec<u8>,
//     sender: u64,
// }
