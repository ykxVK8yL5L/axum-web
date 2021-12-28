use structopt::StructOpt;


#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "axum-web")]
pub struct ServerConfig {
    /// Listen host
    #[structopt(long, env = "HOST", default_value = "0.0.0.0")]
    pub host: String,
    /// Listen port
    #[structopt(short, env = "PORT", long, default_value = "10099")]
    pub port: u16,
    /// Listen webdav port
    #[structopt(short, env = "WEBDAV_PORT", long, default_value = "10098")]
    pub webdav_port: u16,
    /// Data save path
    #[structopt(short, env = "ROOT", long, default_value = ".")]
    pub root: String,
    /// Database name
    #[structopt(short, env = "DATABASE", long, default_value = "axum-web.db")]
    pub database: String,
}
