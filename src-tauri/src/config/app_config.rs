use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub idle_threshold_secs: u64,
    pub poll_interval_ms: u64,
    pub data_dir: PathBuf,
    pub productive_apps: Vec<String>,
    pub distraction_apps: Vec<String>,
}

impl AppConfig {
    /// Loads config from `data_dir/config.json`, returning defaults if absent or invalid.
    pub fn load(app_data_dir: &Path) -> Self {
        let config_path = app_data_dir.join("config.json");
        if let Ok(content) = std::fs::read_to_string(&config_path) {
            if let Ok(cfg) = serde_json::from_str::<AppConfig>(&content) {
                return cfg;
            }
        }
        Self::default_for_dir(app_data_dir)
    }

    /// Persists config to `path/config.json`.
    pub fn save(&self, data_dir: &Path) -> Result<(), std::io::Error> {
        let path = data_dir.join("config.json");
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        std::fs::write(path, content)
    }

    fn default_for_dir(data_dir: &Path) -> Self {
        Self {
            idle_threshold_secs: 120,
            poll_interval_ms: 1000,
            data_dir: data_dir.to_path_buf(),
            productive_apps: vec![
                // редакторы и ide
                "code".into(),
                "idea".into(),
                "visual studio".into(),
                "rider".into(),
                "webstorm".into(),
                "pycharm".into(),
                "phpstorm".into(),
                "clion".into(),
                "goland".into(),
                "datagrip".into(),
                "rustrover".into(),
                "fleet".into(),
                "aqua".into(),
                "studio64".into(),
                "android studio".into(),
                "vim".into(),
                "nvim".into(),
                "neovim".into(),
                "sublime_text".into(),
                "sublime text".into(),
                "notepad++".into(),
                "zed".into(),
                "cursor".into(),
                "emacs".into(),
                "helix".into(),
                // терминалы
                "windowsterminal".into(),
                "alacritty".into(),
                "kitty".into(),
                "warp".into(),
                "tabby".into(),
                "hyper".into(),
                "iterm".into(),
                "cmd".into(),
                "powershell".into(),
                "bash".into(),
                "wsl".into(),
                "mintty".into(),
                // инструменты разработчика
                "postman".into(),
                "insomnia".into(),
                "dbeaver".into(),
                "tableplus".into(),
                "docker".into(),
                "git".into(),
                "gh".into(),
                "sourcetree".into(),
                "gitkraken".into(),
                "fork".into(),
                "beyond compare".into(),
                "winmerge".into(),
                "wireshark".into(),
                "burpsuite".into(),
                "charles".into(),
                "proxyman".into(),
                // дизайн
                "figma".into(),
                "sketch".into(),
                "xd".into(),
                "affinity".into(),
                "blender".into(),
                "inkscape".into(),
                "gimp".into(),
                "krita".into(),
                // заметки и документы
                "notion".into(),
                "obsidian".into(),
                "logseq".into(),
                "onenote".into(),
                "roamresearch".into(),
                "typora".into(),
                "marktext".into(),
                "joplin".into(),
                // офисные приложения
                "winword".into(),
                "excel".into(),
                "powerpnt".into(),
                "soffice".into(),
                "libreoffice".into(),
                "abiword".into(),
                // чтение и обучение
                "kindle".into(),
                "calibre".into(),
                "zotero".into(),
                "mendeley".into(),
                "anki".into(),
                "duolingo".into(),
            ],
            distraction_apps: vec![
                "youtube".into(),
                "telegram".into(),
                "discord".into(),
                "vk".into(),
                "tiktok".into(),
                "netflix".into(),
                "twitch".into(),
                "instagram".into(),
                "facebook".into(),
                "twitter".into(),
                "reddit".into(),
                "9gag".into(),
                "pinterest".into(),
                "snapchat".into(),
                "whatsapp".into(),
                "viber".into(),
                "steam".into(),
                "epicgames".into(),
                "battle.net".into(),
                "origin".into(),
                "uplay".into(),
                "spotify".into(),
                "deezer".into(),
            ],
        }
    }
}
