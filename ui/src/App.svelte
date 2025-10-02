<script lang="ts">
import { onDestroy, onMount } from 'svelte';
  import QueueList from '$lib/components/QueueList.svelte';
  import Controls from '$lib/components/Controls.svelte';
  import Importer from '$lib/components/Importer.svelte';
  import SettingsPanel from '$lib/components/SettingsPanel.svelte';
  import {
    queue,
    loadQueue,
    fetchVoices,
    enqueue,
    remove,
    clear,
    synthesizeNext,
    registerListeners,
    lastError,
    voices,
    exportAudio
  } from '$lib/stores/queue';
import type { QueueItem, AppConfig } from '$lib/types';
import { get } from 'svelte/store';
  import { invoke } from '@tauri-apps/api/core';
  import { open } from '@tauri-apps/api/dialog';

  let selectedVoice = '';
  let rate = 1.0;
  let exportDir = '';
  let queueItems: QueueItem[] = [];
  let unlistenQueue: (() => void) | null = null;
  let unsubscribe: (() => void) | null = null;

  onMount(async () => {
    unlistenQueue = queue.subscribe((items) => (queueItems = items));
    await Promise.all([loadQueue(), fetchVoices()]);
    const config = await invoke<AppConfig>('get_config');
    exportDir = config.export_dir;
    const availableVoices = get(voices);
    selectedVoice = availableVoices[0]?.name ?? selectedVoice || 'en_US-amy-medium';
    unsubscribe = await registerListeners();
  });

  async function addToQueue(event: CustomEvent<{ title: string; text: string; voice: string; rate: number }>) {
    const { title, text } = event.detail;
    await enqueue(title, text, selectedVoice, rate);
  }

  async function startNext() {
    await synthesizeNext();
  }

  function updateVoice(voice: string) {
    selectedVoice = voice;
  }

  function updateRate(value: number) {
    rate = value;
  }

  async function changeExportDir() {
    const directory = await open({ directory: true, multiple: false });
    if (typeof directory === 'string') {
      await invoke('update_export_dir', { path: directory });
      exportDir = directory;
    }
  }

  async function exportItem(event: CustomEvent<string>) {
    await exportAudio(event.detail, 'mp3', { voice: selectedVoice, speed: rate });
  }

  $: errorMessage = $lastError;

  onDestroy(() => {
    unlistenQueue?.();
    unsubscribe?.();
  });
</script>

<main>
  <header class="hero">
    <h1>PiperDesk</h1>
    <p>Convierte tus libros y apuntes en audio con Piper TTS, gestiona la cola y exporta fácilmente.</p>
  </header>
  {#if errorMessage}
    <div class="error">{errorMessage}</div>
  {/if}
  <section class="grid">
    <Importer voice={selectedVoice} rate={rate} on:add={addToQueue} />
    <SettingsPanel
      {selectedVoice}
      {rate}
      exportDir={exportDir}
      onVoiceChange={updateVoice}
      onRateChange={updateRate}
      onChangeExportDir={changeExportDir}
    />
  </section>
  <QueueList
    items={queueItems}
    on:remove={async (event) => {
      await remove(event.detail);
    }}
    on:export={exportItem}
  >
    <Controls slot="actions" items={queueItems} onStart={startNext} onClear={clear} />
  </QueueList>
</main>

<style>
  main {
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem;
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }

  .hero {
    text-align: center;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .grid {
    display: grid;
    grid-template-columns: 1.5fr 1fr;
    gap: 1.5rem;
  }

  .error {
    background: rgba(239, 68, 68, 0.2);
    border: 1px solid rgba(239, 68, 68, 0.4);
    padding: 0.75rem 1rem;
    border-radius: 12px;
  }

  @media (max-width: 900px) {
    .grid {
      grid-template-columns: 1fr;
    }
  }
</style>
