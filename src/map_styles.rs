use crate::i18n::Lang;
use once_cell::sync::Lazy;
use serde_json::Value;
use std::sync::Mutex;

const POSITRON_STYLE: &str = include_str!("styles/positron.json");
const BRIGHT_STYLE: &str = include_str!("styles/bright.json");
const DARK_STYLE: &str = include_str!("styles/dark.json");
const FIORD_STYLE: &str = include_str!("styles/fiord.json");

const OFM_TILEJSON_URL: &str = "https://tiles.openfreemap.org/planet";

/// Cached TileJSON response — fetched once, reused for all style switches.
static TILEJSON_CACHE: Lazy<Mutex<Option<Value>>> = Lazy::new(|| Mutex::new(None));

/// Fetch (or return cached) TileJSON from OpenFreeMap.
/// This is called once at startup; subsequent calls return the cached value.
fn get_cached_tilejson() -> Option<Value> {
    // Fast path: already cached
    if let Ok(cache) = TILEJSON_CACHE.lock() {
        if cache.is_some() {
            return cache.clone();
        }
    }

    // Slow path: fetch from network
    if let Ok(resp) = reqwest::blocking::get(OFM_TILEJSON_URL) {
        if let Ok(tilejson) = resp.json::<Value>() {
            if let Ok(mut cache) = TILEJSON_CACHE.lock() {
                *cache = Some(tilejson.clone());
            }
            return Some(tilejson);
        }
    }
    None
}

/// Pre-fetch the TileJSON at startup so style switching is instant later.
pub fn prefetch_tilejson() {
    std::thread::spawn(|| {
        let _ = get_cached_tilejson();
    });
}

/// Available OpenFreeMap vector tile styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapStyle {
    Positron,
    Bright,
    Dark,
    Fiord,
}

impl MapStyle {
    /// All available styles in display order.
    pub fn all() -> &'static [MapStyle] {
        &[
            MapStyle::Positron,
            MapStyle::Bright,
            MapStyle::Dark,
            MapStyle::Fiord,
        ]
    }

    /// Short label shown in the style picker pill (language-independent).
    pub fn label(&self) -> &'static str {
        match self {
            MapStyle::Positron => "Positron",
            MapStyle::Bright => "Bright",
            MapStyle::Dark => "Dark",
            MapStyle::Fiord => "Fiord",
        }
    }

    /// Returns the default style based on the system dark mode preference.
    pub fn default_for_system() -> MapStyle {
        if adw::StyleManager::default().is_dark() {
            MapStyle::Dark
        } else {
            MapStyle::Positron
        }
    }

    /// Whether this style has a dark background.
    pub fn is_dark_style(&self) -> bool {
        matches!(self, MapStyle::Dark | MapStyle::Fiord)
    }

    fn style_json(&self) -> &'static str {
        match self {
            MapStyle::Positron => POSITRON_STYLE,
            MapStyle::Bright => BRIGHT_STYLE,
            MapStyle::Dark => DARK_STYLE,
            MapStyle::Fiord => FIORD_STYLE,
        }
    }
}

/// Returns a localized style JSON for the given style and language.
pub fn get_map_style(style: MapStyle, lang: Lang) -> String {
    let base = style.style_json();
    let lang_code = lang_to_code(lang);

    match serde_json::from_str(base) {
        Ok(mut json) => {
            replace_maptiler_urls(&mut json);
            strip_extra_sources(&mut json);
            inject_tilejson(&mut json);
            localize_text_fields(&mut json, lang_code);
            serde_json::to_string(&json).unwrap_or_else(|_| base.to_string())
        }
        Err(_) => base.to_string(),
    }
}

fn lang_to_code(lang: Lang) -> &'static str {
    match lang {
        Lang::De => "name:de",
        Lang::En => "name:en",
        Lang::Es => "name:es",
        Lang::Fr => "name:fr",
        Lang::It => "name:it",
        Lang::PtBr => "name:pt",
        Lang::Ja => "name:ja",
        Lang::Ko => "name:ko",
        Lang::ZhCn => "name:zh",
    }
}

/// Inject cached TileJSON data into the vector source (no HTTP call).
fn inject_tilejson(style: &mut Value) {
    if let Some(sources) = style.get_mut("sources").and_then(|s| s.as_object_mut()) {
        for (_, source) in sources.iter_mut() {
            if source.get("type").and_then(|t| t.as_str()) == Some("vector") {
                if let Some(obj) = source.as_object_mut() {
                    // Remove the TileJSON URL — VectorRenderer doesn't support it
                    obj.remove("url");

                    // Try cached TileJSON first, then hardcode as fallback
                    let tilejson = get_cached_tilejson();
                    if let Some(ref tj) = tilejson {
                        if let Some(tiles) = tj.get("tiles") {
                            obj.insert("tiles".to_string(), tiles.clone());
                        }
                        if let Some(maxzoom) = tj.get("maxzoom") {
                            obj.insert("maxzoom".to_string(), maxzoom.clone());
                        }
                        if let Some(minzoom) = tj.get("minzoom") {
                            obj.insert("minzoom".to_string(), minzoom.clone());
                        }
                    } else {
                        // Hardcoded fallback — OpenFreeMap planet tiles
                        obj.insert(
                            "tiles".to_string(),
                            serde_json::json!([
                                "https://tiles.openfreemap.org/planet/{z}/{x}/{y}.pbf"
                            ]),
                        );
                        obj.insert("maxzoom".to_string(), Value::Number(14.into()));
                        obj.insert("minzoom".to_string(), Value::Number(0.into()));
                    }
                }
            }
        }
    }
}

/// Replace MapTiler glyph/sprite URLs with OpenFreeMap equivalents.
fn replace_maptiler_urls(style: &mut Value) {
    if let Some(glyphs) = style.get_mut("glyphs") {
        if let Some(url) = glyphs.as_str() {
            if url.contains("api.maptiler.com") {
                *glyphs = Value::String(
                    "https://tiles.openfreemap.org/fonts/{fontstack}/{range}.pbf".to_string(),
                );
            }
        }
    }
}

fn strip_extra_sources(style: &mut Value) {
    if let Some(sources) = style.get_mut("sources").and_then(|s| s.as_object_mut()) {
        let raster_sources: Vec<String> = sources
            .iter()
            .filter(|(_, v)| v.get("type").and_then(|t| t.as_str()) != Some("vector"))
            .map(|(k, _)| k.clone())
            .collect();
        for name in &raster_sources {
            sources.remove(name);
        }
        if let Some(layers) = style.get_mut("layers").and_then(|l| l.as_array_mut()) {
            layers.retain(|layer| {
                layer
                    .get("source")
                    .and_then(|s| s.as_str())
                    .map_or(true, |s| !raster_sources.contains(&s.to_string()))
            });
        }
    }
}

fn localize_text_fields(style: &mut Value, lang_code: &str) {
    if let Some(layers) = style.get_mut("layers").and_then(|l| l.as_array_mut()) {
        for layer in layers.iter_mut() {
            if let Some(layout) = layer.get_mut("layout") {
                if let Some(text_field) = layout.get_mut("text-field") {
                    if is_localizable_text_field(text_field) {
                        *text_field = make_localized_field(lang_code);
                    }
                }
            }
        }
    }
}

fn is_localizable_text_field(tf: &Value) -> bool {
    if let Some(arr) = tf.as_array() {
        // MapLibre expression format: ["case", ["has", "name:nonlatin"], ...]
        if arr.first().and_then(|v| v.as_str()) == Some("case") {
            return true;
        }
    }
    if let Some(s) = tf.as_str() {
        // Template string format: "{name:latin}" or "{name:latin}\n{name:nonlatin}"
        if s.contains("name:latin") || s.contains("name:nonlatin") {
            return true;
        }
    }
    false
}

fn make_localized_field(lang_code: &str) -> Value {
    serde_json::json!(["coalesce", ["get", lang_code], ["get", "name"]])
}
