pub use cli_build_containerd::*;
pub use cli_build_crio::*;
pub use cli_build_docker::*;
pub use cli_root::*;
pub use cli_runtime::*;
use std::fmt;
use std::str::FromStr;

mod cli_build_containerd;
mod cli_build_crio;
mod cli_build_docker;
#[allow(clippy::module_inception)]
mod cli_root;
mod cli_runtime;

#[derive(Debug, Copy, Clone)]
pub enum ContainerRuntime {
    RunC,
    CRun,
    Youki,
}

fn parse_container_runtime(s: &str) -> Result<ContainerRuntime, &'static str> {
    s.parse()
}

impl From<ContainerRuntime> for &'static str {
    fn from(f: ContainerRuntime) -> Self {
        match f {
            ContainerRuntime::RunC => "runc",
            ContainerRuntime::CRun => "crun",
            ContainerRuntime::Youki => "youki",
        }
    }
}

impl FromStr for ContainerRuntime {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "runc" => Ok(ContainerRuntime::RunC),
            "crun" => Ok(ContainerRuntime::CRun),
            "youki" => Ok(ContainerRuntime::Youki),
            _ => Err("Invalid file format"),
        }
    }
}

impl fmt::Display for ContainerRuntime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
