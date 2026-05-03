# Changelog

All notable changes to this project will be documented in this file.

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
