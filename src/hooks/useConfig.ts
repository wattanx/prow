import Conf from "conf";
import type { AppConfig } from "../types.js";

const defaults: AppConfig = {
  pollInterval: 60,
  defaultSection: "mine",
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
  return {
    pollInterval: s.get("pollInterval"),
    defaultSection: s.get("defaultSection"),
  };
}
