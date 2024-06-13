use super::tx_dispatcher::TransactionDispatcher;
use crate::{backend::prepared_transaction::PreparedTransaction, common::chain::Chain};
use async_trait::async_trait;
use ethers::prelude::*;
use futures::future::TryJoinAll;
use std::{env, error::Error, str::FromStr};

const SIGNER_PK_PATH: &'static str = ".tx-lang/signer.pk";

pub struct LocalDispatcher {
    wallet: LocalWallet,
}

#[async_trait]
impl TransactionDispatcher for LocalDispatcher {
    async fn dispatch(&self, tx: Vec<PreparedTransaction>) -> Result<(), Box<dyn Error>> {
        tx.into_iter()
            .map(|t| {
                let middleware = self.get_signer_middleware(&t.chain);
                let tx_request = self.parse_transaction(t);
                println!("Sending tx: {:#?}", tx_request);

                tokio::spawn(async move {
                    // TODO: handle error
                    let pending_tx = middleware.send_transaction(tx_request, None).await;

                    match pending_tx {
                        Ok(tx) => tx.await,
                        Err(e) => {
                            panic!("Error sending tx: {:#?}", e);
                        }
                    }
                })
            })
            .collect::<TryJoinAll<_>>()
            .await?;

        Ok(())
    }
}

impl LocalDispatcher {
    pub fn new(signer_pk: Option<&str>) -> Self {
        LocalDispatcher {
            wallet: LocalDispatcher::get_local_wallet(signer_pk),
        }
    }

    fn get_signer_pk_from_system() -> String {
        match env::var("HOME") {
            Ok(home) => std::fs::read_to_string(format!("{}/{}", home, SIGNER_PK_PATH))
                .unwrap()
                .trim()
                .to_string(),
            Err(_) => {
                panic!(
                    "Unable to load signer. Ensure that the signer is present in the correct path."
                )
            }
        }
    }

    fn get_local_wallet(signer_pk: Option<&str>) -> LocalWallet {
        match signer_pk {
            Some(pk) => LocalWallet::from_str(&pk).unwrap(),
            None => {
                let pk = &LocalDispatcher::get_signer_pk_from_system();

                println!("Signer pk: {} | Len: {}", pk, pk.len());

                match LocalWallet::from_str(pk) {
                    Ok(wallet) => wallet,
                    Err(e) => {
                        panic!("Unable to load signer: {}", e);
                    }
                }
            }
        }
    }

    pub fn parse_transaction(&self, tx: PreparedTransaction) -> TransactionRequest {
        TransactionRequest {
            from: Some(self.wallet.address()),
            to: Some(tx.to.into()),
            value: Some(tx.value.into()),
            chain_id: Some(tx.chain.into()),

            gas: None,
            gas_price: None,
            data: None,
            nonce: None,
        }
    }

    /// TODO - Cache
    pub fn get_signer_middleware(
        &self,
        chain: &Chain,
    ) -> SignerMiddleware<Provider<Http>, LocalWallet> {
        let rpc_url = chain.rpc_url();
        let provider =
            Provider::<Http>::try_from(rpc_url).expect("Unable to connect to the provider");
        self.wallet.with_chain_id(chain);
        SignerMiddleware::new(provider, self.wallet.clone())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TO: &'static str = "0xadcdf1cc67362d0d61ad8954d077b78a1d80087b";
    const SIGNER_PUBLIC: &'static str = "0xf6a996ce046f5b65c2c3183e9bcbe22d001441f2";
    const SIGNER_PRIVATE: &'static str =
        "725fd1619b2653b7ff1806bf29ae11d0568606d83777afd5b1f2e649bd5132a9";

    #[test]
    fn test_parse_prepared_transaction_to_transaction_request() {
        let to = TO.parse::<Address>().unwrap();
        let tx = PreparedTransaction {
            to,
            value: U256::from_dec_str("1000000000000000000").unwrap(),
            chain: Chain::Ethereum,
        };
        let expected = TransactionRequest {
            from: Some(SIGNER_PUBLIC.parse::<Address>().unwrap()),
            to: Some(NameOrAddress::Address(to)),
            value: Some(U256::from_dec_str("1000000000000000000").unwrap()),
            chain_id: Some(Chain::Ethereum.into()),

            gas: None,
            gas_price: None,
            data: None,
            nonce: None,
        };
        let dispatcher = LocalDispatcher::new(Some(SIGNER_PRIVATE));

        assert_eq!(dispatcher.parse_transaction(tx), expected);
    }
}
