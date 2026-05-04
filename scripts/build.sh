#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

VERSION=$(grep '^version' crates/cicada-client/src-tauri/Cargo.toml 2>/dev/null | head -1 | sed 's/.*"\(.*\)".*/\1/' || echo "0.1.0")
DIST_DIR="dist/${VERSION}"
RED='\033[0;31m'; GREEN='\033[0;32m'; CYAN='\033[0;36m'; NC='\033[0m'

usage() {
  cat <<'EOF'
用法: ./scripts/build.sh [平台...]

  不带参数 = 当前平台
  可选:
    win        Windows  (msi + exe + zip)
    linux      Linux    (deb + AppImage + zip)
    macos      macOS    (dmg)
    android    Android  (apk)
    web        Web PWA
    all        当前平台 + Web + Android

示例:
  ./scripts/build.sh              # 当前平台
  ./scripts/build.sh linux web    # Linux + Web
  ./scripts/build.sh all          # 全平台

EOF
  exit 0
}

step() { echo -e "${CYAN}>>> $1${NC}"; }
ok()   { echo -e "${GREEN}    OK: $1${NC}"; }
err()  { echo -e "${RED}    FAIL: $1${NC}"; }

not_this_os() { echo -e "${RED}  $1 需要在 $2 上构建，当前是 $(uname -s)${NC}"; }

mkdir -p "$DIST_DIR"

build_web() {
  step "构建 Web PWA..."
  cd mobile
  npm install --silent 2>/dev/null || true
  npm run build
  cd ..
  cp -r mobile/dist "$DIST_DIR/web"
  ok "Web PWA -> $DIST_DIR/web/"
}

build_android() {
  step "构建 Android APK..."

  if ! command -v npx &>/dev/null; then
    err "需要 Node.js"
    return 1
  fi

  mkdir -p mobile-android
  cd mobile-android

  if [ ! -f package.json ]; then
    npm init -y >/dev/null 2>&1
    npm install @capacitor/core @capacitor/cli @capacitor/android >/dev/null 2>&1
    npx cap init "知了" "com.cicada.app" --web-dir=../mobile/dist >/dev/null 2>&1
  fi

  npx cap add android 2>/dev/null || true
  npx cap copy android 2>/dev/null || true

  cd android
  ./gradlew assembleDebug 2>/dev/null || {
    err "Android SDK / gradlew 未就绪，需要安装 Android Studio"
    cd ../..
    return 1
  }

  mkdir -p "../../$DIST_DIR/android"
  find . -name "*.apk" -exec cp {} "../../$DIST_DIR/android/" \;
  ok "Android APK -> $DIST_DIR/android/"
  cd ../..
}

build_tauri() {
  local t="$1"
  step "构建 Tauri ($t)..."

  cd crates/cicada-client/ui
  npm install --silent 2>/dev/null || true
  npm run build
  cd ../..

  cd crates/cicada-client
  if [ "$t" = "current" ]; then
    cargo tauri build
    local bundle="src-tauri/target/release/bundle"
  else
    rustup target add "$t" 2>/dev/null || true
    cargo tauri build --target "$t"
    local bundle="src-tauri/target/${t}/release/bundle"
  fi
  cd ../..

  local out="$DIST_DIR"

  if ls "$bundle/msi/"*.msi 2>/dev/null; then
    cp "$bundle/msi/"*.msi "$out/" 2>/dev/null || true
    cp "$bundle/nsis/"*.exe "$out/" 2>/dev/null || true
    ok "Windows -> $out/"
  fi

  if ls "$bundle/deb/"*.deb 2>/dev/null; then
    cp "$bundle/deb/"*.deb "$out/" 2>/dev/null || true
    ok "Linux .deb -> $out/"
  fi

  if ls "$bundle/appimage/"*.AppImage 2>/dev/null; then
    cp "$bundle/appimage/"*.AppImage "$out/" 2>/dev/null || true
    ok "Linux AppImage -> $out/"
  fi

  if ls "$bundle/dmg/"*.dmg 2>/dev/null; then
    cp "$bundle/dmg/"*.dmg "$out/" 2>/dev/null || true
    ok "macOS -> $out/"
  fi
}

[ $# -eq 0 ] && set -- current
[[ "$*" =~ help|--help|-h ]] && usage

for arg in "$@"; do
  case "$arg" in
    current)
      case "$(uname -s)" in
        Linux|Darwin|MINGW*|MSYS*|Windows_NT) build_tauri "current" ;;
        *) err "未知系统" ;;
      esac ;;
    win|windows)
      if [[ "$(uname -s)" =~ MINGW|MSYS|Windows_NT ]]; then
        build_tauri "x86_64-pc-windows-msvc"
      else not_this_os "Windows" "Windows"; fi ;;
    linux)
      [ "$(uname -s)" = "Linux" ] && build_tauri "current" || not_this_os "Linux" "Linux" ;;
    macos|darwin)
      [ "$(uname -s)" = "Darwin" ] && build_tauri "current" || not_this_os "macOS" "macOS" ;;
    android) build_android ;;
    web) build_web ;;
    all)
      [ "$(uname -s)" = "Linux" ] && build_tauri "current" || true
      build_web
      build_android 2>/dev/null || echo "  (跳过 Android - 需要 SDK)"
      ;;
    *) err "未知平台: $arg"; usage ;;
  esac
done

echo ""
echo -e "${GREEN}===== 构建完成 =====${NC}"
echo "产物: $DIST_DIR/"
[ -d "$DIST_DIR" ] && ls -lh "$DIST_DIR/" 2>/dev/null || echo "  (无产物)"
