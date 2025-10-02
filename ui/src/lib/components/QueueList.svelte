<script lang="ts">
  import type { QueueItem } from '$lib/types';
  import { createEventDispatcher } from 'svelte';

  export let items: QueueItem[] = [];
  const dispatch = createEventDispatcher();

  const formatLabel: Record<QueueItem['status'], string> = {
    pending: 'Pendiente',
    processing: 'Procesando',
    completed: 'Completado',
    failed: 'Error',
    cancelled: 'Cancelado'
  };

  const statusClass: Record<QueueItem['status'], string> = {
    pending: 'status-pending',
    processing: 'status-processing',
    completed: 'status-completed',
    failed: 'status-failed',
    cancelled: 'status-cancelled'
  };
</script>

<section class="queue">
  <header>
    <h2>Cola de lectura</h2>
    <slot name="actions"></slot>
  </header>
  {#if items.length === 0}
    <p class="empty">No hay elementos en la cola.</p>
  {:else}
    <ul>
      {#each items as item}
        <li class={statusClass[item.status]}>
          <div class="content">
            <strong>{item.title}</strong>
            <p>{item.text.slice(0, 120)}{item.text.length > 120 ? '…' : ''}</p>
          </div>
          <div class="meta">
            <span>{formatLabel[item.status]}</span>
            <small>{item.voice} · {item.format.toUpperCase()}</small>
            {#if item.output}
              <a class="output" href={`file://${item.output}`}>Abrir archivo</a>
            {/if}
          </div>
          <div class="actions">
            {#if item.status === 'completed'}
              <button class="export" on:click={() => dispatch('export', item.id)}>Exportar MP3</button>
            {/if}
            <button class="remove" on:click={() => dispatch('remove', item.id)}>Eliminar</button>
          </div>
        </li>
      {/each}
    </ul>
  {/if}
</section>

<style>
  .queue {
    background: rgba(15, 23, 42, 0.75);
    border-radius: 16px;
    padding: 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  ul {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  li {
    padding: 1rem;
    border-radius: 12px;
    background: rgba(30, 41, 59, 0.8);
    display: grid;
    grid-template-columns: 1fr auto;
    gap: 1rem;
  }

  .content p {
    margin: 0.25rem 0 0;
    color: #cbd5f5;
    font-size: 0.9rem;
  }

  .meta {
    display: flex;
    flex-direction: column;
    gap: 0.35rem;
    align-items: flex-end;
  }

  .meta span {
    font-weight: 600;
  }

  .actions {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  button {
    border-radius: 8px;
    cursor: pointer;
    padding: 0.3rem 0.6rem;
    border: 1px solid transparent;
  }

  button.export {
    background: rgba(34, 197, 94, 0.15);
    border-color: rgba(34, 197, 94, 0.4);
    color: #86efac;
  }

  button.remove {
    background: transparent;
    border: 1px solid rgba(239, 68, 68, 0.5);
    color: #fca5a5;
  }

  .empty {
    margin: 0;
    color: #94a3b8;
  }

  .status-completed {
    border: 1px solid rgba(34, 197, 94, 0.3);
  }

  .status-processing {
    border: 1px solid rgba(59, 130, 246, 0.3);
  }

  .status-failed {
    border: 1px solid rgba(239, 68, 68, 0.3);
  }

  .output {
    color: #38bdf8;
    text-decoration: none;
    font-size: 0.85rem;
  }
</style>
