export type Locale = "en" | "es";

export type TranslationKeys = {
  // Nav
  navSearch: string;
  navDownloads: string;
  navSettings: string;

  // Search
  searchPlaceholder: string;
  searchButton: string;
  searching: string;
  categoryAll: string;
  categoryAnime: string;
  categoryAudio: string;
  categoryLiterature: string;
  categoryLiveAction: string;
  categoryPictures: string;
  categorySoftware: string;
  categoryGames: string;
  filterAll: string;
  filterTrusted: string;
  filterNoRemakes: string;
  sortDate: string;
  sortSeeders: string;
  sortLeechers: string;
  sortDownloads: string;
  sortSize: string;
  orderDesc: string;
  orderAsc: string;
  btnFilters: string;

  // Torrent list
  colName: string;
  colSize: string;
  colSeeders: string;
  colLeechers: string;
  colDownloads: string;
  colDate: string;
  btnView: string;
  paginationPrev: string;
  paginationNext: string;
  paginationPage: string;
  paginationOf: string;
  emptySearch: string;

  // Torrent detail
  detailCategory: string;
  detailSize: string;
  detailDate: string;
  detailSeeders: string;
  detailLeechers: string;
  detailDownloads: string;
  detailSubmitter: string;
  detailAnonymous: string;
  detailFiles: string;
  detailDescription: string;
  btnStartDownload: string;
  btnAdding: string;
  btnCopyMagnet: string;
  loadingDetails: string;
  selectTorrent: string;

  // Downloads
  downloadActive: string;
  downloadDown: string;
  downloadUp: string;
  downloadPeers: string;
  btnPause: string;
  btnResume: string;
  btnRemove: string;
  btnPlay: string;
  btnPlayFile: string;
  btnOpenFolder: string;
  downloadComplete: string;
  downloadEmpty: string;
  noMediaFiles: string;

  // Settings
  settingsTitle: string;
  settingsDownloadPath: string;
  settingsNyaaUrl: string;
  settingsMaxDown: string;
  settingsMaxUp: string;
  settingsMaxConn: string;
  settingsMaxActive: string;
  settingsStartOnLaunch: string;
  btnSave: string;
  settingsSaved: string;
  settingsLanguage: string;
  settingsTheme: string;
  themeDark: string;
  themeLight: string;
};

const en: TranslationKeys = {
  navSearch: "Search",
  navDownloads: "Downloads",
  navSettings: "Settings",

  searchPlaceholder: "Search torrents...",
  searchButton: "Search",
  searching: "Searching...",
  categoryAll: "All",
  categoryAnime: "Anime",
  categoryAudio: "Audio",
  categoryLiterature: "Literature",
  categoryLiveAction: "Live Action",
  categoryPictures: "Pictures",
  categorySoftware: "Software",
  categoryGames: "Games",
  filterAll: "All",
  filterTrusted: "Trusted",
  filterNoRemakes: "No Remakes",
  sortDate: "Date",
  sortSeeders: "Seeders",
  sortLeechers: "Leechers",
  sortDownloads: "Downloads",
  sortSize: "Size",
  orderDesc: "Descending",
  orderAsc: "Ascending",
  btnFilters: "Filters",

  colName: "Name",
  colSize: "Size",
  colSeeders: "Seeders",
  colLeechers: "Leechers",
  colDownloads: "Downloads",
  colDate: "Date",
  btnView: "View",
  paginationPrev: "Prev",
  paginationNext: "Next",
  paginationPage: "Page",
  paginationOf: "of",
  emptySearch: "Search for torrents to get started",

  detailCategory: "Category:",
  detailSize: "Size:",
  detailDate: "Date:",
  detailSeeders: "Seeders:",
  detailLeechers: "Leechers:",
  detailDownloads: "Downloads:",
  detailSubmitter: "Submitter:",
  detailAnonymous: "Anonymous",
  detailFiles: "Files",
  detailDescription: "Description",
  btnStartDownload: "Start Download",
  btnAdding: "Adding...",
  btnCopyMagnet: "Copy Magnet Link",
  loadingDetails: "Loading details...",
  selectTorrent: "Select a torrent to view details",

  downloadActive: "Active:",
  downloadDown: "Down:",
  downloadUp: "Up:",
  downloadPeers: "peers",
  btnPause: "Pause",
  btnResume: "Resume",
  btnRemove: "Remove",
  btnPlay: "Play",
  btnPlayFile: "Play File",
  btnOpenFolder: "Open Folder",
  downloadComplete: "Complete",
  downloadEmpty: "No active downloads",
  noMediaFiles: "No media files found",

  settingsTitle: "Settings",
  settingsDownloadPath: "Download Path",
  settingsNyaaUrl: "Nyaa Instance URL",
  settingsMaxDown: "Max Download Speed (0 = unlimited)",
  settingsMaxUp: "Max Upload Speed (0 = unlimited)",
  settingsMaxConn: "Max Connections",
  settingsMaxActive: "Max Active Downloads",
  settingsStartOnLaunch: "Start downloads on app launch",
  btnSave: "Save Settings",
  settingsSaved: "Saved!",
  settingsLanguage: "Language",
  settingsTheme: "Theme",
  themeDark: "Dark theme",
  themeLight: "Light theme",
};

const es: TranslationKeys = {
  navSearch: "Buscar",
  navDownloads: "Descargas",
  navSettings: "Ajustes",

  searchPlaceholder: "Buscar torrents...",
  searchButton: "Buscar",
  searching: "Buscando...",
  categoryAll: "Todos",
  categoryAnime: "Anime",
  categoryAudio: "Audio",
  categoryLiterature: "Literatura",
  categoryLiveAction: "Accion en Vivo",
  categoryPictures: "Imagenes",
  categorySoftware: "Software",
  categoryGames: "Juegos",
  filterAll: "Todos",
  filterTrusted: "Confiables",
  filterNoRemakes: "Sin Remakes",
  sortDate: "Fecha",
  sortSeeders: "Seeders",
  sortLeechers: "Leechers",
  sortDownloads: "Descargas",
  sortSize: "Tamano",
  orderDesc: "Descendente",
  orderAsc: "Ascendente",
  btnFilters: "Filtros",

  colName: "Nombre",
  colSize: "Tamano",
  colSeeders: "Seeders",
  colLeechers: "Leechers",
  colDownloads: "Descargas",
  colDate: "Fecha",
  btnView: "Ver",
  paginationPrev: "Anterior",
  paginationNext: "Siguiente",
  paginationPage: "Pagina",
  paginationOf: "de",
  emptySearch: "Busca torrents para comenzar",

  detailCategory: "Categoria:",
  detailSize: "Tamano:",
  detailDate: "Fecha:",
  detailSeeders: "Seeders:",
  detailLeechers: "Leechers:",
  detailDownloads: "Descargas:",
  detailSubmitter: "Autor:",
  detailAnonymous: "Anonimo",
  detailFiles: "Archivos",
  detailDescription: "Descripcion",
  btnStartDownload: "Iniciar Descarga",
  btnAdding: "Agregando...",
  btnCopyMagnet: "Copiar Enlace Magnet",
  loadingDetails: "Cargando detalles...",
  selectTorrent: "Selecciona un torrent para ver detalles",

  downloadActive: "Activas:",
  downloadDown: "Bajada:",
  downloadUp: "Subida:",
  downloadPeers: "peers",
  btnPause: "Pausar",
  btnResume: "Reanudar",
  btnRemove: "Eliminar",
  btnPlay: "Reproducir",
  btnPlayFile: "Reproducir Archivo",
  btnOpenFolder: "Abrir Carpeta",
  downloadComplete: "Completado",
  downloadEmpty: "No hay descargas activas",
  noMediaFiles: "No se encontraron archivos multimedia",

  settingsTitle: "Ajustes",
  settingsDownloadPath: "Ruta de Descarga",
  settingsNyaaUrl: "URL de Instancia Nyaa",
  settingsMaxDown: "Velocidad Max de Bajada (0 = sin limite)",
  settingsMaxUp: "Velocidad Max de Subida (0 = sin limite)",
  settingsMaxConn: "Conexiones Maximas",
  settingsMaxActive: "Descargas Activas Maximas",
  settingsStartOnLaunch: "Iniciar descargas al abrir la app",
  btnSave: "Guardar Ajustes",
  settingsSaved: "Guardado!",
  settingsLanguage: "Idioma",
  settingsTheme: "Tema",
  themeDark: "Tema oscuro",
  themeLight: "Tema claro",
};

export const translations: Record<Locale, TranslationKeys> = { en, es };
