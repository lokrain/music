use anyhow::Result;
use clap::ValueEnum;
use serde::Serialize;
use serde_json::to_string_pretty;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum OutputFormat {
    Text,
    Json,
}

impl OutputFormat {
    pub fn emit<T>(&self, payload: &T, render_text: impl FnOnce(&T) -> String) -> Result<()>
    where
        T: Serialize,
    {
        match self {
            Self::Text => println!("{}", render_text(payload)),
            Self::Json => println!("{}", to_string_pretty(payload)?),
        }
        Ok(())
    }
}
