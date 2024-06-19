use crate::common::{chain::Chain, token::Token, unit::Unit};
use ethers::types::{Address, Bytes};
use pest::Parser;
use pest_derive::Parser;
use std::{error::Error, str::FromStr};

// TODO: move this to common module
#[derive(Debug, PartialEq, Eq)]
pub enum UnitOrToken {
    Unit(Unit),
    Token(Token),
}

impl Default for UnitOrToken {
    fn default() -> Self {
        UnitOrToken::Unit(Unit::Ether)
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct TransactionIR {
    pub to: Address,
    // String makes it easier to parse the amount to its unit using [`parse_units`]
    pub amount: String,
    pub unit: UnitOrToken,
    pub chain: Chain,
}

#[derive(Parser)]
#[grammar = "src/frontend/grammar.pest"]
// TODO: this name is really bad, change it
pub struct TxLangParser<'a> {
    // TODO: maybe it's worth to change build_ir to return a Vec<TransactionIR>
    pub txs: Vec<TransactionIR>,
    source: &'a str,
}

impl<'a> TxLangParser<'a> {
    pub fn new(source: &'a str) -> Self {
        TxLangParser {
            txs: Vec::new(),
            source,
        }
    }

    /// Parses the source code to an intermidiary representation of all transactions
    /// present in the file.
    /// Pest yield an iterator with all lexemes an its respective types. This function
    /// builds from those lexemes a [`TransactionIR`], for each transaction
    pub fn build_ir(&mut self) -> Result<(), Box<dyn Error>> {
        let pairs = TxLangParser::parse(Rule::tx_list, self.source).unwrap();

        for pair in pairs {
            match pair.as_rule() {
                Rule::tx => {
                    let mut tx = TransactionIR::default();
                    let inner_pairs = pair.into_inner();

                    for inner_pair in inner_pairs {
                        match inner_pair.as_rule() {
                            Rule::number => {
                                tx.amount = inner_pair.as_str().trim().to_string();
                            }
                            Rule::address => {
                                tx.to = TxLangParser::parse_address(inner_pair.as_str());
                            }
                            Rule::available_chains => {
                                tx.chain = inner_pair.as_str().into();
                            }
                            Rule::token => {
                                tx.unit = TxLangParser::parse_token(inner_pair.as_str());
                            }
                            Rule::unit => {
                                tx.unit = TxLangParser::parse_unit(inner_pair.as_str());
                            }
                            unexpected_token => panic!("Unexpected token {:?}", unexpected_token),
                        }
                    }

                    self.txs.push(tx);
                }
                unexpected_token => panic!("Unexpected token {:?}", unexpected_token),
            }
        }

        Ok(())
    }

    /// TODO: implement ens name resolution
    fn parse_address(str_address: &str) -> Address {
        match Address::from_str(str_address) {
            Ok(address) => address,
            Err(_) => panic!("Invalid address: {str_address}"),
        }
    }

    fn parse_token(str_token: &str) -> UnitOrToken {
        match str_token {
            "usdc" => UnitOrToken::Token(Token::Usdc),
            "usdt" => UnitOrToken::Token(Token::Usdt),
            "dai" => UnitOrToken::Token(Token::Dai),
            "wbtc" => UnitOrToken::Token(Token::Wbtc),
            "weth" => UnitOrToken::Token(Token::WEth),
            invalid_token => panic!("Invalid token: {invalid_token}"),
        }
    }

    fn parse_unit(str_unit: &str) -> UnitOrToken {
        match str_unit {
            "gwei" => UnitOrToken::Unit(Unit::Gwei),
            "wei" => UnitOrToken::Unit(Unit::Wei),
            "ether" => UnitOrToken::Unit(Unit::Ether),
            invalid_unit => panic!("Invalid unit: {invalid_unit}"),
        }
    }
}

#[cfg(test)]
// TODO: add more tests
// - units
// - chains
mod test {
    use super::*;
    use ethers::types::Address;

    #[test]
    fn test_parse_single_tx() {
        let program = "send 1 ether to 0xadcdf1cc67362d0d61ad8954d077b78a1d80087b on eth";
        let mut parser = TxLangParser::new(program);
        parser.build_ir().unwrap();
        let expected_tx = TransactionIR {
            to: Address::from_str("0xadcdf1cc67362d0d61ad8954d077b78a1d80087b").unwrap(),
            amount: String::from("1"),
            unit: UnitOrToken::Unit(Unit::Ether),
            chain: Chain::Ethereum,
        };

        assert_eq!(parser.txs.len(), 1);
        assert_eq!(parser.txs[0], expected_tx);
    }

    #[test]
    fn test_parse_multiple_txs() {
        let program = "send 1 gwei to 0xadcdf1cc67362d0d61ad8954d077b78a1d80087b on base\nsend 2.0 ether to 0xadcdf1cc67362d0d61ad8954d077b78a1d80087b on eth";
        let mut parser = TxLangParser::new(program);
        parser.build_ir().unwrap();
        let expected_txs = vec![
            TransactionIR {
                to: Address::from_str("0xadcdf1cc67362d0d61ad8954d077b78a1d80087b").unwrap(),
                amount: String::from("1"),
                unit: UnitOrToken::Unit(Unit::Gwei),
                chain: Chain::Base,
            },
            TransactionIR {
                to: Address::from_str("0xadcdf1cc67362d0d61ad8954d077b78a1d80087b").unwrap(),
                amount: String::from("2.0"),
                unit: UnitOrToken::Unit(Unit::Ether),
                chain: Chain::Ethereum,
            },
        ];

        assert_eq!(parser.txs.len(), 2);
        assert_eq!(parser.txs, expected_txs);
    }

    #[test]
    fn parse_token() {
        let program = "send 10 usdc to 0xadcdf1cc67362d0d61ad8954d077b78a1d80087b on eth";
        let mut parser = TxLangParser::new(program);
        parser.build_ir().unwrap();
        let expected_txs = TransactionIR {
            to: Address::from_str("0xadcdf1cc67362d0d61ad8954d077b78a1d80087b").unwrap(),
            amount: String::from("10"),
            unit: UnitOrToken::Token(Token::Usdc),
            chain: Chain::Ethereum,
        };

        assert_eq!(parser.txs[0], expected_txs);
    }

    #[test]
    #[should_panic]
    fn test_parse_invalid_chain() {
        let program = "send 1 gwei to 0xadcdf1cc67362d0d61ad8954d077b78a1d80087b on invalid_chain";
        let mut parser = TxLangParser::new(program);
        parser.build_ir().unwrap();
    }
}
