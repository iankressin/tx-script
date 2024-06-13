pub mod backend;
pub mod cli;
pub mod common;
pub mod frontend;

use backend::{
    dispatcher::local_dispatcher::LocalDispatcher,
    dispatcher::tx_dispatcher::TransactionDispatcher, tx_builder::TransactionBuilder,
};
use clap::Parser;
use cli::Arguments;
use frontend::parser::TxLangParser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Arguments::parse();

    match args.subcmd {
        cli::Subcommands::Run(run_args) => {
            let file = std::fs::read_to_string(run_args.file)?;
            let mut parser = TxLangParser::new(&file);
            parser.build_ir()?;

            let prepared_txs = TransactionBuilder::build(parser.txs);
            let dispatcher = LocalDispatcher::new(None);
            dispatcher.dispatch(prepared_txs).await?;
        }
        cli::Subcommands::Repl => {
            println!("REPL not implemented yet");
        }
    }

    Ok(())
}
