#!/usr/bin/env bash
set -euo pipefail

REPO="wattanx/prow"
INSTALL_DIR="${PROW_INSTALL_DIR:-$HOME/.local/bin}"
BINARY_NAME="prow"

get_os() {
  case "$(uname -s)" in
    Linux*)  echo "linux" ;;
    Darwin*) echo "darwin" ;;
    *)
      echo "Unsupported OS: $(uname -s)" >&2
      exit 1
      ;;
  esac
}

get_arch() {
  case "$(uname -m)" in
    x86_64|amd64)  echo "x64" ;;
    arm64|aarch64) echo "arm64" ;;
    *)
      echo "Unsupported architecture: $(uname -m)" >&2
      exit 1
      ;;
  esac
}

main() {
  local os arch asset_name download_url version

  os="$(get_os)"
  arch="$(get_arch)"
  asset_name="prow-${os}-${arch}"

  echo "Detecting platform: ${os}-${arch}"

  # Get latest release version
  version="$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | sed -E 's/.*"([^"]+)".*/\1/')"

  if [ -z "$version" ]; then
    echo "Failed to fetch latest version" >&2
    exit 1
  fi

  echo "Installing prow ${version}..."

  download_url="https://github.com/${REPO}/releases/download/${version}/${asset_name}"

  # Download binary
  local tmp_dir
  tmp_dir="$(mktemp -d)"
  trap 'rm -rf "$tmp_dir"' EXIT

  curl -fsSL "$download_url" -o "${tmp_dir}/${BINARY_NAME}"
  chmod +x "${tmp_dir}/${BINARY_NAME}"

  # Install
  mkdir -p "$INSTALL_DIR"
  if [ -w "$INSTALL_DIR" ]; then
    mv "${tmp_dir}/${BINARY_NAME}" "${INSTALL_DIR}/${BINARY_NAME}"
  else
    echo "Need sudo to install to ${INSTALL_DIR}"
    sudo mv "${tmp_dir}/${BINARY_NAME}" "${INSTALL_DIR}/${BINARY_NAME}"
  fi

  echo "prow ${version} installed to ${INSTALL_DIR}/${BINARY_NAME}"

  # Warn if INSTALL_DIR is not in PATH
  case ":$PATH:" in
    *":${INSTALL_DIR}:"*) ;;
    *) echo "Warning: ${INSTALL_DIR} is not in your PATH. Add it with:" >&2
       echo "  export PATH=\"${INSTALL_DIR}:\$PATH\"" >&2
       ;;
  esac
}

main
