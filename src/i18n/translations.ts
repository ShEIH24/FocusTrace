// все строки интерфейса на двух языках

export type Language = "en" | "ru";

export interface Translations {
  locale: string;

  // шапка приложения
  appName: string;
  appSubtitle: string;

  // боковое меню
  views: string;
  system: string;
  lightMode: string;
  darkMode: string;

  // навигация
  navDashboard: string;
  navTimeline: string;
  navAnalytics: string;
  navDailyReport: string;
  navFocus: string;
  navSettings: string;

  // дашборд
  dashboardTitle: string;
  totalActive: string;
  focusScore: string;
  outOf100: string;
  productive: string;
  sessions: string;
  calculating: string;
  timeBreakdown: string;
  loading: string;
  noDataYet: string;
  topAppsToday: string;
  appsTracked: string;
  topActivityToday: string;
  topActivity: string;

  // категории приложений
  categoryProductive: string;
  categoryNeutral: string;
  categoryDistraction: string;

  // аналитика
  analyticsTitle: string;
  sevenDayOverview: string;
  loadingAnalytics: string;
  failedToLoadAnalytics: string;
  avgFocusScore: string;
  last7Days: string;
  bestDay: string;
  daysTracked: string;
  of: string;
  focusTrend: string;
  scoreOverTime: string;
  dailyScores: string;
  barChartView: string;
  activityHeatmap: string;
  focusScoreByDay: string;
  heatmapLow: string;
  heatmapMid: string;
  heatmapHigh: string;
  scoreDistribution: string;

  // страница фокуса
  focusTitle: string;
  pomodoroAndGoals: string;
  todaysGoals: string;
  streak: string;
  focusMode: string;
  focusModeDesc: string;
  timerSettings: string;
  work: string;
  shortBreak: string;
  longBreak: string;
  sessionsUntilLong: string;
  dailyPomodoroTarget: string;
  autoStartBreaks: string;
  autoStartWork: string;
  save: string;

  // хронология
  timelineTitle: string;
  loadingSessions: string;
  failedToLoadSessions: string;
  noSessionsForDay: string;
  tracked: string;
  sessionsWord: string;

  // ежедневный отчёт
  dailyReportTitle: string;
  totalTime: string;
  peakHour: string;
  mostActive: string;
  categorySplit: string;
  noData: string;
  topApplications: string;
  apps: string;
  noAppData: string;
  sessionLog: string;
  noSessionsRecorded: string;
  moreSessions: string;

  // настройки
  settingsTitle: string;
  settingsSubtitle: string;
  appearance: string;
  theme: string;
  chooseColorScheme: string;
  themeLight: string;
  themeDark: string;
  themeSystem: string;
  languageLabel: string;
  interfaceLanguage: string;
  tracking: string;
  idleThreshold: string;
  idleThresholdDesc: string;
  sec: string;
  pollInterval: string;
  pollIntervalDesc: string;
  ms: string;
  appClassification: string;
  appClassificationDesc: string;
  productiveApps: string;
  distractionApps: string;
  data: string;
  dataDirectory: string;
  dataDirectoryDesc: string;
  saved: string;
  saving: string;
  saveSettings: string;
  loadingSettings: string;

  // карточка активного окна
  noActiveWindow: string;
  activeNow: string;

  // баннер простоя
  awayFromKeyboard: string;
  idleSinceLabel: string;
  idleLabel: string;

  // таймер помодоро
  pomodoroReady: string;
  pomodoroWorking: string;
  pomodoroShortBreak: string;
  pomodoroLongBreak: string;
  pausedLabel: string;
  start: string;
  pause: string;
  resume: string;
  skip: string;
  stop: string;

  // прогресс целей
  productiveTime: string;
  pomodoros: string;
  allGoalsMet: string;

  // серия (стрик)
  currentStreak: string;
  longestStreak: string;
  lastProductive: string;
  dayLabels: string[];

  // шкала фокуса
  gaugeDistracted: string;
  gaugeModerate: string;
  gaugeFocused: string;
  gaugeDeepFocus: string;
  gaugeFocusScore: string;

  // браузерный трекинг
  browserTracking: string;
  browserTrackingDesc: string;
  wsToken: string;
  wsTokenDesc: string;
  copyToken: string;
  tokenCopied: string;
  productiveSites: string;
  distractionSites: string;
  siteClassificationDesc: string;
  installExtensionHint: string;
  popularSites: string;
  popularSitesHint: string;

  // система / автозапуск
  systemSection: string;
  launchAtStartup: string;
  launchAtStartupDesc: string;

  // обновления
  updatesSection: string;
  checkForUpdates: string;
  checkingForUpdates: string;
  upToDate: string;
  updateAvailable: string;
  updateVersion: string;
  installUpdate: string;
  downloadingUpdate: string;
  restartToUpdate: string;
  updateCheckFailed: string;
}

const en: Translations = {
  locale: "en-US",

  appName: "FocusTrace",
  appSubtitle: "Focus Tracker",

  views: "Views",
  system: "System",
  lightMode: "Light mode",
  darkMode: "Dark mode",

  navDashboard: "Dashboard",
  navTimeline: "Timeline",
  navAnalytics: "Analytics",
  navDailyReport: "Daily Report",
  navFocus: "Focus",
  navSettings: "Settings",

  dashboardTitle: "Dashboard",
  totalActive: "Total Active",
  focusScore: "Focus Score",
  outOf100: "out of 100",
  productive: "Productive",
  sessions: "Sessions",
  calculating: "Calculating…",
  timeBreakdown: "Time Breakdown",
  loading: "Loading…",
  noDataYet: "No data yet",
  topAppsToday: "Top Apps Today",
  appsTracked: "apps tracked",
  topActivityToday: "Top Activity Today",
  topActivity: "Top Activity",

  categoryProductive: "Productive",
  categoryNeutral: "Neutral",
  categoryDistraction: "Distraction",

  analyticsTitle: "Analytics",
  sevenDayOverview: "7-day performance overview",
  loadingAnalytics: "Loading analytics…",
  failedToLoadAnalytics: "Failed to load analytics",
  avgFocusScore: "Avg Focus Score",
  last7Days: "last 7 days",
  bestDay: "Best Day",
  daysTracked: "Days Tracked",
  of: "of",
  focusTrend: "Focus Trend",
  scoreOverTime: "Score over time",
  dailyScores: "Daily Scores",
  barChartView: "Bar chart view",
  activityHeatmap: "Activity Heatmap",
  focusScoreByDay: "Focus score by day",
  heatmapLow: "< 40 Distracted",
  heatmapMid: "40–70 Moderate",
  heatmapHigh: "> 70 Focused",
  scoreDistribution: "Score Distribution",

  focusTitle: "Focus",
  pomodoroAndGoals: "Pomodoro timer, goals, and streaks",
  todaysGoals: "Today's goals",
  streak: "Streak",
  focusMode: "Focus mode",
  focusModeDesc: "Alert when distraction apps are active",
  timerSettings: "Timer settings",
  work: "Work",
  shortBreak: "Short break",
  longBreak: "Long break",
  sessionsUntilLong: "Sessions until long",
  dailyPomodoroTarget: "Daily pomodoro target",
  autoStartBreaks: "Auto-start breaks",
  autoStartWork: "Auto-start work",
  save: "Save",

  timelineTitle: "Timeline",
  loadingSessions: "Loading sessions…",
  failedToLoadSessions: "Failed to load sessions",
  noSessionsForDay: "No sessions recorded for this day",
  tracked: "tracked",
  sessionsWord: "sessions",

  dailyReportTitle: "Daily Report",
  totalTime: "Total Time",
  peakHour: "Peak Hour",
  mostActive: "most active",
  categorySplit: "Category Split",
  noData: "No data",
  topApplications: "Top Applications",
  apps: "apps",
  noAppData: "No app data",
  sessionLog: "Session Log",
  noSessionsRecorded: "No sessions recorded",
  moreSessions: "more sessions",

  settingsTitle: "Settings",
  settingsSubtitle: "Configure FocusTrace to your preferences",
  appearance: "Appearance",
  theme: "Theme",
  chooseColorScheme: "Choose your preferred color scheme",
  themeLight: "Light",
  themeDark: "Dark",
  themeSystem: "System",
  languageLabel: "Language",
  interfaceLanguage: "Interface language",
  tracking: "Tracking",
  idleThreshold: "Idle threshold",
  idleThresholdDesc: "Mark user as idle after this many seconds of inactivity",
  sec: "sec",
  pollInterval: "Poll interval",
  pollIntervalDesc: "How often to check the active window",
  ms: "ms",
  appClassification: "App Classification",
  appClassificationDesc: "One executable name per line (e.g. Code.exe)",
  productiveApps: "Productive apps",
  distractionApps: "Distraction apps",
  data: "Data",
  dataDirectory: "Data directory",
  dataDirectoryDesc: "Where FocusTrace stores your data",
  saved: "Saved",
  saving: "Saving…",
  saveSettings: "Save Settings",
  loadingSettings: "Loading settings…",

  noActiveWindow: "No active window detected",
  activeNow: "Active Now",

  awayFromKeyboard: "Away from keyboard",
  idleSinceLabel: "Since",
  idleLabel: "idle",

  pomodoroReady: "Ready",
  pomodoroWorking: "Focus",
  pomodoroShortBreak: "Short break",
  pomodoroLongBreak: "Long break",
  pausedLabel: "paused",
  start: "Start",
  pause: "Pause",
  resume: "Resume",
  skip: "Skip",
  stop: "Stop",

  productiveTime: "Productive time",
  pomodoros: "Pomodoros",
  allGoalsMet: "All goals met today",

  currentStreak: "Current streak",
  longestStreak: "Longest streak",
  lastProductive: "Last productive:",
  dayLabels: ["S", "M", "T", "W", "T", "F", "S"],

  gaugeDistracted: "Distracted",
  gaugeModerate: "Moderate",
  gaugeFocused: "Focused",
  gaugeDeepFocus: "Deep Focus",
  gaugeFocusScore: "Focus Score",

  browserTracking: "Browser Tracking",
  browserTrackingDesc: "Track websites in Chrome, Edge, or Firefox",
  wsToken: "Connection Token",
  wsTokenDesc: "Paste this token into the FocusTrace browser extension popup",
  copyToken: "Copy",
  tokenCopied: "Copied!",
  productiveSites: "Productive sites",
  distractionSites: "Distraction sites",
  siteClassificationDesc: "One domain per line (e.g. github.com). User rules override built-in defaults.",
  installExtensionHint: "Load the extension from the extensions/focustrace folder in Chrome/Edge (chrome://extensions → Load unpacked), or use manifest.firefox.json in Firefox.",
  popularSites: "Popular sites",
  popularSitesHint: "Click to add or remove from the list below",

  systemSection: "System",
  launchAtStartup: "Launch at Windows startup",
  launchAtStartupDesc: "Start FocusTrace automatically when you log in",

  updatesSection: "Updates",
  checkForUpdates: "Check for updates",
  checkingForUpdates: "Checking…",
  upToDate: "You're up to date",
  updateAvailable: "Update available",
  updateVersion: "Version",
  installUpdate: "Download & install",
  downloadingUpdate: "Downloading…",
  restartToUpdate: "Restart to apply update",
  updateCheckFailed: "Could not check for updates",
};

const ru: Translations = {
  locale: "ru-RU",

  appName: "FocusTrace",
  appSubtitle: "Трекер фокуса",

  views: "Разделы",
  system: "Система",
  lightMode: "Светлая тема",
  darkMode: "Тёмная тема",

  navDashboard: "Дашборд",
  navTimeline: "Хронология",
  navAnalytics: "Аналитика",
  navDailyReport: "Отчёт за день",
  navFocus: "Фокус",
  navSettings: "Настройки",

  dashboardTitle: "Дашборд",
  totalActive: "Всего активно",
  focusScore: "Очки фокуса",
  outOf100: "из 100",
  productive: "Продуктивно",
  sessions: "Сессии",
  calculating: "Вычисляем…",
  timeBreakdown: "Разбивка времени",
  loading: "Загрузка…",
  noDataYet: "Данных нет",
  topAppsToday: "Топ приложений сегодня",
  appsTracked: "приложений отслежено",
  topActivityToday: "Топ активности сегодня",
  topActivity: "Топ активности",

  categoryProductive: "Продуктивно",
  categoryNeutral: "Нейтрально",
  categoryDistraction: "Отвлечение",

  analyticsTitle: "Аналитика",
  sevenDayOverview: "Обзор за 7 дней",
  loadingAnalytics: "Загрузка аналитики…",
  failedToLoadAnalytics: "Не удалось загрузить аналитику",
  avgFocusScore: "Средний фокус",
  last7Days: "за 7 дней",
  bestDay: "Лучший день",
  daysTracked: "Дней отслежено",
  of: "из",
  focusTrend: "Тренд фокуса",
  scoreOverTime: "Оценка по времени",
  dailyScores: "Оценки по дням",
  barChartView: "График столбцов",
  activityHeatmap: "Тепловая карта",
  focusScoreByDay: "Фокус по дням",
  heatmapLow: "< 40 Отвлечён",
  heatmapMid: "40–70 Умеренно",
  heatmapHigh: "> 70 Сосредоточен",
  scoreDistribution: "Распределение оценок",

  focusTitle: "Фокус",
  pomodoroAndGoals: "Таймер, цели и серии",
  todaysGoals: "Цели на сегодня",
  streak: "Серия",
  focusMode: "Режим фокуса",
  focusModeDesc: "Оповещать при запуске отвлекающих приложений",
  timerSettings: "Настройки таймера",
  work: "Работа",
  shortBreak: "Короткий перерыв",
  longBreak: "Долгий перерыв",
  sessionsUntilLong: "Сессий до долгого",
  dailyPomodoroTarget: "Цель по помодоро",
  autoStartBreaks: "Авто-старт перерывов",
  autoStartWork: "Авто-старт работы",
  save: "Сохранить",

  timelineTitle: "Хронология",
  loadingSessions: "Загрузка сессий…",
  failedToLoadSessions: "Не удалось загрузить сессии",
  noSessionsForDay: "За этот день сессий нет",
  tracked: "отслежено",
  sessionsWord: "сессий",

  dailyReportTitle: "Отчёт за день",
  totalTime: "Всего времени",
  peakHour: "Пиковый час",
  mostActive: "самый активный",
  categorySplit: "По категориям",
  noData: "Нет данных",
  topApplications: "Топ приложений",
  apps: "прил.",
  noAppData: "Нет данных",
  sessionLog: "Журнал сессий",
  noSessionsRecorded: "Сессий нет",
  moreSessions: "ещё сессий",

  settingsTitle: "Настройки",
  settingsSubtitle: "Настройте FocusTrace под себя",
  appearance: "Внешний вид",
  theme: "Тема",
  chooseColorScheme: "Выберите цветовую схему",
  themeLight: "Светлая",
  themeDark: "Тёмная",
  themeSystem: "Системная",
  languageLabel: "Язык",
  interfaceLanguage: "Язык интерфейса",
  tracking: "Отслеживание",
  idleThreshold: "Порог простоя",
  idleThresholdDesc: "Считать неактивным после указанного числа секунд",
  sec: "сек",
  pollInterval: "Интервал опроса",
  pollIntervalDesc: "Как часто проверять активное окно",
  ms: "мс",
  appClassification: "Классификация приложений",
  appClassificationDesc: "По одному имени exe на строку (напр. Code.exe)",
  productiveApps: "Продуктивные",
  distractionApps: "Отвлекающие",
  data: "Данные",
  dataDirectory: "Папка с данными",
  dataDirectoryDesc: "Где FocusTrace хранит данные",
  saved: "Сохранено",
  saving: "Сохраняем…",
  saveSettings: "Сохранить",
  loadingSettings: "Загрузка настроек…",

  noActiveWindow: "Активное окно не найдено",
  activeNow: "Сейчас активно",

  awayFromKeyboard: "Отошёл от компьютера",
  idleSinceLabel: "С",
  idleLabel: "простой",

  pomodoroReady: "Готов",
  pomodoroWorking: "Фокус",
  pomodoroShortBreak: "Короткий перерыв",
  pomodoroLongBreak: "Долгий перерыв",
  pausedLabel: "на паузе",
  start: "Старт",
  pause: "Пауза",
  resume: "Продолжить",
  skip: "Пропустить",
  stop: "Стоп",

  productiveTime: "Продуктивное время",
  pomodoros: "Помодоро",
  allGoalsMet: "Все цели выполнены",

  currentStreak: "Текущая серия",
  longestStreak: "Лучшая серия",
  lastProductive: "Последний продуктивный:",
  dayLabels: ["В", "П", "В", "С", "Ч", "П", "С"],

  gaugeDistracted: "Отвлечён",
  gaugeModerate: "Умеренно",
  gaugeFocused: "Сосредоточен",
  gaugeDeepFocus: "Глубокий фокус",
  gaugeFocusScore: "Очки фокуса",

  browserTracking: "Браузерный трекинг",
  browserTrackingDesc: "Отслеживание сайтов в Chrome, Edge или Firefox",
  wsToken: "Токен подключения",
  wsTokenDesc: "Вставьте этот токен в попап расширения FocusTrace",
  copyToken: "Копировать",
  tokenCopied: "Скопировано!",
  productiveSites: "Продуктивные сайты",
  distractionSites: "Отвлекающие сайты",
  siteClassificationDesc: "По одному домену на строку (напр. github.com). Пользовательские правила имеют приоритет над встроенными.",
  installExtensionHint: "Загрузите расширение из папки extensions/focustrace в Chrome/Edge (chrome://extensions → Загрузить распакованное) или через manifest.firefox.json в Firefox.",
  popularSites: "Популярные сайты",
  popularSitesHint: "Нажмите, чтобы добавить или убрать из списка ниже",

  systemSection: "Система",
  launchAtStartup: "Запускать при входе в Windows",
  launchAtStartupDesc: "Автоматически запускать FocusTrace при входе",

  updatesSection: "Обновления",
  checkForUpdates: "Проверить обновления",
  checkingForUpdates: "Проверяем…",
  upToDate: "Установлена последняя версия",
  updateAvailable: "Доступно обновление",
  updateVersion: "Версия",
  installUpdate: "Скачать и установить",
  downloadingUpdate: "Скачиваем…",
  restartToUpdate: "Перезапустить для применения",
  updateCheckFailed: "Не удалось проверить обновления",
};

export const translations: Record<Language, Translations> = { en, ru };

// вспомогательная функция для счётчика сессий помодоро
export function sessionCountLabel(count: number, lang: Language): string {
  if (lang === "ru") {
    const mod10 = count % 10;
    const mod100 = count % 100;
    if (mod10 === 1 && mod100 !== 11) return `${count} сессия завершена сегодня`;
    if (mod10 >= 2 && mod10 <= 4 && (mod100 < 10 || mod100 >= 20))
      return `${count} сессии завершено сегодня`;
    return `${count} сессий завершено сегодня`;
  }
  return `${count} ${count === 1 ? "session" : "sessions"} completed today`;
}
