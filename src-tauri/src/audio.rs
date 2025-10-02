use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AudioFormat {
    Wav,
    Mp3,
}

impl Default for AudioFormat {
    fn default() -> Self {
        AudioFormat::Wav
    }
}

pub fn transcode_wav_to_mp3(path: &Path) -> Result<PathBuf> {
    let wav_path = path;
    let mp3_path = path.with_extension("mp3");
    if wav_path == mp3_path {
        return Ok(mp3_path);
    }
    let status = Command::new("ffmpeg")
        .arg("-y")
        .arg("-i")
        .arg(&wav_path)
        .arg(&mp3_path)
        .status()
        .context("failed to spawn ffmpeg")?;
    if !status.success() {
        anyhow::bail!("ffmpeg exited with status {status}");
    }
    Ok(mp3_path)
}

pub fn generate_output_path(base: &Path, title: &str, format: AudioFormat) -> PathBuf {
    let sanitized = title
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || matches!(c, ' ' | '_' | '-'))
        .collect::<String>()
        .trim()
        .replace(' ', "_");
    let extension = match format {
        AudioFormat::Wav => "wav",
        AudioFormat::Mp3 => "mp3",
    };
    base.join(format!("{sanitized}.{extension}"))
}
