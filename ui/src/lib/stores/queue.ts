import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { QueueItem, VoiceInfo } from '$lib/types';

export const queue = writable<QueueItem[]>([]);
export const voices = writable<VoiceInfo[]>([]);
export const isProcessing = writable(false);
export const lastError = writable<string | null>(null);

export async function loadQueue() {
  const items = await invoke<QueueItem[]>('get_queue');
  queue.set(items);
}

export async function fetchVoices() {
  const list = await invoke<VoiceInfo[]>('list_voices');
  voices.set(list);
}

export async function enqueue(title: string, text: string, voice?: string, rate?: number) {
  await invoke('enqueue_text', {
    items: [
      {
        title,
        text,
        voice,
        rate,
        format: 'wav'
      }
    ]
  });
  await loadQueue();
}

export async function remove(id: string) {
  await invoke('remove_from_queue', { id });
  await loadQueue();
}

export async function clear() {
  await invoke('clear_queue');
  await loadQueue();
}

export async function synthesizeNext() {
  try {
    isProcessing.set(true);
    const item = await invoke<QueueItem>('synthesize_next');
    queue.update((items) => items.map((q) => (q.id === item.id ? item : q)));
    isProcessing.set(false);
    return item;
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    lastError.set(message);
    isProcessing.set(false);
    throw error;
  }
}

export async function registerListeners() {
  const unlistenCompleted = await listen('queue::completed', async () => {
    await loadQueue();
  });
  const unlistenFailed = await listen('queue::failed', async () => {
    await loadQueue();
  });
  return () => {
    unlistenCompleted();
    unlistenFailed();
  };
}

export async function exportAudio(
  itemId: string,
  format: 'wav' | 'mp3',
  options: { voice?: string; speed?: number; directory?: string } = {}
) {
  await invoke('export_audio', {
    options: {
      itemId,
      format,
      voice: options.voice,
      speed: options.speed,
      directory: options.directory
    }
  });
  await loadQueue();
}
