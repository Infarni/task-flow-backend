use std::path::PathBuf;

use clap::{command, Parser};
use config::{Config as ConfigLoader, File};
use task_flow_backend::config::Config;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    config: PathBuf,
}

fn main() {
    let args: Args = Args::parse();
    let config: ConfigLoader = ConfigLoader::builder()
        .add_source(File::with_name(args.config.to_str().unwrap()))
        .build()
        .expect("Can't parse config file");

    println!(
        "{:?}",
        config
            .try_deserialize::<Config>()
            .expect("Can't deserialize")
    );
}
