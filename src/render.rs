use crate::helpers::*;
use crate::svg::*;
use crate::types::*;
use image::{Rgba, RgbaImage};
use rayon::prelude::*;
use std::path::PathBuf;

/// Vector-first QR render: generates SVG then rasterizes to pixels
pub fn render_vector_qr(
    data: &str,
    dot_style: DotStyle,
    corner_square_style: CornerSquareStyle,
    corner_dot_style: CornerDotStyle,
    fg_color: Rgba<u8>,
    bg_color: Rgba<u8>,
    corner_color: Rgba<u8>,
    logo_path: Option<&PathBuf>,
    logo_size: f64,
    outer_text_top: &str,
    outer_text_bottom: &str,
    outer_text_color: Rgba<u8>,
    ec_level: ErrorCorrectionLevel,
    transparent_bg: bool,
    module_size: u32,
    gradient_enabled: bool,
    gradient_color: Rgba<u8>,
    gradient_direction: GradientDirection,
    logo_shape: LogoShape,
    quiet_zone: u32,
    module_gap: f64,
    frame_style: FrameStyle,
    frame_color: Rgba<u8>,
    gradient_phase: f64,
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
    custom_dot_path: &str,
    outer_text_font: &str,
    outer_text_font_size: u32,
) -> Option<RgbaImage> {
    let svg = render_vector_svg(
        data,
        dot_style,
        corner_square_style,
        corner_dot_style,
        fg_color,
        bg_color,
        corner_color,
        ec_level,
        transparent_bg,
        gradient_enabled,
        gradient_color,
        gradient_direction,
        logo_path,
        logo_size,
        outer_text_top,
        outer_text_bottom,
        outer_text_color,
        module_size,
        logo_shape,
        quiet_zone,
        module_gap,
        frame_style,
        frame_color,
        shadow_enabled,
        shadow_offset,
        frame_width,
        frame_outer_radius,
        _frame_inner_radius,
        logo_color,
        logo_border_width,
        logo_border_color,
        bg_image_path,
        logo_vectorize,
        logo_vectorize_bg_color,
        logo_bg_transparent,
        logo_clear_area,
        logo_clear_padding,
        logo_outer_radius,
        logo_inner_radius,
        gradient_phase,
        custom_dot_path,
        outer_text_font,
        outer_text_font_size,
    )?;

    // Calculate pixel dimensions from SVG viewBox units
    let qr =
        qrcode::QrCode::with_error_correction_level(data.as_bytes(), ec_level_to_qrcode(ec_level))
            .ok()?;
    let qr_width = qr.width();
    let total = (qr_width as u32 + quiet_zone * 2) as f64;
    let fw = if frame_style != FrameStyle::None {
        frame_width.max(1) as f64
    } else {
        0.0
    };
    let banner_units = if frame_style == FrameStyle::Banner {
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
    let full_w = total + fw * 2.0;
    let full_h = total + top_units + bottom_units + fw * 2.0 + banner_units;
    let pixel_w = (full_w * module_size as f64) as u32;
    let pixel_h = (full_h * module_size as f64) as u32;

    rasterize_svg(&svg, pixel_w.max(1), pixel_h.max(1))
}

/// Vector-first animated GIF: SVG frames rasterized individually
pub fn render_vector_gif(
    data: &str,
    dot_style: DotStyle,
    corner_square_style: CornerSquareStyle,
    corner_dot_style: CornerDotStyle,
    fg_color: Rgba<u8>,
    bg_color: Rgba<u8>,
    corner_color: Rgba<u8>,
    ec_level: ErrorCorrectionLevel,
    transparent_bg: bool,
    module_size: u32,
    gradient_enabled: bool,
    gradient_color: Rgba<u8>,
    gradient_direction: GradientDirection,
    logo_path: Option<&PathBuf>,
    logo_size: f64,
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
    custom_dot_path: &str,
) -> Option<Vec<u8>> {
    let num_frames = 12u32;

    // Pre-compute frame-independent dimensions
    let qr =
        qrcode::QrCode::with_error_correction_level(data.as_bytes(), ec_level_to_qrcode(ec_level))
            .ok()?;
    let qr_width = qr.width();
    let total = (qr_width as u32 + quiet_zone * 2) as f64;
    let fw = if frame_style != FrameStyle::None {
        frame_width.max(1) as f64
    } else {
        0.0
    };
    let banner_units = if frame_style == FrameStyle::Banner {
        4.0
    } else {
        0.0
    };
    let full_w = total + fw * 2.0;
    let full_h = total + fw * 2.0 + banner_units;
    let pixel_w = (full_w * module_size as f64) as u32;
    let pixel_h = (full_h * module_size as f64) as u32;

    // Generate all frames in parallel using rayon (4-8x faster on multi-core CPUs)
    let frames: Vec<(usize, RgbaImage)> = (0..num_frames)
        .into_par_iter()
        .filter_map(|i| {
            let phase = i as f64 / num_frames as f64;
            let svg = render_vector_svg(
                data,
                dot_style,
                corner_square_style,
                corner_dot_style,
                fg_color,
                bg_color,
                corner_color,
                ec_level,
                transparent_bg,
                gradient_enabled,
                gradient_color,
                gradient_direction,
                logo_path,
                logo_size,
                "",
                "",
                fg_color,
                module_size,
                logo_shape,
                quiet_zone,
                module_gap,
                frame_style,
                frame_color,
                shadow_enabled,
                shadow_offset,
                frame_width,
                frame_outer_radius,
                _frame_inner_radius,
                logo_color,
                logo_border_width,
                logo_border_color,
                bg_image_path,
                logo_vectorize,
                logo_vectorize_bg_color,
                logo_bg_transparent,
                logo_clear_area,
                logo_clear_padding,
                logo_outer_radius,
                logo_inner_radius,
                phase,
                custom_dot_path,
                "sans-serif",
                14,
            )?;
            let img = rasterize_svg(&svg, pixel_w.max(1), pixel_h.max(1))?;
            Some((i as usize, img))
        })
        .collect();

    // Verify all frames were generated successfully
    if frames.len() != num_frames as usize {
        return None;
    }

    // Sort by frame index and encode into GIF sequentially
    let mut frames = frames;
    frames.sort_by_key(|(i, _)| *i);

    let mut buf = Vec::new();
    {
        let mut encoder = image::codecs::gif::GifEncoder::new(&mut buf);
        for (_, img) in frames {
            let delay = image::Delay::from_numer_denom_ms(100, 1);
            let frame = image::Frame::from_parts(img, 0, 0, delay);
            if encoder.encode_frame(frame).is_err() {
                return None;
            }
        }
    }
    Some(buf)
}

/// Renders QR from AppState for export (PNG via SVG rasterization)
pub fn render_qr_from_state(state: &AppState) -> Option<RgbaImage> {
    let data = get_qr_data(state)?;
    let dot_style = *state.dot_style.borrow();
    let corner_sq = *state.corner_square_style.borrow();
    let corner_dot = *state.corner_dot_style.borrow();
    let fg = *state.fg_color.borrow();
    let bg = *state.bg_color.borrow();
    let cc = *state.corner_color.borrow();
    let logo_path = state.logo_path.borrow().clone();
    let logo_size = *state.logo_size.borrow();
    let top = state.outer_text_top.borrow().clone();
    let bottom = state.outer_text_bottom.borrow().clone();
    let tc = *state.outer_text_color.borrow();
    let ec_level = *state.ec_level.borrow();
    let transparent = *state.transparent_bg.borrow();
    let module_size = *state.module_size.borrow();
    let grad_enabled = *state.gradient_enabled.borrow();
    let grad_color = *state.gradient_color.borrow();
    let grad_dir = *state.gradient_direction.borrow();
    let logo_shape = *state.logo_shape.borrow();
    let quiet_zone = *state.quiet_zone.borrow();
    let module_gap = *state.module_gap.borrow();
    let frame_style = *state.frame_style.borrow();
    let frame_color = *state.frame_color.borrow();
    let shadow_enabled = *state.shadow_enabled.borrow();
    let shadow_offset = *state.shadow_offset.borrow();
    let bg_image_path = state.bg_image_path.borrow().clone();
    let logo_vectorize = *state.logo_vectorize.borrow();
    let logo_vectorize_bg_color = *state.logo_vectorize_bg_color.borrow();
    let logo_bg_transparent = *state.logo_bg_transparent.borrow();
    let logo_clear_area = *state.logo_clear_area.borrow();
    let logo_clear_padding = *state.logo_clear_padding.borrow();
    let custom_dot_path = state.custom_dot_path.borrow().clone();
    let outer_text_font = state.outer_text_font.borrow().clone();
    let outer_text_font_size = *state.outer_text_font_size.borrow();
    render_vector_qr(
        &data,
        dot_style,
        corner_sq,
        corner_dot,
        fg,
        bg,
        cc,
        logo_path.as_ref(),
        logo_size,
        &top,
        &bottom,
        tc,
        ec_level,
        transparent,
        module_size,
        grad_enabled,
        grad_color,
        grad_dir,
        logo_shape,
        quiet_zone,
        module_gap,
        frame_style,
        frame_color,
        0.0,
        shadow_enabled,
        shadow_offset,
        *state.frame_width.borrow(),
        *state.frame_outer_radius.borrow(),
        *state.frame_inner_radius.borrow(),
        *state.logo_color.borrow(),
        *state.logo_border_width.borrow(),
        *state.logo_border_color.borrow(),
        bg_image_path.as_ref(),
        logo_vectorize,
        logo_vectorize_bg_color,
        logo_bg_transparent,
        logo_clear_area,
        logo_clear_padding,
        *state.logo_outer_radius.borrow(),
        *state.logo_inner_radius.borrow(),
        &custom_dot_path,
        &outer_text_font,
        outer_text_font_size,
    )
}

/// Renders SVG from AppState for export
pub fn render_svg_from_state(state: &AppState) -> Option<String> {
    // Use cached SVG from preview if available
    if let Some(ref cached) = *state.cached_svg.borrow() {
        return Some(cached.clone());
    }

    let data = get_qr_data(state)?;
    let dot_style = *state.dot_style.borrow();
    let corner_sq = *state.corner_square_style.borrow();
    let corner_dot = *state.corner_dot_style.borrow();
    let fg = *state.fg_color.borrow();
    let bg = *state.bg_color.borrow();
    let cc = *state.corner_color.borrow();
    let ec_level = *state.ec_level.borrow();
    let transparent = *state.transparent_bg.borrow();
    let grad_enabled = *state.gradient_enabled.borrow();
    let grad_color = *state.gradient_color.borrow();
    let grad_dir = *state.gradient_direction.borrow();
    let logo_path = state.logo_path.borrow().clone();
    let logo_size = *state.logo_size.borrow();
    let top = state.outer_text_top.borrow().clone();
    let bottom = state.outer_text_bottom.borrow().clone();
    let tc = *state.outer_text_color.borrow();
    let module_size = *state.module_size.borrow();
    let logo_shape = *state.logo_shape.borrow();
    let quiet_zone = *state.quiet_zone.borrow();
    let module_gap = *state.module_gap.borrow();
    let frame_style = *state.frame_style.borrow();
    let frame_color = *state.frame_color.borrow();
    let shadow_enabled = *state.shadow_enabled.borrow();
    let shadow_offset = *state.shadow_offset.borrow();
    let logo_vectorize = *state.logo_vectorize.borrow();
    let logo_vectorize_bg_color = *state.logo_vectorize_bg_color.borrow();
    let logo_bg_transparent = *state.logo_bg_transparent.borrow();
    let logo_clear_area = *state.logo_clear_area.borrow();
    let logo_clear_padding = *state.logo_clear_padding.borrow();
    let custom_dot_path = state.custom_dot_path.borrow().clone();
    let outer_text_font = state.outer_text_font.borrow().clone();
    let outer_text_font_size = *state.outer_text_font_size.borrow();
    render_vector_svg(
        &data,
        dot_style,
        corner_sq,
        corner_dot,
        fg,
        bg,
        cc,
        ec_level,
        transparent,
        grad_enabled,
        grad_color,
        grad_dir,
        logo_path.as_ref(),
        logo_size,
        &top,
        &bottom,
        tc,
        module_size,
        logo_shape,
        quiet_zone,
        module_gap,
        frame_style,
        frame_color,
        shadow_enabled,
        shadow_offset,
        *state.frame_width.borrow(),
        *state.frame_outer_radius.borrow(),
        *state.frame_inner_radius.borrow(),
        *state.logo_color.borrow(),
        *state.logo_border_width.borrow(),
        *state.logo_border_color.borrow(),
        None,
        logo_vectorize,
        logo_vectorize_bg_color,
        logo_bg_transparent,
        logo_clear_area,
        logo_clear_padding,
        *state.logo_outer_radius.borrow(),
        *state.logo_inner_radius.borrow(),
        0.0,
        &custom_dot_path,
        &outer_text_font,
        outer_text_font_size,
    )
}

/// Renders GIF from AppState for export
pub fn render_gif_from_state(state: &AppState) -> Option<Vec<u8>> {
    let data = get_qr_data(state)?;
    let dot_style = *state.dot_style.borrow();
    let corner_sq = *state.corner_square_style.borrow();
    let corner_dot = *state.corner_dot_style.borrow();
    let fg = *state.fg_color.borrow();
    let bg = *state.bg_color.borrow();
    let cc = *state.corner_color.borrow();
    let ec_level = *state.ec_level.borrow();
    let transparent = *state.transparent_bg.borrow();
    let module_size = *state.module_size.borrow();
    let grad_enabled = *state.gradient_enabled.borrow();
    let grad_color = *state.gradient_color.borrow();
    let grad_dir = *state.gradient_direction.borrow();
    let logo_path = state.logo_path.borrow().clone();
    let logo_size = *state.logo_size.borrow();
    let logo_shape = *state.logo_shape.borrow();
    let quiet_zone = *state.quiet_zone.borrow();
    let module_gap = *state.module_gap.borrow();
    let frame_style = *state.frame_style.borrow();
    let frame_color = *state.frame_color.borrow();
    let shadow_enabled = *state.shadow_enabled.borrow();
    let shadow_offset = *state.shadow_offset.borrow();
    let bg_image_path = state.bg_image_path.borrow().clone();
    let logo_vectorize = *state.logo_vectorize.borrow();
    let logo_vectorize_bg_color = *state.logo_vectorize_bg_color.borrow();
    let logo_bg_transparent = *state.logo_bg_transparent.borrow();
    let logo_clear_area = *state.logo_clear_area.borrow();
    let logo_clear_padding = *state.logo_clear_padding.borrow();
    let custom_dot_path = state.custom_dot_path.borrow().clone();
    render_vector_gif(
        &data,
        dot_style,
        corner_sq,
        corner_dot,
        fg,
        bg,
        cc,
        ec_level,
        transparent,
        module_size,
        grad_enabled,
        grad_color,
        grad_dir,
        logo_path.as_ref(),
        logo_size,
        logo_shape,
        quiet_zone,
        module_gap,
        frame_style,
        frame_color,
        shadow_enabled,
        shadow_offset,
        *state.frame_width.borrow(),
        *state.frame_outer_radius.borrow(),
        *state.frame_inner_radius.borrow(),
        *state.logo_color.borrow(),
        *state.logo_border_width.borrow(),
        *state.logo_border_color.borrow(),
        bg_image_path.as_ref(),
        logo_vectorize,
        logo_vectorize_bg_color,
        logo_bg_transparent,
        logo_clear_area,
        logo_clear_padding,
        *state.logo_outer_radius.borrow(),
        *state.logo_inner_radius.borrow(),
        &custom_dot_path,
    )
}

/// Renders PDF from AppState for export (vector PDF with embedded SVG)
pub fn render_pdf_from_state(state: &AppState) -> Option<Vec<u8>> {
    use printpdf::*;

    let svg_string = render_svg_from_state(state)?;
    let data = get_qr_data(state)?;
    let ec_level = *state.ec_level.borrow();
    let quiet_zone = *state.quiet_zone.borrow();
    let module_size = *state.module_size.borrow();

    let qr = qrcode::QrCode::with_error_correction_level(
        data.as_bytes(),
        crate::helpers::ec_level_to_qrcode(ec_level),
    )
    .ok()?;
    let qr_width = qr.width();
    let total = (qr_width as u32 + quiet_zone * 2) as f64;

    let frame_style = *state.frame_style.borrow();
    let frame_width = *state.frame_width.borrow();
    let fw = if frame_style != crate::types::FrameStyle::None {
        frame_width.max(1) as f64
    } else {
        0.0
    };
    let top_text = state.outer_text_top.borrow().clone();
    let bottom_text = state.outer_text_bottom.borrow().clone();
    let banner_units = if frame_style == crate::types::FrameStyle::Banner {
        if !bottom_text.is_empty() { 4.0 } else { 2.0 }
    } else {
        0.0
    };
    let top_units: f64 = if !top_text.is_empty() { 5.0 } else { 0.0 };
    let bottom_units: f64 =
        if !bottom_text.is_empty() && frame_style != crate::types::FrameStyle::Banner {
            5.0
        } else {
            0.0
        };
    let full_w = total + fw * 2.0;
    let full_h = total + top_units + bottom_units + fw * 2.0 + banner_units;

    // A4 page — scale QR to fit nicely
    let page_w_mm: f32 = 210.0;
    let page_h_mm: f32 = 297.0;
    let qr_size_mm: f32 = 100.0; // 10cm QR code
    let scale_factor = qr_size_mm / full_w.max(1.0) as f32;
    let qr_h_mm = full_h as f32 * scale_factor;
    let x_offset_mm = (page_w_mm - qr_size_mm) / 2.0;
    let y_offset_mm = (page_h_mm - qr_h_mm) / 2.0;

    // Rasterize SVG to high-res pixels
    let pixel_w = (full_w * module_size as f64) as u32;
    let pixel_h = (full_h * module_size as f64) as u32;
    let img = crate::svg::rasterize_svg(&svg_string, pixel_w.max(1), pixel_h.max(1))?;

    let mut doc = PdfDocument::new("QR Code");

    // Build RawImage directly from RGBA pixel data (printpdf 0.8 handles alpha internally)
    let raw_rgba = img.into_raw();
    let pdf_image = RawImage {
        pixels: RawImageData::U8(raw_rgba),
        width: pixel_w as usize,
        height: pixel_h as usize,
        data_format: RawImageFormat::RGBA8,
        tag: Vec::new(),
    };
    let image_id = doc.add_image(&pdf_image);

    // At the default 300 DPI the image occupies pixel_w*72/300 points.
    // Scale it so it fills qr_size_mm × qr_h_mm instead.
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
    Some(buf)
}

/// Renders a label sheet PDF with multiple QR codes arranged in a grid on an A4 page.
///
/// - `cols`: number of columns (1–10)
/// - `rows`: number of rows (1–15)
/// - `margin_mm`: margin from page edges in mm
/// - `spacing_mm`: spacing between cells in mm
pub fn render_label_sheet(
    state: &AppState,
    cols: u32,
    rows: u32,
    margin_mm: f64,
    spacing_mm: f64,
) -> Option<Vec<u8>> {
    use printpdf::*;

    let cols = cols.max(1).min(10);
    let rows = rows.max(1).min(15);

    // Render QR code as high-res PNG using the SVG pipeline
    let svg_string = render_svg_from_state(state)?;
    let module_size = *state.module_size.borrow();
    let data = get_qr_data(state)?;
    let ec_level = *state.ec_level.borrow();
    let quiet_zone = *state.quiet_zone.borrow();

    let qr = qrcode::QrCode::with_error_correction_level(
        data.as_bytes(),
        crate::helpers::ec_level_to_qrcode(ec_level),
    )
    .ok()?;
    let qr_width = qr.width();
    let total = (qr_width as u32 + quiet_zone * 2) as f64;

    let frame_style = *state.frame_style.borrow();
    let frame_width = *state.frame_width.borrow();
    let fw = if frame_style != crate::types::FrameStyle::None {
        frame_width.max(1) as f64
    } else {
        0.0
    };
    let top_text = state.outer_text_top.borrow().clone();
    let bottom_text = state.outer_text_bottom.borrow().clone();
    let banner_units = if frame_style == crate::types::FrameStyle::Banner {
        if !bottom_text.is_empty() { 4.0 } else { 2.0 }
    } else {
        0.0
    };
    let top_units: f64 = if !top_text.is_empty() { 5.0 } else { 0.0 };
    let bottom_units: f64 =
        if !bottom_text.is_empty() && frame_style != crate::types::FrameStyle::Banner {
            5.0
        } else {
            0.0
        };
    let full_w = total + fw * 2.0;
    let full_h = total + top_units + bottom_units + fw * 2.0 + banner_units;

    // Rasterize SVG to pixels
    let pixel_w = (full_w * module_size as f64) as u32;
    let pixel_h = (full_h * module_size as f64) as u32;
    let img = crate::svg::rasterize_svg(&svg_string, pixel_w.max(1), pixel_h.max(1))?;

    // A4 dimensions in mm
    let page_w_mm: f32 = 210.0;
    let page_h_mm: f32 = 297.0;

    // Calculate cell dimensions in mm
    let usable_w_mm = page_w_mm as f64 - 2.0 * margin_mm - (cols as f64 - 1.0) * spacing_mm;
    let usable_h_mm = page_h_mm as f64 - 2.0 * margin_mm - (rows as f64 - 1.0) * spacing_mm;
    let cell_w_mm = (usable_w_mm / cols as f64).max(1.0);
    let cell_h_mm = (usable_h_mm / rows as f64).max(1.0);

    // Fit QR into each cell maintaining aspect ratio
    let aspect = full_w.max(1.0) / full_h.max(1.0);
    let (qr_w_mm, qr_h_mm) = if aspect >= cell_w_mm / cell_h_mm {
        // Width-limited
        (cell_w_mm, cell_w_mm / aspect)
    } else {
        // Height-limited
        (cell_h_mm * aspect, cell_h_mm)
    };

    // Center the grid on the page
    let total_grid_w = cols as f64 * cell_w_mm + (cols as f64 - 1.0) * spacing_mm;
    let total_grid_h = rows as f64 * cell_h_mm + (rows as f64 - 1.0) * spacing_mm;
    let grid_offset_x = (page_w_mm as f64 - total_grid_w) / 2.0;
    let grid_offset_y = (page_h_mm as f64 - total_grid_h) / 2.0;

    // Create PDF
    let mut doc = PdfDocument::new("Etiketten-Druckbogen");

    // Build RawImage directly from RGBA pixel data (printpdf 0.8 handles alpha internally)
    let raw_rgba = img.into_raw();
    let pdf_image = RawImage {
        pixels: RawImageData::U8(raw_rgba),
        width: pixel_w as usize,
        height: pixel_h as usize,
        data_format: RawImageFormat::RGBA8,
        tag: Vec::new(),
    };
    let image_id = doc.add_image(&pdf_image);

    // Scale: default PDF image size at 300 DPI
    let target_w_pt = (qr_w_mm * 72.0 / 25.4) as f32;
    let target_h_pt = (qr_h_mm * 72.0 / 25.4) as f32;
    let default_w_pt = pixel_w as f32 * 72.0 / 300.0;
    let default_h_pt = pixel_h as f32 * 72.0 / 300.0;
    let sx = target_w_pt / default_w_pt;
    let sy = target_h_pt / default_h_pt;

    // Place each QR code in the grid (reuse the same image_id for all placements)
    let mut ops = Vec::new();
    for row in 0..rows {
        for col in 0..cols {
            let cell_x = grid_offset_x + col as f64 * (cell_w_mm + spacing_mm);
            let cell_y = grid_offset_y + row as f64 * (cell_h_mm + spacing_mm);
            // Center QR within cell
            let qr_x = cell_x + (cell_w_mm - qr_w_mm) / 2.0;
            let qr_y = cell_y + (cell_h_mm - qr_h_mm) / 2.0;

            let x_pt = (qr_x * 72.0 / 25.4) as f32;
            let y_pt = (qr_y * 72.0 / 25.4) as f32;

            ops.push(Op::UseXobject {
                id: image_id.clone(),
                transform: XObjectTransform {
                    translate_x: Some(Pt(x_pt)),
                    translate_y: Some(Pt(y_pt)),
                    scale_x: Some(sx),
                    scale_y: Some(sy),
                    ..Default::default()
                },
            });
        }
    }

    let page = PdfPage::new(Mm(page_w_mm), Mm(page_h_mm), ops);
    doc.with_pages(vec![page]);

    let buf = doc.save(&PdfSaveOptions::default(), &mut Vec::new());
    Some(buf)
}
