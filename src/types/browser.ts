export type BrowserName = "chrome" | "edge" | "firefox";

export type BrowserCategory = "productive" | "neutral" | "distraction";

export interface TabInfo {
  browser: BrowserName;
  url: string;
  domain: string;
  title: string;
  category: BrowserCategory;
  subcategory: string;
}

export interface BrowserEventRow {
  id: number;
  startedAt: string;
  durationMs: number;
  browser: BrowserName;
  url: string;
  domain: string;
  title: string;
  category: BrowserCategory;
  subcategory: string;
}

export interface DomainStat {
  domain: string;
  category: BrowserCategory;
  subcategory: string;
  durationMs: number;
  visits: number;
}

export interface UserCategoryRule {
  id: number;
  domainPattern: string;
  category: BrowserCategory;
  subcategory: string;
}
