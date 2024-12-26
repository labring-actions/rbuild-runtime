use clap::Parser;
use log::{info, trace, LevelFilter};
use rbuild_runtime::BuildOpts;
use rbuild_runtime::CmdExec;
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "files/"]
struct Asset;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let asset = Asset::get("sealos").unwrap();
    info!("log level set to: {:?}", LevelFilter::Info);
    trace!("asset: {:?}", asset.data.len());
    let opts: BuildOpts = BuildOpts::parse();
    opts.cmd.execute(opts.base_opts.clone()).await?;
    Ok(())
}
