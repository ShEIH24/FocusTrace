use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Category {
    Productive,
    Neutral,
    Distraction,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Category::Productive => write!(f, "Productive"),
            Category::Neutral => write!(f, "Neutral"),
            Category::Distraction => write!(f, "Distraction"),
        }
    }
}

/// Classifies applications into productivity categories using keyword matching.
pub struct RulesClassifier {
    productive: Vec<String>,
    distraction: Vec<String>,
}

impl RulesClassifier {
    #[allow(dead_code)]
    pub fn new(productive: Vec<String>, distraction: Vec<String>) -> Self {
        Self {
            productive,
            distraction,
        }
    }

    /// Creates a classifier with built-in defaults merged with user-configured extras.
    /// User entries take effect on top of (not instead of) the comprehensive default lists.
    pub fn with_config(extra_productive: &[String], extra_distraction: &[String]) -> Self {
        let mut this = Self::with_defaults();
        this.productive.extend(extra_productive.iter().map(|s| s.to_lowercase()));
        this.distraction.extend(extra_distraction.iter().map(|s| s.to_lowercase()));
        this
    }

    pub fn with_defaults() -> Self {
        Self {
            productive: vec![
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
                "pwsh".into(),       // PowerShell 7+
                "bash".into(),
                "wsl".into(),
                "wslhost".into(),
                "mintty".into(),
                "conemu".into(),
                "cmder".into(),
                "fluent terminal".into(),
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
                "inkscape".into(),
                "gimp".into(),
                "krita".into(),
                // Adobe Creative Suite
                "photoshop".into(),
                "illustrator".into(),
                "premiere".into(),    // Premiere Pro
                "afterfx".into(),     // After Effects
                "indesign".into(),
                "lightroom".into(),
                "audition".into(),
                "dreamweaver".into(),
                "acrobat".into(),
                "substance".into(),   // Substance Painter/Designer
                "media encoder".into(),
                // 3D / CAD / Engineering
                "blender".into(),
                "maya".into(),
                "3dsmax".into(),
                "cinema4d".into(),
                "zbrush".into(),
                "rhinoceros".into(),
                "solidworks".into(),
                "autocad".into(),
                "revit".into(),
                "fusion360".into(),
                "fusion 360".into(),
                "freecad".into(),
                "houdini".into(),
                "modo".into(),
                // Монтаж и видео
                "resolve".into(),     // DaVinci Resolve
                "davinci".into(),
                "kdenlive".into(),
                "vegas".into(),       // Vegas Pro
                "handbrake".into(),
                "shotcut".into(),
                "openshot".into(),
                "natron".into(),
                "nuke".into(),
                // Аудиопроизводство
                "audacity".into(),
                "reaper".into(),
                "ableton".into(),
                "flstudio".into(),
                "fl studio".into(),
                "fl64".into(),        // FL Studio exe
                "protools".into(),
                "pro tools".into(),
                "cubase".into(),
                "nuendo".into(),
                "studio one".into(),
                "cakewalk".into(),
                "sonar".into(),
                "waveform".into(),
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
                // Наука и анализ данных
                "matlab".into(),
                "rstudio".into(),
                "rgui".into(),
                "spyder".into(),
                "mathematica".into(),
                "wolfram".into(),
                "tableau".into(),
                "powerbi".into(),
                "power bi".into(),
                "anaconda".into(),
                // Дополнительные IDE
                "xcode".into(),
                "eclipse".into(),
                "netbeans".into(),
                "codeblocks".into(),
                // Рабочая коммуникация
                "teams".into(),       // Microsoft Teams
                "zoom".into(),
                "slack".into(),
                "webex".into(),
                // чтение и обучение
                "kindle".into(),
                "calibre".into(),
                "zotero".into(),
                "mendeley".into(),
                "anki".into(),
                "duolingo".into(),
            ],
            distraction: vec![
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
                // игровые платформы (лаунчеры)
                "steam".into(),
                "epicgameslauncher".into(),
                "epicgames".into(),
                "battle.net".into(),
                "battlenet".into(),
                "origin".into(),
                "eadesktop".into(),
                "uplay".into(),
                "ubisoft connect".into(),
                "ubisoftconnect".into(),
                "gog galaxy".into(),
                "goggalaxy".into(),
                "rockstar games launcher".into(),
                "xboxapp".into(),
                "gamelauncher".into(),
                // музыка / видео без дела
                "spotify".into(),
                "deezer".into(),
                "vlc".into(),
            ],
        }
    }

    /// Classifies an executable by name and full path into a productivity category.
    pub fn classify(&self, exe: &str, exe_path: &str) -> Category {
        let lower = exe.to_lowercase();
        let path_lower = exe_path.to_lowercase();

        // Path-based game detection: Steam, Epic Games, GOG, EA, Ubisoft Connect.
        let game_path_markers = [
            "steamapps\\common\\",
            "steamapps/common/",
            "epic games\\",
            "epic games/",
            "gog galaxy\\games\\",
            "gog games\\",
            "ea games\\",
            "ubisoft game launcher\\games\\",
            "ubisoft connect\\games\\",
            "battle.net\\games\\",
            "rockstar games\\",
            "xbox games\\",
        ];
        if game_path_markers.iter().any(|m| path_lower.contains(m)) {
            return Category::Distraction;
        }

        if self.productive.iter().any(|p| lower.contains(p.as_str())) {
            Category::Productive
        } else if self.distraction.iter().any(|d| lower.contains(d.as_str())) {
            Category::Distraction
        } else {
            Category::Neutral
        }
    }
}
