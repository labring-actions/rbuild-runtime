use crate::CmdExec;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct DockerOpts {
    #[arg(long, long_help = "docker cli version", default_value = "20.10.9")]
    pub version: String,
    #[arg(long, long_help = "cri dockerd version", default_value = "v0.3.14")]
    pub cri_docker_version: String,
}

impl CmdExec for DockerOpts {
    async fn execute(self) -> anyhow::Result<()> {
        println!(
            "docker version: {}, cri version: {:?}",
            self.version, self.cri_docker_version
        );
        Ok(())
    }
}
