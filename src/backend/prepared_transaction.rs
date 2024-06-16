use core::fmt;
use std::fmt::{Display, Formatter};

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

impl Display for PreparedTransaction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "to: {}, value: {} ETH, chain: {:?}",
            self.to,
            self.value.as_u128() as f64 / 1e18,
            self.chain
        )
    }
}
