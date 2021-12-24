use structopt::StructOpt;


#[derive(StructOpt, Debug)]
#[structopt(name = "axum-web")]
pub struct ServerConfig {
    /// Listen host
    #[structopt(long, env = "HOST", default_value = "0.0.0.0")]
    pub host: String,
    /// Listen port
    #[structopt(short, env = "PORT", long, default_value = "10099")]
    pub port: u16,
}