use config::{Config, File, FileFormat};
use serde::Deserialize;
pub mod types;

pub fn read_config<'a, T>() -> T
where
    T: Deserialize<'a>,
{
    let config_data = Config::builder()
        .add_source(File::new("configs/src/Development.toml", FileFormat::Toml))
        .build();

    // Unwrap here because without config application cannot be run
    config_data.unwrap().try_deserialize().unwrap()
}
