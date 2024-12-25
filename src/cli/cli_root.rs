use clap::Parser;

use crate::Runtime;

#[derive(Parser, Debug)]
#[command(name = "rbuild-runtime", about, version, author, long_about = None)]
pub struct BuildOpts {
    #[clap(subcommand)]
    pub cmd: Runtime,
    #[arg(long, default_value = "v1.25.0")]
    pub kube_version: String,
    #[arg(long, default_value = "v4.3.7")]
    pub sealos_version: String,
    #[arg(long, default_value = "2.8.2")]
    pub registry_version: String,
}
