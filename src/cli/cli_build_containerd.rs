use super::parse_container_runtime;
use crate::{CmdExec, ContainerRuntime};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct ContainerdOpts {
    #[arg(long, default_value = "v1.6.23")]
    pub version: String,
    #[arg(
        short, long, long_help = "using container runtime name", default_value = "runc", value_parser = parse_container_runtime
    )]
    pub runtime: ContainerRuntime,
}

impl CmdExec for ContainerdOpts {
    async fn execute(self) -> anyhow::Result<()> {
        println!(
            "containerd version: {}, runtime: {:?}",
            self.version, self.runtime
        );
        Ok(())
    }
}
