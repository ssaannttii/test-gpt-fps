use crate::audio::{generate_output_path, AudioFormat};
use crate::piper::{SynthesisRequest, VoiceInfo};
use crate::{QueueItem, QueueStatus, SharedState};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::{AppHandle, Manager, Runtime, State};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewQueueItem {
    pub title: String,
    pub text: String,
    pub voice: Option<String>,
    pub rate: Option<f32>,
    pub format: Option<AudioFormat>,
}

pub fn enqueue_text_impl(state: &SharedState, items: Vec<NewQueueItem>) -> Result<Vec<QueueItem>, String> {
    let mut queue = state.queue.lock();
    let mut created = Vec::new();
    for item in items {
        let id = Uuid::new_v4();
        let queue_item = QueueItem {
            id,
            title: item.title.clone(),
            text: item.text.clone(),
            voice: item
                .voice
                .unwrap_or_else(|| state.piper.config().default_voice.clone()),
            rate: item.rate.unwrap_or(1.0),
            format: item.format.unwrap_or(AudioFormat::Wav),
            status: QueueStatus::Pending,
            output: None,
        };
        queue.push_back(queue_item.clone());
        created.push(queue_item);
    }
    Ok(created)
}

#[tauri::command]
pub fn enqueue_text(state: State<'_, SharedState>, items: Vec<NewQueueItem>) -> Result<Vec<QueueItem>, String> {
    enqueue_text_impl(state.inner(), items)
}

pub fn remove_from_queue_impl(state: &SharedState, id: Uuid) {
    let mut queue = state.queue.lock();
    if let Some(pos) = queue.iter().position(|item| item.id == id) {
        queue.remove(pos);
    }
}

#[tauri::command]
pub fn remove_from_queue(state: State<'_, SharedState>, id: Uuid) -> Result<(), String> {
    remove_from_queue_impl(state.inner(), id);
    Ok(())
}

pub fn clear_queue_impl(state: &SharedState) {
    state.queue.lock().clear();
}

#[tauri::command]
pub fn clear_queue(state: State<'_, SharedState>) -> Result<(), String> {
    clear_queue_impl(state.inner());
    Ok(())
}

#[tauri::command]
pub fn get_queue(state: State<'_, SharedState>) -> Result<Vec<QueueItem>, String> {
    let queue = state.queue.lock();
    Ok(queue.iter().cloned().collect())
}

#[tauri::command]
pub fn list_voices(state: State<'_, SharedState>) -> Result<Vec<VoiceInfo>, String> {
    Ok(state.piper.voices())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportOptions {
    pub item_id: Uuid,
    pub format: AudioFormat,
    pub speed: Option<f32>,
    pub voice: Option<String>,
    pub directory: Option<PathBuf>,
}

#[tauri::command]
pub async fn synthesize_next<R: Runtime>(app: AppHandle<R>, state: State<'_, SharedState>) -> Result<QueueItem, String> {
    let next = {
        let mut queue = state.queue.lock();
        queue
            .iter_mut()
            .find(|item| item.status == QueueStatus::Pending)
            .map(|item| {
                item.status = QueueStatus::Processing;
                item.clone()
            })
    };
    let mut item = match next {
        Some(item) => item,
        None => return Err("No pending items".to_string()),
    };

    let export_dir = {
        let config = state.config.lock();
        config.export_dir.clone()
    };
    let format = item.format;
    let output_path = generate_output_path(&export_dir, &item.title, format);
    let request = SynthesisRequest {
        text: item.text.clone(),
        voice: Some(item.voice.clone()),
        output: output_path.clone(),
        speed: Some(item.rate),
        format,
    };
    let result = state
        .piper
        .synthesize(request)
        .await
        .map_err(|err| err.to_string());

    let mut queue = state.queue.lock();
    if let Some(stored) = queue.iter_mut().find(|q| q.id == item.id) {
        match result {
            Ok(path) => {
                stored.status = QueueStatus::Completed;
                stored.output = Some(path.clone());
                item.status = QueueStatus::Completed;
                item.output = Some(path);
                if let Err(err) = app.emit_all("queue::completed", &stored) {
                    log::warn!("failed to emit completion event: {err}");
                }
            }
            Err(err) => {
                stored.status = QueueStatus::Failed;
                if let Err(event_err) = app.emit_all("queue::failed", &(stored.id, err.clone())) {
                    log::warn!("failed to emit failure event: {event_err}");
                }
                return Err(err);
            }
        }
    }
    Ok(item)
}

#[tauri::command]
pub async fn export_audio(state: State<'_, SharedState>, options: ExportOptions) -> Result<PathBuf, String> {
    let item = {
        let queue = state.queue.lock();
        queue.iter().find(|item| item.id == options.item_id).cloned()
    }
    .ok_or_else(|| "Item not found".to_string())?;

    let directory = options.directory.unwrap_or_else(|| {
        let config = state.config.lock();
        config.export_dir.clone()
    });
    let format = options.format;
    let output_path = generate_output_path(&directory, &item.title, format);
    let request = SynthesisRequest {
        text: item.text,
        voice: options.voice.or(Some(item.voice)),
        output: output_path.clone(),
        speed: options.speed.or(Some(item.rate)),
        format,
    };
    state
        .piper
        .synthesize(request)
        .await
        .map_err(|err| err.to_string())?;
    if let Some(stored) = state
        .queue
        .lock()
        .iter_mut()
        .find(|existing| existing.id == options.item_id)
    {
        stored.output = Some(output_path.clone());
        stored.format = format;
    }
    Ok(output_path)
}

#[tauri::command]
pub async fn import_document(path: String) -> Result<String, String> {
    let path = PathBuf::from(&path);
    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    let script = match extension.as_str() {
        "epub" => "scripts/import_epub.py",
        "pdf" => "scripts/import_pdf.py",
        "txt" => return std::fs::read_to_string(&path).map_err(|err| err.to_string()),
        other => {
            return Err(format!("Unsupported extension: {other}"));
        }
    };
    let mut command = async_process::Command::new("python3");
    command.arg(script).arg("--file").arg(&path);
    let output = command.output().await.map_err(|err| err.to_string())?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }
    String::from_utf8(output.stdout)
        .map_err(|err| err.to_string())
        .map(|content| content.trim().to_string())
}

#[tauri::command]
pub fn get_config(state: State<'_, SharedState>) -> Result<AppConfig, String> {
    Ok(state.config.lock().clone())
}

#[tauri::command]
pub fn update_export_dir(state: State<'_, SharedState>, path: PathBuf) -> Result<(), String> {
    let mut config = state.config.lock();
    config.export_dir = path;
    Ok(())
}

pub fn register_commands() -> tauri::InvokeHandler<()> {
    tauri::generate_handler![
        enqueue_text,
        remove_from_queue,
        clear_queue,
        get_queue,
        list_voices,
        synthesize_next,
        export_audio,
        import_document,
        get_config,
        update_export_dir
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AppConfig, SharedState};

    fn state() -> SharedState {
        SharedState::new(AppConfig::default()).expect("state")
    }

    #[test]
    fn enqueue_and_remove_items() {
        let state = state();
        let items = enqueue_text_impl(
            &state,
            vec![NewQueueItem {
                title: "Sample".into(),
                text: "Hello".into(),
                voice: None,
                rate: Some(1.0),
                format: None,
            }],
        )
        .expect("enqueue");
        assert_eq!(items.len(), 1);
        assert_eq!(state.queue.lock().len(), 1);
        remove_from_queue_impl(&state, items[0].id);
        assert!(state.queue.lock().is_empty());
    }

    #[test]
    fn clear_queue_removes_all() {
        let state = state();
        enqueue_text_impl(
            &state,
            vec![
                NewQueueItem {
                    title: "One".into(),
                    text: "A".into(),
                    voice: None,
                    rate: None,
                    format: None,
                },
                NewQueueItem {
                    title: "Two".into(),
                    text: "B".into(),
                    voice: None,
                    rate: None,
                    format: None,
                },
            ],
        )
        .expect("enqueue");
        assert_eq!(state.queue.lock().len(), 2);
        clear_queue_impl(&state);
        assert!(state.queue.lock().is_empty());
    }
}
