# TXScript
A simple, non-turing complete scripting language for sending EVM transactions.

## Sentence structure 
TXS supports a simple sentence structure for sending transactions. The sentence structure is as follows:
```
send <amount> <unit | token> to <address> on <chain>
```

### Example
```bash
send 3.1 ether to vitalik.eth on eth
send 10000000 gwei to 0x1234567890abcdef1234567890abcdef12345678 on base
```

## Running transactions
To run a transaction you can use one of the following methods:

### .txs files
Write your transactions in a `.txs` file:
```bash
# payments.txs
send 1 ether to vitalik.eth on eth
send 2 ether to joe.eth on base
```
And run them using the following command:
```bash
txs run payments.txs
```

### REPL
TXS comes with a simple REPL sending out transactions. To start the REPL, run the following command:
```bash
txs repl
```
Then you can start sending transactions:
```bash
tx-script > send 1 ether to vitalik.eth on eth
🛫 Sending tx  | to: vitalik.eth, value: 1 ETH, chain: Ethereum
🛬 Tx included | hash: 0xc4e3…e1db
```

## Installation
First, you should install `txsup`, the TXScript version manager. You can do this by running the following command:
```bash
curl https://raw.githubusercontent.com/iankressin/tx-script/main/txsup/install.sh | sh
````

Then you can install the latest version of TXScript by running the following command:
```bash
txsup
```

### Updating TXScript
To update TXScript to the latest version, you can simply run `txsup` again:
```bash
txsup
```

## Support

### Tokens
The following tokens are supported for the production of `<unit | token>`:

- ether (wei | gwei | ether)
- dai
- usdc
- usdt
- wbtc
- weth

### Chains
The following chains are supported for the production of `<chain>`:

- Ethereum (eth)
- Arbitrum (arb)
- Optimism (op)
- Blast (blast)
- Base (base)
- BNB Smart Chain (bnb)
- Anvil (anvil)
