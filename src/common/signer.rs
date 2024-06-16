use ethers::prelude::*;
use std::{env, fs, str::FromStr};

pub const SIGNER_PK_PATH: &'static str = ".tx-script/signer.pk";

pub struct TxSigner {
    pk_file_path: String,
}

impl TxSigner {
    pub fn new() -> Self {
        TxSigner {
            pk_file_path: TxSigner::get_signer_path(),
        }
    }

    pub fn get_signer_path() -> String {
        let home = env::var("HOME");

        match home {
            Ok(home) => format!("{}/{}", home, SIGNER_PK_PATH),
            Err(_) => panic!(
                "Unable to read env var HOME. \n\
                Ensure that the HOME env var is set. \n\
                "
            ),
        }
    }

    pub fn get_local_wallet(&self, signer_pk: Option<&str>) -> LocalWallet {
        match signer_pk {
            Some(pk) => LocalWallet::from_str(&pk).unwrap(),
            None => {
                let pk = self.get_pk();

                match LocalWallet::from_str(&pk) {
                    Ok(wallet) => wallet,
                    Err(e) => {
                        panic!("Unable to load signer: {}", e);
                    }
                }
            }
        }
    }

    pub fn set_pk(&self, pk: &str) {
        match fs::write(&self.pk_file_path, pk) {
            Ok(_) => println!("Private key set!"),
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    let env = env::var("HOME").unwrap();
                    fs::create_dir_all(format!("{}/.tx-script", env)).unwrap();
                    self.set_pk(pk);
                    return;
                }

                panic!("Unable to set private key: {}", e);
            },
        }
    }

    fn get_pk(&self) -> String {
        match fs::read_to_string(&self.pk_file_path) {
            Ok(pk) => pk.trim().to_string(),
            Err(_) => panic!(
                "[ERROR] Unable to load signer. \n \
                Ensure that the signer is present in the correct path. \n \
                If it is your first time using txscript, you can set your private key using the command: \n \
                `tx-script set-pk <private-key>`"
            ),
        }
    }
}
