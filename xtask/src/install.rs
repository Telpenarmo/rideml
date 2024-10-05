use std::ffi::OsStr;

use anyhow::{Context, Ok, Result};
use xshell::{cmd, Shell};

use crate::Install;

pub(crate) fn install(sh: &Shell, opts: &Install) -> Result<()> {
    if !opts.skip_server {
        install_server(sh)?;
    }
    if !opts.skip_extension {
        let vscode_path = OsStr::new("code").to_os_string();
        let vscode_path = opts.code_path.as_ref().unwrap_or(&vscode_path);
        install_client(sh, vscode_path)?;
    }
    Ok(())
}

fn install_server(sh: &Shell) -> Result<()> {
    let cmd = cmd!(sh, "cargo install --path crates/server --locked");
    cmd.run()?;
    Ok(())
}

fn install_client(sh: &Shell, vscode_path: &OsStr) -> Result<()> {
    let _dir = sh.push_dir("editors/vscode");

    cmd!(sh, "pnpm --version")
        .run()
        .context("`pnpm` is required to build the VS Code plugin")?;

    cmd!(sh, "pnpm install").run()?;
    cmd!(sh, "pnpm run package").run()?;

    cmd!(
        sh,
        "{vscode_path} --install-extension out/camlot-analyzer.vsix"
    )
    .run()?;

    Ok(())
}
