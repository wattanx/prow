import Conf from "conf";
import type { AppConfig } from "../types.js";

const defaults: AppConfig = {
  columns: ["repo", "title", "ci", "reviews", "labels", "updatedAt"],
  pollInterval: 60,
  defaultTab: "created",
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
    columns: s.get("columns"),
    pollInterval: s.get("pollInterval"),
    defaultTab: s.get("defaultTab"),
  };
}
