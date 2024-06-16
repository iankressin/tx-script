use std::io::{stdin, stdout, Write};

use crate::{
    backend::{
        dispatcher::local_dispatcher::LocalDispatcher,
        dispatcher::tx_dispatcher::TransactionDispatcher, tx_builder::TransactionBuilder,
    },
    frontend::parser::TxLangParser,
};

pub struct Repl;

impl Repl {
    pub fn new() -> Self {
        Repl
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.clear_terminal();
        let mut stdout_instance = stdout();

        loop {
            stdout_instance.write_all(b"tx-script > ")?;
            stdout_instance.flush()?;
            let tx = self.read();
            self.eval(tx).await?;
        }
    }

    fn read(&self) -> String {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input
    }

    async fn eval(&self, tx: String) -> Result<(), Box<dyn std::error::Error>> {
        let mut parser = TxLangParser::new(&tx);
        parser.build_ir()?;

        let prepared_txs = TransactionBuilder::build(parser.txs);
        let dispatcher = LocalDispatcher::new(None);
        dispatcher.dispatch(prepared_txs).await?;

        Ok(())
    }

    fn clear_terminal(&self) {
        print!("\x1B[2J\x1B[1;1H");
    }
}
