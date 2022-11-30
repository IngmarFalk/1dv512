#[derive(clap::Parser)]
pub struct Config {
    #[clap(long, env)]
    pub input: String,
    #[clap(long, env)]
    pub output: String,
}
