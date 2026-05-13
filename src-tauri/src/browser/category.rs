use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Static rule table: (registered_domain, category, subcategory)
// Registered domain = last two label segments (e.g. "github.com")
// Subdomain overrides come first in SUBDOMAIN_RULES and take priority.
// ---------------------------------------------------------------------------

static SUBDOMAIN_RULES: &[(&str, &str, &str)] = &[
    // Google – per-subdomain distinctions
    ("mail.google.com",    "productive", "email"),
    ("drive.google.com",   "productive", "cloud_storage"),
    ("docs.google.com",    "productive", "writing"),
    ("sheets.google.com",  "productive", "writing"),
    ("slides.google.com",  "productive", "writing"),
    ("calendar.google.com","productive", "planning"),
    ("meet.google.com",    "productive", "communication"),
    ("classroom.google.com","productive","learning"),
    // Microsoft 365
    ("outlook.office365.com", "productive", "email"),
    ("outlook.live.com",      "productive", "email"),
    ("teams.microsoft.com",   "productive", "communication"),
    ("learn.microsoft.com",   "productive", "documentation"),
    ("docs.microsoft.com",    "productive", "documentation"),
    // Jira / Confluence (Atlassian cloud)
    ("jira.atlassian.com",       "productive", "development"),
    ("confluence.atlassian.com", "productive", "writing"),
    // Stack Exchange network
    ("stackoverflow.com",   "productive", "development"),
    ("superuser.com",       "productive", "development"),
    ("serverfault.com",     "productive", "development"),
    ("askubuntu.com",       "productive", "development"),
    ("math.stackexchange.com", "productive", "learning"),
    // Developer docs
    ("developer.mozilla.org",  "productive", "documentation"),
    ("developer.android.com",  "productive", "documentation"),
    ("developer.apple.com",    "productive", "documentation"),
    ("docs.rust-lang.org",     "productive", "documentation"),
    ("doc.rust-lang.org",      "productive", "documentation"),
    ("docs.python.org",        "productive", "documentation"),
    ("pkg.go.dev",             "productive", "documentation"),
    ("reactjs.org",            "productive", "documentation"),
    ("vuejs.org",              "productive", "documentation"),
    ("angular.io",             "productive", "documentation"),
    ("svelte.dev",             "productive", "documentation"),
    ("nextjs.org",             "productive", "documentation"),
    ("tailwindcss.com",        "productive", "documentation"),
    // Mail subdomains
    ("mail.yahoo.com",    "productive", "email"),
    ("mail.proton.me",    "productive", "email"),
    // YouTube – entertainment (override for google.com base)
    ("youtube.com",       "distraction", "entertainment"),
    ("music.youtube.com", "distraction", "entertainment"),
    // Reddit
    ("reddit.com",        "distraction", "social"),
    ("old.reddit.com",    "distraction", "social"),
    // LinkedIn – neutral work networking
    ("linkedin.com",      "neutral",     "networking"),
    // Discord – neutral (often used for communities)
    ("discord.com",       "neutral",     "communication"),
    ("discordapp.com",    "neutral",     "communication"),
];

static DOMAIN_RULES: &[(&str, &str, &str)] = &[
    // ── Development ────────────────────────────────────────────────────────
    ("github.com",          "productive", "development"),
    ("gitlab.com",          "productive", "development"),
    ("bitbucket.org",       "productive", "development"),
    ("codeberg.org",        "productive", "development"),
    ("crates.io",           "productive", "development"),
    ("npmjs.com",           "productive", "development"),
    ("pypi.org",            "productive", "development"),
    ("hub.docker.com",      "productive", "development"),
    ("hub.docker.com",      "productive", "development"),
    ("codepen.io",          "productive", "development"),
    ("jsfiddle.net",        "productive", "development"),
    ("codesandbox.io",      "productive", "development"),
    ("replit.com",          "productive", "development"),
    ("stackblitz.com",      "productive", "development"),
    ("vscode.dev",          "productive", "development"),
    ("github.dev",          "productive", "development"),
    ("vercel.com",          "productive", "development"),
    ("netlify.com",         "productive", "development"),
    ("railway.app",         "productive", "development"),
    ("fly.io",              "productive", "development"),
    // ── Documentation ──────────────────────────────────────────────────────
    ("docs.rs",             "productive", "documentation"),
    ("devdocs.io",          "productive", "documentation"),
    ("mdn.io",              "productive", "documentation"),
    ("w3schools.com",       "productive", "documentation"),
    ("cheatography.com",    "productive", "documentation"),
    // ── Writing / PM ───────────────────────────────────────────────────────
    ("notion.so",           "productive", "writing"),
    ("obsidian.md",         "productive", "writing"),
    ("roamresearch.com",    "productive", "writing"),
    ("logseq.com",          "productive", "writing"),
    ("overleaf.com",        "productive", "writing"),
    ("confluence.com",      "productive", "writing"),
    // ── Planning / Project ─────────────────────────────────────────────────
    ("linear.app",          "productive", "planning"),
    ("trello.com",          "productive", "planning"),
    ("asana.com",           "productive", "planning"),
    ("clickup.com",         "productive", "planning"),
    ("basecamp.com",        "productive", "planning"),
    ("monday.com",          "productive", "planning"),
    ("height.app",          "productive", "planning"),
    // ── Design ─────────────────────────────────────────────────────────────
    ("figma.com",           "productive", "design"),
    ("sketch.com",          "productive", "design"),
    ("miro.com",            "productive", "design"),
    ("whimsical.com",       "productive", "design"),
    ("lucidchart.com",      "productive", "design"),
    ("canva.com",           "productive", "design"),
    ("zeplin.io",           "productive", "design"),
    // ── Communication (work) ───────────────────────────────────────────────
    ("slack.com",           "productive", "communication"),
    ("zoom.us",             "productive", "communication"),
    ("whereby.com",         "productive", "communication"),
    ("webex.com",           "productive", "communication"),
    // ── Learning ───────────────────────────────────────────────────────────
    ("coursera.org",        "productive", "learning"),
    ("udemy.com",           "productive", "learning"),
    ("udacity.com",         "productive", "learning"),
    ("pluralsight.com",     "productive", "learning"),
    ("egghead.io",          "productive", "learning"),
    ("frontendmasters.com", "productive", "learning"),
    ("educative.io",        "productive", "learning"),
    ("codecademy.com",      "productive", "learning"),
    ("freecodecamp.org",    "productive", "learning"),
    ("khanacademy.org",     "productive", "learning"),
    ("brilliant.org",       "productive", "learning"),
    ("leetcode.com",        "productive", "learning"),
    ("hackerrank.com",      "productive", "learning"),
    ("exercism.org",        "productive", "learning"),
    ("scrimba.com",         "productive", "learning"),
    // ── Finance / Work neutral ─────────────────────────────────────────────
    ("protonmail.com",      "productive", "email"),
    ("tutanota.com",        "productive", "email"),
    ("fastmail.com",        "productive", "email"),
    ("hey.com",             "productive", "email"),
    // ── Neutral / Search ───────────────────────────────────────────────────
    ("google.com",          "neutral",    "search"),
    ("bing.com",            "neutral",    "search"),
    ("duckduckgo.com",      "neutral",    "search"),
    ("ecosia.org",          "neutral",    "search"),
    ("kagi.com",            "neutral",    "search"),
    ("startpage.com",       "neutral",    "search"),
    ("yahoo.com",           "neutral",    "search"),
    // ── Neutral / Cloud storage ─────────────────────────────────────────────
    ("dropbox.com",         "neutral",    "cloud_storage"),
    ("box.com",             "neutral",    "cloud_storage"),
    ("icloud.com",          "neutral",    "cloud_storage"),
    // ── Neutral / News ─────────────────────────────────────────────────────
    ("bbc.com",             "neutral",    "news"),
    ("bbc.co.uk",           "neutral",    "news"),
    ("cnn.com",             "neutral",    "news"),
    ("nytimes.com",         "neutral",    "news"),
    ("theguardian.com",     "neutral",    "news"),
    ("reuters.com",         "neutral",    "news"),
    ("apnews.com",          "neutral",    "news"),
    ("techcrunch.com",      "neutral",    "tech_news"),
    ("theverge.com",        "neutral",    "tech_news"),
    ("arstechnica.com",     "neutral",    "tech_news"),
    ("wired.com",           "neutral",    "tech_news"),
    ("hackernews.com",      "neutral",    "tech_news"),
    ("news.ycombinator.com","neutral",    "tech_news"),
    ("dev.to",              "productive", "development"),
    ("hashnode.com",        "neutral",    "tech_news"),
    ("medium.com",          "neutral",    "reading"),
    ("substack.com",        "neutral",    "reading"),
    // ── Neutral / Finance ──────────────────────────────────────────────────
    ("chase.com",           "neutral",    "finance"),
    ("bankofamerica.com",   "neutral",    "finance"),
    ("paypal.com",          "neutral",    "finance"),
    ("stripe.com",          "neutral",    "finance"),
    // ── Distraction / Entertainment ────────────────────────────────────────
    ("netflix.com",         "distraction","entertainment"),
    ("hulu.com",            "distraction","entertainment"),
    ("disneyplus.com",      "distraction","entertainment"),
    ("primevideo.com",      "distraction","entertainment"),
    ("hbomax.com",          "distraction","entertainment"),
    ("max.com",             "distraction","entertainment"),
    ("twitch.tv",           "distraction","entertainment"),
    ("crunchyroll.com",     "distraction","entertainment"),
    ("funimation.com",      "distraction","entertainment"),
    ("imdb.com",            "distraction","entertainment"),
    ("rottentomatoes.com",  "distraction","entertainment"),
    ("buzzfeed.com",        "distraction","entertainment"),
    ("9gag.com",            "distraction","entertainment"),
    ("ifunny.co",           "distraction","entertainment"),
    ("boredpanda.com",      "distraction","entertainment"),
    // ── Distraction / Music ────────────────────────────────────────────────
    ("spotify.com",         "distraction","music"),
    ("soundcloud.com",      "distraction","music"),
    ("bandcamp.com",        "distraction","music"),
    ("music.apple.com",     "distraction","music"),
    ("tidal.com",           "distraction","music"),
    // ── Distraction / Social ───────────────────────────────────────────────
    ("twitter.com",         "distraction","social"),
    ("x.com",               "distraction","social"),
    ("facebook.com",        "distraction","social"),
    ("instagram.com",       "distraction","social"),
    ("tiktok.com",          "distraction","social"),
    ("tumblr.com",          "distraction","social"),
    ("pinterest.com",       "distraction","social"),
    ("snapchat.com",        "distraction","social"),
    ("mastodon.social",     "distraction","social"),
    ("threads.net",         "distraction","social"),
    // ── Distraction / Gaming ───────────────────────────────────────────────
    ("steampowered.com",    "distraction","gaming"),
    ("store.steampowered.com","distraction","gaming"),
    ("poki.com",            "distraction","gaming"),
    ("miniclip.com",        "distraction","gaming"),
    ("coolmathgames.com",   "distraction","gaming"),
    ("kongregate.com",      "distraction","gaming"),
    ("addictinggames.com",  "distraction","gaming"),
    ("itch.io",             "distraction","gaming"),
    // ── Shopping ───────────────────────────────────────────────────────────
    ("amazon.com",          "distraction","shopping"),
    ("ebay.com",            "distraction","shopping"),
    ("aliexpress.com",      "distraction","shopping"),
    ("etsy.com",            "distraction","shopping"),
    ("wish.com",            "distraction","shopping"),
];

// ---------------------------------------------------------------------------
// Engine
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Default)]
pub struct CategoryResult {
    pub category: String,
    pub subcategory: String,
}

pub struct CategoryEngine {
    subdomain_map: HashMap<&'static str, (&'static str, &'static str)>,
    domain_map: HashMap<&'static str, (&'static str, &'static str)>,
    /// Runtime user-defined rules (domain_pattern → (category, subcategory))
    user_rules: Vec<(String, String, String)>,
}

impl CategoryEngine {
    pub fn new() -> Self {
        let subdomain_map: HashMap<_, _> = SUBDOMAIN_RULES
            .iter()
            .map(|&(d, c, s)| (d, (c, s)))
            .collect();
        let domain_map: HashMap<_, _> = DOMAIN_RULES
            .iter()
            .map(|&(d, c, s)| (d, (c, s)))
            .collect();
        Self {
            subdomain_map,
            domain_map,
            user_rules: Vec::new(),
        }
    }

    pub fn add_user_rule(&mut self, pattern: String, category: String, subcategory: String) {
        self.user_rules.retain(|(p, _, _)| p != &pattern);
        self.user_rules.push((pattern, category, subcategory));
    }

    pub fn categorize(&self, domain: &str) -> CategoryResult {
        let domain = domain.trim_start_matches("www.").to_lowercase();

        // 1. User rules (exact or suffix match, highest priority).
        for (pattern, cat, sub) in &self.user_rules {
            let pat = pattern.trim_start_matches("www.").to_lowercase();
            if domain == pat || domain.ends_with(&format!(".{pat}")) {
                return CategoryResult {
                    category: cat.clone(),
                    subcategory: sub.clone(),
                };
            }
        }

        // 2. Subdomain / full-domain built-in rules (e.g. mail.google.com).
        if let Some(&(cat, sub)) = self.subdomain_map.get(domain.as_str()) {
            return CategoryResult {
                category: cat.to_string(),
                subcategory: sub.to_string(),
            };
        }

        // 3. Registered-domain built-in rules (last two labels).
        let registered = registered_domain(&domain);
        if let Some(&(cat, sub)) = self.domain_map.get(registered.as_str()) {
            return CategoryResult {
                category: cat.to_string(),
                subcategory: sub.to_string(),
            };
        }

        // 4. Fallback.
        CategoryResult {
            category: "neutral".into(),
            subcategory: "general".into(),
        }
    }
}

/// Extract the last two DNS labels (e.g. "mail.google.com" → "google.com").
fn registered_domain(host: &str) -> String {
    let parts: Vec<&str> = host.split('.').collect();
    if parts.len() >= 2 {
        parts[parts.len() - 2..].join(".")
    } else {
        host.to_string()
    }
}

/// Extract the host from a URL string without external crates.
pub fn extract_domain(url: &str) -> Option<String> {
    let without_scheme = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .or_else(|| url.strip_prefix("ftp://"))
        .unwrap_or(url);

    let host_and_path = without_scheme.split('/').next()?;
    // Remove port
    let host = host_and_path.split(':').next()?;
    if host.is_empty() {
        return None;
    }
    Some(host.trim_start_matches("www.").to_lowercase())
}
