# Changelog

All notable changes to this project will be documented in this file.

## [0.4.0] — 2025-05-10

### Added
- **CLI mode** — Headless QR code generation without GUI. Supports all content types (text, URL, WiFi, vCard, Calendar, GPS, SMS) and all style options (dot style, colors, gradient, frame, logo, etc.). Output formats: PNG, SVG, PDF. Usage: `qr_studio --cli --text "Hello" --output qr.png --dot-style rounded --fg-color "#ff0000"`. Ideal for scripting, automation, and batch processing.
- **Unit tests (97 tests)** — Comprehensive test suite covering: `format_qr_data()` for all 7 content types, parsing helpers with English/German input, SVG helpers (rgba_to_svg, base64_encode, xml_escape, parse_svg_viewbox), i18n completeness (all 9 languages checked for missing keys), contrast ratio, and color harmonies.
- **Keyboard shortcuts overlay** — Press `Ctrl+?` to show all keyboard shortcuts in a GTK ShortcutsWindow. Integrated via GResource bundle with `gtk/help-overlay.ui` (standard GNOME pattern).
- **GSettings schema** — Persistent user settings via `io.github.SlobCoder.qr_studio` GSettings schema. Window size, maximized state, sidebar width, and other preferences are saved on close and restored on startup.

### Changed
- **GResource compilation** — Added `build.rs` with `glib-build-tools` for compiling UI resources at build time.
- **AppImage** — Build script now bundles the app's own GSettings schema.

## [0.3.0] — 2025-05-06

### Added
- **URL content type** — Dedicated "Website" tab for URL QR codes. Auto-prepends `https://` for domains without scheme. Available in all 9 languages.
- **QR code import** — Import and decode existing QR codes from image files (PNG, JPEG, SVG, etc.). Auto-detects content type (WiFi, vCard, Calendar, GPS, SMS, URL, Text) and fills in all corresponding fields. SVG files are rasterized via gdk-pixbuf/librsvg. Uses `rqrr` for decoding.
- **Performance profiling** — Optional `hotpath` integration for profiling render pipeline, SVG rasterization, QR decoding, and scan verification. Zero-cost when disabled (`--features hotpath,hotpath-alloc` to enable).

### Fixed
- **Memory: 180× less allocation in scan verification** — `verify_qr_scanability` now converts RGBA→grayscale+downscale in a single pass instead of cloning the full image. Per-call allocation reduced from ~110 MB to ~0.6 MB for large QR codes.
- **UI: scan verification moved to background thread** — The 438ms `verify_qr_scanability` call no longer blocks the main thread. Scan results arrive alongside the rendered image.
- **Memory: cached RGBA buffer released after display** — The preview RGBA image (up to 85 MB) is consumed directly into a GDK texture instead of being cloned and cached. Export operations re-render from cached SVG.
- **Preview: style changes now visible with empty content** — When no QR content is entered, a placeholder "QR Studio" QR code is rendered so style changes (dot style, colors, etc.) are immediately visible. Scan verification is skipped for placeholders.
- **Background image not displayed in preview** — librsvg does not render PNG images embedded as `data:` URIs in `<image>` elements (JPEG works fine). Fixed by converting PNG background images to JPEG in-memory before embedding in SVG. No visible quality loss since the image is displayed at 30% opacity.
- **Double rendering on startup** — Session/style settings triggered intermediate renders with incomplete state while UI widgets were being updated. Fixed by blocking `schedule_preview` during `is_restoring` until all widgets are fully synchronized.

### Changed
- **App ID**: `com.example.qr_studio` → `io.github.SlobCoder.qr_studio` — Proper reverse-DNS identifier for D-Bus, icon lookup, desktop integration, and future Flatpak support.
- **Desktop file**: Fixed hardcoded path, added localized `Comment` and `Keywords` entries for all 9 languages.

## [0.2.2] — 2025-05-03

### Fixed
- **Linux AppImage: GPU rendering broken on non-Ubuntu systems** — Excluded Mesa/EGL/Vulkan/DRM/GBM libraries from bundling. The bundled Ubuntu 24.04 GPU libraries caused EGL/Vulkan initialization failures on other distros (Arch, Fedora, etc.). GPU libraries now come from the host system.

## [0.2.1] — 2025-05-03

### Fixed
- **Windows build completely broken** — Replaced hardcoded DLL list with automatic copy of all DLLs from mingw64/bin. The old list had wrong DLL names (libharfbuzz.dll vs libharfbuzz-0.dll) and was missing many dependencies (zlib1.dll, libffi-8.dll, libgmodule-2.0-0.dll, libappstream-5.dll, libLerc.dll, GStreamer libs, etc.), making the Windows build fail to start at all.
- Added MSYS2 packages: appstream, gstreamer, gst-plugins-good, gst-plugins-bad
- Added GStreamer plugin DLLs and GI typelibs to packaging

## [0.2.0] — 2025-05-03

### Added
- **9-language i18n** — English, German, Spanish, French, Italian, Portuguese (BR), Japanese, Korean, Chinese (Simplified)
- **OpenFreeMap vector tiles** — 4 map styles (Positron, Bright, Dark, Fiord) with language-aware labels via libshumate VectorRenderer
- **Map style picker** — Gear icon OSD pill widget for switching styles with smooth transitions
- **Toast notifications** — Success (✅), Error (❌), Info (ℹ️) toast types with color-coded styling
- **Print size calculator** — Calculate physical print dimensions from module size and DPI
- **Batch export** — CSV-driven bulk QR code generation with custom format strings
- **Label sheet export** — PDF label sheets with configurable columns, rows, margins, spacing
- **GIF export** — Animated gradient QR code export
- **Undo/Redo** — Full undo/redo support for style changes
- **Keyboard shortcuts** — Global shortcuts for common actions (Ctrl+Z, Ctrl+Y, Ctrl+S, etc.)
- **Style presets** — Built-in presets (Classic, Neon, Pastel) and user-defined with JSON import/export
- **Template system** — Save/load full QR code templates (style + content) with delete support
- **Drag & Drop logo** — Drop images directly onto the preview or logo area
- **Shadow** — Adjustable drop shadow with configurable offset
- **GPS search autocomplete** — Photon API-powered place search with inline suggestions

### Fixed
- **i18n dropdown handlers** — All style dropdowns (dot style, corners, gradient, EC level, module size, logo shape, frame style, WiFi encryption) now use index-based matching, fixing style application in all 9 languages (previously only German and English worked)
- **Logo tint color** — Fixed SVG filter color space mismatch (`color-interpolation-filters="sRGB"`) so tint color matches solid border fill exactly
- **Map style switching** — Fixed blank map on style switch using GNOME Maps pattern (insert_layer_above + remove_layer)
- **Map labels** — Fixed language-aware map labels by detecting `{name:latin}` template strings in addition to `["case", ...]` expressions
- **Sidebar highlight** — Changed from accent red to neutral gray, matching standard GTK4 sidebar behavior
- **WiFi encryption dropdown** — Now uses i18n strings instead of hardcoded German

### Changed
- Initial map view changed from Berlin street-level to Europe overview (zoom 3.5)
- Gear rotation animation uses CSS transition instead of @keyframes for smoother reverse
- Removed debug `eprintln!` statements from production code
- Cleaned up 149 unused i18n keys across all 9 languages

## [0.1.0] — 2025-04-18

### Added
- Initial release
- GTK4/libadwaita UI with Material 3 design
- 6 content types: Text, WiFi, vCard, Calendar, GPS, SMS
- QR code customization: dot styles, corner styles, colors, gradients, logo embedding
- Export: PNG, SVG, PDF, Clipboard
- GPS map with marker placement
- 153 country codes with flag emojis
- Color harmony suggestions
- Vector-first SVG rendering pipeline
- 10 GPU-accelerated CSS animations
- Drag & Drop support
- Capacity indicator with pulse animation
