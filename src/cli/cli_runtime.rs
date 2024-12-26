use crate::cli::cli_runtime_containerd::ContainerdOpts;
use crate::cli::cli_runtime_crio::CrioOpts;
use crate::cli::cli_runtime_docker::DockerOpts;
use crate::CacheOpts;
use clap::Parser;
use enum_dispatch::enum_dispatch;

#[derive(Parser, Debug, Clone)]
#[enum_dispatch(CmdExec)]
pub enum Runtime {
    #[clap(
        name = "containerd",
        about = "build the runtime for containerd runtime images"
    )]
    Containerd(ContainerdOpts),
    #[clap(name = "docker", about = "build the runtime for docker runtime images")]
    Docker(DockerOpts),
    #[clap(name = "crio", about = "build the runtime for crio runtime images")]
    Crio(CrioOpts),
    #[clap(name = "cache", about = "cache the runtime images")]
    Cache(CacheOpts),
}
