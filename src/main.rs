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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    use std::process::Command;

    #[test]
    fn test_sealos_file() -> anyhow::Result<()> {
        let asset = Asset::get("sealos").unwrap();
        assert!(asset.data.len() > 0);
        let path = Path::new("sealos");
        let mut file = File::create(path)?;
        file.write_all(&asset.data)?;
        Command::new("chmod")
            .arg("a+x")
            .arg("sealos")
            .status()
            .expect("Failed to execute chmod");
        assert!(path.exists());
        Command::new("./sealos")
            .arg("version")
            .status()
            .expect("Failed to execute sealos");
        Ok(())
    }
}
