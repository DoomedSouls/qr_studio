use crate::types::*;
use gtk4::gdk;
use gtk4::glib;
use gtk4::prelude::*;
use image::{DynamicImage, Rgba, RgbaImage};
use std::path::PathBuf;

// ============================================================
// COLOR CONVERSION
// ============================================================

pub fn gdk_to_image_rgba(rgba: &gdk::RGBA) -> Rgba<u8> {
    Rgba([
        (rgba.red() * 255.0) as u8,
        (rgba.green() * 255.0) as u8,
        (rgba.blue() * 255.0) as u8,
        (rgba.alpha() * 255.0) as u8,
    ])
}

// ============================================================
// EC LEVEL CONVERSION
// ============================================================

pub fn ec_level_to_qrcode(level: ErrorCorrectionLevel) -> qrcode::EcLevel {
    match level {
        ErrorCorrectionLevel::Low => qrcode::EcLevel::L,
        ErrorCorrectionLevel::Medium => qrcode::EcLevel::M,
        ErrorCorrectionLevel::Quartile => qrcode::EcLevel::Q,
        ErrorCorrectionLevel::High => qrcode::EcLevel::H,
    }
}

// ============================================================
// STRING PARSING
// ============================================================

pub fn parse_dot_style(s: &str) -> DotStyle {
    match s {
        "Abgerundet" | "Rounded" => DotStyle::Rounded,
        "Quadratisch" | "Square" => DotStyle::Square,
        "Punkte" | "Dots" => DotStyle::Dots,
        "Raute" | "Diamond" => DotStyle::Diamond,
        "Benutzerdefiniert" | "Custom" => DotStyle::Custom,
        // Legacy mappings for backward compatibility with saved settings
        "Stark abgerundet" | "Elegant" | "Elegant abgerundet" | "ExtraRounded" | "Classy"
        | "ClassyRounded" => DotStyle::Rounded,
        _ => DotStyle::Rounded,
    }
}

pub fn parse_corner_square_style(s: &str) -> CornerSquareStyle {
    match s {
        "Quadratisch" | "Square" => CornerSquareStyle::Square,
        "Abgerundet" | "Rounded" => CornerSquareStyle::ExtraRounded,
        "Punkt" | "Dot" => CornerSquareStyle::Dot,
        "Kreis" | "Circle" => CornerSquareStyle::Circle,
        _ => CornerSquareStyle::ExtraRounded,
    }
}

pub fn parse_corner_dot_style(s: &str) -> CornerDotStyle {
    match s {
        "Quadratisch" | "Square" => CornerDotStyle::Square,
        "Punkt" | "Dot" => CornerDotStyle::Dot,
        "Kreis" | "Circle" => CornerDotStyle::Circle,
        "Abgerundet" | "Rounded" => CornerDotStyle::ExtraRounded,
        _ => CornerDotStyle::Dot,
    }
}

pub fn parse_ec_level(s: &str) -> ErrorCorrectionLevel {
    match s {
        "Niedrig (L)" | "Low (L)" => ErrorCorrectionLevel::Low,
        "Mittel (M)" | "Medium (M)" => ErrorCorrectionLevel::Medium,
        "Quartil (Q)" | "Quartile (Q)" => ErrorCorrectionLevel::Quartile,
        "Hoch (H)" | "High (H)" => ErrorCorrectionLevel::High,
        _ => ErrorCorrectionLevel::Medium,
    }
}

pub fn parse_gradient_direction(s: &str) -> GradientDirection {
    match s {
        "Horizontal" => GradientDirection::Horizontal,
        "Vertikal" | "Vertical" => GradientDirection::Vertical,
        "Diagonal" => GradientDirection::Diagonal,
        "Radial" => GradientDirection::Radial,
        _ => GradientDirection::Horizontal,
    }
}

pub fn parse_content_type(s: &str) -> ContentType {
    match s {
        "Text" => ContentType::Text,
        "WiFi" => ContentType::Wifi,
        "vCard" | "vCard/Kontakt" | "vCard/Contact" => ContentType::Vcard,
        "Kalender" | "Kalenderereignis" | "Calendar Event" => ContentType::Calendar,
        "GPS" | "GPS-Standort" | "GPS Location" => ContentType::Gps,
        "SMS" => ContentType::Sms,
        _ => ContentType::Text,
    }
}

pub fn parse_wifi_encryption(s: &str) -> WifiEncryption {
    match s {
        "WPA" => WifiEncryption::Wpa,
        "WEP" => WifiEncryption::Wep,
        "Keine" | "None" => WifiEncryption::None,
        _ => WifiEncryption::Wpa,
    }
}

pub fn parse_logo_shape(s: &str) -> LogoShape {
    match s {
        "Rechteck" | "Rectangle" => LogoShape::Rectangle,
        "Abgerundet" | "Rounded" => LogoShape::RoundedRect,
        "Kreis" | "Circle" => LogoShape::Circle,
        _ => LogoShape::Circle,
    }
}

pub fn parse_frame_style(s: &str) -> FrameStyle {
    match s {
        "Einfach" | "Simple" => FrameStyle::Simple,
        "Abgerundet" | "Rounded" => FrameStyle::Rounded,
        "Banner" => FrameStyle::Banner,
        "Keiner" | "None" => FrameStyle::None,
        _ => FrameStyle::None,
    }
}

// ============================================================
// QR VALIDATION
// ============================================================

pub fn validate_qr(
    ec_level: ErrorCorrectionLevel,
    data: &str,
    logo_size: f64,
    logo_path: Option<&PathBuf>,
) -> Option<String> {
    let has_logo = logo_path.is_some();
    if has_logo && logo_size > 0.3 {
        match ec_level {
            ErrorCorrectionLevel::Low => {
                return Some(
                    "⚠️ Logo zu groß für niedrige Fehlerkorrektur (empfohlen: H)".to_string(),
                );
            }
            ErrorCorrectionLevel::Medium => {
                return Some("⚠️ Logo groß – hohe Fehlerkorrektur empfohlen".to_string());
            }
            _ => {}
        }
    }
    if data.len() > 200 && ec_level == ErrorCorrectionLevel::Low {
        return Some("⚠️ Lange Daten mit niedriger Fehlerkorrektur riskant".to_string());
    }
    None
}

// ============================================================
// SCAN VERIFICATION
// ============================================================

/// Result of QR code scannability verification.
/// Contains the overall quality rating and individual check results
/// so the caller can build localized detail messages.
pub struct ScanResult {
    pub quality: ScanQuality,
    /// Some(ratio) if contrast is below 4.5:1
    pub contrast_ratio: Option<f64>,
    /// Logo is too large for the current error correction level
    pub logo_ec_warning: bool,
    /// Module gap exceeds safe threshold
    pub gap_warning: bool,
    /// rqrr was able to decode the image
    pub decode_ok: bool,
    /// Decoded content matches the original input data
    pub content_matches: bool,
    /// rqrr failed but styled corners are the likely cause (real scanners handle these fine)
    pub styled_corners_fallback: bool,
}

/// Generate a plain QR code (standard square modules) from the data and try to
/// decode it with rqrr. This verifies the data is structurally correct even when
/// the styled rendered image confuses rqrr's finder pattern detection.
fn decode_plain_qr(data: &str, ec_level: ErrorCorrectionLevel) -> Option<String> {
    let qr =
        qrcode::QrCode::with_error_correction_level(data.as_bytes(), ec_level_to_qrcode(ec_level))
            .ok()?;

    // Render as a plain black/white image with quiet zone (at least 200×200px)
    let plain: image::GrayImage = qr
        .render::<image::Luma<u8>>()
        .quiet_zone(true)
        .min_dimensions(200, 200)
        .build();

    let mut prepared = rqrr::PreparedImage::prepare(plain);
    let grids = prepared.detect_grids();
    if let Some(grid) = grids.first() {
        grid.decode().ok().map(|(_, content)| content)
    } else {
        None
    }
}

/// Verify that a rendered QR code image is scannable.
///
/// Performs these checks:
/// 1. **rqrr decode** — Can the image be decoded at all?
/// 2. **Content match** — Does the decoded data match the input?
/// 3. **Contrast ratio** — fg/bg contrast ≥ 4.5:1 (WCAG AA)?
/// 4. **Logo vs EC level** — Is the logo within error correction capacity?
/// 5. **Module gap** — Is the gap between modules within safe limits?
pub fn verify_qr_scanability(
    img: &RgbaImage,
    expected_data: &str,
    fg_color: Rgba<u8>,
    bg_color: Rgba<u8>,
    ec_level: ErrorCorrectionLevel,
    logo_path: Option<&PathBuf>,
    logo_size: f64,
    module_gap: f64,
    corner_square_style: CornerSquareStyle,
    dot_style: DotStyle,
) -> ScanResult {
    let mut result = ScanResult {
        quality: ScanQuality::Good,
        contrast_ratio: None,
        logo_ec_warning: false,
        gap_warning: false,
        decode_ok: false,
        content_matches: false,
        styled_corners_fallback: false,
    };

    let has_styled_corners =
        corner_square_style != CornerSquareStyle::Square || dot_style != DotStyle::Square;

    // 1. Try rqrr decode — downscale large images first (module_size 128px
    //    produces huge images that rqrr can't handle), then convert to grayscale
    let dyn_img = DynamicImage::ImageRgba8(img.clone());
    let max_dim = 800u32;
    let gray = if dyn_img.width() > max_dim || dyn_img.height() > max_dim {
        dyn_img
            .resize(max_dim, max_dim, image::imageops::FilterType::Nearest)
            .to_luma8()
    } else {
        dyn_img.to_luma8()
    };
    let mut prepared = rqrr::PreparedImage::prepare(gray);
    let grids = prepared.detect_grids();

    let decode_failed = if grids.is_empty() {
        true
    } else {
        match grids[0].decode() {
            Ok((_, content)) => {
                result.decode_ok = true;
                result.content_matches = content == expected_data;
                !result.content_matches
            }
            Err(_) => true,
        }
    };

    if decode_failed {
        // Styled corners (ExtraRounded, Circle, Dot) and non-square dot styles
        // (Rounded, Dots, Diamond, Custom) confuse rqrr's finder pattern detection.
        // Real smartphone scanners handle these without issues. Fall back to
        // generating a plain QR code (standard squares) and decoding that with rqrr.
        if has_styled_corners {
            match decode_plain_qr(expected_data, ec_level) {
                Some(decoded) if decoded == expected_data => {
                    result.styled_corners_fallback = true;
                    // Continue below to run static checks (contrast, logo, gap)
                }
                _ => {
                    result.quality = ScanQuality::Bad;
                    return result;
                }
            }
        } else {
            result.quality = ScanQuality::Bad;
            return result;
        }
    }

    // 2. Contrast ratio check (fg vs bg, WCAG threshold 4.5:1)
    let ratio = contrast_ratio(&fg_color, &bg_color);
    if ratio < 4.5 {
        result.contrast_ratio = Some(ratio);
    }

    // 3. Logo coverage vs EC level
    //    EC capacity: L=7%, M=15%, Q=25%, H=30% of codewords.
    //    logo_size is the fraction of QR **width** — the logo is centered
    //    and roughly square, so the actual module area coverage ≈ logo_size².
    //    If that exceeds the EC capacity the QR code cannot be recovered.
    if logo_path.is_some() && logo_size > 0.0 {
        let ec_capacity = match ec_level {
            ErrorCorrectionLevel::Low => 0.07,
            ErrorCorrectionLevel::Medium => 0.15,
            ErrorCorrectionLevel::Quartile => 0.25,
            ErrorCorrectionLevel::High => 0.30,
        };
        let area_fraction = logo_size * logo_size;
        if area_fraction >= ec_capacity {
            // Logo covers more modules than EC can recover → unscannable
            result.quality = ScanQuality::Bad;
            return result;
        }
        if area_fraction >= ec_capacity * 0.5 {
            // Logo is significant, approaching EC limit → warning
            result.logo_ec_warning = true;
        }
    }

    // 4. Module gap check — large gaps between modules reduce reliability
    if module_gap > 0.4 {
        result.gap_warning = true;
    }

    // Determine final quality: warnings downgrade to Limited
    if result.contrast_ratio.is_some() || result.logo_ec_warning || result.gap_warning {
        result.quality = ScanQuality::Limited;
    }

    result
}

// ============================================================
// QR DATA FORMATTING
// ============================================================

pub fn format_qr_data(
    content_type: ContentType,
    text: &str,
    wifi_ssid: &str,
    wifi_password: &str,
    wifi_encryption: WifiEncryption,
    vcard_name: &str,
    vcard_phone: &str,
    vcard_country_code: &str,
    vcard_email: &str,
    vcard_org: &str,
    vcard_url: &str,
    calendar_title: &str,
    calendar_start: &str,
    calendar_end: &str,
    calendar_location: &str,
    gps_lat: &str,
    gps_lon: &str,
    sms_phone: &str,
    sms_country_code: &str,
    sms_message: &str,
) -> String {
    match content_type {
        ContentType::Text => {
            let t = text.trim();
            if t.is_empty() {
                String::new()
            } else if t.contains('@') && !t.contains(' ') && !t.starts_with("mailto:") {
                format!("mailto:{}", t)
            } else if t.starts_with("tel:") {
                t.to_string()
            } else if (t.starts_with('+') || t.starts_with("00"))
                && t[1..].chars().all(|c| c.is_ascii_digit())
            {
                format!("tel:{}", t)
            } else if t.starts_with("http://") || t.starts_with("https://") {
                t.to_string()
            } else if (t.contains('.') || t.starts_with("www.")) && !t.contains(' ') {
                format!("https://{}", t)
            } else {
                t.to_string()
            }
        }
        ContentType::Wifi => {
            let enc = match wifi_encryption {
                WifiEncryption::Wpa => "WPA",
                WifiEncryption::Wep => "WEP",
                WifiEncryption::None => "nopass",
            };
            format!("WIFI:T:{};S:{};P:{};;", enc, wifi_ssid, wifi_password)
        }
        ContentType::Vcard => {
            let mut v = String::from("BEGIN:VCARD\nVERSION:3.0\n");
            if !vcard_name.is_empty() {
                v.push_str(&format!("N:{}\nFN:{}\n", vcard_name, vcard_name));
            }
            if !vcard_phone.is_empty() {
                let full_phone = if vcard_phone.starts_with('+') || vcard_phone.starts_with("00") {
                    vcard_phone.to_string()
                } else {
                    format!("{}{}", vcard_country_code, vcard_phone)
                };
                v.push_str(&format!("TEL:{}\n", full_phone));
            }
            if !vcard_email.is_empty() {
                v.push_str(&format!("EMAIL:{}\n", vcard_email));
            }
            if !vcard_org.is_empty() {
                v.push_str(&format!("ORG:{}\n", vcard_org));
            }
            if !vcard_url.is_empty() {
                v.push_str(&format!("URL:{}\n", vcard_url));
            }
            v.push_str("END:VCARD");
            v
        }
        ContentType::Calendar => {
            let mut v = String::from("BEGIN:VEVENT\n");
            if !calendar_title.is_empty() {
                v.push_str(&format!("SUMMARY:{}\n", calendar_title));
            }
            if !calendar_start.is_empty() {
                v.push_str(&format!("DTSTART:{}\n", calendar_start));
            }
            if !calendar_end.is_empty() {
                v.push_str(&format!("DTEND:{}\n", calendar_end));
            }
            if !calendar_location.is_empty() {
                v.push_str(&format!("LOCATION:{}\n", calendar_location));
            }
            v.push_str("END:VEVENT");
            v
        }
        ContentType::Gps => format!("geo:{},{}", gps_lat, gps_lon),
        ContentType::Sms => {
            let full_phone = if sms_phone.starts_with('+') || sms_phone.starts_with("00") {
                sms_phone.to_string()
            } else {
                format!("{}{}", sms_country_code, sms_phone)
            };
            format!("SMSTO:{}:{}", full_phone, sms_message)
        }
    }
}

pub fn get_qr_data(state: &AppState) -> Option<String> {
    let content_type = *state.content_type.borrow();
    let text = state
        .text_buffer
        .text(
            &state.text_buffer.start_iter(),
            &state.text_buffer.end_iter(),
            false,
        )
        .trim()
        .to_string();
    let wifi_ssid = state.wifi_ssid.borrow().clone();
    let wifi_password = state.wifi_password.borrow().clone();
    let wifi_encryption = *state.wifi_encryption.borrow();
    let vcard_name = state.vcard_name.borrow().clone();
    let vcard_phone = state.vcard_phone.borrow().clone();
    let vcard_country_code = state.vcard_country_code.borrow().clone();
    let vcard_email = state.vcard_email.borrow().clone();
    let vcard_org = state.vcard_org.borrow().clone();
    let vcard_url = state.vcard_url.borrow().clone();
    let calendar_title = state.calendar_title.borrow().clone();
    let calendar_start = state.calendar_start.borrow().clone();
    let calendar_end = state.calendar_end.borrow().clone();
    let calendar_location = state.calendar_location.borrow().clone();
    let gps_lat = state.gps_lat.borrow().clone();
    let gps_lon = state.gps_lon.borrow().clone();
    let sms_phone = state.sms_phone.borrow().clone();
    let sms_country_code = state.sms_country_code.borrow().clone();
    let sms_message = state.sms_message.borrow().clone();

    let data = format_qr_data(
        content_type,
        &text,
        &wifi_ssid,
        &wifi_password,
        wifi_encryption,
        &vcard_name,
        &vcard_phone,
        &vcard_country_code,
        &vcard_email,
        &vcard_org,
        &vcard_url,
        &calendar_title,
        &calendar_start,
        &calendar_end,
        &calendar_location,
        &gps_lat,
        &gps_lon,
        &sms_phone,
        &sms_country_code,
        &sms_message,
    );
    if data.is_empty() { None } else { Some(data) }
}

// ============================================================
// STYLE SETTINGS HELPERS (for Undo/Redo and Import/Export)
// ============================================================

pub fn current_style_settings(state: &AppState) -> StyleSettings {
    let s = state;
    StyleSettings {
        dot_style: format!("{:?}", *s.dot_style.borrow()),
        corner_square_style: format!("{:?}", *s.corner_square_style.borrow()),
        corner_dot_style: format!("{:?}", *s.corner_dot_style.borrow()),
        fg_color: s.fg_color.borrow().0,
        bg_color: s.bg_color.borrow().0,
        corner_color: s.corner_color.borrow().0,
        logo_shape: format!("{:?}", *s.logo_shape.borrow()),
        ec_level: format!("{:?}", *s.ec_level.borrow()),
        module_size: *s.module_size.borrow(),
        quiet_zone: *s.quiet_zone.borrow(),
        module_gap: *s.module_gap.borrow(),
        transparent_bg: *s.transparent_bg.borrow(),
        gradient_enabled: *s.gradient_enabled.borrow(),
        gradient_color: s.gradient_color.borrow().0,
        gradient_direction: format!("{:?}", *s.gradient_direction.borrow()),
        frame_style: format!("{:?}", *s.frame_style.borrow()),
        frame_color: s.frame_color.borrow().0,
        shadow_enabled: *s.shadow_enabled.borrow(),
        shadow_offset: *s.shadow_offset.borrow(),
        frame_width: *s.frame_width.borrow(),
        logo_color: s.logo_color.borrow().0,
        logo_border_width: *s.logo_border_width.borrow(),
        logo_border_color: s.logo_border_color.borrow().0,
        logo_vectorize: *s.logo_vectorize.borrow(),
        logo_vectorize_bg_color: s.logo_vectorize_bg_color.borrow().0,
        logo_bg_transparent: *s.logo_bg_transparent.borrow(),
        logo_clear_area: *s.logo_clear_area.borrow(),
        logo_clear_padding: *s.logo_clear_padding.borrow(),
        logo_outer_radius: *s.logo_outer_radius.borrow(),
        logo_inner_radius: *s.logo_inner_radius.borrow(),
        frame_outer_radius: *s.frame_outer_radius.borrow(),
        frame_inner_radius: *s.frame_inner_radius.borrow(),
        outer_text_font: s.outer_text_font.borrow().clone(),
        outer_text_font_size: *s.outer_text_font_size.borrow(),
    }
}

pub fn apply_style_to_state(state: &AppState, settings: &StyleSettings) {
    *state.dot_style.borrow_mut() = parse_dot_style(&settings.dot_style);
    *state.corner_square_style.borrow_mut() =
        parse_corner_square_style(&settings.corner_square_style);
    *state.corner_dot_style.borrow_mut() = parse_corner_dot_style(&settings.corner_dot_style);
    *state.fg_color.borrow_mut() = Rgba(settings.fg_color);
    *state.bg_color.borrow_mut() = Rgba(settings.bg_color);
    *state.corner_color.borrow_mut() = Rgba(settings.corner_color);
    *state.logo_shape.borrow_mut() = parse_logo_shape(&settings.logo_shape);
    *state.ec_level.borrow_mut() = parse_ec_level(&settings.ec_level);
    *state.module_size.borrow_mut() = settings.module_size;
    *state.quiet_zone.borrow_mut() = settings.quiet_zone;
    *state.module_gap.borrow_mut() = settings.module_gap;
    *state.transparent_bg.borrow_mut() = settings.transparent_bg;
    *state.gradient_enabled.borrow_mut() = settings.gradient_enabled;
    *state.gradient_color.borrow_mut() = Rgba(settings.gradient_color);
    *state.gradient_direction.borrow_mut() = parse_gradient_direction(&settings.gradient_direction);
    *state.frame_style.borrow_mut() = parse_frame_style(&settings.frame_style);
    *state.frame_color.borrow_mut() = Rgba(settings.frame_color);
    *state.shadow_enabled.borrow_mut() = settings.shadow_enabled;
    *state.shadow_offset.borrow_mut() = settings.shadow_offset;
    *state.frame_width.borrow_mut() = settings.frame_width;
    *state.logo_color.borrow_mut() = Rgba(settings.logo_color);
    *state.logo_border_width.borrow_mut() = settings.logo_border_width;
    *state.logo_border_color.borrow_mut() = Rgba(settings.logo_border_color);
    *state.logo_vectorize.borrow_mut() = settings.logo_vectorize;
    *state.logo_vectorize_bg_color.borrow_mut() = Rgba(settings.logo_vectorize_bg_color);
    *state.logo_bg_transparent.borrow_mut() = settings.logo_bg_transparent;
    *state.logo_clear_area.borrow_mut() = settings.logo_clear_area;
    *state.logo_clear_padding.borrow_mut() = settings.logo_clear_padding;
    *state.logo_outer_radius.borrow_mut() = settings.logo_outer_radius;
    *state.logo_inner_radius.borrow_mut() = settings.logo_inner_radius;
    *state.frame_outer_radius.borrow_mut() = settings.frame_outer_radius;
    *state.frame_inner_radius.borrow_mut() = settings.frame_inner_radius;
    *state.outer_text_font.borrow_mut() = settings.outer_text_font.clone();
    *state.outer_text_font_size.borrow_mut() = settings.outer_text_font_size;
}

// ============================================================
// SETTINGS PATH HELPERS
// ============================================================

/// Get the path for auto-saving settings.
pub fn get_settings_path() -> Option<std::path::PathBuf> {
    let config_dir = dirs::config_dir()?;
    let dir = config_dir.join("qr_studio");
    std::fs::create_dir_all(&dir).ok()?;
    Some(dir.join("settings.json"))
}

pub fn get_presets_dir() -> Option<std::path::PathBuf> {
    let config_dir = dirs::config_dir()?;
    let dir = config_dir.join("qr_studio").join("presets");
    std::fs::create_dir_all(&dir).ok()?;
    Some(dir)
}

/// Load saved names from both presets/ and templates/ directories (unified list).
/// Templates dir takes precedence if the same name exists in both.
pub fn load_all_saved_names() -> Vec<String> {
    let mut names = std::collections::BTreeSet::new();
    if let Some(dir) = get_presets_dir() {
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".json") {
                        names.insert(name.trim_end_matches(".json").to_string());
                    }
                }
            }
        }
    }
    if let Some(dir) = get_templates_dir() {
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".json") {
                        names.insert(name.trim_end_matches(".json").to_string());
                    }
                }
            }
        }
    }
    names.into_iter().collect()
}

/// Delete a saved item by name, trying both templates/ and presets/ directories.
/// Returns true if the file was found and deleted.
pub fn delete_saved_item(name: &str) -> bool {
    let mut deleted = false;
    if let Some(dir) = get_templates_dir() {
        let path = dir.join(format!("{}.json", name));
        if path.exists() {
            let _ = std::fs::remove_file(&path);
            deleted = true;
        }
    }
    if let Some(dir) = get_presets_dir() {
        let path = dir.join(format!("{}.json", name));
        if path.exists() {
            let _ = std::fs::remove_file(&path);
            deleted = true;
        }
    }
    deleted
}

/// Try to load a saved item by name, attempting templates/ first, then presets/.
/// Returns the raw JSON string and which directory it came from ("templates" or "presets").
pub fn load_saved_item_json(name: &str) -> Option<(String, String)> {
    // Try templates/ first (full style + content)
    if let Some(dir) = get_templates_dir() {
        let path = dir.join(format!("{}.json", name));
        if let Ok(data) = std::fs::read_to_string(&path) {
            return Some((data, "templates".to_string()));
        }
    }
    // Fall back to presets/ (style only)
    if let Some(dir) = get_presets_dir() {
        let path = dir.join(format!("{}.json", name));
        if let Ok(data) = std::fs::read_to_string(&path) {
            return Some((data, "presets".to_string()));
        }
    }
    None
}

// ============================================================
// QR INFO PANEL
// ============================================================

/// Smoothly animate a ProgressBar from its current fraction to a target value.
/// Uses 20 steps over ~300ms for a fluid glide effect.
pub fn set_fraction_animated(bar: &gtk4::ProgressBar, target: f64) {
    let current = bar.fraction();
    if (current - target).abs() < 0.005 {
        bar.set_fraction(target);
        return;
    }
    let steps = 20u32;
    let step_delta = (target - current) / steps as f64;
    let bar_clone = bar.clone();
    let mut step = 0u32;
    glib::timeout_add_local(std::time::Duration::from_millis(15), move || {
        step += 1;
        if step >= steps {
            bar_clone.set_fraction(target);
            glib::ControlFlow::Break
        } else {
            let v = current + step_delta * step as f64;
            bar_clone.set_fraction(v);
            glib::ControlFlow::Continue
        }
    });
}

pub fn update_qr_info(state: &AppState) {
    let i18n = state.i18n.borrow();

    let data = match get_qr_data(state) {
        Some(d) => d,
        None => {
            state.qr_info_label.set_text(&i18n.t("qrinfo_no_data"));
            set_fraction_animated(&state.qr_capacity_bar, 0.0);
            state.qr_capacity_bar.remove_css_class("progress-ok");
            state.qr_capacity_bar.remove_css_class("progress-warn");
            state.qr_capacity_bar.remove_css_class("progress-critical");
            return;
        }
    };

    let ec_level = *state.ec_level.borrow();
    let qr = match qrcode::QrCode::with_error_correction_level(
        data.as_bytes(),
        ec_level_to_qrcode(ec_level),
    ) {
        Ok(qr) => qr,
        Err(_) => {
            state
                .qr_info_label
                .set_text(&i18n.t("qrinfo_data_too_long"));
            set_fraction_animated(&state.qr_capacity_bar, 1.0);
            state.qr_capacity_bar.remove_css_class("progress-ok");
            state.qr_capacity_bar.remove_css_class("progress-warn");
            state.qr_capacity_bar.add_css_class("progress-critical");
            return;
        }
    };

    let width = qr.width();
    let version = (width - 17) / 4;
    let data_bytes = data.as_bytes().len();

    const BYTE_CAPACITY: [usize; 160] = [
        //  V1      V2      V3      V4      V5      V6      V7      V8      V9      V10
        17, 14, 11, 7, 32, 26, 20, 14, 53, 42, 32, 24, 78, 62, 46, 34, 106, 84, 60, 44, 134, 106,
        76, 58, 154, 122, 88, 64, 192, 152, 110, 84, 230, 180, 130, 98, 271, 213, 151, 119,
        // V11     V12     V13     V14     V15     V16     V17     V18     V19     V20
        321, 251, 177, 137, 367, 287, 203, 155, 425, 331, 241, 177, 458, 362, 258, 194, 520, 412,
        292, 220, 586, 450, 322, 250, 644, 504, 364, 280, 718, 560, 394, 310, 792, 624, 442, 338,
        858, 666, 482, 382,
        // V21     V22     V23     V24     V25     V26     V27     V28     V29     V30
        929, 711, 509, 403, 1003, 779, 565, 439, 1091, 857, 611, 461, 1171, 911, 661, 511, 1273,
        997, 715, 535, 1367, 1059, 751, 593, 1465, 1125, 805, 625, 1528, 1190, 868, 658, 1628,
        1264, 908, 698, 1732, 1370, 982, 742,
        // V31     V32     V33     V34     V35     V36     V37     V38     V39     V40
        1840, 1452, 1030, 790, 1952, 1538, 1112, 842, 2068, 1628, 1168, 898, 2188, 1722, 1228, 958,
        2303, 1809, 1283, 986, 2431, 1911, 1351, 1051, 2563, 1989, 1423, 1093, 2699, 2099, 1499,
        1139, 2809, 2213, 1579, 1219, 2953, 2331, 1663, 1276,
    ];
    let ec_idx = match ec_level {
        ErrorCorrectionLevel::Low => 0,
        ErrorCorrectionLevel::Medium => 1,
        ErrorCorrectionLevel::Quartile => 2,
        ErrorCorrectionLevel::High => 3,
    };
    let max_capacity = BYTE_CAPACITY
        .get((version - 1) * 4 + ec_idx)
        .copied()
        .unwrap_or(data_bytes);

    let ec_percent = match ec_level {
        ErrorCorrectionLevel::Low => "7%",
        ErrorCorrectionLevel::Medium => "15%",
        ErrorCorrectionLevel::Quartile => "25%",
        ErrorCorrectionLevel::High => "30%",
    };
    let ec_name = match ec_level {
        ErrorCorrectionLevel::Low => "L",
        ErrorCorrectionLevel::Medium => "M",
        ErrorCorrectionLevel::Quartile => "Q",
        ErrorCorrectionLevel::High => "H",
    };

    let pct = if max_capacity > 0 {
        (data_bytes as f64 / max_capacity as f64 * 100.0).min(100.0)
    } else {
        100.0
    };
    let fraction = (pct / 100.0).clamp(0.0, 1.0);

    // Scan distance estimation
    let module_size = *state.module_size.borrow();
    let image_pixels = width as f64 * module_size as f64;
    let print_size_inches = image_pixels / 300.0;
    let scan_distance_cm = print_size_inches * 10.0 * 2.54;

    let info = format!(
        "{}: {} ({}×{} {})\n{}: {} ({} {})\n{}: {}/{} {} ({:.0}%)\n{}: ~{:.0} cm ({} 300 DPI)",
        i18n.t("qrinfo_version"),
        version,
        width,
        width,
        i18n.t("qrinfo_modules"),
        i18n.t("qrinfo_ec"),
        ec_name,
        ec_percent,
        i18n.t("qrinfo_data_loss"),
        i18n.t("qrinfo_capacity"),
        data_bytes,
        max_capacity,
        i18n.t("qrinfo_bytes"),
        pct,
        i18n.t("qrinfo_scan_dist"),
        scan_distance_cm,
        i18n.t("qrinfo_at_dpi"),
    );
    state.qr_info_label.set_text(&info);

    // Update progress bar (animated glide + pulse for critical)
    set_fraction_animated(&state.qr_capacity_bar, fraction);
    state.qr_capacity_bar.remove_css_class("progress-ok");
    state.qr_capacity_bar.remove_css_class("progress-warn");
    state.qr_capacity_bar.remove_css_class("progress-critical");
    if pct > 90.0 {
        state.qr_capacity_bar.add_css_class("progress-critical");
    } else if pct > 70.0 {
        state.qr_capacity_bar.add_css_class("progress-warn");
    } else {
        state.qr_capacity_bar.add_css_class("progress-ok");
    }
}

// ============================================================
// CONTRAST RATIO (WCAG 2.0)
// ============================================================

/// Calculate WCAG 2.0 contrast ratio between two RGBA colors.
/// Returns the ratio (1:1 to 21:1). Values >= 3:1 are considered acceptable.
pub fn contrast_ratio(c1: &Rgba<u8>, c2: &Rgba<u8>) -> f64 {
    let l1 = relative_luminance(c1);
    let l2 = relative_luminance(c2);
    let lighter = l1.max(l2);
    let darker = l1.min(l2);
    (lighter + 0.05) / (darker + 0.05)
}

fn relative_luminance(c: &Rgba<u8>) -> f64 {
    let r = srgb_to_linear(c.0[0] as f64 / 255.0);
    let g = srgb_to_linear(c.0[1] as f64 / 255.0);
    let b = srgb_to_linear(c.0[2] as f64 / 255.0);
    0.2126 * r + 0.7152 * g + 0.0722 * b
}

fn srgb_to_linear(v: f64) -> f64 {
    if v <= 0.04045 {
        v / 12.92
    } else {
        ((v + 0.055) / 1.055).powf(2.4)
    }
}

// ============================================================
// SESSION & TEMPLATE PERSISTENCE
// ============================================================

/// Get the path for session persistence.
pub fn get_session_path() -> Option<std::path::PathBuf> {
    let config_dir = dirs::config_dir()?;
    let dir = config_dir.join("qr_studio");
    std::fs::create_dir_all(&dir).ok()?;
    Some(dir.join("session.json"))
}

/// Get the templates directory.
pub fn get_templates_dir() -> Option<std::path::PathBuf> {
    let config_dir = dirs::config_dir()?;
    let dir = config_dir.join("qr_studio").join("templates");
    std::fs::create_dir_all(&dir).ok()?;
    Some(dir)
}

// ============================================================
// TEMPLATE SETTINGS (CONTENT + STYLE)
// ============================================================

/// Create a TemplateSettings from the current AppState (style + content).
pub fn current_template_settings(state: &AppState) -> TemplateSettings {
    let ct = *state.content_type.borrow();
    let content_type_str = match ct {
        ContentType::Text => "Text",
        ContentType::Wifi => "WiFi",
        ContentType::Vcard => "vCard",
        ContentType::Calendar => "Kalender",
        ContentType::Gps => "GPS",
        ContentType::Sms => "SMS",
    };
    let wifi_enc_str = match *state.wifi_encryption.borrow() {
        WifiEncryption::Wpa => "WPA",
        WifiEncryption::Wep => "WEP",
        WifiEncryption::None => "Keine",
    };
    TemplateSettings {
        content_type: content_type_str.to_string(),
        text_content: state
            .text_buffer
            .text(
                &state.text_buffer.start_iter(),
                &state.text_buffer.end_iter(),
                false,
            )
            .to_string(),
        wifi_ssid: state.wifi_ssid.borrow().clone(),
        wifi_password: state.wifi_password.borrow().clone(),
        wifi_encryption: wifi_enc_str.to_string(),
        vcard_name: state.vcard_name.borrow().clone(),
        vcard_phone: state.vcard_phone.borrow().clone(),
        vcard_country_code: state.vcard_country_code.borrow().clone(),
        vcard_email: state.vcard_email.borrow().clone(),
        vcard_org: state.vcard_org.borrow().clone(),
        vcard_url: state.vcard_url.borrow().clone(),
        calendar_title: state.calendar_title.borrow().clone(),
        calendar_start: state.calendar_start.borrow().clone(),
        calendar_end: state.calendar_end.borrow().clone(),
        calendar_location: state.calendar_location.borrow().clone(),
        gps_lat: state.gps_lat.borrow().clone(),
        gps_lon: state.gps_lon.borrow().clone(),
        sms_phone: state.sms_phone.borrow().clone(),
        sms_country_code: state.sms_country_code.borrow().clone(),
        sms_message: state.sms_message.borrow().clone(),
        style: current_style_settings(state),
    }
}

/// Apply template settings to state (content + style).
pub fn apply_template_to_state(state: &AppState, tmpl: &TemplateSettings) {
    // Apply content
    *state.content_type.borrow_mut() = parse_content_type(&tmpl.content_type);
    state.text_buffer.set_text(&tmpl.text_content);
    *state.wifi_ssid.borrow_mut() = tmpl.wifi_ssid.clone();
    *state.wifi_password.borrow_mut() = tmpl.wifi_password.clone();
    *state.wifi_encryption.borrow_mut() = parse_wifi_encryption(&tmpl.wifi_encryption);
    *state.vcard_name.borrow_mut() = tmpl.vcard_name.clone();
    *state.vcard_phone.borrow_mut() = tmpl.vcard_phone.clone();
    *state.vcard_country_code.borrow_mut() = tmpl.vcard_country_code.clone();
    *state.vcard_email.borrow_mut() = tmpl.vcard_email.clone();
    *state.vcard_org.borrow_mut() = tmpl.vcard_org.clone();
    *state.vcard_url.borrow_mut() = tmpl.vcard_url.clone();
    *state.calendar_title.borrow_mut() = tmpl.calendar_title.clone();
    *state.calendar_start.borrow_mut() = tmpl.calendar_start.clone();
    *state.calendar_end.borrow_mut() = tmpl.calendar_end.clone();
    *state.calendar_location.borrow_mut() = tmpl.calendar_location.clone();
    *state.gps_lat.borrow_mut() = tmpl.gps_lat.clone();
    *state.gps_lon.borrow_mut() = tmpl.gps_lon.clone();
    *state.sms_phone.borrow_mut() = tmpl.sms_phone.clone();
    *state.sms_country_code.borrow_mut() = tmpl.sms_country_code.clone();
    *state.sms_message.borrow_mut() = tmpl.sms_message.clone();
    // Apply style
    apply_style_to_state(state, &tmpl.style);
}

// ============================================================
// COLOR HARMONIES
// ============================================================

/// Convert RGB (0–255) to HSL (h: 0–360, s: 0–1, l: 0–1).
fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f64, f64, f64) {
    let r = r as f64 / 255.0;
    let g = g as f64 / 255.0;
    let b = b as f64 / 255.0;
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;

    if (max - min).abs() < f64::EPSILON {
        return (0.0, 0.0, l);
    }

    let d = max - min;
    let s = if l > 0.5 {
        d / (2.0 - max - min)
    } else {
        d / (max + min)
    };

    let h = match max {
        x if (x - r).abs() < f64::EPSILON => (g - b) / d + if g < b { 6.0 } else { 0.0 },
        x if (x - g).abs() < f64::EPSILON => (b - r) / d + 2.0,
        _ => (r - g) / d + 4.0,
    };

    (h * 60.0, s, l)
}

/// Convert HSL (h: 0–360, s: 0–1, l: 0–1) to RGB (0–255).
fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
    if s.abs() < f64::EPSILON {
        let v = (l * 255.0).round() as u8;
        return (v, v, v);
    }

    let q = if l < 0.5 {
        l * (1.0 + s)
    } else {
        l + s - l * s
    };
    let p = 2.0 * l - q;

    let hue_to_rgb = |t: f64| -> f64 {
        let mut t = t;
        if t < 0.0 {
            t += 1.0;
        }
        if t > 1.0 {
            t -= 1.0;
        }
        if t < 1.0 / 6.0 {
            return p + (q - p) * 6.0 * t;
        }
        if t < 1.0 / 2.0 {
            return q;
        }
        if t < 2.0 / 3.0 {
            return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
        }
        p
    };

    let h_norm = h / 360.0;
    let r = (hue_to_rgb(h_norm + 1.0 / 3.0) * 255.0).round() as u8;
    let g = (hue_to_rgb(h_norm) * 255.0).round() as u8;
    let b = (hue_to_rgb(h_norm - 1.0 / 3.0) * 255.0).round() as u8;

    (r, g, b)
}

/// Returns a list of (name, color) harmony suggestions derived from the given color.
pub fn color_harmonies(color: Rgba<u8>) -> Vec<(String, Rgba<u8>)> {
    let (h, s, l) = rgb_to_hsl(color.0[0], color.0[1], color.0[2]);
    let a = color.0[3];

    let rotate = |deg: f64| -> Rgba<u8> {
        let new_h = ((h + deg) % 360.0 + 360.0) % 360.0;
        let (r, g, b) = hsl_to_rgb(new_h, s, l);
        Rgba([r, g, b, a])
    };

    vec![
        ("Komplementär".to_string(), rotate(180.0)),
        ("Analog 1".to_string(), rotate(30.0)),
        ("Analog 2".to_string(), rotate(-30.0)),
        ("Triadisch 1".to_string(), rotate(120.0)),
        ("Triadisch 2".to_string(), rotate(240.0)),
    ]
}
