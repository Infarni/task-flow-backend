use actix_web::{middleware::Logger, web, App, HttpServer};

use crate::{
    api::service_configure,
    client::{postgres::PostgresClient, ClientBuilder},
    config::Config,
    error::server::{ServerError, ServerResult},
};

#[derive(Debug, Clone)]
pub struct State {
    pub postgres: PostgresClient,
    pub config: Config,
}

pub struct Server {
    pub host: String,
    pub port: u16,
    pub state: State,
}

impl State {
    pub async fn new(config: &Config) -> ServerResult<Self> {
        let postgres: PostgresClient = PostgresClient::from_config(config).await?;

        Ok(Self {
            config: config.clone(),
            postgres,
        })
    }
}

impl Server {
    pub async fn new(config: Config) -> ServerResult<Self> {
        Ok(Self {
            host: config.server.host.clone(),
            port: config.server.port,
            state: State::new(&config).await?,
        })
    }

    pub async fn run(&self) -> ServerResult {
        let app_data: web::Data<State> = web::Data::new(self.state.clone());

        match HttpServer::new(move || {
            App::new()
                .wrap(Logger::default())
                .app_data(app_data.clone())
                .configure(service_configure)
        })
        .bind((self.host.clone(), self.port))
        {
            Ok(value) => match value.run().await {
                Ok(_) => Ok(()),
                Err(err) => Err(ServerError::Run(err.to_string())),
            },
            Err(err) => Err(ServerError::Run(err.to_string())),
        }
    }
}
