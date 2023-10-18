#![allow(unused)]
use clap::Parser;

mod user;
mod message;
mod args;
mod server;
mod client;

fn main() {
    let args = args::Args::parse();
    match args.kind {
        args::ArgKind::Server { port, size } => server::run(port, size),
        args::ArgKind::Client { server_ip, server_port } => client::run(server_ip, server_port),
    }
}