use std::{error::Error, fs};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Config {
    pub network_info: NetWorkConfig,
    pub account_info: AccountInfoConfig,
    pub token_info: TokenConfig,
    pub mint_info: MintConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct NetWorkConfig {
    pub https: String,
    pub chain_id: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AccountInfoConfig {
    pub private_key: String,
    pub address: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TokenConfig {
    pub tick: String,
    pub total: u64,
    pub amt: u64,
    pub protocol: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MintConfig {
    pub amount: u32,
}

impl Config {
    pub fn load(path: &str) -> Result<Config, Box<dyn Error>> {
        match fs::read_to_string(path) {
            Ok(config) => {
                let config: Self = toml::from_str(&config).unwrap();
                Ok(config)
            }
            Err(e) => {
                info!("***** 加载配置文件异常 *****");
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    e,
                )))
            }
        }
    }
}
