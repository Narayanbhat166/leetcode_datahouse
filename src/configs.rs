use config::{Config, File, FileFormat};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ConfigData {
  pub redis: RedisConfigData,
}

#[derive(Deserialize, Debug)]
pub struct RedisConfigData {
  pub username: Option<String>,
  pub host: String,
  pub port: u16,
  pub password: Option<String>,
}

pub fn read_config() -> ConfigData {
    // let res = scrape::get_submission("1".to_string()).unwrap();
    // println!("{:#?}", res);
    let config_data = Config::builder()
        .add_source(File::new("src/configs/Development.toml", FileFormat::Toml))
        .build();

        
        let config: ConfigData =  config_data.unwrap().try_deserialize().unwrap();
        // println!("{:?}", config);

    config
}
