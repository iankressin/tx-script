use crate::common::chain::Chain;
use ethers::types::{Address, U256};

#[derive(Debug, PartialEq, Eq)]
pub struct PreparedTransaction {
    pub to: Address,
    pub value: U256,
    pub chain: Chain,
}

impl PreparedTransaction {
    pub fn new(to: Address, value: U256, chain: Chain) -> Self {
        PreparedTransaction { to, value, chain }
    }
}
