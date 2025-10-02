import { beforeEach, describe, expect, it, vi } from 'vitest';
import type { Mock } from 'vitest';
import { queue, loadQueue, enqueue } from '../src/lib/stores/queue';

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn()
}));

const { invoke } = (await import('@tauri-apps/api/core')) as { invoke: Mock };

describe('queue store', () => {
  beforeEach(() => {
    (invoke as Mock).mockReset();
    queue.set([]);
  });

  it('loads queue data from backend', async () => {
    (invoke as Mock).mockResolvedValueOnce([
      {
        id: '1',
        title: 'Sample',
        text: 'Hello',
        voice: 'voice',
        rate: 1,
        format: 'wav',
        status: 'pending',
        output: null
      }
    ]);
    await loadQueue();
    queue.subscribe((value) => {
      expect(value).toHaveLength(1);
    })();
  });

  it('enqueues items', async () => {
    (invoke as Mock).mockResolvedValue([]);
    await enqueue('Title', 'Content');
    expect(invoke).toHaveBeenCalledWith('enqueue_text', {
      items: [
        {
          title: 'Title',
          text: 'Content',
          voice: undefined,
          rate: undefined,
          format: 'wav'
        }
      ]
    });
  });
});
