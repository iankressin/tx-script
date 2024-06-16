pub mod backend;
pub mod cli;
pub mod common;
pub mod frontend;
pub mod repl;

use backend::{
    dispatcher::local_dispatcher::LocalDispatcher,
    dispatcher::tx_dispatcher::TransactionDispatcher, tx_builder::TransactionBuilder,
};
use clap::Parser;
use cli::Arguments;
use frontend::parser::TxLangParser;
use repl::Repl;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Arguments::parse();

    match args.subcmd {
        cli::Subcommands::Run(run_args) => {
            let file_content = std::fs::read_to_string(run_args.file)?;
            let mut parser = TxLangParser::new(&file_content);
            parser.build_ir()?;

            let prepared_txs = TransactionBuilder::build(parser.txs);
            let dispatcher = LocalDispatcher::new(None);
            dispatcher.dispatch(prepared_txs).await?;

            println!("Done!");
        }
        cli::Subcommands::Repl => {
            let repl = Repl::new();
            repl.run().await?;
        }
    }

    Ok(())
}
