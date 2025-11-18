use anyhow::{Result, bail};

pub fn handle_placeholder(command: &str) -> Result<()> {
    bail!(
        "`music {command}` is not implemented yet. Use `music {command} --help` to preview its planned behavior."
    )
}
