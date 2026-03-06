import { execSync } from "node:child_process";

export function getGitHubToken(): string {
  try {
    const token = execSync("gh auth token", {
      encoding: "utf-8",
      stdio: ["pipe", "pipe", "pipe"],
    }).trim();

    if (!token) {
      throw new Error("Empty token returned from gh auth token");
    }

    return token;
  } catch {
    throw new Error(
      "Failed to get GitHub token. Make sure gh CLI is installed and you are logged in.\n" +
        "Run: gh auth login"
    );
  }
}
