#[derive(Debug, Clone)]
pub struct DotEnvyConfig {
    pub server: Server,
}

#[derive(Debug, Clone)]
pub struct Server {
    pub port: u16,
}
