use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PronunciationDictionary {
    entries: HashMap<String, String>,
}

impl PronunciationDictionary {
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = fs::read_to_string(path).context("failed to read dictionary")?;
        let entries: HashMap<String, String> = serde_json::from_str(&content).context("invalid dictionary format")?;
        Ok(Self { entries })
    }

    pub fn translate<'a>(&'a self, input: &'a str) -> &'a str {
        self.entries.get(input).map(|s| s.as_str()).unwrap_or(input)
    }
}
