use clap::{Parser, Subcommand};

#[derive(Debug, Clone, Parser)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(subcommand)]
    pub kind: ArgKind
}

#[derive(Debug, Clone, PartialEq, Subcommand)]
pub enum ArgKind {
    Server {
        port: u16,
        size: usize
    },
    Client {
        server_ip: String,
        #[arg(default_value_t = 4444)]
        server_port: u16
    }
}