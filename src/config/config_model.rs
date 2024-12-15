#[derive(Debug, Clone)]
pub struct DotEnvyConfig {
    pub server: Server,
}

#[derive(Debug, Clone)]
pub struct Server {
    pub course_reg_url: String,
    pub port: u16,
}
