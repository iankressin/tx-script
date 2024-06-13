use super::prepared_transaction::PreparedTransaction;
use crate::frontend::parser::TransactionIR;
use ethers::core::utils::{parse_units, ParseUnits};

/// TODO: docs
pub struct TransactionBuilder;

impl TransactionBuilder {
    pub fn build(parser: Vec<TransactionIR>) -> Vec<PreparedTransaction> {
        let mut prepared_txs = Vec::new();

        for tx in parser {
            // TODO: rename
            let parse_units_res = match parse_units(&tx.amount, tx.unit) {
                Ok(value) => value,
                Err(e) => panic!("Error parsing units: {e}"),
            };
            let value = match parse_units_res {
                ParseUnits::U256(value) => value,
                _ => panic!("Invalid value: tx.value must be unsiged"),
            };

            prepared_txs.push(PreparedTransaction::new(tx.to, value, tx.chain));
        }

        prepared_txs
    }
}

#[cfg(test)]
// TODO: write more tests
// - value with different units
mod test {
    use super::*;
    use crate::common::{chain::Chain, unit::Unit};
    use ethers::types::{Address, U256};

    const ADDRESS: &'static str = "0xadcdf1cc67362d0d61ad8954d077b78a1d80087b";

    #[test]
    fn test_build_single_tx() {
        let tx_ir = vec![TransactionIR {
            to: ADDRESS.parse::<Address>().unwrap(),
            amount: String::from("1"),
            unit: Unit::Ether,
            chain: Chain::Ethereum,
        }];
        let expected_tx = PreparedTransaction {
            to: ADDRESS.parse::<Address>().unwrap(),
            value: U256::from_dec_str("1000000000000000000").unwrap(),
            chain: Chain::Ethereum,
        };
        let prepared_tx = TransactionBuilder::build(tx_ir);

        assert_eq!(prepared_tx.len(), 1);
        assert_eq!(prepared_tx, vec![expected_tx]);
    }

    #[test]
    fn test_tx_with_decimal_value() {
        let tx_ir = vec![TransactionIR {
            to: ADDRESS.parse::<Address>().unwrap(),
            amount: String::from("1.0"),
            unit: Unit::Ether,
            chain: Chain::Ethereum,
        }];
        let prepared_txs = TransactionBuilder::build(tx_ir);

        assert_eq!(
            prepared_txs[0].value,
            U256::from_dec_str("1000000000000000000").unwrap()
        );
    }
}
