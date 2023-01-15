use config::{Config, File, FileFormat};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ConfigData {
    pub redis: RedisConfigData,
    pub db: DbConfigData,
}

#[derive(Deserialize, Debug)]
pub struct RedisConfigData {
    pub username: Option<String>,
    pub host: String,
    pub port: u16,
    pub password: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct DbConfigData {
    
}

pub fn read_config() -> ConfigData {
    let config_data = Config::builder()
        .add_source(File::new("src/configs/Development.toml", FileFormat::Toml))
        .build();

    let config: ConfigData = config_data.unwrap().try_deserialize().unwrap();
    println!("{:?}", config);

    config
}
