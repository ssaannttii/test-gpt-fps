mod audio;
mod cmds;
mod dict;
mod piper;
mod ssml;

use crate::audio::AudioFormat;
use crate::cmds::register_commands;
use crate::piper::{PiperConfig, PiperEngine};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueItem {
    pub id: Uuid,
    pub title: String,
    pub text: String,
    pub voice: String,
    pub rate: f32,
    pub format: AudioFormat,
    pub status: QueueStatus,
    pub output: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum QueueStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    pub piper: PiperConfig,
    pub export_dir: PathBuf,
}

impl Default for AppConfig {
    fn default() -> Self {
        let piper = PiperConfig::from_env().unwrap_or_default();
        let export_dir = dirs::audio_dir().unwrap_or_else(|| std::env::temp_dir());
        Self { piper, export_dir }
    }
}

pub struct SharedState {
    pub queue: Mutex<VecDeque<QueueItem>>,
    pub current: Mutex<Option<Uuid>>,
    pub config: Mutex<AppConfig>,
    pub piper: PiperEngine,
}

impl SharedState {
    pub fn new(config: AppConfig) -> anyhow::Result<Self> {
        let engine = PiperEngine::new(config.piper.clone())?;
        Ok(Self {
            queue: Mutex::new(VecDeque::new()),
            current: Mutex::new(None),
            config: Mutex::new(config),
            piper: engine,
        })
    }
}

fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    tauri::Builder::default()
        .manage(SharedState::new(AppConfig::default()).expect("failed to initialise state"))
        .invoke_handler(register_commands())
        .setup(|app| {
            let handle = app.handle();
            initialise_window(&handle)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn initialise_window(app: &AppHandle) -> anyhow::Result<()> {
    if let Some(window) = app.get_window("main") {
        window.set_title("PiperDesk")?;
    }
    Ok(())
}

