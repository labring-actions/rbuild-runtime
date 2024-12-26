pub use cli::*;
use enum_dispatch::enum_dispatch;

mod cli;

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExec {
    async fn execute(self, base_opts: BaseOpts) -> anyhow::Result<()>;
}
