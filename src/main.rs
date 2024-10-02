use std::path::PathBuf;

use clap::{command, Parser};
use config::{Config as ConfigLoader, File};
use sea_orm_migration::MigratorTrait;
use task_flow_backend::{
    client::{postgres::PostgresClient, ClientBuilder},
    config::Config,
    migration::Migrator,
};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    config: PathBuf,
}

#[actix_web::main]
async fn main() {
    let args: Args = Args::parse();
    let config: Config = ConfigLoader::builder()
        .add_source(File::with_name(args.config.to_str().unwrap()))
        .build()
        .expect("Can't parse config file")
        .try_deserialize::<Config>()
        .expect("Can't deserialize");

    let db: PostgresClient = PostgresClient::from_config(&config)
        .await
        .expect("Creating postgres client error");

    Migrator::up(&db, None).await.expect("Up migrations error");
}
