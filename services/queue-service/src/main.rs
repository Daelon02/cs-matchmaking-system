use crate::config::command_line::Opt;
use crate::errors::AppResult;
use common::log::init_logging;
use common::log::log_config::LoggingOpts;
use structopt::StructOpt;

mod api;
mod config;
mod errors;
mod services;
mod state;
mod utils;

#[tokio::main]
async fn main() -> AppResult<()> {
    let opts = Opt::from_args();
    init_logging(opts.application.logging)?;
    println!("Hello, world!");
    Ok(())
}
