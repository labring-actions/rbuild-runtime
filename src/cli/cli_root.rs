use clap::Parser;

use crate::Runtime;

#[derive(Parser, Debug, Clone)]
#[command(
    name = "rbuild-runtime",
    about,
    version,
    author,
    long_about = "build the runtime images"
)]
pub struct BuildOpts {
    #[clap(subcommand)]
    pub cmd: Runtime,
    #[clap(flatten)]
    pub base_opts: BaseOpts,
}

#[derive(Parser, Debug, Clone)]
pub struct BaseOpts {
    #[arg(
        long,
        default_value = "v1.25.0",
        long_help = "kubernetes version, >=1.26 using v1 cri version"
    )]
    pub kube_version: String,
    #[arg(long, default_value = "v4.3.7", long_help = "sealos version")]
    pub sealos_version: String,
    #[arg(long, default_value = "2.8.2", long_help = "registry version",value_parser = verify_registry)]
    pub registry_version: String,
    #[arg(long,long_help="if using proxy address,use_proxy=true", default_value = "false",value_parser = parse_proxy)]
    pub use_proxy: bool,
}

fn parse_proxy(s: &str) -> Result<bool, &'static str> {
    match s {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err("invalid value, only true or false"),
    }
}

fn verify_registry(registry: &str) -> Result<String, &'static str> {
    let v = registry.split('.').collect::<Vec<&str>>();
    if v.len() != 3 {
        return Err("invalid registry version, must be x.x.x");
    }
    if v[0].parse::<u32>().is_err() || v[1].parse::<u32>().is_err() || v[2].parse::<u32>().is_err()
    {
        return Err("invalid registry version, must be x.x.x");
    }
    if let Ok(num) = v[0].parse::<u32>() {
        if num < 2 {
            return Err("registry version must be >= 2.0.0");
        }
    }
    if let Ok(num) = v[1].parse::<u32>() {
        if num < 8 {
            return Err("registry version must be >= 2.8.0");
        }
    }
    Ok(registry.to_string())
}

//kubeadm config  images list --image-repository k8s.m.daocloud.io
//m.daocloud.io/ghcr.io/labring/lvscare:v4.0.0
//https://files.m.daocloud.io/dl.k8s.io/release/v1.23.0/bin/linux/amd64/kubeadm
