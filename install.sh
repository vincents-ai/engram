#!/usr/bin/env bash
set -euo pipefail

REPO="vincents-ai/engram"
GITHUB="https://github.com"

info()  { printf "\033[1;34m[info]\033[0m  %s\n" "$*"; }
ok()    { printf "\033[1;32m[ok]\033[0m    %s\n" "$*"; }
warn()  { printf "\033[1;33m[warn]\033[0m  %s\n" "$*"; }
die()   { printf "\033[1;31m[error]\033[0m %s\n" "$*" >&2; exit 1; }

confirm() {
  local prompt="$1"
  local response
  printf "%s [Y/n] " "$prompt"
  read -r response
  case "$response" in
    [yY]|[yY][eE][sS]|"") return 0 ;;
    *) return 1 ;;
  esac
}

detect_platform() {
  local os arch
  os="$(uname -s)"
  arch="$(uname -m)"

  case "$os" in
    Linux)
      case "$arch" in
        x86_64|amd64)
          if ldd --version 2>&1 | grep -q musl; then
            echo "linux-musl-amd64"
          else
            echo "linux-amd64"
          fi
          ;;
        aarch64|arm64)  echo "linux-arm64" ;;
        *) die "Unsupported architecture: $arch on $os" ;;
      esac
      ;;
    Darwin)
      case "$arch" in
        x86_64|amd64) echo "macos-amd64" ;;
        aarch64|arm64) echo "macos-arm64" ;;
        *) die "Unsupported architecture: $arch on $os" ;;
      esac
      ;;
    MINGW*|MSYS*|CYGWIN*)
      case "$arch" in
        x86_64|amd64) echo "windows-amd64" ;;
        *) die "Unsupported architecture: $arch on $os" ;;
      esac
      ;;
    *) die "Unsupported OS: $os" ;;
  esac
}

get_latest_tag() {
  local tag
  tag="$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | sed -E 's/.*"([^"]+)".*/\1/')"
  [ -n "$tag" ] || die "Could not determine latest release tag"
  echo "$tag"
}

install_binary() {
  local tag="$1"
  local platform="$2"
  local asset_name ext

  if [ "$platform" = "windows-amd64" ]; then
    asset_name="engram-windows-amd64"
    ext="zip"
  else
    asset_name="engram-${platform}"
    ext="tar.gz"
  fi

  local url="${GITHUB}/${REPO}/releases/download/${tag}/${asset_name}.${ext}"
  local tmp
  tmp="$(mktemp -d)"

  info "Downloading ${asset_name}.${ext} ..."
  curl -fsSL "$url" -o "${tmp}/${asset_name}.${ext}" || die "Download failed: $url"

  info "Extracting ..."
  if [ "$ext" = "zip" ]; then
    unzip -qo "${tmp}/${asset_name}.${ext}" -d "${tmp}" || die "Extraction failed"
  else
    tar xzf "${tmp}/${asset_name}.${ext}" -C "${tmp}" || die "Extraction failed"
  fi

  local bin="${tmp}/engram"
  if [ "$ext" = "zip" ]; then
    bin="${tmp}/engram.exe"
  fi
  [ -f "$bin" ] || die "Binary not found after extraction"

  local install_dir="${ENGRAM_INSTALL_DIR:-/usr/local/bin}"

  if [ "$platform" = "windows-amd64" ]; then
    install_dir="${ENGRAM_INSTALL_DIR:-$HOME/bin}"
    mkdir -p "$install_dir"
    cp "$bin" "${install_dir}/engram.exe"
    ok "Installed to ${install_dir}/engram.exe"
    rm -rf "${tmp}"
  else
    if [ "$(id -u)" -eq 0 ]; then
      cp "$bin" "${install_dir}/engram"
      chmod 755 "${install_dir}/engram"
      ok "Installed to ${install_dir}/engram"
    elif confirm "Install to ${install_dir} (requires sudo)?"; then
      sudo cp "$bin" "${install_dir}/engram"
      sudo chmod 755 "${install_dir}/engram"
      ok "Installed to ${install_dir}/engram"
    else
      mkdir -p "$HOME/.local/bin"
      cp "$bin" "$HOME/.local/bin/engram"
      chmod 755 "$HOME/.local/bin/engram"
      ok "Installed to $HOME/.local/bin/engram"
      warn "Make sure \$HOME/.local/bin is in your PATH"
    fi
    rm -rf "${tmp}"
  fi
}

verify_install() {
  if command -v engram >/dev/null 2>&1; then
    local version
    version="$(engram --version 2>/dev/null || echo "unknown")"
    ok "engram is installed: ${version}"
    return 0
  else
    warn "engram not found in PATH"
    return 1
  fi
}

run_bootstrap() {
  printf "\n"
  info "=== Bootstrap for AI agent setup ==="
  printf "\n"

  if confirm "Initialise engram workspace in current directory?"; then
    engram setup workspace || warn "Workspace setup failed"
  fi

  printf "\n"
  if confirm "Register an agent profile?"; then
    local agent_name
    printf "Agent name (default: claude): "
    read -r agent_name
    agent_name="${agent_name:-claude}"

    local agent_type
    printf "Agent type (implementation|operator|quality_assurance|architecture, default: implementation): "
    read -r agent_type
    agent_type="${agent_type:-implementation}"

    engram setup agent --name "$agent_name" --agent-type "$agent_type" || warn "Agent setup failed"
  fi

  printf "\n"
  if confirm "Install core engram skills (14 skills for your AI coding tool)?"; then
    engram skills setup || warn "Skills setup failed"
  fi

  printf "\n"
  if confirm "Install all skills (44 skills across all categories)?"; then
    engram setup skills --force || warn "Full skills setup failed"
  fi

  printf "\n"
  if confirm "Install the commit-msg hook (enforces task linkage)?"; then
    engram validate hook install || warn "Hook install failed"
  fi

  printf "\n"
  ok "Bootstrap complete!"
}

main() {
  printf "\n\033[1m  Engram Installer\033[0m\n\n"

  local tag="${ENGRAM_VERSION:-}"
  if [ -z "$tag" ]; then
    tag="$(get_latest_tag)"
  fi
  info "Installing Engram ${tag}"

  local platform
  platform="$(detect_platform)"
  info "Detected platform: ${platform}"

  install_binary "$tag" "$platform"

  export PATH="${PATH}:$HOME/.local/bin"

  if verify_install; then
    printf "\n"
    if confirm "Run interactive bootstrap for an AI agent?"; then
      run_bootstrap
    fi

    printf "\n"
    ok "Done! Run 'engram --help' to get started."
  else
    printf "\n"
    warn "Install succeeded but engram is not in your PATH."
    warn "Add its location to your PATH and run 'engram --help'."
  fi
}

main "$@"
