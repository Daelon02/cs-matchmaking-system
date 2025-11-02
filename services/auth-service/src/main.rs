use structopt::StructOpt;
use crate::errors::AppResult;
use common::log::init_logging;
use crate::config::command_line::Opt;

mod api;
mod config;
mod errors;
mod services;
mod state;
mod utils;
mod routes;

#[tokio::main]
async fn main() -> AppResult<()> {
    let opts = Opt::from_args();
    init_logging(opts.application.logging)?;
    println!("Hello, world!");
    Ok(())
}
