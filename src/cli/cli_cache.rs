use crate::{BaseOpts, CmdExec};
use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct CacheOpts {
    #[arg(
        long,
        long_help = "registry host address",
        default_value = "sealos.hub:5000"
    )]
    pub registry_host: String,
    #[arg(long, long_help = "registry repo name", default_value = "5000")]
    pub registry_repo: String,
    #[arg(long, long_help = "registry username", default_value = "admin")]
    pub registry_username: String,
    #[arg(long, long_help = "registry password", default_value = "passw0rd")]
    pub registry_password: String,
}

impl CmdExec for CacheOpts {
    async fn execute(self, _base_opts: BaseOpts) -> anyhow::Result<()> {
        println!(
            "registry info host: {}, username: {}, repo: {}",
            self.registry_host, self.registry_username, self.registry_repo
        );
        Ok(())
    }
}
