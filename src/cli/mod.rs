use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(
    name = "txlang",
    author = "Ian K. Guimaraes <ianguimaraes31@gmail.com>",
    version = "0.0.1"
)]

pub struct Arguments {
    #[clap(subcommand)]
    pub subcmd: Subcommands,
}

#[derive(Debug, Subcommand)]
pub enum Subcommands {
    #[clap(name = "run", about = "Run the transactions in the file")]
    Run(RunArgs),

    #[clap(name = "repl", about = "Starts a REPL to run transactions")]
    Repl,
    
    #[clap(name = "set-pk", about = "Set the private key to sign transactions")]
    SetPk(SetPkArgs),
}

#[derive(Debug, Parser)]
pub struct RunArgs {
    pub file: String,
}

#[derive(Debug, Parser)]
pub struct SetPkArgs {
    pub pk: String,
}
