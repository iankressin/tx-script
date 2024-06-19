use std::str::FromStr;
use std::sync::Arc;

use super::prepared_transaction::PreparedTransaction;
use crate::common::token::Token;
use crate::common::unit::Unit;
use crate::frontend::parser::{TransactionIR, UnitOrToken};
use ethers::prelude::*;
use ethers::{
    contract::abigen,
    core::utils::{parse_units, ParseUnits},
    providers::Provider,
    types::Bytes,
};

abigen!(
    IERC20,
    r#"[
        function transfer(address recipient, uint256 amount) external returns (bool)
    ]"#,
);

/// TODO: docs
pub struct TransactionBuilder;

impl TransactionBuilder {
    pub fn build(transaction_irs: Vec<TransactionIR>) -> Vec<PreparedTransaction> {
        let mut prepared_txs = Vec::new();

        for tx_ir in transaction_irs {
            let mut value = ethers::types::U256::default();
            let mut data = Bytes::default();
            let mut to = tx_ir.to;

            // TODO: split each match in a function
            match &tx_ir.unit {
                UnitOrToken::Unit(unit) => {
                    value = TransactionBuilder::parse_units(&tx_ir.amount, unit);
                }
                UnitOrToken::Token(token) => {
                    to = H160::from_str(token.address(&tx_ir.chain))
                        .expect(format!("Invalid address for token: {:?}", token).as_str());
                    data = TransactionBuilder::build_data(&tx_ir, &token);
                }
            }

            prepared_txs.push(PreparedTransaction::new(to, value, tx_ir.chain, data));
        }

        prepared_txs
    }

    fn build_data(tx_ir: &TransactionIR, token: &Token) -> Bytes {
        let contract_address = H160::from_str(token.address(&tx_ir.chain))
            .expect(format!("Invalid address for token: {:?}", token).as_str());
        let provider = Provider::<Http>::try_from(tx_ir.chain.rpc_url())
            .expect("Unable to connect to the provider");
        let contract = IERC20::new(contract_address, Arc::new(provider));
        // TODO: this is considering that every token has 18 decimals, which is not true
        let amount = token.parse_value(&tx_ir.amount);

        contract
            .encode("transfer", (tx_ir.to, amount))
            .expect("Error encoding data")
    }

    fn parse_units(amount: &str, unit: &Unit) -> U256 {
        let parse_units_res = match parse_units(amount, *unit) {
            Ok(value) => value,
            Err(e) => panic!("Error parsing units: {e}"),
        };

        match parse_units_res {
            ParseUnits::U256(parsed_value) => parsed_value,
            _ => panic!("Invalid value: tx.value must be unsiged"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::{chain::Chain, token::Token, unit::Unit};
    use ethers::types::{Address, U256};

    const ADDRESS: &'static str = "0xadcdf1cc67362d0d61ad8954d077b78a1d80087b";

    #[test]
    fn test_build_single_tx() {
        let tx_ir = vec![TransactionIR {
            to: ADDRESS.parse::<Address>().unwrap(),
            amount: String::from("1"),
            unit: UnitOrToken::Unit(Unit::Ether),
            chain: Chain::Ethereum,
        }];
        let expected_tx = PreparedTransaction::new(
            ADDRESS.parse::<Address>().unwrap(),
            U256::from_dec_str("1000000000000000000").unwrap(),
            Chain::Ethereum,
            Bytes::default(),
        );
        let prepared_tx = TransactionBuilder::build(tx_ir);

        assert_eq!(prepared_tx.len(), 1);
        assert_eq!(prepared_tx, vec![expected_tx]);
    }

    #[test]
    fn test_tx_with_decimal_value() {
        let tx_ir = vec![TransactionIR {
            to: ADDRESS.parse::<Address>().unwrap(),
            amount: String::from("1.0"),
            unit: UnitOrToken::Unit(Unit::Ether),
            chain: Chain::Ethereum,
        }];
        let prepared_txs = TransactionBuilder::build(tx_ir);

        assert_eq!(
            prepared_txs[0].value,
            U256::from_dec_str("1000000000000000000").unwrap()
        );
    }

    #[test]
    fn test_tx_with_gwei_unit() {
        let tx_ir = vec![TransactionIR {
            to: ADDRESS.parse::<Address>().unwrap(),
            amount: String::from("1"),
            unit: UnitOrToken::Unit(Unit::Gwei),
            chain: Chain::Ethereum,
        }];
        let prepared_txs = TransactionBuilder::build(tx_ir);

        assert_eq!(
            prepared_txs[0].value,
            U256::from_dec_str("1000000000").unwrap()
        );
    }

    #[test]
    fn test_tx_sending_weth() {
        let tx_ir = vec![TransactionIR {
            to: ADDRESS.parse::<Address>().unwrap(),
            amount: String::from("1"),
            unit: UnitOrToken::Token(Token::WEth),
            chain: Chain::Ethereum,
        }];
        let prepared_txs = TransactionBuilder::build(tx_ir);
        let expected_tx = PreparedTransaction::new(
            Token::WEth.address(&Chain::Ethereum).parse::<Address>().unwrap(),
            U256::from_dec_str("0").unwrap(),
            Chain::Ethereum,
            Bytes::from_str(
                "0xa9059cbb000000000000000000000000adcdf1cc67362d0d61ad8954d077b78a1d80087b0000000000000000000000000000000000000000000000000de0b6b3a7640000"
            )
            .unwrap(),
        );

        assert_eq!(prepared_txs[0], expected_tx);
    }

    #[test]
    fn test_tx_sending_usdc() {
        let tx_ir = vec![TransactionIR {
            to: ADDRESS.parse::<Address>().unwrap(),
            amount: String::from("1"),
            unit: UnitOrToken::Token(Token::Usdc),
            chain: Chain::Ethereum,
        }];
        let prepared_txs = TransactionBuilder::build(tx_ir);
        let expected_tx = PreparedTransaction::new(
            Token::Usdc.address(&Chain::Ethereum).parse::<Address>().unwrap(),
            U256::from_dec_str("0").unwrap(),
            Chain::Ethereum,
            Bytes::from_str(
                "0xa9059cbb000000000000000000000000adcdf1cc67362d0d61ad8954d077b78a1d80087b00000000000000000000000000000000000000000000000000000000000f4240",
            )
            .unwrap(),
        );

        assert_eq!(prepared_txs[0], expected_tx);
    }

    #[test]
    fn test_tx_sending_wbtc() {
        let tx_ir = vec![TransactionIR {
            to: ADDRESS.parse::<Address>().unwrap(),
            amount: String::from("1"),
            unit: UnitOrToken::Token(Token::Wbtc),
            chain: Chain::Ethereum,
        }];
        let prepared_txs = TransactionBuilder::build(tx_ir);
        let expected_tx = PreparedTransaction::new(
            Token::Wbtc.address(&Chain::Ethereum).parse::<Address>().unwrap(),
            U256::from_dec_str("0").unwrap(),
            Chain::Ethereum,
            Bytes::from_str(
                "0xa9059cbb000000000000000000000000adcdf1cc67362d0d61ad8954d077b78a1d80087b0000000000000000000000000000000000000000000000000000000005f5e100",
            )
            .unwrap(),
        );

        assert_eq!(prepared_txs[0], expected_tx);
    }
}
