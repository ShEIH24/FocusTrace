import { useEffect, useMemo, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { check as checkUpdate } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";
import { useSettingsStore } from "@/store/settingsStore";
import { useUiStore } from "@/store/uiStore";
import { useT } from "@/hooks/useT";
import { api } from "@/ipc/commands";
import { Card, CardHeader } from "@/components/ui/Card";
import { TbCheck, TbSun, TbMoon, TbDeviceDesktop, TbFolder, TbRefresh, TbDownload, TbRotate, TbCopy, TbWorld } from "react-icons/tb";
import type { AppConfig } from "@/types/config";
import type { UserCategoryRule } from "@/types/browser";
import type { Language } from "@/i18n/translations";

type UpdateStatus = "idle" | "checking" | "up-to-date" | "available" | "downloading" | "ready" | "error";

const POPULAR_PRODUCTIVE_SITES: { domain: string; label: string }[] = [
  { domain: "github.com",          label: "GitHub" },
  { domain: "gitlab.com",          label: "GitLab" },
  { domain: "stackoverflow.com",   label: "Stack Overflow" },
  { domain: "notion.so",           label: "Notion" },
  { domain: "figma.com",           label: "Figma" },
  { domain: "linear.app",          label: "Linear" },
  { domain: "trello.com",          label: "Trello" },
  { domain: "asana.com",           label: "Asana" },
  { domain: "slack.com",           label: "Slack" },
  { domain: "zoom.us",             label: "Zoom" },
  { domain: "coursera.org",        label: "Coursera" },
  { domain: "udemy.com",           label: "Udemy" },
  { domain: "leetcode.com",        label: "LeetCode" },
  { domain: "freecodecamp.org",    label: "freeCodeCamp" },
  { domain: "docs.google.com",     label: "Google Docs" },
  { domain: "overleaf.com",        label: "Overleaf" },
  { domain: "obsidian.md",         label: "Obsidian" },
  { domain: "vercel.com",          label: "Vercel" },
];

const POPULAR_DISTRACTION_SITES: { domain: string; label: string }[] = [
  { domain: "youtube.com",    label: "YouTube" },
  { domain: "reddit.com",     label: "Reddit" },
  { domain: "x.com",          label: "X / Twitter" },
  { domain: "instagram.com",  label: "Instagram" },
  { domain: "tiktok.com",     label: "TikTok" },
  { domain: "facebook.com",   label: "Facebook" },
  { domain: "netflix.com",    label: "Netflix" },
  { domain: "twitch.tv",      label: "Twitch" },
  { domain: "spotify.com",    label: "Spotify" },
  { domain: "discord.com",    label: "Discord" },
  { domain: "9gag.com",       label: "9GAG" },
  { domain: "pinterest.com",  label: "Pinterest" },
  { domain: "vk.com",         label: "ВКонтакте" },
  { domain: "amazon.com",     label: "Amazon" },
  { domain: "aliexpress.com", label: "AliExpress" },
];

function toggleSiteInText(text: string, domain: string): string {
  const lines = text.split("\n").map((s) => s.trim()).filter(Boolean);
  if (lines.includes(domain)) {
    return lines.filter((l) => l !== domain).join("\n");
  }
  return [...lines, domain].join("\n");
}

function appsToText(apps: string[]): string { return apps.join("\n"); }
function textToApps(text: string): string[] {
  return text.split("\n").map((s) => s.trim()).filter(Boolean);
}

type Theme = "light" | "dark" | "system";

interface FieldProps {
  label: string;
  description?: string;
  children: React.ReactNode;
}

function Field({ label, description, children }: FieldProps) {
  return (
    <div className="flex items-start justify-between gap-6 py-4">
      <div className="flex-1 min-w-0">
        <p className="text-sm font-medium text-zinc-800 dark:text-zinc-200">{label}</p>
        {description && <p className="text-xs text-muted mt-0.5">{description}</p>}
      </div>
      <div className="flex-shrink-0">{children}</div>
    </div>
  );
}

interface PopularSiteChipsProps {
  label: string;
  hint: string;
  productiveSitesText: string;
  distractionSitesText: string;
  onToggleProductive: (domain: string) => void;
  onToggleDistraction: (domain: string) => void;
}

function PopularSiteChips({
  label,
  hint,
  productiveSitesText,
  distractionSitesText,
  onToggleProductive,
  onToggleDistraction,
}: PopularSiteChipsProps) {
  const productiveSet = useMemo(
    () => new Set(productiveSitesText.split("\n").map((s) => s.trim()).filter(Boolean)),
    [productiveSitesText],
  );
  const distractionSet = useMemo(
    () => new Set(distractionSitesText.split("\n").map((s) => s.trim()).filter(Boolean)),
    [distractionSitesText],
  );

  return (
    <div className="rounded-xl border border-zinc-200 dark:border-zinc-800 bg-zinc-50/50 dark:bg-zinc-800/30 p-3 space-y-3">
      <div>
        <p className="text-xs font-semibold text-zinc-700 dark:text-zinc-300">{label}</p>
        <p className="text-xs text-muted mt-0.5">{hint}</p>
      </div>
      <div>
        <p className="text-xs font-medium text-green-600 dark:text-green-400 mb-1.5">Продуктивные</p>
        <div className="flex flex-wrap gap-1.5">
          {POPULAR_PRODUCTIVE_SITES.map(({ domain, label: siteName }) => {
            const active = productiveSet.has(domain);
            return (
              <button
                key={domain}
                type="button"
                onClick={() => onToggleProductive(domain)}
                className={[
                  "px-2.5 py-1 rounded-full text-xs font-medium border transition-all select-none",
                  active
                    ? "bg-green-100 dark:bg-green-900/40 border-green-300 dark:border-green-700 text-green-700 dark:text-green-400"
                    : "bg-white dark:bg-zinc-800 border-zinc-200 dark:border-zinc-700 text-zinc-500 hover:border-green-300 dark:hover:border-green-700 hover:text-green-600 dark:hover:text-green-400",
                ].join(" ")}
              >
                {active ? "✓ " : ""}{siteName}
              </button>
            );
          })}
        </div>
      </div>
      <div>
        <p className="text-xs font-medium text-red-600 dark:text-red-400 mb-1.5">Отвлекающие</p>
        <div className="flex flex-wrap gap-1.5">
          {POPULAR_DISTRACTION_SITES.map(({ domain, label: siteName }) => {
            const active = distractionSet.has(domain);
            return (
              <button
                key={domain}
                type="button"
                onClick={() => onToggleDistraction(domain)}
                className={[
                  "px-2.5 py-1 rounded-full text-xs font-medium border transition-all select-none",
                  active
                    ? "bg-red-100 dark:bg-red-900/40 border-red-300 dark:border-red-700 text-red-700 dark:text-red-400"
                    : "bg-white dark:bg-zinc-800 border-zinc-200 dark:border-zinc-700 text-zinc-500 hover:border-red-300 dark:hover:border-red-700 hover:text-red-600 dark:hover:text-red-400",
                ].join(" ")}
              >
                {active ? "✓ " : ""}{siteName}
              </button>
            );
          })}
        </div>
      </div>
    </div>
  );
}

export default function Settings() {
  const { config, loadConfig, setConfig } = useSettingsStore();
  const { theme, setTheme, language, setLanguage, pendingUpdateVersion, setPendingUpdateVersion } = useUiStore();
  const t = useT();
  const [saving, setSaving] = useState(false);
  const [saved, setSaved] = useState(false);
  const [form, setForm] = useState<Partial<AppConfig>>({});
  const [productiveText, setProductiveText] = useState("");
  const [distractionText, setDistractionText] = useState("");
  const [excludedText, setExcludedText] = useState("");

  // браузерный трекинг
  const [browserToken, setBrowserToken] = useState("");
  const [tokenCopied, setTokenCopied] = useState(false);
  const [browserRules, setBrowserRules] = useState<UserCategoryRule[]>([]);
  const [productiveSitesText, setProductiveSitesText] = useState("");
  const [distractionSitesText, setDistractionSitesText] = useState("");

  // ── autostart ────────────────────────────────────────────────────────────
  const [autostart, setAutostart] = useState(false);
  useEffect(() => {
    invoke<boolean>("plugin:autostart|is_enabled")
      .then(setAutostart)
      .catch(() => {});
  }, []);

  async function toggleAutostart(enabled: boolean) {
    try {
      await invoke(enabled ? "plugin:autostart|enable" : "plugin:autostart|disable");
      setAutostart(enabled);
    } catch {/* autostart unavailable in dev */ }
  }

  // ── updater ──────────────────────────────────────────────────────────────
  const [updateStatus, setUpdateStatus] = useState<UpdateStatus>("idle");
  const [updateVersion, setUpdateVersion] = useState<string | null>(null);
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const [pendingUpdate, setPendingUpdate] = useState<any>(null);

  // Если фоновая проверка уже нашла обновление — сразу показываем его.
  useEffect(() => {
    if (!pendingUpdateVersion || updateStatus !== "idle") return;
    setUpdateStatus("checking");
    checkUpdate()
      .then((update) => {
        if (update?.available) {
          setUpdateVersion(update.version);
          setPendingUpdate(update);
          setUpdateStatus("available");
        } else {
          setUpdateStatus("idle");
        }
      })
      .catch(() => setUpdateStatus("idle"))
      .finally(() => setPendingUpdateVersion(null));
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [pendingUpdateVersion]);

  async function handleCheckUpdate() {
    setUpdateStatus("checking");
    try {
      const update = await checkUpdate();
      if (update?.available) {
        setUpdateVersion(update.version);
        setPendingUpdate(update);
        setUpdateStatus("available");
      } else {
        setUpdateStatus("up-to-date");
        setTimeout(() => setUpdateStatus("idle"), 4000);
      }
    } catch {
      setUpdateStatus("error");
      setTimeout(() => setUpdateStatus("idle"), 4000);
    }
  }

  async function handleInstallUpdate() {
    if (!pendingUpdate) return;
    setUpdateStatus("downloading");
    try {
      await pendingUpdate.downloadAndInstall();
      setUpdateStatus("ready");
    } catch {
      setUpdateStatus("error");
      setTimeout(() => setUpdateStatus("idle"), 4000);
    }
  }

  useEffect(() => { loadConfig(); }, [loadConfig]);

  useEffect(() => {
    if (config) {
      setForm(config);
      setProductiveText(appsToText(config.productive_apps));
      setDistractionText(appsToText(config.distraction_apps));
      setExcludedText(appsToText(config.excluded_apps ?? []));
    }
  }, [config]);

  useEffect(() => {
    api.getBrowserWsToken().then(setBrowserToken).catch(() => {});
    api.getBrowserCategoryRules().then((rules) => {
      setBrowserRules(rules);
      setProductiveSitesText(
        rules.filter((r) => r.category === "productive").map((r) => r.domainPattern).join("\n"),
      );
      setDistractionSitesText(
        rules.filter((r) => r.category === "distraction").map((r) => r.domainPattern).join("\n"),
      );
    }).catch(() => {});
  }, []);

  function handleCopyToken() {
    if (!browserToken) return;
    navigator.clipboard.writeText(browserToken).then(() => {
      setTokenCopied(true);
      setTimeout(() => setTokenCopied(false), 2000);
    });
  }

  async function handleSave() {
    const patch: Partial<AppConfig> = {
      ...form,
      productive_apps: textToApps(productiveText),
      distraction_apps: textToApps(distractionText),
      excluded_apps: textToApps(excludedText),
    };
    setSaving(true);
    try {
      await api.updateConfig(patch);
      const updated = await api.getConfig();
      setConfig(updated);

      // Сохранение правил классификации сайтов
      const newProductive = textToApps(productiveSitesText);
      const newDistraction = textToApps(distractionSitesText);
      const newAll = new Set([...newProductive, ...newDistraction]);
      await Promise.all([
        ...newProductive.map((d) => api.addBrowserCategoryRule(d, "productive", "")),
        ...newDistraction.map((d) => api.addBrowserCategoryRule(d, "distraction", "")),
        ...browserRules
          .filter((r) => !newAll.has(r.domainPattern))
          .map((r) => api.removeBrowserCategoryRule(r.domainPattern)),
      ]);
      setBrowserRules([
        ...newProductive.map((d, i) => ({ id: i, domainPattern: d, category: "productive" as const, subcategory: "" })),
        ...newDistraction.map((d, i) => ({ id: newProductive.length + i, domainPattern: d, category: "distraction" as const, subcategory: "" })),
      ]);

      setSaved(true);
      setTimeout(() => setSaved(false), 2000);
    } finally {
      setSaving(false);
    }
  }

  // варианты темы собираем внутри, чтобы метки были переведены
  const themeOptions: { value: Theme; icon: React.ReactNode; label: string }[] = [
    { value: "light",  icon: <TbSun size={16} />,           label: t.themeLight },
    { value: "dark",   icon: <TbMoon size={16} />,          label: t.themeDark },
    { value: "system", icon: <TbDeviceDesktop size={16} />, label: t.themeSystem },
  ];

  if (!config) {
    return (
      <div className="p-6 flex items-center justify-center h-64">
        <p className="text-sm text-muted">{t.loadingSettings}</p>
      </div>
    );
  }

  return (
    <div className="p-6 space-y-5 max-w-2xl mx-auto animate-fade-in">
      <div>
        <h1 className="text-xl font-bold text-zinc-900 dark:text-zinc-100">{t.settingsTitle}</h1>
        <p className="text-sm text-muted mt-0.5">{t.settingsSubtitle}</p>
      </div>

      {/* внешний вид: тема и язык */}
      <Card>
        <CardHeader title={t.appearance} />
        <div className="divide-y divide-zinc-100 dark:divide-zinc-800/60">
          <Field label={t.theme} description={t.chooseColorScheme}>
            <div className="flex gap-1 bg-zinc-100 dark:bg-zinc-800 p-1 rounded-lg">
              {themeOptions.map((opt) => (
                <button
                  key={opt.value}
                  onClick={() => setTheme(opt.value)}
                  className={[
                    "flex items-center gap-1.5 px-3 py-1.5 rounded-md text-xs font-medium transition-all",
                    theme === opt.value
                      ? "bg-white dark:bg-zinc-700 text-zinc-900 dark:text-zinc-100 shadow-sm"
                      : "text-zinc-500 hover:text-zinc-700 dark:hover:text-zinc-300",
                  ].join(" ")}
                >
                  {opt.icon}
                  {opt.label}
                </button>
              ))}
            </div>
          </Field>

          <Field label={t.languageLabel} description={t.interfaceLanguage}>
            <div className="flex gap-1 bg-zinc-100 dark:bg-zinc-800 p-1 rounded-lg">
              {([
                { value: "en" as Language, label: "English" },
                { value: "ru" as Language, label: "Русский" },
              ]).map((opt) => (
                <button
                  key={opt.value}
                  onClick={() => setLanguage(opt.value)}
                  className={[
                    "px-3 py-1.5 rounded-md text-xs font-medium transition-all",
                    language === opt.value
                      ? "bg-white dark:bg-zinc-700 text-zinc-900 dark:text-zinc-100 shadow-sm"
                      : "text-zinc-500 hover:text-zinc-700 dark:hover:text-zinc-300",
                  ].join(" ")}
                >
                  {opt.label}
                </button>
              ))}
            </div>
          </Field>
        </div>
      </Card>

      {/* параметры трекинга */}
      <Card>
        <CardHeader title={t.tracking} />
        <div className="divide-y divide-zinc-100 dark:divide-zinc-800/60">
          <Field label={t.idleThreshold} description={t.idleThresholdDesc}>
            <div className="flex items-center gap-2">
              <input
                type="number"
                min={10}
                value={form.idle_threshold_secs ?? config.idle_threshold_secs}
                onChange={(e) => setForm((f) => ({ ...f, idle_threshold_secs: Number(e.target.value) }))}
                className="input-base w-24 text-right"
              />
              <span className="text-xs text-muted">{t.sec}</span>
            </div>
          </Field>
          <Field label={t.pollInterval} description={t.pollIntervalDesc}>
            <div className="flex items-center gap-2">
              <input
                type="number"
                min={100}
                value={form.poll_interval_ms ?? config.poll_interval_ms}
                onChange={(e) => setForm((f) => ({ ...f, poll_interval_ms: Number(e.target.value) }))}
                className="input-base w-24 text-right"
              />
              <span className="text-xs text-muted">{t.ms}</span>
            </div>
          </Field>
        </div>
      </Card>

      {/* классификация приложений */}
      <Card>
        <CardHeader
          title={t.appClassification}
          subtitle={t.appClassificationDesc}
        />
        <div className="space-y-4">
          <div>
            <label className="flex items-center gap-1.5 text-xs font-semibold text-green-600 dark:text-green-400 mb-1.5 uppercase tracking-wide">
              <span className="w-2 h-2 rounded-full bg-green-500 inline-block" />
              {t.productiveApps}
            </label>
            <textarea
              rows={5}
              value={productiveText}
              onChange={(e) => setProductiveText(e.target.value)}
              placeholder="Code.exe&#10;idea64.exe&#10;..."
              className="input-base font-mono resize-y"
            />
          </div>
          <div>
            <label className="flex items-center gap-1.5 text-xs font-semibold text-red-600 dark:text-red-400 mb-1.5 uppercase tracking-wide">
              <span className="w-2 h-2 rounded-full bg-red-500 inline-block" />
              {t.distractionApps}
            </label>
            <textarea
              rows={5}
              value={distractionText}
              onChange={(e) => setDistractionText(e.target.value)}
              placeholder="chrome.exe&#10;Discord.exe&#10;..."
              className="input-base font-mono resize-y"
            />
          </div>
          <div>
            <label className="flex items-center gap-1.5 text-xs font-semibold text-zinc-500 dark:text-zinc-400 mb-1.5 uppercase tracking-wide">
              <span className="w-2 h-2 rounded-full bg-zinc-400 inline-block" />
              {t.excludedApps}
            </label>
            <p className="text-xs text-muted mb-2">{t.excludedAppsDesc}</p>
            <textarea
              rows={3}
              value={excludedText}
              onChange={(e) => setExcludedText(e.target.value)}
              placeholder="msedge.exe&#10;chrome.exe&#10;..."
              className="input-base font-mono resize-y"
            />
          </div>
        </div>
      </Card>

      {/* браузерный трекинг */}
      <Card>
        <CardHeader
          title={t.browserTracking}
          subtitle={t.browserTrackingDesc}
        />
        <div className="divide-y divide-zinc-100 dark:divide-zinc-800/60">
          <Field label={t.wsToken} description={t.wsTokenDesc}>
            <div className="flex items-center gap-2">
              <code className="text-xs font-mono bg-zinc-100 dark:bg-zinc-800 px-2 py-1.5 rounded-lg truncate max-w-[160px] select-all">
                {browserToken || "…"}
              </code>
              <button
                onClick={handleCopyToken}
                className="flex items-center gap-1 px-2.5 py-1.5 rounded-lg text-xs font-medium bg-zinc-100 dark:bg-zinc-800 hover:bg-zinc-200 dark:hover:bg-zinc-700 transition-colors"
              >
                {tokenCopied ? <TbCheck size={13} /> : <TbCopy size={13} />}
                {tokenCopied ? t.tokenCopied : t.copyToken}
              </button>
            </div>
          </Field>
        </div>

        <p className="text-xs text-muted py-3 flex items-start gap-1.5">
          <TbWorld size={13} className="mt-0.5 shrink-0" />
          {t.installExtensionHint}
        </p>

        <div className="space-y-4 pt-1">
          {/* популярные сайты */}
          <PopularSiteChips
            label={t.popularSites}
            hint={t.popularSitesHint}
            productiveSitesText={productiveSitesText}
            distractionSitesText={distractionSitesText}
            onToggleProductive={(domain) => setProductiveSitesText((prev) => toggleSiteInText(prev, domain))}
            onToggleDistraction={(domain) => setDistractionSitesText((prev) => toggleSiteInText(prev, domain))}
          />

          <div>
            <label className="flex items-center gap-1.5 text-xs font-semibold text-green-600 dark:text-green-400 mb-1.5 uppercase tracking-wide">
              <span className="w-2 h-2 rounded-full bg-green-500 inline-block" />
              {t.productiveSites}
            </label>
            <textarea
              rows={4}
              value={productiveSitesText}
              onChange={(e) => setProductiveSitesText(e.target.value)}
              placeholder={"github.com\nnotion.so\n..."}
              className="input-base font-mono resize-y"
            />
          </div>
          <div>
            <label className="flex items-center gap-1.5 text-xs font-semibold text-red-600 dark:text-red-400 mb-1.5 uppercase tracking-wide">
              <span className="w-2 h-2 rounded-full bg-red-500 inline-block" />
              {t.distractionSites}
            </label>
            <textarea
              rows={4}
              value={distractionSitesText}
              onChange={(e) => setDistractionSitesText(e.target.value)}
              placeholder={"youtube.com\nreddit.com\n..."}
              className="input-base font-mono resize-y"
            />
          </div>
          <p className="text-xs text-muted pb-1">{t.siteClassificationDesc}</p>
        </div>
      </Card>

      {/* папка с данными */}
      {config.data_dir && (
        <Card>
          <CardHeader title={t.data} />
          <Field label={t.dataDirectory} description={t.dataDirectoryDesc}>
            <div className="flex items-center gap-2 text-xs text-muted font-mono bg-zinc-100 dark:bg-zinc-800 px-3 py-1.5 rounded-lg">
              <TbFolder size={13} />
              <span className="truncate max-w-[200px]">{config.data_dir}</span>
            </div>
          </Field>
        </Card>
      )}

      {/* система: автозапуск */}
      <Card>
        <CardHeader title={t.systemSection} />
        <div className="divide-y divide-zinc-100 dark:divide-zinc-800/60">
          <Field label={t.launchAtStartup} description={t.launchAtStartupDesc}>
            <button
              role="switch"
              aria-checked={autostart}
              onClick={() => toggleAutostart(!autostart)}
              className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors duration-200 ${
                autostart ? "bg-indigo-600" : "bg-zinc-300 dark:bg-zinc-700"
              }`}
            >
              <span
                className={`inline-block h-4 w-4 transform rounded-full bg-white shadow transition-transform duration-200 ${
                  autostart ? "translate-x-6" : "translate-x-1"
                }`}
              />
            </button>
          </Field>
        </div>
      </Card>

      {/* обновления */}
      <Card>
        <CardHeader title={t.updatesSection} />
        <div className="py-4 flex items-center justify-between gap-4">
          <div className="text-sm text-zinc-500 dark:text-zinc-400">
            {updateStatus === "idle" && null}
            {updateStatus === "up-to-date" && (
              <span className="flex items-center gap-1.5 text-green-600 dark:text-green-400">
                <TbCheck size={15} /> {t.upToDate}
              </span>
            )}
            {updateStatus === "available" && updateVersion && (
              <span className="text-indigo-500 font-medium">
                {t.updateVersion} {updateVersion}
              </span>
            )}
            {updateStatus === "ready" && (
              <span className="flex items-center gap-1.5 text-amber-500">
                <TbRotate size={15} /> {t.restartToUpdate}
              </span>
            )}
            {updateStatus === "error" && (
              <span className="text-red-500 text-xs">{t.updateCheckFailed}</span>
            )}
          </div>

          <div className="flex items-center gap-2">
            {updateStatus === "available" && (
              <button
                onClick={handleInstallUpdate}
                className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium bg-indigo-600 hover:bg-indigo-500 text-white transition-colors"
              >
                <TbDownload size={14} /> {t.installUpdate}
              </button>
            )}
            {updateStatus === "ready" && (
              <button
                onClick={() => relaunch()}
                className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium bg-amber-500 hover:bg-amber-400 text-white transition-colors"
              >
                <TbRotate size={14} /> {t.restartToUpdate}
              </button>
            )}
            <button
              onClick={handleCheckUpdate}
              disabled={updateStatus === "checking" || updateStatus === "downloading"}
              className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium bg-zinc-100 dark:bg-zinc-800 hover:bg-zinc-200 dark:hover:bg-zinc-700 text-zinc-700 dark:text-zinc-300 disabled:opacity-50 transition-colors"
            >
              <TbRefresh
                size={14}
                className={updateStatus === "checking" || updateStatus === "downloading" ? "animate-spin" : ""}
              />
              {updateStatus === "checking"
                ? t.checkingForUpdates
                : updateStatus === "downloading"
                ? t.downloadingUpdate
                : t.checkForUpdates}
            </button>
          </div>
        </div>
      </Card>

      {/* кнопка сохранения */}
      <div className="flex items-center justify-end gap-3">
        {saved && (
          <div className="flex items-center gap-1.5 text-sm text-green-600 dark:text-green-400 animate-fade-in">
            <TbCheck size={16} />
            {t.saved}
          </div>
        )}
        <button onClick={handleSave} disabled={saving} className="btn-primary">
          {saving ? t.saving : t.saveSettings}
        </button>
      </div>
    </div>
  );
}
