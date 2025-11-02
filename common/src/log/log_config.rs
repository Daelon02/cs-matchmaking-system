use serde::{Deserialize, Deserializer};
use serde_json::error::Error as SerdeError;
use std::collections::HashMap;
use std::ops::Deref;
use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LoggingOpts {
    /// Sets a logging level
    #[structopt(case_insensitive = true, long, short = "L", env = "LOG_LEVEL", default_value = crate::log::consts::DEFAULT_LOGGING)]
    pub level: String,
    // pub logging: LogLevel,
    /// File to which application will write logs
    #[structopt(long, short = "O", env = "LOG_OUTPUT_FILE")]
    pub log_output_file: PathBuf,

    #[structopt(flatten)]
    pub enable_additional_files: EnableAdditionalFilesOpts,

    #[structopt(
        long,
        env = "LOG_USE_FILE_INFO",
        default_value = "true",
        parse(try_from_str)
    )]
    pub log_level_for: LogLevelFor,
}

#[derive(StructOpt, Debug, Deserialize)]
pub struct EnableAdditionalFilesOpts {
    #[structopt(
        long,
        env = "LOG_USE_FILE_INFO",
        default_value = "true",
        parse(try_from_str)
    )]
    pub info: bool,

    #[structopt(
        long,
        env = "LOG_USE_FILE_WARNING",
        default_value = "true",
        parse(try_from_str)
    )]
    pub warning: bool,

    #[structopt(
        long,
        env = "LOG_USE_FILE_ERROR",
        default_value = "true",
        parse(try_from_str)
    )]
    pub error: bool,
}

#[derive(Clone, Debug)]
pub struct LogLevelFor {
    pub inner: HashMap<String, String>,
}

impl Deref for LogLevelFor {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a> Deserialize<'a> for LogLevelFor {
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        let map: HashMap<String, String> = Deserialize::deserialize(deserializer)?;
        Ok(LogLevelFor { inner: map })
    }
}

impl FromStr for LogLevelFor {
    type Err = SerdeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}
