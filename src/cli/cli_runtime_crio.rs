use super::parse_container_runtime;
use crate::{BaseOpts, CmdExec, ContainerRuntime};
use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct CrioOpts {
    #[arg(long, long_help = "cril version", default_value = "20.10.9")]
    pub version: String,
    #[arg(
        short, long, long_help = "using container runtime name", default_value = "runc", value_parser = parse_container_runtime
    )]
    pub runtime: ContainerRuntime,
}

impl CmdExec for CrioOpts {
    async fn execute(self, _base_opts: BaseOpts) -> anyhow::Result<()> {
        println!(
            "crio version: {}, runtime: {:?}",
            self.version, self.runtime
        );
        Ok(())
    }
}
