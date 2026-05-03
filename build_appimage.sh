#!/usr/bin/env bash
set -euo pipefail

# ═══════════════════════════════════════════════════════════════
#  QR Studio — AppImage Builder
#
#  Bundles the GTK4/libadwaita QR code generator into a
#  portable AppImage. Run from the project root:
#
#    ./build_appimage.sh
#
#  Requirements: cargo, mksquashfs, wget (to download appimagetool)
# ═══════════════════════════════════════════════════════════════

APP_NAME="QR_Studio"
APP_ID="com.example.qr_studio"
BINARY="qr_studio"
VERSION="0.2.4"
ARCH="x86_64"

PROJECT_DIR="$(cd "$(dirname "$0")" && pwd)"
BUILD_DIR="${PROJECT_DIR}/target/release"
WORK_DIR="${PROJECT_DIR}/build"
APPDIR="${WORK_DIR}/${APP_NAME}.AppDir"
OUTPUT="${WORK_DIR}/${APP_NAME}-${VERSION}-${ARCH}.AppImage"

# System/glibc libraries to EXCLUDE from bundling
# (these must come from the host system to avoid ABI conflicts)
EXCLUDE_RE='ld-linux|linux-vdso|/libc\.so|/libm\.so|/libdl\.so|/libpthread\.so|/librt\.so|/libresolv\.so|/libnss|libEGL|libGL[._]|libGLdispatch|libGLX|libGLES|libglapi|libvulkan|libdrm|libgbm|libwayland-egl|libepoxy'

STEP=0
step()  { STEP=$((STEP + 1)); printf '\n▸ [%d/8] %s\n' "$STEP" "$1"; }
ok()    { printf '  ✓ %s\n' "$1"; }
die()   { printf '  ✗ ERROR: %s\n' "$1" >&2; exit 1; }

# ── Banner ────────────────────────────────────────────────────
printf '╔═══════════════════════════════════════════╗\n'
printf '║   QR Studio — AppImage Builder            ║\n'
printf '╚═══════════════════════════════════════════╝\n'

# ── 1. Build release binary ───────────────────────────────────
step "Building release binary"
cargo build --release --manifest-path "${PROJECT_DIR}/Cargo.toml" \
    || die "cargo build failed"
ok "$(du -h "${BUILD_DIR}/${BINARY}" | cut -f1)"

# ── 2. Create AppDir structure ────────────────────────────────
step "Creating AppDir structure"
rm -rf "${APPDIR}"
mkdir -p "${APPDIR}"/usr/{bin,lib,share/{glib-2.0/schemas,icons/hicolor/scalable/apps}}
mkdir -p "${APPDIR}/usr/lib/gdk-pixbuf-2.0/2.10.0/loaders"
ok "${APPDIR}"

# ── 3. Copy application files ─────────────────────────────────
step "Copying application files"

# Binary (stripped)
cp "${BUILD_DIR}/${BINARY}" "${APPDIR}/usr/bin/"
chmod +x "${APPDIR}/usr/bin/${BINARY}"
strip --strip-unneeded "${APPDIR}/usr/bin/${BINARY}" 2>/dev/null || true
ok "binary  →  $(du -h "${APPDIR}/usr/bin/${BINARY}" | cut -f1)"

# .desktop file (must live in AppDir root for appimagetool)
DESKTOP_SRC="${HOME}/.local/share/applications/${APP_ID}.desktop"
[ -f "$DESKTOP_SRC" ] && cp "$DESKTOP_SRC" "${APPDIR}/${APP_ID}.desktop" && ok "desktop file"

# Icon (must live in AppDir root for appimagetool)
ICON_SRC="${HOME}/.local/share/icons/hicolor/scalable/apps/${APP_ID}.svg"
if [ -f "$ICON_SRC" ]; then
    cp "$ICON_SRC" "${APPDIR}/${APP_ID}.svg"
    cp "$ICON_SRC" "${APPDIR}/usr/share/icons/hicolor/scalable/apps/${APP_ID}.svg"
    ok "icon"
fi

# ── 4. Bundle shared libraries ────────────────────────────────
step "Bundling shared libraries"

DEPS_FILE=$(mktemp /tmp/qr_studio_deps.XXXXXX)
trap 'rm -f "$DEPS_FILE"' EXIT

# ldd resolves transitive deps, so one call covers the full tree
ldd "${BUILD_DIR}/${BINARY}" 2>/dev/null | awk '/=>/{print $3}' >> "$DEPS_FILE"

# librsvg — loaded dynamically by gdk-pixbuf for SVG, not in ldd output
if [ -f /usr/lib/librsvg-2.so.2 ]; then
    echo "/usr/lib/librsvg-2.so.2" >> "$DEPS_FILE"
    ldd /usr/lib/librsvg-2.so.2 2>/dev/null | awk '/=>/{print $3}' >> "$DEPS_FILE"
fi

# gdk-pixbuf external loaders (dynamically dlopen'd)
LOADERS_SRC="/usr/lib/gdk-pixbuf-2.0/2.10.0/loaders"
if [ -d "$LOADERS_SRC" ]; then
    for loader in "${LOADERS_SRC}"/*.so; do
        [ -f "$loader" ] || continue
        echo "$loader" >> "$DEPS_FILE"
        ldd "$loader" 2>/dev/null | awk '/=>/{print $3}' >> "$DEPS_FILE"
    done
fi

# Deduplicate → filter → copy
sort -u "$DEPS_FILE" \
    | grep -vE "$EXCLUDE_RE" \
    | while IFS= read -r lib; do
          [ -f "$lib" ] && cp -Ln "$lib" "${APPDIR}/usr/lib/" 2>/dev/null || true
      done

LIB_COUNT=$(find "${APPDIR}/usr/lib" -maxdepth 1 -name '*.so*' | wc -l)
ok "${LIB_COUNT} libraries"

# ── 5. GSettings schemas ─────────────────────────────────────
step "Copying GSettings schemas"

SCHEMA_SRC="/usr/share/glib-2.0/schemas"
SCHEMA_DST="${APPDIR}/usr/share/glib-2.0/schemas"

for f in "${SCHEMA_SRC}"/org.gtk.gtk4.*.gschema.xml \
         "${SCHEMA_SRC}"/org.gtk.Settings.*.gschema.xml; do
    [ -f "$f" ] && cp "$f" "$SCHEMA_DST/"
done

glib-compile-schemas "$SCHEMA_DST" 2>/dev/null || true
ok "$(ls "$SCHEMA_DST"/*.xml 2>/dev/null | wc -l) schemas compiled"

# ── 6. gdk-pixbuf loaders ────────────────────────────────────
step "Configuring gdk-pixbuf loaders"

LOADERS_DST="${APPDIR}/usr/lib/gdk-pixbuf-2.0/2.10.0/loaders"

if [ -d "$LOADERS_SRC" ] && ls "${LOADERS_SRC}"/*.so &>/dev/null; then
    cp -Ln "${LOADERS_SRC}"/*.so "$LOADERS_DST/" 2>/dev/null || true

    # Regenerate loaders.cache with our bundled paths
    GDK_PIXBUF_MODULEDIR="$LOADERS_DST" gdk-pixbuf-query-loaders 2>/dev/null \
        > "${APPDIR}/usr/lib/gdk-pixbuf-2.0/2.10.0/loaders.cache" || true

    ok "$(ls "$LOADERS_DST"/*.so 2>/dev/null | wc -l) loaders"
else
    : > "${APPDIR}/usr/lib/gdk-pixbuf-2.0/2.10.0/loaders.cache"
    ok "built-in only (no external loaders)"
fi

# ── 7. Create AppRun entry point ─────────────────────────────
step "Creating AppRun"

cat > "${APPDIR}/AppRun" << 'APPRUN'
#!/bin/sh
HERE="$(dirname "$(readlink -f "$0")")"

# Bundled libraries take priority
export LD_LIBRARY_PATH="${HERE}/usr/lib${LD_LIBRARY_PATH:+:${LD_LIBRARY_PATH}}"

# gdk-pixbuf: use bundled loaders
export GDK_PIXBUF_MODULEDIR="${HERE}/usr/lib/gdk-pixbuf-2.0/2.10.0/loaders"
export GDK_PIXBUF_MODULE_FILE="${HERE}/usr/lib/gdk-pixbuf-2.0/2.10.0/loaders.cache"

# GSettings: use bundled compiled schemas
export GSETTINGS_SCHEMA_DIR="${HERE}/usr/share/glib-2.0/schemas${GSETTINGS_SCHEMA_DIR:+:${GSETTINGS_SCHEMA_DIR}}"

# Icons and themes (hicolor icon theme for our app icon)
export XDG_DATA_DIRS="${HERE}/usr/share${XDG_DATA_DIRS:+:${XDG_DATA_DIRS}}"

# GTK4 module path
export GTK_PATH="${HERE}/usr/lib/gtk-4.0${GTK_PATH:+:${GTK_PATH}}"

exec "${HERE}/usr/bin/qr_studio" "$@"
APPRUN
chmod +x "${APPDIR}/AppRun"
ok "AppRun"

# ── 8. Package into AppImage ─────────────────────────────────
step "Packaging AppImage"
mkdir -p "$(dirname "$OUTPUT")"

# Locate or download appimagetool
APPIMAGETOOL=""
for candidate in appimagetool "${WORK_DIR}/appimagetool"; do
    if command -v "$candidate" &>/dev/null || [ -x "$candidate" ]; then
        APPIMAGETOOL="$candidate"
        break
    fi
done

if [ -z "$APPIMAGETOOL" ]; then
    echo "  Downloading appimagetool…"
    wget -q --show-progress \
        "https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage" \
        -O "${WORK_DIR}/appimagetool" || die "download failed"
    chmod +x "${WORK_DIR}/appimagetool"
    APPIMAGETOOL="${WORK_DIR}/appimagetool"
fi

ARCH="$ARCH" "$APPIMAGETOOL" "$APPDIR" "$OUTPUT" || die "appimagetool failed"

# ── Done ──────────────────────────────────────────────────────
printf '\n════════════════════════════════════════════\n'
printf '  ✓  %s\n' "$OUTPUT"
printf '     %s\n' "$(du -h "$OUTPUT" | cut -f1)"
printf '════════════════════════════════════════════\n'
printf '  Run:  chmod +x %s && ./%s\n' "$(basename "$OUTPUT")" "$(basename "$OUTPUT")"
printf '\n'
