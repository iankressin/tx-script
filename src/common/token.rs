use super::chain::Chain;
use core::panic;
use ethers::types::U256;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    WEth,
    Usdt,
    Usdc,
    Dai,
    Wbtc,
}

impl Default for Token {
    fn default() -> Self {
        Token::Usdt
    }
}

impl Token {
    pub fn address(&self, chain: &Chain) -> &'static str {
        match self {
            Self::Usdt => match chain {
                Chain::Ethereum => "0xdac17f958d2ee523a2206206994597c13d831ec7",
                Chain::Arbitrum => "0xfd086bc7cd5c481dcc9c85ebe478a1c0b69fcbb9",
                Chain::Optimism => "0x94b008aa00579c1307b0ef2c499ad98a8ce58e58",
                Chain::Base => panic!("USDT not deployed on Base"),
                // Chain::Anvil => panic!("USDT not deployed on Anvil"),
                Chain::Anvil => "0xdac17f958d2ee523a2206206994597c13d831ec7",
            },
            Self::Usdc => match chain {
                Chain::Ethereum => "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48",
                Chain::Arbitrum => "0xaf88d065e77c8cc2239327c5edb3a432268e5831",
                Chain::Optimism => "0x0b2c639c533813f4aa9d7837caf62653d097ff85",
                Chain::Base => "0x833589fcd6edb6e08f4c7c32d4f71b54bda02913",
                Chain::Anvil => panic!("USDT not deployed on Anvil"),
            },
            Self::Dai => match chain {
                Chain::Ethereum => "0x6b175474e89094c44da98b954eedeac495271d0f",
                Chain::Arbitrum => "0xda10009cbd5d07dd0cecc66161fc93d7c9000da1",
                Chain::Optimism => "0xda10009cbd5d07dd0cecc66161fc93d7c9000da1",
                Chain::Base => "0x50c5725949a6f0c72e6c4a641f24049a917db0cb",
                Chain::Anvil => panic!("USDT not deployed on Anvil"),
            },
            Self::Wbtc => match chain {
                Chain::Ethereum => "0x2260fac5e5542a773aa44fbcfedf7c193bc2c599",
                Chain::Arbitrum => "0x2f2a2543b76a4166549f7aab2e75bef0aefc5b0f",
                Chain::Optimism => "0x68f180fcce6836688e9084f035309e29bf0a2095",
                Chain::Base => panic!("WBTC not deployed on Base"),
                Chain::Anvil => panic!("USDT not deployed on Anvil"),
            },
            Self::WEth => match chain {
                Chain::Ethereum => "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
                Chain::Arbitrum => "0x82af49447d8a07e3bd95bd0d56f35241523fbab1",
                Chain::Optimism => "0x4200000000000000000000000000000000000006",
                Chain::Base => "0x4200000000000000000000000000000000000006",
                Chain::Anvil => panic!("USDT not deployed on Anvil"),
            },
        }
    }

    pub fn parse_value(&self, value: &str) -> U256 {
        let value = value.parse::<f64>().unwrap();
        let decimals = self.decimals();
        let value = value * 10f64.powi(decimals as i32);
        U256::from(value as u128)
    }

    fn decimals(&self) -> u8 {
        match self {
            Self::WEth => 18,
            Self::Usdt => 6,
            Self::Usdc => 6,
            Self::Dai => 18,
            Self::Wbtc => 8,
        }
    }
}
