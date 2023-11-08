use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ConfigData {
    pub redis: Redis,
    pub db: DbConfigData,
    pub controller: Controller,
}

#[derive(Deserialize, Debug)]
pub struct Redis {
    pub username: Option<String>,
    pub host: String,
    pub port: u16,
    pub password: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Controller {
    pub port: u16,
    pub host: String,
}

#[derive(Deserialize, Debug)]
pub struct RedisConfigData {
    pub redis: Redis,
}

#[derive(Deserialize, Debug)]
pub struct ControllerConfigData {
    pub controller: Controller,
}

impl Controller {
    pub fn get_bind_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[derive(Deserialize, Debug)]
pub struct Db {
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct DbConfigData {
    pub db: Db,
}
