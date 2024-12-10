use anyhow::{Ok, Result};

use super::{
    config_model::{DotEnvyConfig, Server},
    stage::Stage,
};

pub fn load() -> Result<DotEnvyConfig> {
    dotenvy::dotenv().ok();

    let server = Server {
        port: std::env::var("SERVER_PORT")
            .expect("SERVER_PORT is invalid")
            .parse()?,
    };

    Ok(DotEnvyConfig { server })
}

pub fn get_stage() -> Stage {
    dotenvy::dotenv().ok();

    let stage_str = std::env::var("STAGE").unwrap_or("".to_string());

    Stage::try_from(&stage_str).unwrap_or_default()
}
