import { execFile } from "node:child_process";
import { promisify } from "node:util";
import { createWriteStream } from "node:fs";
import { chmod, rename, unlink } from "node:fs/promises";
import { pipeline } from "node:stream/promises";
import { Readable } from "node:stream";
import { VERSION } from "../version.js";

const execFileAsync = promisify(execFile);

const REPO = "wattanx/prow";

interface Release {
  tag_name: string;
  assets: { name: string; url: string }[];
}

function getPlatform(): string {
  const os = process.platform === "darwin" ? "darwin" : "linux";
  const arch = process.arch === "arm64" ? "arm64" : "x64";
  return `${os}-${arch}`;
}

async function getLatestRelease(): Promise<Release> {
  const { stdout } = await execFileAsync("gh", [
    "api",
    `repos/${REPO}/releases/latest`,
  ]);
  return JSON.parse(stdout);
}

export async function selfUpdate(): Promise<void> {
  const release = await getLatestRelease();
  const latest = release.tag_name.replace(/^v/, "");
  const current = VERSION;

  if (latest === current) {
    console.log(`Already up to date (v${current})`);
    return;
  }

  console.log(`Updating prow v${current} -> v${latest}...`);

  const platform = getPlatform();
  const assetName = `prow-${platform}`;
  const asset = release.assets.find((a) => a.name === assetName);

  if (!asset) {
    console.error(`No binary found for ${platform}`);
    process.exit(1);
  }

  // Download via gh api to handle authentication
  const { stdout } = await execFileAsync(
    "gh",
    ["api", asset.url, "-H", "Accept: application/octet-stream"],
    { encoding: "buffer", maxBuffer: 200 * 1024 * 1024 },
  );

  const binaryPath = process.execPath;
  const tmpPath = `${binaryPath}.tmp`;

  await unlink(tmpPath).catch(() => {});
  const ws = createWriteStream(tmpPath);
  await pipeline(Readable.from(stdout), ws);
  await chmod(tmpPath, 0o755);
  await rename(tmpPath, binaryPath);

  console.log(`Updated to v${latest}`);
}

export async function selfUninstall(): Promise<void> {
  const binaryPath = process.execPath;

  await unlink(binaryPath);
  console.log(`Removed ${binaryPath}`);
  console.log("prow has been uninstalled");
}
