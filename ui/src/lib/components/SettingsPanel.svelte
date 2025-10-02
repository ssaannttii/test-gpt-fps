<script lang="ts">
  import { voices } from '$lib/stores/queue';

  export let selectedVoice: string;
  export let rate: number;
  export let onVoiceChange: (voice: string) => void;
  export let onRateChange: (rate: number) => void;
  export let exportDir: string;
  export let onChangeExportDir: () => void;

  $: voiceList = $voices;
</script>

<section class="settings">
  <h2>Ajustes</h2>
  <label>
    <span>Voz</span>
    <select bind:value={selectedVoice} on:change={(event) => onVoiceChange((event.target as HTMLSelectElement).value)}>
      {#if voiceList.length === 0}
        <option value={selectedVoice}>Voz predeterminada ({selectedVoice})</option>
      {:else}
        {#each voiceList as voice}
          <option value={voice.name}>{voice.name}</option>
        {/each}
      {/if}
    </select>
  </label>
  <label>
    <span>Velocidad</span>
    <input
      type="range"
      min="0.5"
      max="1.5"
      step="0.05"
      bind:value={rate}
      on:input={(event) => onRateChange(parseFloat((event.target as HTMLInputElement).value))}
    />
    <small>{rate.toFixed(2)}x</small>
  </label>
  <div class="export">
    <span>Directorio de exportación</span>
    <div class="path">{exportDir}</div>
    <button on:click={onChangeExportDir}>Cambiar…</button>
  </div>
</section>

<style>
  .settings {
    background: rgba(15, 23, 42, 0.75);
    padding: 1.5rem;
    border-radius: 16px;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  label {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  select,
  input[type='range'] {
    width: 100%;
  }

  select {
    padding: 0.75rem;
    border-radius: 12px;
    border: 1px solid rgba(148, 163, 184, 0.4);
    background: rgba(15, 23, 42, 0.5);
    color: inherit;
  }

  input[type='range'] {
    accent-color: #6366f1;
  }

  .export {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .path {
    font-family: 'JetBrains Mono', monospace;
    background: rgba(30, 41, 59, 0.7);
    padding: 0.75rem;
    border-radius: 12px;
    word-break: break-all;
  }

  button {
    align-self: flex-start;
    background: rgba(148, 163, 184, 0.2);
    color: #e2e8f0;
    border: 1px solid rgba(148, 163, 184, 0.3);
    padding: 0.5rem 0.75rem;
    border-radius: 10px;
    cursor: pointer;
  }
</style>
