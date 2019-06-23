extern crate config;

use config::Value;

use config::Config;
use serde::de::Deserialize;
use std::sync::RwLock;

lazy_static! {
    pub static ref SETTINGS: RwLock<Config> = {
        let mut settings = Config::default();
        settings
            .merge(config::File::with_name("Settings"))
            .unwrap()
            .merge(config::Environment::with_prefix("LSYS"))
            .unwrap();
        RwLock::new(settings)
    };
}

pub fn get_config<T>(name: &'static str) -> T
where
    T: Deserialize<'static>,
{
    SETTINGS.read().unwrap().get::<T>(name).unwrap()
}

pub fn set_config<T>(name: &str, value: T)
where
    T: Into<Value>,
{
    match SETTINGS.write().unwrap().set(name, value) {
        Ok(_) => (),
        Err(e) => panic!("couldn't write config: {}", e),
    }
}
