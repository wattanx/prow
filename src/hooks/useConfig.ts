import Conf from "conf";
import type { AppConfig } from "../types.js";

const defaults: AppConfig = {
  pollInterval: 60,
  defaultSection: "all",
  filteredRepos: [],
};

let store: Conf<AppConfig> | null = null;

function getStore(): Conf<AppConfig> {
  if (!store) {
    store = new Conf<AppConfig>({
      projectName: "prow",
      defaults,
    });
  }
  return store;
}

export function loadConfig(): AppConfig {
  const s = getStore();
  let defaultSection = s.get("defaultSection");
  // Migrate legacy "mine" to "all"
  if (defaultSection === ("mine" as string)) {
    defaultSection = "all";
    s.set("defaultSection", "all");
  }
  return {
    pollInterval: s.get("pollInterval"),
    defaultSection,
    filteredRepos: s.get("filteredRepos"),
  };
}

export function saveFilteredRepos(repos: string[]): void {
  const s = getStore();
  s.set("filteredRepos", repos);
}
