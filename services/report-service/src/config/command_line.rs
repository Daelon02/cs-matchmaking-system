use crate::config::consts::CARGO_PKG_NAME;
use common::log::log_config::LoggingOpts;
use serde::Deserialize;
use structopt::StructOpt;

#[derive(StructOpt, Debug, Deserialize)]
#[structopt(name = CARGO_PKG_NAME)]
#[serde(rename_all = "kebab-case")]
pub struct Opt {
    #[structopt(flatten)]
    pub application: AppOpts,
}

#[derive(StructOpt, Debug, Deserialize)]
pub struct AppOpts {
    #[structopt(long = "listen", short = "l", default_value = crate::config::consts::DEFAULT_LISTENER)]
    pub bind: String,

    #[structopt(flatten)]
    pub logging: LoggingOpts,
}
