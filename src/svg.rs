use crate::helpers::ec_level_to_qrcode;
use crate::types::*;
use image::{Rgba, RgbaImage};
use std::path::PathBuf;

// ============================================================
// MODULE TYPE & DARK HELPERS
// ============================================================

pub fn get_module_type(x: usize, y: usize, size: usize) -> ModuleType {
    let in_top_left = x < 7 && y < 7;
    let in_top_right = x >= size - 7 && y < 7;
    let in_bottom_left = x < 7 && y >= size - 7;

    if in_top_left || in_top_right || in_bottom_left {
        let lx = if in_top_right { x - (size - 7) } else { x };
        let ly = if in_bottom_left { y - (size - 7) } else { y };

        if lx >= 2 && lx <= 4 && ly >= 2 && ly <= 4 {
            return ModuleType::CornerDot;
        }
        return ModuleType::CornerSquare;
    }
    ModuleType::Data
}

pub fn is_dark(qr: &qrcode::QrCode, x: i32, y: i32, width: usize) -> bool {
    if x < 0 || y < 0 || x >= width as i32 || y >= width as i32 {
        return false;
    }
    qr[(x as usize, y as usize)] == qrcode::types::Color::Dark
}

// ============================================================
// COLOR & EC LEVEL HELPERS
// ============================================================

pub fn rgba_to_svg(color: Rgba<u8>) -> String {
    if color.0[3] < 255 {
        format!(
            "rgba({},{},{},{:.2})",
            color.0[0],
            color.0[1],
            color.0[2],
            color.0[3] as f64 / 255.0
        )
    } else {
        format!("#{:02x}{:02x}{:02x}", color.0[0], color.0[1], color.0[2])
    }
}

// ============================================================
// SVG PATH GENERATORS
// ============================================================

pub fn svg_selective_rounded_rect_path(
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    r: f64,
    round_tl: bool,
    round_tr: bool,
    round_bl: bool,
    round_br: bool,
) -> String {
    let r = r.min(w / 2.0).min(h / 2.0);
    let mut d = String::new();
    if round_tl {
        d.push_str(&format!("M{:.3},{:.3} ", x, y + r));
        d.push_str(&format!("A{:.3},{:.3} 0 0,1 {:.3},{:.3} ", r, r, x + r, y));
    } else {
        d.push_str(&format!("M{:.3},{:.3} ", x, y));
    }
    if round_tr {
        d.push_str(&format!("L{:.3},{:.3} ", x + w - r, y));
        d.push_str(&format!(
            "A{:.3},{:.3} 0 0,1 {:.3},{:.3} ",
            r,
            r,
            x + w,
            y + r
        ));
    } else {
        d.push_str(&format!("L{:.3},{:.3} ", x + w, y));
    }
    if round_br {
        d.push_str(&format!("L{:.3},{:.3} ", x + w, y + h - r));
        d.push_str(&format!(
            "A{:.3},{:.3} 0 0,1 {:.3},{:.3} ",
            r,
            r,
            x + w - r,
            y + h
        ));
    } else {
        d.push_str(&format!("L{:.3},{:.3} ", x + w, y + h));
    }
    if round_bl {
        d.push_str(&format!("L{:.3},{:.3} ", x + r, y + h));
        d.push_str(&format!(
            "A{:.3},{:.3} 0 0,1 {:.3},{:.3} ",
            r,
            r,
            x,
            y + h - r
        ));
    } else {
        d.push_str(&format!("L{:.3},{:.3} ", x, y + h));
    }
    d.push_str("Z");
    d
}

pub fn svg_side_rounded_path(
    x: f64,
    y: f64,
    ms: f64,
    flat_left: bool,
    _flat_right: bool,
    flat_top: bool,
    flat_bottom: bool,
) -> String {
    let cx = x + ms / 2.0;
    let cy = y + ms / 2.0;
    let r = ms / 2.0;
    if flat_top {
        format!(
            "M{:.3},{:.3}L{:.3},{:.3}L{:.3},{:.3}A{:.3},{:.3} 0 0,1 {:.3},{:.3}Z",
            x,
            y,
            x + ms,
            y,
            x + ms,
            cy,
            r,
            r,
            x,
            cy
        )
    } else if flat_bottom {
        format!(
            "M{:.3},{:.3}A{:.3},{:.3} 0 0,1 {:.3},{:.3}L{:.3},{:.3}L{:.3},{:.3}Z",
            x,
            cy,
            r,
            r,
            x + ms,
            cy,
            x + ms,
            y + ms,
            x,
            y + ms
        )
    } else if flat_left {
        format!(
            "M{:.3},{:.3}L{:.3},{:.3}A{:.3},{:.3} 0 0,1 {:.3},{:.3}L{:.3},{:.3}Z",
            x,
            y,
            cx,
            y,
            r,
            r,
            cx,
            y + ms,
            x,
            y + ms
        )
    } else {
        format!(
            "M{:.3},{:.3}L{:.3},{:.3}L{:.3},{:.3}L{:.3},{:.3}A{:.3},{:.3} 0 0,1 {:.3},{:.3}Z",
            cx,
            y,
            x + ms,
            y,
            x + ms,
            y + ms,
            cx,
            y + ms,
            r,
            r,
            cx,
            y
        )
    }
}

// ============================================================
// BASE64 ENCODER
// ============================================================

pub fn base64_encode(data: &[u8]) -> String {
    const CHARS: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let triple = (b0 << 16) | (b1 << 8) | b2;
        result.push(CHARS[((triple >> 18) & 0x3F) as usize] as char);
        result.push(CHARS[((triple >> 12) & 0x3F) as usize] as char);
        if chunk.len() > 1 {
            result.push(CHARS[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
        if chunk.len() > 2 {
            result.push(CHARS[(triple & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }
    result
}

// ============================================================
// SVG PARSING HELPERS
// ============================================================

/// Extract viewBox width/height from SVG string for dimension calculation
pub fn parse_svg_viewbox(svg: &str) -> Option<(f64, f64)> {
    let start = svg.find("viewBox=\"0 0 ")? + "viewBox=\"0 0 ".len();
    let rest = &svg[start..];
    let end = rest.find('"')?;
    let dims: Vec<f64> = rest[..end]
        .split(' ')
        .filter_map(|s| s.parse().ok())
        .collect();
    if dims.len() >= 2 {
        Some((dims[0], dims[1]))
    } else {
        None
    }
}

pub fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// Process the SVG background from vtracer output.
/// vtracer's Poster preset outputs the background as the first <path> element
/// (a full-image rectangle in the dominant/background color).
/// - bg_color == [0,0,0,0]: no processing (feature disabled / default state)
/// - bg_color alpha == 0 but color != [0,0,0,0]: remove background path (transparent)
/// - bg_color alpha > 0: replace background path's fill with this color
pub fn process_svg_bg(svg_content: &str, bg_color: &Rgba<u8>) -> String {
    // [0,0,0,0] = default/disabled state – no processing
    if bg_color.0 == [0, 0, 0, 0] {
        return svg_content.to_string();
    }

    // Find the first <path element (vtracer outputs background rectangle first)
    let path_start = match svg_content.find("<path") {
        Some(pos) => pos,
        None => return svg_content.to_string(),
    };

    // Find the end of this self-closing path element (vtracer uses <path ... />)
    let after_start = &svg_content[path_start..];
    let path_end_offset = match after_start.find("/>") {
        Some(offset) => offset + 2, // skip past "/>"
        None => return svg_content.to_string(),
    };
    let path_end = path_start + path_end_offset;

    if bg_color.0[3] == 0 {
        // Alpha == 0 (but not [0,0,0,0]): remove background path entirely → transparent
        let mut result = String::with_capacity(svg_content.len());
        result.push_str(&svg_content[..path_start]);
        result.push_str(&svg_content[path_end..]);
        result
    } else {
        // Alpha > 0: replace the fill color of the background path
        let first_path = &svg_content[path_start..path_end];
        let new_fill = format!("fill=\"{}\"", rgba_to_svg(*bg_color));
        if let Some(fill_pos) = first_path.find("fill=\"") {
            let fill_val_start = fill_pos + 6; // after 'fill="'
            if let Some(fill_val_end) = first_path[fill_val_start..].find('"') {
                let abs_fill_start = path_start + fill_pos;
                let abs_fill_end = path_start + fill_val_start + fill_val_end + 1;
                let mut result = String::with_capacity(svg_content.len());
                result.push_str(&svg_content[..abs_fill_start]);
                result.push_str(&new_fill);
                result.push_str(&svg_content[abs_fill_end..]);
                return result;
            }
        }
        // Could not find fill attribute – return unchanged
        svg_content.to_string()
    }
}

// ============================================================
// LOGO TRACING & LOADING
// ============================================================

/// Trace a raster logo image to SVG paths using vtracer.
/// Returns the inner SVG content (paths/shapes) without the outer <svg> wrapper,
/// already scaled and positioned for embedding at the given logo_modules size.
pub fn trace_logo_to_svg_paths(file_bytes: &[u8], _ext: &str, target_size: f64) -> Option<String> {
    // Load image via image crate
    let img = image::load_from_memory(file_bytes).ok()?.to_rgba8();
    let (w, h) = (img.width() as usize, img.height() as usize);

    // Convert to vtracer ColorImage (RGBA bytes)
    let color_image = vtracer::ColorImage {
        pixels: img.into_raw(),
        width: w,
        height: h,
    };

    // Configure tracer using Poster preset for compact logo output
    let mut config = vtracer::Config::from_preset(vtracer::Preset::Poster);
    config.filter_speckle = 4;
    config.color_precision = 6;
    config.layer_difference = 0;
    config.corner_threshold = 60;
    config.length_threshold = 4.0;
    config.splice_threshold = 45;
    config.path_precision = Some(2);

    let svg_file = vtracer::convert(color_image, config).ok()?;
    let svg_string = svg_file.to_string();

    // Extract inner content between <svg> tags
    let content = extract_svg_inner_content(&svg_string)?;

    // Calculate scale factor: original image -> target_size in module units
    let scale_x = target_size / w as f64;
    let scale_y = target_size / h as f64;
    let scale = scale_x.min(scale_y);

    Some(format!(
        "<g transform=\"scale({:.4})\">\n{}\n</g>",
        scale, content
    ))
}

/// Extract the inner content of an SVG file (between <svg> and </svg> tags).
/// Strips the outer wrapper for direct embedding.
pub fn extract_svg_inner_content(svg: &str) -> Option<String> {
    // Find the opening <svg ...> tag end
    let svg_start = svg.find("<svg")?;
    let after_tag = &svg[svg_start..];
    let bracket_pos = after_tag.find('>')?;
    let start = svg_start + bracket_pos + 1;
    // Find the closing </svg> tag start
    let end = svg.rfind("</svg>")?;
    if start < end {
        Some(svg[start..end].trim().to_string())
    } else {
        None
    }
}

/// Load an SVG logo file and extract its inner content for direct vector embedding.
/// Returns SVG content scaled to fit within target_size x target_size.
pub fn load_svg_logo_content(file_bytes: &[u8], target_size: f64) -> Option<String> {
    let svg_string = String::from_utf8_lossy(file_bytes).to_string();
    let content = extract_svg_inner_content(&svg_string)?;

    // Try to parse viewBox to determine original dimensions
    let (orig_w, orig_h) = parse_svg_viewbox(&svg_string).unwrap_or((100.0, 100.0));
    let scale = target_size / orig_w.max(orig_h);

    Some(format!(
        "<g transform=\"scale({:.4})\">\n{}\n</g>",
        scale, content
    ))
}

// ============================================================
// RASTERIZATION
// ============================================================

/// Rasterize an SVG string to an RgbaImage using gdk-pixbuf + system librsvg
pub fn rasterize_svg(svg: &str, width: u32, height: u32) -> Option<RgbaImage> {
    if width == 0 || height == 0 {
        return None;
    }

    // Write SVG to a temporary file
    let temp_path = std::env::temp_dir().join("qr_studio_rasterize.svg");
    std::fs::write(&temp_path, svg.as_bytes()).ok()?;

    // Load via gdk-pixbuf (uses system librsvg for SVG decoding)
    let pixbuf = gdk_pixbuf::Pixbuf::from_file_at_scale(
        &temp_path,
        width as i32,
        height as i32,
        false, // don't preserve aspect ratio — force exact dimensions
    )
    .ok()?;

    // Cleanup temp file after pixbuf has read it
    let _ = std::fs::remove_file(&temp_path);

    // Convert pixbuf pixels → image::RgbaImage
    let w = pixbuf.width() as u32;
    let h = pixbuf.height() as u32;
    if w == 0 || h == 0 {
        return None;
    }
    let n_channels = pixbuf.n_channels() as usize;
    let rowstride = pixbuf.rowstride() as usize;
    let has_alpha = pixbuf.has_alpha();
    let data = unsafe { pixbuf.pixels() };

    let mut rgba = Vec::with_capacity((w * h * 4) as usize);
    for y in 0..h {
        for x in 0..w {
            let offset = y as usize * rowstride + x as usize * n_channels;
            if offset + 2 < data.len() {
                rgba.push(data[offset]); // R
                rgba.push(data[offset + 1]); // G
                rgba.push(data[offset + 2]); // B
                rgba.push(if has_alpha && n_channels >= 4 && offset + 3 < data.len() {
                    data[offset + 3] // A (from pixbuf)
                } else {
                    255 // opaque
                });
            }
        }
    }

    RgbaImage::from_raw(w, h, rgba)
}

// ============================================================
// MAIN SVG RENDER FUNCTION
// ============================================================

#[allow(clippy::too_many_arguments)]
pub fn render_vector_svg(
    data: &str,
    dot_style: DotStyle,
    corner_square_style: CornerSquareStyle,
    corner_dot_style: CornerDotStyle,
    fg_color: Rgba<u8>,
    bg_color: Rgba<u8>,
    corner_color: Rgba<u8>,
    ec_level: ErrorCorrectionLevel,
    transparent_bg: bool,
    gradient_enabled: bool,
    gradient_color: Rgba<u8>,
    gradient_direction: GradientDirection,
    logo_path: Option<&PathBuf>,
    logo_size: f64,
    outer_text_top: &str,
    outer_text_bottom: &str,
    outer_text_color: Rgba<u8>,
    module_size: u32,
    logo_shape: LogoShape,
    quiet_zone: u32,
    module_gap: f64,
    frame_style: FrameStyle,
    frame_color: Rgba<u8>,
    shadow_enabled: bool,
    shadow_offset: f64,
    frame_width: u32,
    frame_outer_radius: f64,
    _frame_inner_radius: f64,
    logo_color: Rgba<u8>,
    logo_border_width: f64,
    logo_border_color: Rgba<u8>,
    bg_image_path: Option<&PathBuf>,
    logo_vectorize: bool,
    logo_vectorize_bg_color: Rgba<u8>,
    logo_bg_transparent: bool,
    logo_clear_area: bool,
    logo_clear_padding: f64,
    logo_outer_radius: f64,
    logo_inner_radius: f64,
    gradient_phase: f64,
    custom_dot_path: &str,
    outer_text_font: &str,
    outer_text_font_size: u32,
) -> Option<String> {
    let qr =
        qrcode::QrCode::with_error_correction_level(data.as_bytes(), ec_level_to_qrcode(ec_level))
            .ok()?;
    let qr_width = qr.width();
    let total = qr_width as u32 + quiet_zone * 2;
    let frame_units: f64 = if frame_style != FrameStyle::None {
        frame_width.max(1) as f64
    } else {
        0.0
    };
    let frame_banner_units: f64 = if frame_style == FrameStyle::Banner {
        if !outer_text_bottom.is_empty() {
            4.0
        } else {
            2.0
        }
    } else {
        0.0
    };
    let top_units: f64 = if !outer_text_top.is_empty() { 5.0 } else { 0.0 };
    let bottom_units: f64 = if !outer_text_bottom.is_empty() && frame_style != FrameStyle::Banner {
        5.0
    } else {
        0.0
    };
    let full_w = total as f64 + frame_units * 2.0;
    let full_h = total as f64 + top_units + bottom_units + frame_units * 2.0 + frame_banner_units;

    let mut svg = String::new();
    svg.push_str(&format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" viewBox=\"0 0 {} {}\" width=\"{}\" height=\"{}\">\n",
        full_w, full_h, (full_w as u32) * 10, (full_h as u32) * 10
    ));

    // ── Defs ──
    svg.push_str("<defs>\n");

    if gradient_enabled {
        let fg = rgba_to_svg(fg_color);
        let gc = rgba_to_svg(gradient_color);
        let t = total as f64;
        match gradient_direction {
            GradientDirection::Horizontal => {
                svg.push_str(&format!(
                    "<linearGradient id=\"grad\" x1=\"{:.2}\" y1=\"0\" x2=\"{:.2}\" y2=\"0\" gradientUnits=\"userSpaceOnUse\">\n",
                    -gradient_phase * t, (1.0 - gradient_phase) * t
                ));
                svg.push_str(&format!(
                    "<stop offset=\"0%\" stop-color=\"{}\"/>\n<stop offset=\"100%\" stop-color=\"{}\"/>\n</linearGradient>\n",
                    fg, gc
                ));
            }
            GradientDirection::Vertical => {
                svg.push_str(&format!(
                    "<linearGradient id=\"grad\" x1=\"0\" y1=\"{:.2}\" x2=\"0\" y2=\"{:.2}\" gradientUnits=\"userSpaceOnUse\">\n",
                    -gradient_phase * t, (1.0 - gradient_phase) * t
                ));
                svg.push_str(&format!(
                    "<stop offset=\"0%\" stop-color=\"{}\"/>\n<stop offset=\"100%\" stop-color=\"{}\"/>\n</linearGradient>\n",
                    fg, gc
                ));
            }
            GradientDirection::Diagonal => {
                svg.push_str(&format!(
                    "<linearGradient id=\"grad\" x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" gradientUnits=\"userSpaceOnUse\">\n",
                    -gradient_phase * t, -gradient_phase * t, (1.0 - gradient_phase) * t, (1.0 - gradient_phase) * t
                ));
                svg.push_str(&format!(
                    "<stop offset=\"0%\" stop-color=\"{}\"/>\n<stop offset=\"100%\" stop-color=\"{}\"/>\n</linearGradient>\n",
                    fg, gc
                ));
            }
            GradientDirection::Radial => {
                svg.push_str(&format!(
                    "<radialGradient id=\"grad\" cx=\"{:.2}\" cy=\"{:.2}\" r=\"{:.2}\" gradientUnits=\"userSpaceOnUse\">\n",
                    t / 2.0, t / 2.0, t / 2.0
                ));
                svg.push_str(&format!(
                    "<stop offset=\"0%\" stop-color=\"{}\"/>\n<stop offset=\"100%\" stop-color=\"{}\"/>\n</radialGradient>\n",
                    fg, gc
                ));
            }
        }
    }

    if shadow_enabled {
        svg.push_str(&format!(
            "<filter id=\"shadow\"><feDropShadow dx=\"{:.1}\" dy=\"{:.1}\" stdDeviation=\"0.3\" flood-opacity=\"0.25\"/></filter>\n",
            shadow_offset, shadow_offset
        ));
    }

    // Logo clip path (shape-aware)
    if logo_path.is_some() && logo_size > 0.0 {
        let logo_modules = total as f64 * logo_size;
        let cx = total as f64 / 2.0;
        let cy = total as f64 / 2.0;
        svg.push_str("<clipPath id=\"logo-clip\">\n");
        match logo_shape {
            LogoShape::Rectangle => {
                svg.push_str(&format!(
                    "<rect x=\"{:.3}\" y=\"{:.3}\" width=\"{:.3}\" height=\"{:.3}\"/>\n",
                    cx - logo_modules / 2.0,
                    cy - logo_modules / 2.0,
                    logo_modules,
                    logo_modules
                ));
            }
            LogoShape::Circle => {
                svg.push_str(&format!(
                    "<circle cx=\"{:.3}\" cy=\"{:.3}\" r=\"{:.3}\"/>\n",
                    cx,
                    cy,
                    logo_modules / 2.0
                ));
            }
            LogoShape::RoundedRect => {
                // Match clip radius to border inner radius for concentric corners
                let r = if logo_border_width > 0.0 {
                    let ms_f = if module_size > 0 {
                        module_size as f64
                    } else {
                        8.0
                    };
                    let pad = 0.5 + logo_border_width / ms_f;
                    let border_size = logo_modules + pad * 2.0;
                    let r_outer = border_size * logo_outer_radius;
                    (r_outer - pad).clamp(0.0, logo_modules / 2.0)
                } else {
                    logo_modules * logo_inner_radius
                };
                svg.push_str(&format!(
                    "<rect x=\"{:.3}\" y=\"{:.3}\" width=\"{:.3}\" height=\"{:.3}\" rx=\"{:.3}\" ry=\"{:.3}\"/>\n",
                    cx - logo_modules / 2.0,
                    cy - logo_modules / 2.0,
                    logo_modules,
                    logo_modules,
                    r,
                    r
                ));
            }
        }
        svg.push_str("</clipPath>\n");

        // Logo tint filter
        if logo_color.0[3] > 0 {
            let tint_a = logo_color.0[3] as f64 / 255.0;
            let slope = 1.0 - tint_a;
            let ir = logo_color.0[0] as f64 / 255.0 * tint_a;
            let ig = logo_color.0[1] as f64 / 255.0 * tint_a;
            let ib = logo_color.0[2] as f64 / 255.0 * tint_a;
            svg.push_str(&format!(
                "<filter id=\"logo-tint\" color-interpolation-filters=\"sRGB\"><feComponentTransfer><feFuncR type=\"linear\" slope=\"{:.3}\" intercept=\"{:.3}\"/><feFuncG type=\"linear\" slope=\"{:.3}\" intercept=\"{:.3}\"/><feFuncB type=\"linear\" slope=\"{:.3}\" intercept=\"{:.3}\"/></feComponentTransfer></filter>\n",
                slope, ir, slope, ig, slope, ib
            ));
        }
    }

    svg.push_str("</defs>\n");

    // ── Frame ──
    if frame_style != FrameStyle::None {
        let fc = rgba_to_svg(frame_color);
        match frame_style {
            FrameStyle::Simple => {
                svg.push_str(&format!(
                    "<rect x=\"0\" y=\"0\" width=\"{:.1}\" height=\"{:.1}\" fill=\"{}\"/>\n",
                    full_w, frame_units, fc
                ));
                svg.push_str(&format!(
                    "<rect x=\"0\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"{}\"/>\n",
                    full_h - frame_units,
                    full_w,
                    frame_units,
                    fc
                ));
                svg.push_str(&format!(
                    "<rect x=\"0\" y=\"0\" width=\"{:.1}\" height=\"{:.1}\" fill=\"{}\"/>\n",
                    frame_units, full_h, fc
                ));
                svg.push_str(&format!(
                    "<rect x=\"{:.1}\" y=\"0\" width=\"{:.1}\" height=\"{:.1}\" fill=\"{}\"/>\n",
                    full_w - frame_units,
                    frame_units,
                    full_h,
                    fc
                ));
            }
            FrameStyle::Rounded => {
                let r_outer = (frame_outer_radius * full_w.min(full_h))
                    .min(full_w / 2.0)
                    .min(full_h / 2.0);
                let inner_x = frame_units;
                let inner_y = frame_units + top_units;
                let inner_w = total as f64;
                let inner_h = total as f64;
                // Derive inner radius from outer for concentric arcs → constant frame thickness
                let r_inner = (r_outer - frame_units)
                    .clamp(0.0, inner_w / 2.0)
                    .min(inner_h / 2.0);
                let outer_path = svg_selective_rounded_rect_path(
                    0.0, 0.0, full_w, full_h, r_outer, true, true, true, true,
                );
                let inner_path = svg_selective_rounded_rect_path(
                    inner_x, inner_y, inner_w, inner_h, r_inner, true, true, true, true,
                );
                svg.push_str(&format!(
                    "<path d=\"{} {}\" fill-rule=\"evenodd\" fill=\"{}\"/>\n",
                    outer_path, inner_path, fc
                ));
            }

            FrameStyle::Banner => {
                svg.push_str(&format!(
                    "<rect x=\"0\" y=\"0\" width=\"{:.1}\" height=\"{:.1}\" fill=\"{}\"/>\n",
                    full_w, frame_units, fc
                ));
                svg.push_str(&format!(
                    "<rect x=\"0\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"{}\"/>\n",
                    full_h - frame_units - frame_banner_units,
                    full_w,
                    frame_units + frame_banner_units,
                    fc
                ));
                svg.push_str(&format!(
                    "<rect x=\"0\" y=\"0\" width=\"{:.1}\" height=\"{:.1}\" fill=\"{}\"/>\n",
                    frame_units, full_h, fc
                ));
                svg.push_str(&format!(
                    "<rect x=\"{:.1}\" y=\"0\" width=\"{:.1}\" height=\"{:.1}\" fill=\"{}\"/>\n",
                    full_w - frame_units,
                    frame_units,
                    full_h,
                    fc
                ));
            }
            FrameStyle::None => {}
        }
    }

    // ── Background ──
    if !transparent_bg {
        let bg_fill = rgba_to_svg(bg_color);
        let bg_x = frame_units as f64;
        let bg_y = frame_units as f64 + top_units;
        let bg_w = total as f64;
        let bg_h = total as f64;
        if frame_width > 0 && matches!(frame_style, FrameStyle::Rounded) {
            // Use same concentric inner radius as the frame for consistent corners
            let r_outer_bg = (frame_outer_radius * full_w.min(full_h))
                .min(full_w / 2.0)
                .min(full_h / 2.0);
            let r_inner = (r_outer_bg - frame_units as f64)
                .clamp(0.0, bg_w / 2.0)
                .min(bg_h / 2.0);
            let bg_path = svg_selective_rounded_rect_path(
                bg_x, bg_y, bg_w, bg_h, r_inner, true, true, true, true,
            );
            svg.push_str(&format!("<path d=\"{}\" fill=\"{}\"/>\n", bg_path, bg_fill));
        } else {
            svg.push_str(&format!(
                "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"{}\"/>\n",
                bg_x, bg_y, bg_w, bg_h, bg_fill
            ));
        }
    }

    // Background image
    if let Some(bg_img) = bg_image_path {
        if let Ok(file_bytes) = std::fs::read(bg_img) {
            let ext = bg_img.extension().and_then(|e| e.to_str()).unwrap_or("png");
            let mime = match ext {
                "jpg" | "jpeg" => "image/jpeg",
                "gif" => "image/gif",
                "svg" => "image/svg+xml",
                _ => "image/png",
            };
            let b64 = base64_encode(&file_bytes);
            // Clip image to rounded frame shape when applicable
            let clip_attr = if frame_width > 0 && matches!(frame_style, FrameStyle::Rounded) {
                let bg_x = frame_units as f64;
                let bg_y = frame_units as f64 + top_units;
                let bg_w = total as f64;
                let bg_h = total as f64;
                let r_outer_bg = (frame_outer_radius * full_w.min(full_h))
                    .min(full_w / 2.0)
                    .min(full_h / 2.0);
                let r_inner = (r_outer_bg - frame_units as f64)
                    .clamp(0.0, bg_w / 2.0)
                    .min(bg_h / 2.0);
                let clip_path = svg_selective_rounded_rect_path(
                    bg_x, bg_y, bg_w, bg_h, r_inner, true, true, true, true,
                );
                svg.push_str(&format!(
                    "<defs><clipPath id=\"bg-img-clip\"><path d=\"{}\"/></clipPath></defs>\n",
                    clip_path
                ));
                " clip-path=\"url(#bg-img-clip)\"".to_string()
            } else {
                String::new()
            };
            svg.push_str(&format!(
                "<image{} href=\"data:{};base64,{}\" x=\"{:.1}\" y=\"{:.1}\" width=\"{}\" height=\"{}\" opacity=\"0.3\" preserveAspectRatio=\"xMidYMid slice\"/>\n",
                clip_attr, mime, b64, frame_units, frame_units + top_units, total, total
            ));
        }
    }

    // ── Top text ──
    if !outer_text_top.is_empty() {
        let text_color = rgba_to_svg(outer_text_color);
        let svg_font_size = 3.5 * (outer_text_font_size as f64 / 14.0);
        svg.push_str(&format!("<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" dominant-baseline=\"central\" font-family=\"{}\" font-size=\"{:.1}\" fill=\"{}\">{}</text>\n", full_w / 2.0, frame_units + top_units / 2.0, outer_text_font, svg_font_size, text_color, xml_escape(outer_text_top)));
    }

    // ── QR modules group ──
    svg.push_str(&format!(
        "<g transform=\"translate({:.1},{:.1})\">\n",
        frame_units,
        frame_units + top_units
    ));

    let data_fill = if gradient_enabled {
        "url(#grad)".to_string()
    } else {
        rgba_to_svg(fg_color)
    };
    let corner_fill = if gradient_enabled {
        "url(#grad)".to_string()
    } else {
        rgba_to_svg(corner_color)
    };
    let m = quiet_zone as f64;

    // Module gap helpers (data modules only)
    let gap = module_gap;
    let ms = 1.0 - gap;
    let go = gap / 2.0;

    // Logo clear area — shape-aware "editorial reflow" effect
    // Modules inside the logo SHAPE are skipped, creating an organic
    // "flow around" effect (e.g. circular gap for circle logos) instead
    // of a simple rectangular bounding box.
    let logo_mods_clear = total as f64 * logo_size;
    let border_pad = if logo_border_width > 0.0 {
        logo_border_width
    } else {
        0.5 // default padding when no border
    };
    let center_clear = qr_width as f64 / 2.0;
    let half_clear = logo_mods_clear / 2.0;
    let clear_enabled = logo_clear_area && logo_path.is_some() && logo_size > 0.0;
    let in_clear_area = |x: usize, y: usize| -> bool {
        if !clear_enabled {
            return false;
        }
        // Module center (offset 0.5 for center of module cell)
        let mx = x as f64 + 0.5;
        let my = y as f64 + 0.5;
        match logo_shape {
            LogoShape::Circle => {
                let dx = mx - center_clear;
                let dy = my - center_clear;
                let dist = (dx * dx + dy * dy).sqrt();
                dist <= half_clear + border_pad + logo_clear_padding
            }
            LogoShape::RoundedRect => {
                let rr = logo_mods_clear * 0.15; // corner radius (matches SVG)
                let total_pad = border_pad + logo_clear_padding;
                let x1 = center_clear - half_clear - total_pad;
                let y1 = center_clear - half_clear - total_pad;
                let x2 = center_clear + half_clear + total_pad;
                let y2 = center_clear + half_clear + total_pad;
                // Quick reject: outside bounding box
                if mx < x1 || mx > x2 || my < y1 || my > y2 {
                    return false;
                }
                // Corner zone check (quarter-circles at corners)
                let rr_exp = rr + total_pad;
                let cx_left = x1 + rr_exp;
                let cx_right = x2 - rr_exp;
                let cy_top = y1 + rr_exp;
                let cy_bottom = y2 - rr_exp;
                if mx < cx_left && my < cy_top {
                    let dx = mx - cx_left;
                    let dy = my - cy_top;
                    return (dx * dx + dy * dy) <= rr_exp * rr_exp;
                }
                if mx > cx_right && my < cy_top {
                    let dx = mx - cx_right;
                    let dy = my - cy_top;
                    return (dx * dx + dy * dy) <= rr_exp * rr_exp;
                }
                if mx < cx_left && my > cy_bottom {
                    let dx = mx - cx_left;
                    let dy = my - cy_bottom;
                    return (dx * dx + dy * dy) <= rr_exp * rr_exp;
                }
                if mx > cx_right && my > cy_bottom {
                    let dx = mx - cx_right;
                    let dy = my - cy_bottom;
                    return (dx * dx + dy * dy) <= rr_exp * rr_exp;
                }
                true // In a straight zone
            }
            LogoShape::Rectangle => {
                let total_pad = border_pad + logo_clear_padding;
                let x1 = center_clear - half_clear - total_pad;
                let y1 = center_clear - half_clear - total_pad;
                let x2 = center_clear + half_clear + total_pad;
                let y2 = center_clear + half_clear + total_pad;
                mx >= x1 && mx <= x2 && my >= y1 && my <= y2
            }
        }
    };

    // Shadow group
    if shadow_enabled {
        svg.push_str("<g filter=\"url(#shadow)\">\n");
    }

    // Corner squares (NO gap)
    let corner_positions: [(usize, usize); 3] = [(0, 0), (qr_width - 7, 0), (0, qr_width - 7)];
    for (sx, sy) in &corner_positions {
        let px = *sx as f64 + m;
        let py = *sy as f64 + m;
        match corner_square_style {
            // All non-Dot styles use compound path with evenodd:
            // outer shape minus inner shape → transparent center, no bg fill needed
            CornerSquareStyle::Square => {
                // Sharp rect outer (7×7, radius 0) + inner (5×5 at +1, radius 0)
                let outer_path =
                    svg_selective_rounded_rect_path(px, py, 7.0, 7.0, 0.0, true, true, true, true);
                let inner_path = svg_selective_rounded_rect_path(
                    px + 1.0,
                    py + 1.0,
                    5.0,
                    5.0,
                    0.0,
                    true,
                    true,
                    true,
                    true,
                );
                svg.push_str(&format!(
                    "<path d=\"{} {}\" fill=\"{}\" fill-rule=\"evenodd\"/>\n",
                    outer_path, inner_path, corner_fill
                ));
            }
            CornerSquareStyle::ExtraRounded => {
                // Rounded rect outer (7×7, radius 2.0) + inner (5×5 at +1, radius 1.0)
                // Inner radius = outer_radius(2.0) - offset(1.0) = 1.0
                // → concentric arcs, constant line thickness everywhere
                let outer_path =
                    svg_selective_rounded_rect_path(px, py, 7.0, 7.0, 2.0, true, true, true, true);
                let inner_path = svg_selective_rounded_rect_path(
                    px + 1.0,
                    py + 1.0,
                    5.0,
                    5.0,
                    1.0,
                    true,
                    true,
                    true,
                    true,
                );
                svg.push_str(&format!(
                    "<path d=\"{} {}\" fill=\"{}\" fill-rule=\"evenodd\"/>\n",
                    outer_path, inner_path, corner_fill
                ));
            }
            CornerSquareStyle::Dot => {
                // Per-module rendering — individual circles create the dotted look
                for dy in 0..7usize {
                    for dx in 0..7usize {
                        if dx >= 2 && dx <= 4 && dy >= 2 && dy <= 4 {
                            continue;
                        }
                        let qx = sx + dx;
                        let qy = sy + dy;
                        if qr[(qx, qy)] == qrcode::types::Color::Dark {
                            svg.push_str(&format!(
                                "<circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"0.45\" fill=\"{}\"/>\n",
                                px + dx as f64 + 0.5,
                                py + dy as f64 + 0.5,
                                corner_fill
                            ));
                        }
                    }
                }
            }
            CornerSquareStyle::Circle => {
                // Circle outer (r=3.0) + inner (r=2.0) → ring via evenodd
                let cx = px + 3.5;
                let cy = py + 3.5;
                svg.push_str(&format!(
                    "<path d=\"M{cx:.2} {cy:.2}m-3.0 0a3.0 3.0 0 1 0 6.0 0a3.0 3.0 0 1 0-6.0 0Z \
                     M{cx:.2} {cy:.2}m-2.0 0a2.0 2.0 0 1 0 4.0 0a2.0 2.0 0 1 0-4.0 0Z\" \
                     fill=\"{}\" fill-rule=\"evenodd\"/>\n",
                    corner_fill,
                    cx = cx,
                    cy = cy
                ));
            }
        }
    }

    // Corner dots (NO gap)
    let dot_positions: [(usize, usize); 3] = [(2, 2), (qr_width - 5, 2), (2, qr_width - 5)];
    for (sx, sy) in &dot_positions {
        let px = *sx as f64 + m;
        let py = *sy as f64 + m;
        match corner_dot_style {
            CornerDotStyle::Square => {
                svg.push_str(&format!(
                    "<rect x=\"{:.2}\" y=\"{:.2}\" width=\"3\" height=\"3\" fill=\"{}\"/>\n",
                    px, py, corner_fill
                ));
            }
            CornerDotStyle::Dot => {
                svg.push_str(&format!(
                    "<circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"1.35\" fill=\"{}\"/>\n",
                    px + 1.5,
                    py + 1.5,
                    corner_fill
                ));
            }
            CornerDotStyle::Circle => {
                svg.push_str(&format!("<circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"1.1\" fill=\"none\" stroke=\"{}\" stroke-width=\"0.7\"/>\n", px + 1.5, py + 1.5, corner_fill));
            }
            CornerDotStyle::ExtraRounded => {
                let path =
                    svg_selective_rounded_rect_path(px, py, 3.0, 3.0, 1.2, true, true, true, true);
                svg.push_str(&format!(
                    "<path d=\"{}\" fill=\"{}\"/>\n",
                    path, corner_fill
                ));
            }
        }
    }

    // Data modules (WITH gap)
    for y in 0..qr_width {
        for x in 0..qr_width {
            if qr[(x, y)] == qrcode::types::Color::Dark {
                let mod_type = get_module_type(x, y, qr_width);
                if mod_type != ModuleType::Data {
                    continue;
                }
                // Skip modules in the logo clear area
                if in_clear_area(x, y) {
                    continue;
                }
                let px = x as f64 + m;
                let py = y as f64 + m;
                let left = is_dark(&qr, x as i32 - 1, y as i32, qr_width);
                let right = is_dark(&qr, x as i32 + 1, y as i32, qr_width);
                let top_n = is_dark(&qr, x as i32, y as i32 - 1, qr_width);
                let bottom_n = is_dark(&qr, x as i32, y as i32 + 1, qr_width);
                let sum = left as u8 + right as u8 + top_n as u8 + bottom_n as u8;

                let fill = data_fill.clone();

                match dot_style {
                    DotStyle::Square => {
                        svg.push_str(&format!(
                            "<rect x=\"{:.3}\" y=\"{:.3}\" width=\"{:.3}\" height=\"{:.3}\" fill=\"{}\"/>\n",
                            px + go, py + go, ms, ms, fill
                        ));
                    }
                    DotStyle::Dots => {
                        svg.push_str(&format!(
                            "<circle cx=\"{:.3}\" cy=\"{:.3}\" r=\"{:.3}\" fill=\"{}\"/>\n",
                            px + 0.5,
                            py + 0.5,
                            0.45 * ms,
                            fill
                        ));
                    }
                    DotStyle::Rounded => {
                        if sum == 0 {
                            svg.push_str(&format!(
                                "<circle cx=\"{:.3}\" cy=\"{:.3}\" r=\"{:.3}\" fill=\"{}\"/>\n",
                                px + 0.5,
                                py + 0.5,
                                0.45 * ms,
                                fill
                            ));
                        } else if sum > 2 || (left && right) || (top_n && bottom_n) {
                            svg.push_str(&format!(
                                "<rect x=\"{:.3}\" y=\"{:.3}\" width=\"{:.3}\" height=\"{:.3}\" fill=\"{}\"/>\n",
                                px + go, py + go, ms, ms, fill
                            ));
                        } else if sum == 2 {
                            let (rtl, rtr, rbl, rbr) = if left && top_n {
                                (false, false, false, true)
                            } else if top_n && right {
                                (false, false, true, false)
                            } else if right && bottom_n {
                                (true, false, false, false)
                            } else {
                                (false, true, false, false)
                            };
                            let r = 0.5 * ms;
                            let path = svg_selective_rounded_rect_path(
                                px + go,
                                py + go,
                                ms,
                                ms,
                                r,
                                rtl,
                                rtr,
                                rbl,
                                rbr,
                            );
                            svg.push_str(&format!("<path d=\"{}\" fill=\"{}\"/>\n", path, fill));
                        } else {
                            let (fl, fr, ft, fb) = if top_n {
                                (false, false, true, false)
                            } else if right {
                                (false, true, false, false)
                            } else if bottom_n {
                                (false, false, false, true)
                            } else {
                                (true, false, false, false)
                            };
                            let path = svg_side_rounded_path(px + go, py + go, ms, fl, fr, ft, fb);
                            svg.push_str(&format!("<path d=\"{}\" fill=\"{}\"/>\n", path, fill));
                        }
                    }
                    DotStyle::Diamond => {
                        let cx = px + go + ms * 0.5;
                        let cy = py + go + ms * 0.5;
                        let s = 0.45 * ms;
                        svg.push_str(&format!(
                            "<polygon points=\"{:.3},{:.3} {:.3},{:.3} {:.3},{:.3} {:.3},{:.3}\" fill=\"{}\"/>\n",
                            cx, cy - s,
                            cx + s, cy,
                            cx, cy + s,
                            cx - s, cy,
                            fill
                        ));
                    }
                    DotStyle::Custom => {
                        let scale = (1.0 - module_gap).max(0.1);
                        let offset = (1.0 - scale) / 2.0;
                        svg.push_str(&format!(
                            "<g transform=\"translate({:.3},{:.3})\"><path d=\"{}\" transform=\"scale({:.3})\" fill=\"{}\"/></g>\n",
                            x as f64 + offset + quiet_zone as f64,
                            y as f64 + offset + quiet_zone as f64,
                            custom_dot_path,
                            scale,
                            fill
                        ));
                    }
                }
            }
        }
    }

    if shadow_enabled {
        svg.push_str("</g>\n");
    }

    // ── Logo (shape-aware, with border and tint) ──
    if let Some(path) = logo_path {
        if let Ok(file_bytes) = std::fs::read(path) {
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("png");
            let mime = match ext {
                "jpg" | "jpeg" => "image/jpeg",
                "gif" => "image/gif",
                "svg" => "image/svg+xml",
                _ => "image/png",
            };
            let b64 = base64_encode(&file_bytes);
            let logo_modules = total as f64 * logo_size;
            let cx = total as f64 / 2.0;
            let cy = total as f64 / 2.0;
            let lx = cx - logo_modules / 2.0;
            let ly = cy - logo_modules / 2.0;

            let ms_f = if module_size > 0 {
                module_size as f64
            } else {
                8.0
            };
            let border_mod = logo_border_width / ms_f;
            let pad = 0.5 + border_mod;

            let bg_fill = rgba_to_svg(bg_color);

            // Logo border (shape-aware, drawn behind logo)
            if logo_border_width > 0.0 {
                let border_color = rgba_to_svg(logo_border_color);
                let border_size = logo_modules + pad * 2.0;
                let bx = cx - border_size / 2.0;
                let by = cy - border_size / 2.0;
                match logo_shape {
                    LogoShape::Rectangle => {
                        // Compound path: outer rect - inner rect → ring with transparent center
                        let outer_path = svg_selective_rounded_rect_path(
                            bx,
                            by,
                            border_size,
                            border_size,
                            0.0,
                            true,
                            true,
                            true,
                            true,
                        );
                        let inner_path = svg_selective_rounded_rect_path(
                            cx - logo_modules / 2.0,
                            cy - logo_modules / 2.0,
                            logo_modules,
                            logo_modules,
                            0.0,
                            true,
                            true,
                            true,
                            true,
                        );
                        svg.push_str(&format!(
                            "<path d=\"{} {}\" fill-rule=\"evenodd\" fill=\"{}\"/>\n",
                            outer_path, inner_path, border_color
                        ));
                    }
                    LogoShape::Circle => {
                        // Compound path: outer circle - inner circle → ring with transparent center
                        let r_outer = border_size / 2.0;
                        let r_inner = logo_modules / 2.0;
                        svg.push_str(&format!(
                            "<path d=\"M{cx:.3} {cy:.3}m-{ro:.3} 0a{ro:.3} {ro:.3} 0 1 0 {ro2:.3} 0a{ro:.3} {ro:.3} 0 1 0-{ro2:.3} 0Z \
                             M{cx:.3} {cy:.3}m-{ri:.3} 0a{ri:.3} {ri:.3} 0 1 0 {ri2:.3} 0a{ri:.3} {ri:.3} 0 1 0-{ri2:.3} 0Z\" \
                             fill-rule=\"evenodd\" fill=\"{}\"/>\n",
                            border_color,
                            cx = cx, cy = cy,
                            ro = r_outer, ro2 = r_outer * 2.0,
                            ri = r_inner, ri2 = r_inner * 2.0,
                        ));
                    }
                    LogoShape::RoundedRect => {
                        let r_outer = border_size * logo_outer_radius;
                        // Derive inner radius from outer for concentric arcs → constant border thickness
                        let r_inner = (r_outer - pad).clamp(0.0, logo_modules / 2.0);
                        let outer_path = svg_selective_rounded_rect_path(
                            bx,
                            by,
                            border_size,
                            border_size,
                            r_outer,
                            true,
                            true,
                            true,
                            true,
                        );
                        let ix = cx - logo_modules / 2.0;
                        let iy = cy - logo_modules / 2.0;
                        let inner_path = svg_selective_rounded_rect_path(
                            ix,
                            iy,
                            logo_modules,
                            logo_modules,
                            r_inner,
                            true,
                            true,
                            true,
                            true,
                        );
                        svg.push_str(&format!(
                            "<path d=\"{} {}\" fill-rule=\"evenodd\" fill=\"{}\"/>\n",
                            outer_path, inner_path, border_color
                        ));
                    }
                }
            }

            // Logo background + image (clipped to shape)
            let bg_size = logo_modules + pad * 2.0;
            let bg_x = cx - bg_size / 2.0;
            let bg_y = cy - bg_size / 2.0;
            svg.push_str("<g clip-path=\"url(#logo-clip)\">\n");
            // Determine effective background fill for the logo area:
            // 1. If logo_bg_transparent is checked → always transparent
            // 2. If logo_vectorize_bg_color is set → use it
            // 3. Otherwise transparent when QR bg is transparent
            let logo_area_bg = if logo_bg_transparent {
                "none".to_string()
            } else if logo_vectorize_bg_color.0 != [0, 0, 0, 0] {
                if logo_vectorize_bg_color.0[3] == 0 {
                    "none".to_string()
                } else {
                    rgba_to_svg(logo_vectorize_bg_color)
                }
            } else if transparent_bg {
                "none".to_string()
            } else {
                bg_fill.clone()
            };
            // Use rounded rect for background when RoundedRect to fill corners properly
            if matches!(logo_shape, LogoShape::RoundedRect) && logo_border_width > 0.0 {
                let r_outer = bg_size * logo_outer_radius;
                let bg_path = svg_selective_rounded_rect_path(
                    bg_x, bg_y, bg_size, bg_size, r_outer, true, true, true, true,
                );
                svg.push_str(&format!(
                    "<path d=\"{}\" fill=\"{}\"/>\n",
                    bg_path, logo_area_bg
                ));
            } else {
                svg.push_str(&format!(
                    "<rect x=\"{:.3}\" y=\"{:.3}\" width=\"{:.3}\" height=\"{:.3}\" fill=\"{}\"/>\n",
                    bg_x, bg_y, bg_size, bg_size, logo_area_bg
                ));
            }

            if logo_vectorize || ext == "svg" {
                // ── Vectorized logo embedding (no base64) ──
                let tint_open = if logo_color.0[3] > 0 {
                    "<g filter=\"url(#logo-tint)\">\n"
                } else {
                    ""
                };
                let tint_close = if logo_color.0[3] > 0 { "</g>\n" } else { "" };

                let vector_content = if ext == "svg" {
                    // SVG logo: extract inner content directly
                    load_svg_logo_content(&file_bytes, logo_modules)
                } else {
                    // Raster logo: trace to vector paths via vtracer
                    trace_logo_to_svg_paths(&file_bytes, ext, logo_modules)
                }
                .map(|svg| process_svg_bg(&svg, &logo_vectorize_bg_color));

                if let Some(content) = vector_content {
                    svg.push_str(&format!(
                        "{}<g transform=\"translate({:.3},{:.3})\">\n{}\n</g>\n{}",
                        tint_open, lx, ly, content, tint_close
                    ));
                }
            } else {
                // ── Original base64 raster embedding ──
                let filter_attr = if logo_color.0[3] > 0 {
                    " filter=\"url(#logo-tint)\""
                } else {
                    ""
                };
                svg.push_str(&format!(
                    "<image href=\"data:{};base64,{}\" x=\"{:.3}\" y=\"{:.3}\" width=\"{:.3}\" height=\"{:.3}\"{} preserveAspectRatio=\"xMidYMid meet\"/>\n",
                    mime, b64, lx, ly, logo_modules, logo_modules, filter_attr
                ));
            }
            svg.push_str("</g>\n");
        }
    }

    svg.push_str("</g>\n"); // end QR modules group

    // ── Bottom text ──
    if !outer_text_bottom.is_empty() && frame_style != FrameStyle::Banner {
        let text_color = rgba_to_svg(outer_text_color);
        let svg_font_size = 3.5 * (outer_text_font_size as f64 / 14.0);
        svg.push_str(&format!("<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" dominant-baseline=\"central\" font-family=\"{}\" font-size=\"{:.1}\" fill=\"{}\">{}</text>\n", full_w / 2.0, frame_units + top_units + total as f64 + bottom_units / 2.0, outer_text_font, svg_font_size, text_color, xml_escape(outer_text_bottom)));
    }
    if frame_style == FrameStyle::Banner && !outer_text_bottom.is_empty() {
        let text_color = rgba_to_svg(outer_text_color);
        let svg_font_size = 3.5 * (outer_text_font_size as f64 / 14.0);
        let banner_cy = full_h - frame_banner_units / 2.0;
        svg.push_str(&format!("<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" dominant-baseline=\"central\" font-family=\"{}\" font-size=\"{:.1}\" fill=\"{}\">{}</text>\n", full_w / 2.0, banner_cy, outer_text_font, svg_font_size, text_color, xml_escape(outer_text_bottom)));
    }

    svg.push_str("</svg>");
    Some(svg)
}
