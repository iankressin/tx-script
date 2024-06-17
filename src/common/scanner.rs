pub struct Scanner {
    url: String,
}

impl Scanner {
    pub fn new(url: String) -> Self {
        Scanner { url }
    }

    pub fn get_tx_url(&self, tx_hash: &str) -> String {
        format!("{}/tx/{}", self.url, tx_hash)
    }

    pub fn get_address_url(&self, address: &str) -> String {
        format!("{}/address/{}", self.url, address)
    }
}
