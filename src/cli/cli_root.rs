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
    #[arg(long, default_value = "2.8.2", long_help = "registry version")]
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
