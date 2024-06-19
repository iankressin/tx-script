use crate::common::chain::Chain;
use core::fmt;
use ethers::types::{Address, Bytes, U256};
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq)]
pub struct PreparedTransaction {
    pub to: Address,
    pub value: U256,
    pub chain: Chain,
    pub data: Bytes,
}

impl PreparedTransaction {
    pub fn new(to: Address, value: U256, chain: Chain, data: Bytes) -> Self {
        PreparedTransaction {
            to,
            value,
            chain,
            data,
        }
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
