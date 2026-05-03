<img width="3802" height="2051" alt="2" src="https://github.com/user-attachments/assets/b8909724-b295-48ef-9759-a775693bfa25" />

<img width="3802" height="2051" alt="1" src="https://github.com/user-attachments/assets/1a608086-cbc3-4d10-8f15-3feede903edc" />

![License](https://img.shields.io/badge/license-MIT-blue)
![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20Windows-informational)

# QR Studio

A modern QR code generator built with GTK4 and libadwaita, featuring a Material 3 design layer and a vector-first rendering architecture.

## Features

### Content Types
- **Plain Text** — Freeform text with full QR capacity indicator
- **WiFi** — Network name, password, encryption type
- **vCard** — Contact cards with name, phone, email, organization
- **Calendar** — Events with title, location, start/end datetime
- **GPS** — Interactive map with search autocomplete (Photon API), marker placement, coordinates
- **SMS** — Phone number with country code dropdown (153 countries with flag emojis)

### Design & Customization
- **Dot styles** — Rounded, Square, Dots, Diamond, Custom SVG
- **Corner styles** — Independent corner square and corner dot styling
- **Color picker** with live harmony suggestions (complementary, triadic, analogous, split-complementary)
- **Gradient fills** — Horizontal, Vertical, Diagonal, Radial with configurable colors
- **Logo embedding** — Drag & drop image support, automatic raster-to-vector tracing (vtracer)
- **Outer text** — Custom label text with Pango system font selection
- **Frame styles** — None, Simple, Rounded, Banner
- **Shadow** — Adjustable drop shadow with offset
- **SVG icon support** — Direct vector embedding for crisp logos at any size

### GPS Map
- **4 Map styles** — Positron, Bright, Dark, Fiord (OpenFreeMap vector tiles)
- **Language-aware labels** — Map labels automatically adapt to the selected UI language
- **Interactive map** — Click to place marker, search with autocomplete (Photon API)
- **Raster fallback** — Automatically falls back to CartoDB raster tiles if vector rendering is unavailable

### Export
- **PNG** — Raster export at custom resolution
- **SVG** — Scalable vector export
- **PDF** — Print-ready PDF output with label sheet support
- **GIF** — Animated gradient export
- **Batch export** — CSV-driven bulk QR code generation
- **Clipboard** — Copy as PNG or SVG directly

### UX
- **10 GPU-accelerated CSS animations** — Sidebar slide, QR appear, error shake, content-type transitions, toast notifications, color button pop, logo drop bounce, preview morph, popover entrance
- **Content validation** — Real-time validation for email, GPS coordinates, phone numbers
- **Capacity indicator** — Color-coded progress bar (green → yellow → red) with pulse animation at >90%
- **Drag & Drop** — Drop images onto preview or logo area
- **Transparency checkerboard** — Visual feedback for transparent backgrounds
- **i18n** — 9 languages: English, German, Spanish, French, Italian, Portuguese (BR), Japanese, Korean, Chinese (Simplified)
- **Print size calculator** — Calculate physical print dimensions from module size and DPI
- **Style presets** — Built-in and user-defined style presets with import/export (JSON)
- **Undo/Redo** — Full undo/redo support for style changes
- **Session save/restore** — Templates with import/export
- **Collapsible sidebar** — Animated toggle for more preview space
- **Keyboard shortcuts** — Global shortcuts for common actions

## Installation

### Linux (AppImage)

Download from [Releases](https://github.com/SlobCoder/qr_studio/releases), then:

```bash
chmod +x QR_Studio-*-x86_64.AppImage
./QR_Studio-*-x86_64.AppImage
```

### Windows

Download the ZIP from [Releases](https://github.com/SlobCoder/qr_studio/releases), extract, and run `qr_studio.exe`.

> All required DLLs, GTK schemas, and icon themes are bundled in the archive.

## Building from Source

### Prerequisites

**Linux (Arch Linux):**
```bash
sudo pacman -S gtk4 libadwaita libshumate librsvg gdk-pixbuf2 pkgconf rust
```

**Linux (Ubuntu 24.04+):**
```bash
sudo apt install libgtk-4-dev libadwaita-1-dev libshumate-dev librsvg2-dev \
  libgdk-pixbuf2.0-dev libsoup-3.0-dev libjson-glib-dev libsqlite3-dev \
  libprotobuf-c-dev pkg-config rustc cargo
```

**Windows:** Build via GitHub Actions (MSYS2/MinGW64). See `.github/workflows/build-windows.yml`.

### Build & Run

```bash
git clone https://github.com/SlobCoder/qr_studio.git
cd qr_studio
cargo build --release
./target/release/qr_studio
```

### Build AppImage

```bash
./build_appimage.sh
```

Requires: `cargo`, `mksquashfs`, `wget`.

## Technology Stack

| Component | Technology |
|---|---|
| Language | Rust |
| UI Framework | GTK4 + libadwaita |
| Design Layer | Material 3 |
| Map Widget | libshumate (OpenFreeMap vector tiles) |
| QR Generation | qrcode crate |
| Vector Tracing | vtracer |
| Image Processing | image crate |
| PDF Export | printpdf |
| SVG Rasterization | gdk-pixbuf + librsvg |
| Font Enumeration | Pango |
| HTTP Client | reqwest (Photon API, Nominatim, TileJSON) |
| Parallelism | rayon |

## Architecture

```
src/
├── main.rs           # Application entry, CSS, window setup
├── ui.rs             # UI layout, signal handlers, widgets
├── render.rs         # QR code rendering pipeline
├── svg.rs            # SVG generation, parsing, rasterization
├── helpers.rs        # QR data generation, templates, file I/O
├── types.rs          # State management, ToastType, shared types
├── i18n.rs           # Internationalization (9 languages)
├── map_styles.rs     # OpenFreeMap vector tile styles, localization
├── country_codes.rs  # 153 country entries with flag emojis
└── styles/           # Embedded map style JSONs (Positron, Bright, Dark, Fiord)
```

Vector-first rendering: `render_vector_svg()` generates a complete SVG first, then `rasterize_svg()` converts to raster via gdk-pixbuf/librsvg for display and export.

## License

This project is licensed under the MIT License.
