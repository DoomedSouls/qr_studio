// ============================================================
// CLI MODULE — Headless QR code generation without GUI
// ============================================================
//
// Usage examples:
//   qr_studio --cli --text "Hello" --output qr.png
//   qr_studio --cli --url "example.com" --output qr.svg --dot-style rounded
//   qr_studio --cli --wifi-ssid "MyWiFi" --wifi-password "secret" --output qr.png --fg-color "#ff0000"
//   qr_studio --cli --vcard-name "Max" --vcard-phone "+491234567" --output qr.png
//   qr_studio --cli --text "Test" --output qr.pdf

use crate::helpers::*;
use crate::svg::{rasterize_svg, render_vector_svg};
use crate::types::*;
use clap::Parser;
use image::Rgba;
use std::path::PathBuf;

/// QR Studio — Generate stylish QR codes from the command line
#[derive(Parser, Debug)]
#[command(
    name = "qr_studio",
    version,
    about = "Generate stylish QR codes from the command line",
    long_about = "QR Studio CLI mode — create QR codes with custom styles without the GUI.\n\
                  Supports all content types (text, URL, WiFi, vCard, etc.) and style options.\n\
                  Output formats: PNG, SVG, PDF."
)]
pub struct Cli {
    /// Enable CLI mode (skip GUI)
    #[arg(long)]
    pub cli: bool,

    // ── Content (at least one content option required) ──
    /// Plain text content
    #[arg(long, group = "content")]
    pub text: Option<String>,

    /// URL content (auto-prepends https:// if no scheme)
    #[arg(long, group = "content")]
    pub url: Option<String>,

    /// WiFi SSID
    #[arg(long)]
    pub wifi_ssid: Option<String>,

    /// WiFi password
    #[arg(long)]
    pub wifi_password: Option<String>,

    /// WiFi encryption: WPA, WEP, none (default: WPA)
    #[arg(long, default_value = "WPA")]
    pub wifi_encryption: String,

    /// vCard name
    #[arg(long)]
    pub vcard_name: Option<String>,

    /// vCard phone number
    #[arg(long)]
    pub vcard_phone: Option<String>,

    /// vCard email
    #[arg(long)]
    pub vcard_email: Option<String>,

    /// vCard organization
    #[arg(long)]
    pub vcard_org: Option<String>,

    /// vCard URL
    #[arg(long)]
    pub vcard_url: Option<String>,

    /// Calendar event title
    #[arg(long)]
    pub cal_title: Option<String>,

    /// Calendar event start (YYYYMMDDTHHMMSS)
    #[arg(long)]
    pub cal_start: Option<String>,

    /// Calendar event end (YYYYMMDDTHHMMSS)
    #[arg(long)]
    pub cal_end: Option<String>,

    /// Calendar event location
    #[arg(long)]
    pub cal_location: Option<String>,

    /// GPS latitude
    #[arg(long)]
    pub gps_lat: Option<String>,

    /// GPS longitude
    #[arg(long)]
    pub gps_lon: Option<String>,

    /// SMS phone number
    #[arg(long)]
    pub sms_phone: Option<String>,

    /// SMS message
    #[arg(long)]
    pub sms_message: Option<String>,

    // ── Output ──
    /// Output file path (required). Format detected from extension: .png, .svg, .pdf
    #[arg(long, short)]
    pub output: PathBuf,

    // ── Style ──
    /// Dot style: rounded, square, dots, diamond (default: rounded)
    #[arg(long, default_value = "rounded")]
    pub dot_style: String,

    /// Corner square style: square, rounded, dot, circle (default: rounded)
    #[arg(long, default_value = "rounded")]
    pub corner_square: String,

    /// Corner dot style: square, dot, circle, rounded (default: dot)
    #[arg(long, default_value = "dot")]
    pub corner_dot: String,

    /// Foreground color in #hex format (default: #000000)
    #[arg(long, default_value = "#000000")]
    pub fg_color: String,

    /// Background color in #hex format (default: #ffffff)
    #[arg(long, default_value = "#ffffff")]
    pub bg_color: String,

    /// Corner color in #hex format (default: same as fg-color)
    #[arg(long)]
    pub corner_color: Option<String>,

    /// Error correction level: L, M, Q, H (default: M)
    #[arg(long, default_value = "M")]
    pub ec_level: String,

    /// Module size in pixels: 16, 32, 64, 128 (default: 32)
    #[arg(long, default_value = "32")]
    pub module_size: u32,

    /// Quiet zone modules (default: 2)
    #[arg(long, default_value = "2")]
    pub quiet_zone: u32,

    /// Module gap (0.0–1.0, default: 0.0)
    #[arg(long, default_value = "0.0")]
    pub module_gap: f64,

    /// Transparent background
    #[arg(long)]
    pub transparent_bg: bool,

    /// Enable gradient
    #[arg(long)]
    pub gradient: bool,

    /// Gradient target color in #hex format
    #[arg(long)]
    pub gradient_color: Option<String>,

    /// Gradient direction: horizontal, vertical, diagonal, radial (default: horizontal)
    #[arg(long, default_value = "horizontal")]
    pub gradient_direction: String,

    /// Frame style: none, simple, rounded, banner (default: none)
    #[arg(long, default_value = "none")]
    pub frame_style: String,

    /// Frame color in #hex format (default: #000000)
    #[arg(long, default_value = "#000000")]
    pub frame_color: String,

    /// Frame width in modules (default: 2)
    #[arg(long, default_value = "2")]
    pub frame_width: u32,

    /// Logo image path (centered in QR code)
    #[arg(long)]
    pub logo: Option<PathBuf>,

    /// Logo size as fraction (0.0–1.0, default: 0.25)
    #[arg(long, default_value = "0.25")]
    pub logo_size: f64,

    /// Logo shape: rectangle, rounded, circle (default: circle)
    #[arg(long, default_value = "circle")]
    pub logo_shape: String,

    /// Top label text
    #[arg(long)]
    pub label_top: Option<String>,

    /// Bottom label text
    #[arg(long)]
    pub label_bottom: Option<String>,
}

/// Parse a #hex color string to Rgba<u8>
fn parse_hex_color(s: &str) -> Result<Rgba<u8>, String> {
    let s = s.trim();
    let hex = s.strip_prefix('#').unwrap_or(s);
    if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).map_err(|e| format!("Invalid red: {}", e))?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(|e| format!("Invalid green: {}", e))?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(|e| format!("Invalid blue: {}", e))?;
        Ok(Rgba([r, g, b, 255]))
    } else if hex.len() == 8 {
        let r = u8::from_str_radix(&hex[0..2], 16).map_err(|e| format!("Invalid red: {}", e))?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(|e| format!("Invalid green: {}", e))?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(|e| format!("Invalid blue: {}", e))?;
        let a = u8::from_str_radix(&hex[6..8], 16).map_err(|e| format!("Invalid alpha: {}", e))?;
        Ok(Rgba([r, g, b, a]))
    } else {
        Err(format!(
            "Expected #RRGGBB or #RRGGBBAA, got '{}' (len={})",
            s,
            hex.len()
        ))
    }
}

/// Determine output format from file extension
fn detect_format(path: &PathBuf) -> &'static str {
    match path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase()
        .as_str()
    {
        "svg" => "svg",
        "pdf" => "pdf",
        _ => "png", // default to PNG for .png and unknown
    }
}

/// Run CLI mode: generate QR code and save to file.
/// Returns exit code (0 = success, 1 = error).
pub fn run_cli(cli: &Cli) -> i32 {
    // ── Determine content type and format QR data ──
    let content_type: ContentType;
    let qr_data: String;

    if cli.text.is_some()
        || (cli.wifi_ssid.is_none()
            && cli.url.is_none()
            && cli.vcard_name.is_none()
            && cli.cal_title.is_none()
            && cli.gps_lat.is_none()
            && cli.sms_phone.is_none())
    {
        content_type = ContentType::Text;
        qr_data = format_qr_data(
            ContentType::Text,
            cli.text.as_deref().unwrap_or("QR Studio"),
            "",
            "",
            "",
            WifiEncryption::Wpa,
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        );
    } else if cli.url.is_some() {
        content_type = ContentType::Url;
        qr_data = format_qr_data(
            ContentType::Url,
            "",
            cli.url.as_deref().unwrap_or(""),
            "",
            "",
            WifiEncryption::Wpa,
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        );
    } else if cli.wifi_ssid.is_some() {
        content_type = ContentType::Wifi;
        let enc = match cli.wifi_encryption.to_lowercase().as_str() {
            "wep" => WifiEncryption::Wep,
            "none" | "nopass" => WifiEncryption::None,
            _ => WifiEncryption::Wpa,
        };
        qr_data = format_qr_data(
            ContentType::Wifi,
            "",
            "",
            cli.wifi_ssid.as_deref().unwrap_or(""),
            cli.wifi_password.as_deref().unwrap_or(""),
            enc,
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        );
    } else if cli.vcard_name.is_some() {
        content_type = ContentType::Vcard;
        qr_data = format_qr_data(
            ContentType::Vcard,
            "",
            "",
            "",
            "",
            WifiEncryption::Wpa,
            cli.vcard_name.as_deref().unwrap_or(""),
            cli.vcard_phone.as_deref().unwrap_or(""),
            "", // country_code — not used in CLI (use + prefix directly)
            cli.vcard_email.as_deref().unwrap_or(""),
            cli.vcard_org.as_deref().unwrap_or(""),
            cli.vcard_url.as_deref().unwrap_or(""),
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        );
    } else if cli.cal_title.is_some() {
        content_type = ContentType::Calendar;
        qr_data = format_qr_data(
            ContentType::Calendar,
            "",
            "",
            "",
            "",
            WifiEncryption::Wpa,
            "",
            "",
            "",
            "",
            "",
            "",
            cli.cal_title.as_deref().unwrap_or(""),
            cli.cal_start.as_deref().unwrap_or(""),
            cli.cal_end.as_deref().unwrap_or(""),
            cli.cal_location.as_deref().unwrap_or(""),
            "",
            "",
            "",
            "",
            "",
        );
    } else if cli.gps_lat.is_some() {
        content_type = ContentType::Gps;
        qr_data = format_qr_data(
            ContentType::Gps,
            "",
            "",
            "",
            "",
            WifiEncryption::Wpa,
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            cli.gps_lat.as_deref().unwrap_or("0"),
            cli.gps_lon.as_deref().unwrap_or("0"),
            "",
            "",
            "",
        );
    } else if cli.sms_phone.is_some() {
        content_type = ContentType::Sms;
        qr_data = format_qr_data(
            ContentType::Sms,
            "",
            "",
            "",
            "",
            WifiEncryption::Wpa,
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            cli.sms_phone.as_deref().unwrap_or(""),
            "", // country_code
            cli.sms_message.as_deref().unwrap_or(""),
        );
    } else {
        eprintln!(
            "Error: No content specified. Use --text, --url, --wifi-ssid, --vcard-name, --cal-title, --gps-lat, or --sms-phone"
        );
        return 1;
    }

    if qr_data.is_empty() {
        eprintln!("Error: QR data is empty after formatting");
        return 1;
    }

    // ── Parse style parameters ──
    let dot_style = match cli.dot_style.to_lowercase().as_str() {
        "rounded" => DotStyle::Rounded,
        "square" => DotStyle::Square,
        "dots" => DotStyle::Dots,
        "diamond" => DotStyle::Diamond,
        _ => {
            eprintln!(
                "Warning: Unknown dot style '{}', using 'rounded'",
                cli.dot_style
            );
            DotStyle::Rounded
        }
    };

    let corner_sq = match cli.corner_square.to_lowercase().as_str() {
        "square" => CornerSquareStyle::Square,
        "rounded" => CornerSquareStyle::ExtraRounded,
        "dot" => CornerSquareStyle::Dot,
        "circle" => CornerSquareStyle::Circle,
        _ => {
            eprintln!(
                "Warning: Unknown corner square style '{}', using 'rounded'",
                cli.corner_square
            );
            CornerSquareStyle::ExtraRounded
        }
    };

    let corner_dot = match cli.corner_dot.to_lowercase().as_str() {
        "square" => CornerDotStyle::Square,
        "dot" => CornerDotStyle::Dot,
        "circle" => CornerDotStyle::Circle,
        "rounded" => CornerDotStyle::ExtraRounded,
        _ => {
            eprintln!(
                "Warning: Unknown corner dot style '{}', using 'dot'",
                cli.corner_dot
            );
            CornerDotStyle::Dot
        }
    };

    let ec_level = match cli.ec_level.to_uppercase().as_str() {
        "L" => ErrorCorrectionLevel::Low,
        "M" => ErrorCorrectionLevel::Medium,
        "Q" => ErrorCorrectionLevel::Quartile,
        "H" => ErrorCorrectionLevel::High,
        _ => {
            eprintln!("Warning: Unknown EC level '{}', using 'M'", cli.ec_level);
            ErrorCorrectionLevel::Medium
        }
    };

    let grad_dir = match cli.gradient_direction.to_lowercase().as_str() {
        "horizontal" | "h" => GradientDirection::Horizontal,
        "vertical" | "v" => GradientDirection::Vertical,
        "diagonal" | "d" => GradientDirection::Diagonal,
        "radial" | "r" => GradientDirection::Radial,
        _ => {
            eprintln!(
                "Warning: Unknown gradient direction '{}', using 'horizontal'",
                cli.gradient_direction
            );
            GradientDirection::Horizontal
        }
    };

    let frame_style_enum = match cli.frame_style.to_lowercase().as_str() {
        "none" => FrameStyle::None,
        "simple" => FrameStyle::Simple,
        "rounded" => FrameStyle::Rounded,
        "banner" => FrameStyle::Banner,
        _ => {
            eprintln!(
                "Warning: Unknown frame style '{}', using 'none'",
                cli.frame_style
            );
            FrameStyle::None
        }
    };

    let logo_shape = match cli.logo_shape.to_lowercase().as_str() {
        "rectangle" | "rect" => LogoShape::Rectangle,
        "rounded" => LogoShape::RoundedRect,
        "circle" => LogoShape::Circle,
        _ => {
            eprintln!(
                "Warning: Unknown logo shape '{}', using 'circle'",
                cli.logo_shape
            );
            LogoShape::Circle
        }
    };

    // Parse colors
    let fg_color = match parse_hex_color(&cli.fg_color) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: Invalid fg-color: {}", e);
            return 1;
        }
    };
    let bg_color = match parse_hex_color(&cli.bg_color) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: Invalid bg-color: {}", e);
            return 1;
        }
    };
    let corner_color = match cli.corner_color.as_deref() {
        Some(c) => match parse_hex_color(c) {
            Ok(color) => color,
            Err(e) => {
                eprintln!("Error: Invalid corner-color: {}", e);
                return 1;
            }
        },
        None => fg_color, // default: same as foreground
    };
    let gradient_color = if cli.gradient {
        match cli.gradient_color.as_deref() {
            Some(c) => match parse_hex_color(c) {
                Ok(color) => color,
                Err(e) => {
                    eprintln!("Error: Invalid gradient-color: {}", e);
                    return 1;
                }
            },
            None => {
                eprintln!("Error: --gradient requires --gradient-color");
                return 1;
            }
        }
    } else {
        Rgba([0, 0, 0, 255])
    };
    let frame_color = match parse_hex_color(&cli.frame_color) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: Invalid frame-color: {}", e);
            return 1;
        }
    };

    // ── Generate SVG ──
    let svg = match render_vector_svg(
        &qr_data,
        dot_style,
        corner_sq,
        corner_dot,
        fg_color,
        bg_color,
        corner_color,
        ec_level,
        cli.transparent_bg,
        cli.gradient,
        gradient_color,
        grad_dir,
        cli.logo.as_ref(),
        cli.logo_size,
        cli.label_top.as_deref().unwrap_or(""),
        cli.label_bottom.as_deref().unwrap_or(""),
        fg_color, // outer_text_color
        cli.module_size,
        logo_shape,
        cli.quiet_zone,
        cli.module_gap,
        frame_style_enum,
        frame_color,
        false, // shadow_enabled
        2.0,   // shadow_offset
        cli.frame_width,
        4.0,                        // frame_outer_radius
        4.0,                        // frame_inner_radius
        fg_color,                   // logo_color
        2.0,                        // logo_border_width
        Rgba([255, 255, 255, 255]), // logo_border_color
        None,                       // bg_image_path — no background image in CLI
        None,                       // bg_image_data
        false,                      // logo_vectorize
        Rgba([255, 255, 255, 255]), // logo_vectorize_bg_color
        false,                      // logo_bg_transparent
        false,                      // logo_clear_area
        0.5,                        // logo_clear_padding
        8.0,                        // logo_outer_radius
        8.0,                        // logo_inner_radius
        0.0,                        // gradient_phase
        "",                         // custom_dot_path
        "sans-serif",               // outer_text_font
        14,                         // outer_text_font_size
    ) {
        Some(s) => s,
        None => {
            eprintln!(
                "Error: Failed to generate QR code SVG. Data may be too long for the selected error correction level."
            );
            return 1;
        }
    };

    // ── Save output ──
    let format = detect_format(&cli.output);

    match format {
        "svg" => {
            if let Err(e) = std::fs::write(&cli.output, &svg) {
                eprintln!("Error: Failed to write SVG: {}", e);
                return 1;
            }
        }
        "pdf" => {
            // For PDF, rasterize SVG and embed in PDF
            match render_pdf_from_svg(&svg, cli.module_size, cli.quiet_zone, ec_level) {
                Ok(pdf_bytes) => {
                    if let Err(e) = std::fs::write(&cli.output, &pdf_bytes) {
                        eprintln!("Error: Failed to write PDF: {}", e);
                        return 1;
                    }
                }
                Err(msg) => {
                    eprintln!("Error: Failed to generate PDF: {}", msg);
                    return 1;
                }
            }
        }
        _ => {
            // PNG: rasterize SVG via gdk-pixbuf
            // Calculate pixel dimensions from SVG
            let qr = match qrcode::QrCode::with_error_correction_level(
                qr_data.as_bytes(),
                ec_level_to_qrcode(ec_level),
            ) {
                Ok(q) => q,
                Err(_) => {
                    eprintln!("Error: Failed to create QR code for rasterization");
                    return 1;
                }
            };
            let qr_width = qr.width();
            let total = (qr_width as u32 + cli.quiet_zone * 2) as f64;
            let fw = if frame_style_enum != FrameStyle::None {
                cli.frame_width.max(1) as f64
            } else {
                0.0
            };
            let banner_units = if frame_style_enum == FrameStyle::Banner {
                if !cli.label_bottom.as_deref().unwrap_or("").is_empty() {
                    4.0
                } else {
                    2.0
                }
            } else {
                0.0
            };
            let top_units: f64 = if cli.label_top.as_deref().unwrap_or("").is_empty() {
                0.0
            } else {
                5.0
            };
            let bottom_units: f64 = if cli.label_bottom.as_deref().unwrap_or("").is_empty()
                || frame_style_enum == FrameStyle::Banner
            {
                0.0
            } else {
                5.0
            };
            let full_w = total + fw * 2.0;
            let full_h = total + top_units + bottom_units + fw * 2.0 + banner_units;
            let pixel_w = (full_w * cli.module_size as f64) as u32;
            let pixel_h = (full_h * cli.module_size as f64) as u32;

            let img = match rasterize_svg(&svg, pixel_w.max(1), pixel_h.max(1)) {
                Some(i) => i,
                None => {
                    eprintln!("Error: Failed to rasterize SVG to PNG");
                    return 1;
                }
            };

            if let Err(e) = img.save(&cli.output) {
                eprintln!("Error: Failed to write PNG: {}", e);
                return 1;
            }
        }
    }

    println!(
        "✅ QR code saved to {} (format: {}, content: {})",
        cli.output.display(),
        format.to_uppercase(),
        match content_type {
            ContentType::Text => "text",
            ContentType::Url => "url",
            ContentType::Wifi => "wifi",
            ContentType::Vcard => "vcard",
            ContentType::Calendar => "calendar",
            ContentType::Gps => "gps",
            ContentType::Sms => "sms",
        }
    );
    0
}

/// Render PDF from a pre-generated SVG string.
/// Simplified version of render_pdf_from_state that works without AppState.
fn render_pdf_from_svg(
    svg: &str,
    module_size: u32,
    _quiet_zone: u32,
    _ec_level: ErrorCorrectionLevel,
) -> Result<Vec<u8>, String> {
    use printpdf::*;

    // Parse SVG viewBox to get dimensions
    // The SVG viewBox is in the format: viewBox="0 0 W H"
    let (full_w, full_h) = parse_svg_viewbox(svg).ok_or("Failed to parse SVG viewBox")?;

    let pixel_w = (full_w * module_size as f64) as u32;
    let pixel_h = (full_h * module_size as f64) as u32;

    let img = rasterize_svg(svg, pixel_w.max(1), pixel_h.max(1))
        .ok_or("Failed to rasterize SVG for PDF")?;

    // A4 page — scale QR to fit nicely
    let page_w_mm: f32 = 210.0;
    let page_h_mm: f32 = 297.0;
    let qr_size_mm: f32 = 100.0; // 10cm QR code
    let scale_factor = qr_size_mm / full_w.max(1.0) as f32;
    let qr_h_mm = full_h as f32 * scale_factor;
    let x_offset_mm = (page_w_mm - qr_size_mm) / 2.0;
    let y_offset_mm = (page_h_mm - qr_h_mm) / 2.0;

    let mut doc = PdfDocument::new("QR Code");

    let raw_rgba = img.into_raw();
    let pdf_image = RawImage {
        pixels: RawImageData::U8(raw_rgba),
        width: pixel_w as usize,
        height: pixel_h as usize,
        data_format: RawImageFormat::RGBA8,
        tag: Vec::new(),
    };
    let image_id = doc.add_image(&pdf_image);

    let target_w_pt = qr_size_mm * 72.0 / 25.4;
    let target_h_pt = qr_h_mm * 72.0 / 25.4;
    let default_w_pt = pixel_w as f32 * 72.0 / 300.0;
    let default_h_pt = pixel_h as f32 * 72.0 / 300.0;
    let sx = target_w_pt / default_w_pt;
    let sy = target_h_pt / default_h_pt;

    let x_pt = x_offset_mm * 72.0 / 25.4;
    let y_pt = y_offset_mm * 72.0 / 25.4;

    let ops = vec![Op::UseXobject {
        id: image_id,
        transform: XObjectTransform {
            translate_x: Some(Pt(x_pt)),
            translate_y: Some(Pt(y_pt)),
            scale_x: Some(sx),
            scale_y: Some(sy),
            ..Default::default()
        },
    }];

    let page = PdfPage::new(Mm(page_w_mm), Mm(page_h_mm), ops);
    doc.with_pages(vec![page]);

    let buf = doc.save(&PdfSaveOptions::default(), &mut Vec::new());
    Ok(buf)
}

/// Parse viewBox dimensions from an SVG string.
/// Returns (width, height) as f64 values.
fn parse_svg_viewbox(svg: &str) -> Option<(f64, f64)> {
    // Find viewBox="0 0 W H" in the SVG
    let view_box_start = svg.find("viewBox=\"")?;
    let after_viewbox = &svg[view_box_start + 9..];
    let quote_end = after_viewbox.find('"')?;
    let viewbox_str = &after_viewbox[..quote_end];
    let parts: Vec<&str> = viewbox_str.split_whitespace().collect();
    if parts.len() >= 4 {
        let w: f64 = parts[2].parse().ok()?;
        let h: f64 = parts[3].parse().ok()?;
        Some((w, h))
    } else {
        None
    }
}
