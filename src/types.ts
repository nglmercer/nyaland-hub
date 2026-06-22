export interface Torrent {
  id: number;
  name: string;
  magnet: string;
  size: string;
  category: string;
  sub_category: string | null;
  date: string;
  seeders: number;
  leechers: number;
  downloads: number;
  hash: string | null;
  submitter: string | null;
  submitter_id: string | null;
  information: string | null;
  completed: number | null;
  description: string | null;
  torrent_url: string | null;
  view_url: string | null;
  comments: number | null;
}

export interface TorrentDetail {
  id: number;
  title: string;
  name: string;
  category: string;
  sub_category: string;
  date: string;
  seeders: number;
  leechers: number;
  downloads: number;
  completed: number | null;
  magnet: string;
  size: string;
  hash: string | null;
  submitter: string | null;
  submitter_id: string | null;
  information: string | null;
  description: string;
  files: TorrentFile[];
  comments: number;
}

export interface TorrentFile {
  name: string;
  size: string;
}

export interface SearchResult {
  data: Torrent[];
  total: number | null;
  page: number;
  total_page: number | null;
  per_page: number;
  range: string | null;
  next_page: boolean;
  time_taken: number;
}

export interface DownloadStatus {
  hash: string;
  name: string;
  progress: number;
  download_rate: number;
  upload_rate: number;
  total_size: number;
  downloaded: number;
  num_peers: number;
  state: DownloadState;
  save_path: string;
  added_date: string | null;
}

export type DownloadState =
  | "Queued"
  | "Downloading"
  | "Paused"
  | "Finished"
  | "Error"
  | "Moving";

export interface SearchParams {
  query: string;
  page?: number;
  category?: string;
  filter?: string;
  sort?: string;
  order?: string;
}

export interface AddDownloadParams {
  magnet: string;
  save_path: string;
}

export interface AppSettings {
  save_path: string;
  nyaa_base_url: string;
  max_download_speed: number;
  max_upload_speed: number;
  max_connections: number;
  max_active_downloads: number;
  start_on_launch: boolean;
}
