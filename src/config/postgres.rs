use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct PostgresConfig {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database_name: String
}

impl PostgresConfig {
    pub fn get_url(&self) -> String {
        format!(
            "postgres://{username}:{password}@{host}:{port}/{database_name}",
            username = self.username,
            password = self.password,
            host = self.host,
            port = self.port,
            database_name = self.database_name,
        )
    }
}
