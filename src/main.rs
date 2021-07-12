extern crate clap;
extern crate console;
extern crate octocrab as github;
extern crate regex;
extern crate reqwest;
extern crate semver;
extern crate serde;
extern crate serde_derive;
extern crate tokio;
extern crate tokio_stream;
extern crate tokio_util;
extern crate zip;

mod builder;
mod create;
mod download;
mod error;
mod moving;
mod options;
mod smalltalking;
mod unzip;

use crate::builder::Builder;
use crate::options::SubCommand;
use clap::Clap;
use options::AppOptions;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let options: AppOptions = AppOptions::parse();

    match options.command() {
        SubCommand::Build(build_options) => {
            Builder::new().build(&options, &build_options).await?;
        }
        SubCommand::Get => {}
        SubCommand::Clone => {}
    };

    Ok(())
}
