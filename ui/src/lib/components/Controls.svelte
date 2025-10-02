<script lang="ts">
  import { isProcessing } from '$lib/stores/queue';
  import type { QueueItem } from '$lib/types';

  export let items: QueueItem[] = [];
  export let onStart: () => void | Promise<void>;
  export let onClear: () => void | Promise<void>;

</script>

<section class="controls">
  <div>
    <button on:click={onStart} disabled={$isProcessing}>▶️ Reproducir siguiente</button>
    <button class="secondary" on:click={onClear}>🧹 Limpiar</button>
  </div>
  <p class="info">{items.filter((item) => item.status === 'pending').length} pendientes</p>
</section>

<style>
  .controls {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
  }

  button {
    background: linear-gradient(135deg, #6366f1, #8b5cf6);
    border: none;
    color: white;
    padding: 0.75rem 1.25rem;
    border-radius: 12px;
    cursor: pointer;
    font-size: 0.95rem;
  }

  button.secondary {
    background: rgba(148, 163, 184, 0.2);
  }

  button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .info {
    margin: 0;
    color: #cbd5f5;
  }
</style>
