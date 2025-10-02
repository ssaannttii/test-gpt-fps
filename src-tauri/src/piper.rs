use crate::audio::AudioFormat;
use anyhow::{anyhow, Context, Result};
use async_process::Command;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;
use thiserror::Error;
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PiperConfig {
    pub binary_path: PathBuf,
    pub voices_dir: PathBuf,
    pub default_voice: String,
    pub models_manifest: Option<PathBuf>,
}

impl Default for PiperConfig {
    fn default() -> Self {
        let binary_path = PathBuf::from(std::env::var("PIPER_BIN").unwrap_or_else(|_| "piper".into()));
        let voices_dir = std::env::var("PIPER_VOICES")
            .map(PathBuf::from)
            .unwrap_or_else(|_| default_voice_dir());
        Self {
            binary_path,
            voices_dir,
            default_voice: "en_US-amy-medium".to_string(),
            models_manifest: None,
        }
    }
}

impl PiperConfig {
    pub fn from_env() -> Result<Self> {
        let mut config = PiperConfig::default();
        if let Ok(default_voice) = std::env::var("PIPER_DEFAULT_VOICE") {
            config.default_voice = default_voice;
        }
        if let Ok(manifest) = std::env::var("PIPER_MANIFEST") {
            config.models_manifest = Some(PathBuf::from(manifest));
        }
        Ok(config)
    }
}

#[derive(Debug, Error)]
pub enum PiperError {
    #[error("voice '{0}' not found")]
    VoiceNotFound(String),
    #[error("binary not executable: {0}")]
    InvalidBinary(String),
    #[error("command execution failed: {0}")]
    CommandFailed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoiceInfo {
    pub name: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub quality: Option<String>,
    pub model_path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct PiperEngine {
    config: PiperConfig,
    voices: HashMap<String, VoiceInfo>,
}

#[derive(Debug, Clone)]
pub struct SynthesisRequest {
    pub text: String,
    pub voice: Option<String>,
    pub output: PathBuf,
    pub speed: Option<f32>,
    pub format: AudioFormat,
}

impl PiperEngine {
    pub fn new(config: PiperConfig) -> Result<Self> {
        if !config.binary_path.as_path().is_absolute() && which::which(&config.binary_path).is_err() {
            log::warn!("Piper binary '{}' not found in PATH", config.binary_path.display());
        }
        let voices = discover_voices(&config)?;
        Ok(Self { config, voices })
    }

    pub fn config(&self) -> &PiperConfig {
        &self.config
    }

    pub fn voices(&self) -> Vec<VoiceInfo> {
        self.voices.values().cloned().collect()
    }

    pub fn resolve_voice(&self, requested: Option<&str>) -> Result<VoiceInfo> {
        let name = requested.unwrap_or(&self.config.default_voice);
        self.voices
            .get(name)
            .cloned()
            .ok_or_else(|| PiperError::VoiceNotFound(name.to_string()).into())
    }

    pub async fn synthesize(&self, request: SynthesisRequest) -> Result<PathBuf> {
        let voice = self.resolve_voice(request.voice.as_deref())?;
        let mut final_output = request.output;
        if let Some(parent) = final_output.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut piper_output = final_output.clone();
        if request.format == AudioFormat::Mp3 {
            piper_output = final_output.with_extension("wav");
        }
        let mut cmd = Command::new(&self.config.binary_path);
        cmd.arg("--model")
            .arg(&voice.model_path)
            .arg("--output_file")
            .arg(&piper_output)
            .arg("--sentence_silence")
            .arg("0.2");
        if let Some(speed) = request.speed {
            cmd.arg("--length_scale").arg(format!("{speed}"));
        }
        cmd.stdin(async_process::Stdio::piped());
        let mut child = cmd.spawn().map_err(|err| anyhow!("failed to spawn Piper: {err}"))?;
        if let Some(mut stdin) = child.stdin.take() {
            use async_std::io::WriteExt;
            stdin
                .write_all(request.text.as_bytes())
                .await
                .context("write text to Piper")?;
        }
        let status = child.status().await?;
        if !status.success() {
            return Err(PiperError::CommandFailed(status.to_string()).into());
        }
        if request.format == AudioFormat::Mp3 {
            final_output = crate::audio::transcode_wav_to_mp3(&piper_output)?;
            let _ = fs::remove_file(&piper_output);
        } else {
            final_output = piper_output;
        }
        Ok(final_output)
    }
}

fn discover_voices(config: &PiperConfig) -> Result<HashMap<String, VoiceInfo>> {
    let mut voices = HashMap::new();
    if !config.voices_dir.exists() {
        log::warn!("voices directory '{}' missing", config.voices_dir.display());
        return Ok(voices);
    }
    for entry in WalkDir::new(&config.voices_dir) {
        let entry = entry?;
        if entry.file_type().is_file() {
            let path = entry.into_path();
            if let Some(ext) = path.extension() {
                if ext == "onnx" {
                    let stem = path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| path.display().to_string());
                    voices.insert(
                        stem.clone(),
                        VoiceInfo {
                            name: stem,
                            description: None,
                            language: None,
                            quality: None,
                            model_path: path,
                        },
                    );
                }
            }
        }
    }
    Ok(voices)
}

fn default_voice_dir() -> PathBuf {
    dirs::data_dir()
        .map(|mut dir| {
            dir.push("piper");
            dir.push("voices");
            dir
        })
        .unwrap_or_else(|| PathBuf::from("./voices"))
}
