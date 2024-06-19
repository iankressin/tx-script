use ethers::types::U64;

use super::scanner::Scanner;

#[derive(Debug, PartialEq, Eq)]
pub enum Chain {
    Ethereum,
    Arbitrum,
    Base,
    Optimism,
    Anvil,
}

impl Default for Chain {
    fn default() -> Self {
        Chain::Ethereum
    }
}

impl From<Chain> for U64 {
    fn from(chain: Chain) -> U64 {
        match chain {
            Chain::Ethereum => 1.into(),
            Chain::Arbitrum => 42161.into(),
            Chain::Base => 8453.into(),
            Chain::Optimism => 10.into(),
            Chain::Anvil => 31337.into(),
        }
    }
}

impl From<&str> for Chain {
    fn from(chain: &str) -> Chain {
        match chain {
            "eth" => Chain::Ethereum,
            "arb" => Chain::Arbitrum,
            "base" => Chain::Base,
            "optimism" => Chain::Optimism,
            "anvil" => Chain::Anvil,
            invalid_chain => panic!("Invalid chain: {invalid_chain}"),
        }
    }
}

impl From<&Chain> for u64 {
    fn from(value: &Chain) -> Self {
        match value {
            Chain::Ethereum => 1,
            Chain::Arbitrum => 42161,
            Chain::Base => 8453,
            Chain::Optimism => 10,
            Chain::Anvil => 31337,
        }
    }
}

impl Chain {
    pub fn rpc_url(&self) -> &str {
        match self {
            Chain::Ethereum => "https://mainnet.infura.io/v3/",
            Chain::Arbitrum => "https://arbitrum.infura.io/v3/",
            Chain::Base => "https://base.infura.io/v3/",
            Chain::Optimism => "https://optimism.infura.io/v3/",
            Chain::Anvil => "http://localhost:8545/",
        }
    }

    pub fn scanner(&self) -> Scanner {
        match self {
            Chain::Ethereum => Scanner::new(String::from("https://etherscan.io")),
            Chain::Arbitrum => Scanner::new(String::from("https://arbiscan.io")),
            Chain::Base => Scanner::new(String::from("https://basescan.org")),
            Chain::Optimism => Scanner::new(String::from("https://optimism.com")),
            Chain::Anvil => Scanner::new(String::from("http://localhost:8545")),
        }
    }
}
