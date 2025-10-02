<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';

  export let voice: string;
  export let rate: number;
  const dispatch = createEventDispatcher();

  let title = '';
  let text = '';
  let loading = false;
  let error: string | null = null;

  async function loadDocument(path: string) {
    try {
      loading = true;
      const content = await invoke<string>('import_document', { path });
      text = content;
      title = title || extractTitle(path);
      error = null;
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
    } finally {
      loading = false;
    }
  }

  function extractTitle(path: string) {
    const parts = path.split(/[\\/]/);
    const last = parts[parts.length - 1] ?? 'Documento';
    return last.replace(/\.[^.]+$/, '');
  }

  async function handleFile(event: Event) {
    const input = event.currentTarget as HTMLInputElement;
    const file = input.files?.[0] as File & { path?: string };
    if (!file?.path) {
      error = 'No se pudo leer la ruta del archivo.';
      return;
    }
    await loadDocument(file.path);
  }

  function reset() {
    title = '';
    text = '';
  }

  function addToQueue() {
    if (!title.trim() || !text.trim()) {
      error = 'Ingresa un título y contenido para la cola.';
      return;
    }
    dispatch('add', { title: title.trim(), text: text.trim(), voice, rate });
    reset();
  }
</script>

<section class="importer">
  <header>
    <h2>Importar documento</h2>
    <p>Selecciona un archivo EPUB, PDF o TXT para convertirlo a audio.</p>
  </header>
  <div class="input-group">
    <label class="file">
      <span>📁 Seleccionar archivo</span>
      <input type="file" accept=".epub,.pdf,.txt" on:change={handleFile} />
    </label>
    <button on:click={addToQueue} disabled={loading}>Añadir a cola</button>
  </div>
  <label class="field">
    <span>Título</span>
    <input type="text" bind:value={title} placeholder="Título del fragmento" />
  </label>
  <label class="field">
    <span>Texto</span>
    <textarea bind:value={text} rows={8} placeholder="Contenido a sintetizar"></textarea>
  </label>
  {#if loading}
    <p class="status">Importando documento…</p>
  {/if}
  {#if error}
    <p class="error">{error}</p>
  {/if}
</section>

<style>
  .importer {
    background: rgba(15, 23, 42, 0.75);
    padding: 1.5rem;
    border-radius: 16px;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .input-group {
    display: flex;
    gap: 1rem;
  }

  label.file {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    background: rgba(148, 163, 184, 0.2);
    padding: 0.75rem 1rem;
    border-radius: 12px;
    cursor: pointer;
  }

  label.file input {
    display: none;
  }

  button {
    background: linear-gradient(135deg, #f97316, #facc15);
    border: none;
    color: #0f172a;
    padding: 0.75rem 1.25rem;
    border-radius: 12px;
    cursor: pointer;
    font-weight: 600;
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .field input,
  .field textarea {
    padding: 0.75rem;
    border-radius: 12px;
    border: 1px solid rgba(148, 163, 184, 0.4);
    background: rgba(15, 23, 42, 0.5);
    color: inherit;
  }

  .status {
    margin: 0;
    color: #38bdf8;
  }

  .error {
    margin: 0;
    color: #fca5a5;
  }
</style>
