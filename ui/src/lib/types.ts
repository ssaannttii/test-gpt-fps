export type QueueStatus = 'pending' | 'processing' | 'completed' | 'failed' | 'cancelled';

export interface QueueItem {
  id: string;
  title: string;
  text: string;
  voice: string;
  rate: number;
  format: 'wav' | 'mp3';
  status: QueueStatus;
  output: string | null;
}

export interface VoiceInfo {
  name: string;
  description?: string | null;
  language?: string | null;
  quality?: string | null;
  model_path: string;
}

export interface AppConfig {
  export_dir: string;
}
