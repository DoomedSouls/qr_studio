// ============================================================
// UI MODULE - Preview, scheduling, undo, and UI builder
// ============================================================

use crate::country_codes::*;
use crate::helpers::*;
use crate::i18n::*;
use crate::render::*;
use crate::svg::*;
use crate::types::ToastType;
use crate::types::*;

use adw::prelude::*;
use adw::{Application, ApplicationWindow, HeaderBar, ToastOverlay};
use gtk4::gdk;
use gtk4::glib;
use gtk4::pango::prelude::{FontFamilyExt, FontMapExt};
use gtk4::{
    Align, Box, Button, Calendar, CheckButton, ColorButton, DropDown, Entry, FileChooserDialog,
    FileFilter, Image, Label, ListBox, ListBoxRow, Orientation, Overlay, Paned, Picture, Popover,
    ProgressBar, Scale, ScrolledWindow, SelectionMode, Separator, SignalListItemFactory,
    SpinButton, Stack, StackSwitcher, StackTransitionType, StringList, TextBuffer, TextView,
};
use gtk4::{CallbackAction, Shortcut, ShortcutController, ShortcutTrigger};
use image::Rgba;
use libshumate as shumate;
use shumate::prelude::*;
use shumate::{Map, MapLayer, Marker, MarkerLayer, RasterRenderer, VectorRenderer};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use std::time::Duration;

// ============================================================
// CONTENT SNAPSHOT — preserves input values across language switches
// ============================================================
struct ContentSnapshot {
    text: String,
    content_type_idx: u32,
    wifi_ssid: String,
    wifi_password: String,
    wifi_enc_idx: u32,
    vcard_name: String,
    vcard_phone: String,
    vcard_email: String,
    vcard_org: String,
    vcard_url: String,
    vcard_country_idx: u32,
    cal_title: String,
    cal_location: String,
    cal_start_date: glib::DateTime,
    cal_end_date: glib::DateTime,
    cal_start_hour: f64,
    cal_start_minute: f64,
    gps_lat: String,
    gps_lon: String,
    gps_search: String,
    sms_phone: String,
    sms_message: String,
    sms_country_idx: u32,
}

thread_local! {
    static CONTENT_SNAPSHOT: RefCell<Option<ContentSnapshot>> = RefCell::new(None);
}

// ============================================================
// PREVIEW & SCHEDULING
// ============================================================

/// Async preview: SVG generation + rasterization runs in a background thread.
/// Only the GTK widget update happens on the main thread (via polling channel).
/// A generation counter discards stale results if the user changed settings
/// while a render was in progress.
pub fn update_preview(state: &Rc<RefCell<AppState>>) {
    // Phase 1: Collect all rendering parameters from state (fast, main thread)
    let s = state.borrow();

    let data = match get_qr_data(&s) {
        Some(d) => {
            *s.cached_qr_data.borrow_mut() = Some(d.clone());
            d
        }
        None => {
            *s.cached_qr_data.borrow_mut() = None;
            s.scan_verify_btn.set_visible(false);
            return;
        }
    };

    let dot_style = *s.dot_style.borrow();
    let corner_sq = *s.corner_square_style.borrow();
    let corner_dot = *s.corner_dot_style.borrow();
    let fg = *s.fg_color.borrow();
    let bg = *s.bg_color.borrow();
    let cc = *s.corner_color.borrow();
    let logo_path = s.logo_path.borrow().clone();
    let logo_size = *s.logo_size.borrow();
    let top = s.outer_text_top.borrow().clone();
    let bottom = s.outer_text_bottom.borrow().clone();
    let tc = *s.outer_text_color.borrow();
    let ec_level = *s.ec_level.borrow();
    let transparent = *s.transparent_bg.borrow();
    let module_size = *s.module_size.borrow();
    let grad_enabled = *s.gradient_enabled.borrow();
    let grad_color = *s.gradient_color.borrow();
    let grad_dir = *s.gradient_direction.borrow();
    let logo_shape = *s.logo_shape.borrow();
    let quiet_zone = *s.quiet_zone.borrow();
    let module_gap = *s.module_gap.borrow();
    let frame_style = *s.frame_style.borrow();
    let frame_color = *s.frame_color.borrow();
    let shadow_enabled = *s.shadow_enabled.borrow();
    let shadow_offset = *s.shadow_offset.borrow();
    let bg_image_path = s.bg_image_path.borrow().clone();
    let logo_vectorize = *s.logo_vectorize.borrow();
    let logo_vectorize_bg_color = *s.logo_vectorize_bg_color.borrow();
    let logo_bg_transparent = *s.logo_bg_transparent.borrow();
    let logo_clear_area = *s.logo_clear_area.borrow();
    let logo_clear_padding = *s.logo_clear_padding.borrow();
    let frame_width = *s.frame_width.borrow();
    let frame_outer_radius = *s.frame_outer_radius.borrow();
    let frame_inner_radius = *s.frame_inner_radius.borrow();
    let logo_color = *s.logo_color.borrow();
    let logo_border_width = *s.logo_border_width.borrow();
    let logo_border_color = *s.logo_border_color.borrow();
    let logo_outer_radius = *s.logo_outer_radius.borrow();
    let logo_inner_radius = *s.logo_inner_radius.borrow();
    let custom_dot_path = s.custom_dot_path.borrow().clone();
    let outer_text_font = s.outer_text_font.borrow().clone();
    let outer_text_font_size = *s.outer_text_font_size.borrow();

    let _ = validate_qr(ec_level, &data, logo_size, logo_path.as_ref());

    // Contrast warning (WCAG 2.0)
    let ratio = contrast_ratio(&fg, &bg);
    if ratio < 3.0 {
        let i18n = s.i18n.borrow();
        let warning_template = i18n.t("scan_detail_low_contrast");
        s.contrast_warning_label.set_text(&format!(
            "⚠️ {}",
            warning_template.replace("{:.1}", &format!("{:.1}", ratio))
        ));
        s.contrast_warning_label.set_visible(true);
        // Animation 10: Contrast warning shake
        s.contrast_warning_label.add_css_class("contrast-shake");
        let lbl = s.contrast_warning_label.clone();
        glib::timeout_add_local(Duration::from_millis(450), move || {
            lbl.remove_css_class("contrast-shake");
            glib::ControlFlow::Break
        });
    } else {
        s.contrast_warning_label.set_visible(false);
    }

    let preview_gen = *s.preview_generation.borrow();
    drop(s); // Release state borrow before spawning thread

    // Show skeleton pulse while rendering + morph transition
    state
        .borrow()
        .preview_picture
        .add_css_class("preview-updating");
    state
        .borrow()
        .preview_picture
        .add_css_class("preview-skeleton");
    state
        .borrow()
        .preview_picture
        .add_css_class("preview-morphing");

    // Phase 2: Spawn background thread for SVG generation + rasterization
    let (tx, rx) = std::sync::mpsc::sync_channel(1);

    std::thread::spawn(move || {
        let svg = render_vector_svg(
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
            frame_width,
            frame_outer_radius,
            frame_inner_radius,
            logo_color,
            logo_border_width,
            logo_border_color,
            bg_image_path.as_ref(),
            logo_vectorize,
            logo_vectorize_bg_color,
            logo_bg_transparent,
            logo_clear_area,
            logo_clear_padding,
            logo_outer_radius,
            logo_inner_radius,
            0.0,
            &custom_dot_path,
            &outer_text_font,
            outer_text_font_size,
        );

        let result = match svg {
            Some(svg_string) => {
                let (pixel_w, pixel_h) = parse_svg_viewbox(&svg_string)
                    .map(|(w, h)| {
                        (
                            (w * module_size as f64) as u32,
                            (h * module_size as f64) as u32,
                        )
                    })
                    .unwrap_or((500, 500));
                let img = rasterize_svg(&svg_string, pixel_w.max(1), pixel_h.max(1));
                (Some(svg_string), img)
            }
            None => (None, None),
        };
        let _ = tx.send(result);
    });

    // Phase 3: Poll for result on main thread (lightweight channel check every 5ms)
    let state_clone = state.clone();
    glib::timeout_add_local(Duration::from_millis(5), move || {
        match rx.try_recv() {
            Ok((svg_opt, img_opt)) => {
                // Discard stale results from previous renders
                if *state_clone.borrow().preview_generation.borrow() != preview_gen {
                    state_clone
                        .borrow()
                        .preview_picture
                        .remove_css_class("preview-updating");
                    state_clone
                        .borrow()
                        .preview_picture
                        .remove_css_class("preview-skeleton");
                    state_clone
                        .borrow()
                        .preview_picture
                        .remove_css_class("preview-morphing");
                    return glib::ControlFlow::Break;
                }

                match svg_opt {
                    Some(svg_string) => {
                        *state_clone.borrow().cached_svg.borrow_mut() = Some(svg_string);
                        match img_opt {
                            Some(img) => {
                                let w = img.width();
                                let h = img.height();
                                *state_clone.borrow().cached_rgba.borrow_mut() = Some(img.clone());
                                let stride = (w as usize) * 4;
                                let bytes = glib::Bytes::from(&img.into_raw());
                                let texture = gdk::MemoryTexture::new(
                                    w as i32,
                                    h as i32,
                                    gdk::MemoryFormat::R8g8b8a8,
                                    &bytes,
                                    stride,
                                );
                                state_clone
                                    .borrow()
                                    .preview_picture
                                    .set_paintable(Some(&texture));
                            }
                            None => {}
                        }
                    }
                    None => {
                        *state_clone.borrow().cached_svg.borrow_mut() = None;
                        *state_clone.borrow().cached_rgba.borrow_mut() = None;
                        state_clone.borrow().scan_verify_btn.set_visible(false);
                    }
                }
                state_clone
                    .borrow()
                    .preview_picture
                    .remove_css_class("preview-updating");
                state_clone
                    .borrow()
                    .preview_picture
                    .remove_css_class("preview-skeleton");
                state_clone
                    .borrow()
                    .preview_picture
                    .remove_css_class("preview-morphing");
                // Animation 9: QR code particle assembly effect
                state_clone
                    .borrow()
                    .preview_picture
                    .add_css_class("qr-assemble");
                {
                    let pic = state_clone.borrow().preview_picture.clone();
                    glib::timeout_add_local(Duration::from_millis(550), move || {
                        pic.remove_css_class("qr-assemble");
                        glib::ControlFlow::Break
                    });
                }
                // Auto-verify scan quality after render
                {
                    let state_v = state_clone.clone();
                    {
                        let s = state_clone.borrow();
                        s.scan_verify_btn.set_visible(true);
                        s.scan_verify_btn
                            .set_label(&s.i18n.borrow().t("btn_verify_scan"));
                        s.scan_verify_btn.remove_css_class("scan-good");
                        s.scan_verify_btn.remove_css_class("scan-limited");
                        s.scan_verify_btn.remove_css_class("scan-bad");
                    }
                    glib::timeout_add_local(Duration::from_millis(100), move || {
                        let (
                            img,
                            data,
                            fg,
                            bg,
                            ec_level,
                            logo_path,
                            logo_size,
                            module_gap,
                            corner_sq,
                            dot_st,
                            has_data,
                        ) = {
                            let s = state_v.borrow();
                            let img = s.cached_rgba.borrow().clone();
                            let data = s.cached_qr_data.borrow().clone();
                            let has = img.is_some() && data.is_some();
                            (
                                img,
                                data,
                                *s.fg_color.borrow(),
                                *s.bg_color.borrow(),
                                *s.ec_level.borrow(),
                                s.logo_path.borrow().clone(),
                                *s.logo_size.borrow(),
                                *s.module_gap.borrow(),
                                *s.corner_square_style.borrow(),
                                *s.dot_style.borrow(),
                                has,
                            )
                        };

                        if !has_data {
                            state_v.borrow().scan_verify_btn.set_visible(false);
                            return glib::ControlFlow::Break;
                        }

                        let result = verify_qr_scanability(
                            &img.unwrap(),
                            &data.unwrap(),
                            fg,
                            bg,
                            ec_level,
                            logo_path.as_ref(),
                            logo_size,
                            module_gap,
                            corner_sq,
                            dot_st,
                        );

                        let s = state_v.borrow();
                        let btn = &s.scan_verify_btn;
                        btn.remove_css_class("scan-good");
                        btn.remove_css_class("scan-limited");
                        btn.remove_css_class("scan-bad");

                        match result.quality {
                            ScanQuality::Good => {
                                btn.set_label(&s.i18n.borrow().t("scan_status_good"));
                                btn.add_css_class("scan-good");
                                if result.styled_corners_fallback {
                                    btn.set_tooltip_text(Some(
                                        &s.i18n.borrow().t("scan_detail_styled_corners"),
                                    ));
                                } else {
                                    btn.set_tooltip_text(None);
                                }
                            }
                            ScanQuality::Limited => {
                                btn.set_label(&s.i18n.borrow().t("scan_status_limited"));
                                btn.add_css_class("scan-limited");
                                let mut tips = Vec::new();
                                if let Some(ratio) = result.contrast_ratio {
                                    let tmpl = s.i18n.borrow().t("scan_detail_low_contrast");
                                    tips.push(tmpl.replace("{:.1}:1", &format!("{:.1}:1", ratio)));
                                }
                                if result.logo_ec_warning {
                                    tips.push(s.i18n.borrow().t("scan_detail_logo_ec").to_string());
                                }
                                if result.gap_warning {
                                    tips.push(
                                        s.i18n.borrow().t("scan_detail_large_gap").to_string(),
                                    );
                                }
                                if result.styled_corners_fallback {
                                    tips.push(
                                        s.i18n.borrow().t("scan_detail_styled_corners").to_string(),
                                    );
                                }
                                btn.set_tooltip_text(Some(&tips.join("\n")));
                            }
                            ScanQuality::Bad => {
                                btn.set_label(&s.i18n.borrow().t("scan_status_bad"));
                                btn.add_css_class("scan-bad");
                                btn.set_tooltip_text(None);
                            }
                        }
                        glib::ControlFlow::Break
                    });
                }
                glib::ControlFlow::Break // Stop polling — result received
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => glib::ControlFlow::Continue, // Keep polling
            Err(_) => glib::ControlFlow::Break, // Channel closed, stop polling
        }
    });
}

pub fn schedule_preview(state: &Rc<RefCell<AppState>>) {
    update_qr_info(&state.borrow());
    update_preview(state);
}

// ============================================================
// UNDO HELPERS
// ============================================================

pub fn save_undo_state(state: &AppState) {
    if *state.is_restoring.borrow() {
        return;
    }
    let current = current_style_settings(state);
    state.undo_stack.borrow_mut().push(current);
    state.redo_stack.borrow_mut().clear();
}

fn rgba_to_gdk(c: &Rgba<u8>) -> gdk::RGBA {
    gdk::RGBA::new(
        c.0[0] as f32 / 255.0,
        c.0[1] as f32 / 255.0,
        c.0[2] as f32 / 255.0,
        c.0[3] as f32 / 255.0,
    )
}

pub fn get_dropdown_string(dd: &DropDown) -> String {
    dd.selected_item()
        .and_then(|item| item.downcast::<gtk4::StringObject>().ok())
        .map(|obj| obj.string().to_string())
        .unwrap_or_default()
}

pub fn set_dropdown_by_string(dd: &DropDown, target: &str) {
    let model = dd.model();
    if let Some(model) = model {
        for i in 0..model.n_items() {
            if let Some(item) = model.item(i) {
                if let Ok(obj) = item.downcast::<gtk4::StringObject>() {
                    if obj.string() == target {
                        dd.set_selected(i);
                        return;
                    }
                }
            }
        }
    }
    // Fallback: select index 0 if target not found
    dd.set_selected(0);
}

// ============================================================
// COUNTRY CODE DROPDOWN (intl-tel-input style)
// ============================================================

/// Loads an SVG flag file via gdk-pixbuf (which supports SVG through librsvg)
/// and returns a `gdk::Texture` suitable for `Image::set_from_paintable()`.
///
/// GTK4's `Image::set_from_file()` cannot load SVG directly because it uses
/// `gdk::Texture::from_file()` which only supports raster formats (PNG, JPEG, TIFF).
fn load_flag_texture(path: &std::path::Path, size: i32) -> Option<gdk::Texture> {
    let pixbuf = gtk4::gdk_pixbuf::Pixbuf::from_file_at_scale(path, size, size, true).ok()?;
    Some(gdk::Texture::for_pixbuf(&pixbuf))
}

/// Creates a country code dropdown with flag icons, country names, and dial codes.
///
/// The dropdown button shows flag + dial code (compact).
/// The popup list shows flag + country name + dial code (full info).
fn create_country_dropdown() -> DropDown {
    let model = StringList::new(&[]);
    for c in countries() {
        model.append(c.iso_code);
    }

    // Button factory: compact flag + dial code
    let btn_factory = SignalListItemFactory::new();
    btn_factory.connect_setup(|_, list_item| {
        let hbox = Box::new(Orientation::Horizontal, 4);
        let img = Image::new();
        img.set_pixel_size(18);
        img.set_valign(Align::Center);
        let lbl = Label::new(None);
        lbl.set_valign(Align::Center);
        hbox.append(&img);
        hbox.append(&lbl);
        list_item.set_child(Some(&hbox));
    });
    btn_factory.connect_bind(|_, list_item| {
        if let Some(child) = list_item.child() {
            let hbox = child.downcast::<Box>().unwrap();
            let img = hbox.first_child().unwrap().downcast::<Image>().unwrap();
            let lbl = hbox.last_child().unwrap().downcast::<Label>().unwrap();
            if let Some(item) = list_item.item() {
                if let Ok(obj) = item.downcast::<gtk4::StringObject>() {
                    let iso = obj.string().to_string();
                    if let Some(c) = countries().iter().find(|ci| ci.iso_code == iso) {
                        let p = flag_svg_path(c.iso_code);
                        if let Some(tex) = load_flag_texture(&p, 36) {
                            img.set_paintable(Some(&tex));
                        }
                        lbl.set_text(c.calling_code);
                    }
                }
            }
        }
    });

    // List factory: flag + country name + dial code
    let list_factory = SignalListItemFactory::new();
    list_factory.connect_setup(|_, list_item| {
        let hbox = Box::new(Orientation::Horizontal, 6);
        let img = Image::new();
        img.set_pixel_size(20);
        img.set_valign(Align::Center);
        let name_lbl = Label::new(None);
        name_lbl.set_hexpand(true);
        name_lbl.set_halign(Align::Start);
        name_lbl.set_valign(Align::Center);
        let code_lbl = Label::new(None);
        code_lbl.set_halign(Align::End);
        code_lbl.set_valign(Align::Center);
        hbox.append(&img);
        hbox.append(&name_lbl);
        hbox.append(&code_lbl);
        list_item.set_child(Some(&hbox));
    });
    list_factory.connect_bind(|_, list_item| {
        if let Some(child) = list_item.child() {
            let hbox = child.downcast::<Box>().unwrap();
            let img = hbox.first_child().unwrap().downcast::<Image>().unwrap();
            let name_lbl = hbox
                .first_child()
                .unwrap()
                .next_sibling()
                .unwrap()
                .downcast::<Label>()
                .unwrap();
            let code_lbl = hbox.last_child().unwrap().downcast::<Label>().unwrap();
            if let Some(item) = list_item.item() {
                if let Ok(obj) = item.downcast::<gtk4::StringObject>() {
                    let iso = obj.string().to_string();
                    if let Some(c) = countries().iter().find(|ci| ci.iso_code == iso) {
                        let p = flag_svg_path(c.iso_code);
                        if let Some(tex) = load_flag_texture(&p, 40) {
                            img.set_paintable(Some(&tex));
                        }
                        name_lbl.set_text(c.name_de);
                        code_lbl.set_text(c.calling_code);
                    }
                }
            }
        }
    });

    let dd = DropDown::new(
        Some(model.upcast::<gtk4::gio::ListModel>()),
        None::<gtk4::Expression>,
    );
    dd.set_factory(Some(&btn_factory));
    dd.set_list_factory(Some(&list_factory));
    dd.set_selected(default_country_index());

    dd
}

// ============================================================
// BUILD UI
// ============================================================

/// Animation 3: Trigger horizontal shake animation on a widget with validation error.
/// Adds the CSS class and auto-removes it after the animation duration.
fn trigger_shake(entry: &gtk4::Entry) {
    entry.add_css_class("input-error-shake");
    let e = entry.clone();
    glib::timeout_add_local(Duration::from_millis(450), move || {
        e.remove_css_class("input-error-shake");
        glib::ControlFlow::Break
    });
}

pub fn build_ui(app: &Application) {
    let lang = load_lang();
    let i18n = I18n::new(lang);

    // Helper: create button with icon + text
    let make_icon_btn = |icon: &str, text: &str| -> Button {
        let btn = Button::new();
        let hbox = Box::new(Orientation::Horizontal, 4);
        hbox.append(&Image::from_icon_name(icon));
        hbox.append(&Label::new(Some(text)));
        btn.set_child(Some(&hbox));
        btn
    };

    // Check for existing window (language switch — reuse with crossfade)
    let windows = app.windows();
    let existing: Option<ApplicationWindow> = windows
        .iter()
        .find_map(|w| w.clone().downcast::<ApplicationWindow>().ok());
    let old_content = existing.as_ref().and_then(|w| w.content());
    let window = existing.unwrap_or_else(|| {
        ApplicationWindow::builder()
            .application(app)
            .title("QR Code Studio")
            .default_width(1100)
            .default_height(750)
            .build()
    });

    // ============================================================
    // MAIN LAYOUT
    // ============================================================
    let main_box = Box::new(Orientation::Vertical, 0);
    let toast_overlay = ToastOverlay::new();
    toast_overlay.set_child(Some(&main_box));

    // Reuse existing window (language switch) or create fresh
    // Direct swap — avoids the crossfade re-parenting bug where toast_overlay
    // remains a child of the fade Stack, causing gtk_widget_snapshot_child
    // errors on every frame and breaking the QR preview rendering.
    let _old_content = old_content; // drop old content (auto-unreferenced)
    window.set_content(Some(&toast_overlay));

    // Header bar
    let header = HeaderBar::new();
    let undo_btn = Button::new();
    undo_btn.set_child(Some(&Image::from_icon_name("edit-undo-symbolic")));
    undo_btn.set_tooltip_text(Some(&i18n.t("tooltip_undo")));
    let redo_btn = Button::new();
    redo_btn.set_child(Some(&Image::from_icon_name("edit-redo-symbolic")));
    redo_btn.set_tooltip_text(Some(&i18n.t("tooltip_redo")));
    let sidebar_toggle_btn = Button::new();
    sidebar_toggle_btn.set_icon_name("sidebar-show-symbolic");
    sidebar_toggle_btn.set_tooltip_text(Some(&i18n.t("tooltip_sidebar_toggle")));
    header.pack_start(&undo_btn);
    header.pack_start(&redo_btn);
    header.pack_end(&sidebar_toggle_btn);
    main_box.append(&header);

    // Paned layout
    let paned = Paned::new(Orientation::Horizontal);
    paned.set_position(420);
    paned.set_shrink_start_child(false);
    paned.set_resize_end_child(true);
    paned.set_shrink_end_child(false);
    main_box.append(&paned);

    // ============================================================
    // SIDEBAR (LEFT) — Tabbed: Inhalt | Stil | Export
    // ============================================================
    let tab_content_box = Box::new(Orientation::Vertical, 6);
    tab_content_box.set_margin_start(8);
    tab_content_box.set_margin_end(8);
    tab_content_box.set_margin_top(8);
    tab_content_box.set_margin_bottom(8);
    let tab_content_scroll = ScrolledWindow::new();
    tab_content_scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    tab_content_scroll.set_min_content_width(400);
    tab_content_scroll.set_vexpand(true);
    tab_content_scroll.set_child(Some(&tab_content_box));

    // Style tab: Master-Detail split view
    let style_detail_stack = Stack::new();
    style_detail_stack.set_transition_type(StackTransitionType::SlideLeftRight);
    style_detail_stack.set_transition_duration(200);
    style_detail_stack.set_vhomogeneous(false);
    style_detail_stack.set_hhomogeneous(true);

    let style_sidebar_list = ListBox::new();
    style_sidebar_list.add_css_class("navigation-sidebar");
    style_sidebar_list.set_selection_mode(SelectionMode::Single);

    let style_sidebar_scroll = ScrolledWindow::new();
    style_sidebar_scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    style_sidebar_scroll.set_child(Some(&style_sidebar_list));

    let style_detail_scroll = ScrolledWindow::new();
    style_detail_scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    style_detail_scroll.set_child(Some(&style_detail_stack));

    let style_split_pane = Paned::new(Orientation::Horizontal);
    style_split_pane.set_position(180);
    style_split_pane.set_shrink_start_child(false);
    style_split_pane.set_shrink_end_child(false);
    style_split_pane.set_start_child(Some(&style_sidebar_scroll));
    style_split_pane.set_end_child(Some(&style_detail_scroll));
    style_split_pane.set_vexpand(true);

    let sidebar_stack = Stack::new();
    sidebar_stack.set_vexpand(true);
    sidebar_stack.add_titled(&tab_content_scroll, Some("content"), &i18n.t("tab_content"));
    sidebar_stack.add_titled(&style_split_pane, Some("style"), &i18n.t("tab_style"));
    sidebar_stack.set_transition_type(gtk4::StackTransitionType::Crossfade);
    sidebar_stack.set_transition_duration(300);

    let stack_switcher = StackSwitcher::new();
    stack_switcher.set_stack(Some(&sidebar_stack));
    stack_switcher.set_halign(Align::Center);

    let left_box = Box::new(Orientation::Vertical, 0);
    left_box.set_vexpand(true);

    left_box.append(&stack_switcher);
    left_box.append(&sidebar_stack);

    // Animation 1: Sidebar — GPU-accelerated CSS opacity+transform
    // Layout change (hide + paned position) in ONE frame; CSS handles the visual animation.
    {
        let sidebar_css = gtk4::CssProvider::new();
        sidebar_css.load_from_data(
            ".sidebar-panel { transition: opacity 180ms ease-out, transform 180ms ease-out; }
             .sidebar-panel.sidebar-collapsed { opacity: 0; transform: translate(-30px, 0); }",
        );
        gtk4::style_context_add_provider_for_display(
            &gtk4::gdk::Display::default().unwrap(),
            &sidebar_css,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
    left_box.add_css_class("sidebar-panel");

    {
        let left_box = left_box.clone();
        let paned = paned.clone();
        sidebar_toggle_btn.connect_clicked(move |_| {
            if left_box.is_visible() {
                // Collapse: CSS animates opacity 1→0 + slide left, then hide in one frame
                left_box.add_css_class("sidebar-collapsed");
                let lb = left_box.clone();
                let pn = paned.clone();
                glib::timeout_add_local(Duration::from_millis(190), move || {
                    lb.set_visible(false);
                    pn.set_position(0);
                    lb.remove_css_class("sidebar-collapsed");
                    glib::ControlFlow::Break
                });
            } else {
                // Expand: show invisible (sidebar-collapsed), then CSS animates opacity 0→1 + slide in
                left_box.add_css_class("sidebar-collapsed");
                paned.set_position(420);
                left_box.set_visible(true);
                let lb = left_box.clone();
                glib::timeout_add_local(Duration::from_millis(20), move || {
                    lb.remove_css_class("sidebar-collapsed");
                    glib::ControlFlow::Break
                });
            }
        });
    }

    // ============================================================
    // LANGUAGE SELECTOR
    // ============================================================
    let lang_box = Box::new(Orientation::Horizontal, 6);
    lang_box.set_margin_bottom(4);
    let lang_label = Image::from_icon_name("preferences-desktop-locale-symbolic");
    lang_box.append(&lang_label);
    let lang_items = StringList::new(&[
        "Deutsch",
        "English",
        "Español",
        "Français",
        "Italiano",
        "Português (BR)",
        "日本語",
        "한국어",
        "简体中文",
    ]);
    let lang_dd = DropDown::new(
        Some(lang_items.upcast::<gtk4::gio::ListModel>()),
        None::<gtk4::Expression>,
    );
    lang_dd.set_selected(match lang {
        Lang::De => 0,
        Lang::En => 1,
        Lang::Es => 2,
        Lang::Fr => 3,
        Lang::It => 4,
        Lang::PtBr => 5,
        Lang::Ja => 6,
        Lang::Ko => 7,
        Lang::ZhCn => 8,
    });
    lang_dd.set_hexpand(true);
    lang_box.append(&lang_dd);
    tab_content_box.append(&lang_box);

    // Clear All button — resets every content input field
    let clear_all_btn = Button::new();
    let clear_all_inner = Box::new(Orientation::Horizontal, 4);
    clear_all_inner.append(&Image::from_icon_name("edit-clear-all-symbolic"));
    clear_all_inner.append(&Label::new(Some(&i18n.t("btn_clear_all"))));
    clear_all_btn.set_child(Some(&clear_all_inner));
    clear_all_btn.set_tooltip_text(Some(&i18n.t("tooltip_clear_all")));
    clear_all_btn.set_halign(Align::End);
    clear_all_btn.add_css_class("flat");
    clear_all_btn.add_css_class("clear-all-btn");
    tab_content_box.append(&clear_all_btn);

    // ============================================================
    // SECTION: Inhalt (Content)
    // ============================================================
    let content_section_label = Label::new(Some(&i18n.t("exp_content")));
    content_section_label.add_css_class("heading");
    content_section_label.set_halign(Align::Start);
    content_section_label.set_margin_bottom(4);
    tab_content_box.append(&content_section_label);

    let content_types = StringList::new(&[]);
    content_types.append(&i18n.t("dd_content_text"));
    content_types.append(&i18n.t("dd_content_wifi"));
    content_types.append(&i18n.t("dd_content_vcard"));
    content_types.append(&i18n.t("dd_content_calendar"));
    content_types.append(&i18n.t("dd_content_gps"));
    content_types.append(&i18n.t("dd_content_sms"));
    let content_type_dd = DropDown::new(
        Some(content_types.clone().upcast::<gtk4::gio::ListModel>()),
        None::<gtk4::Expression>,
    );
    content_type_dd.set_tooltip_text(Some(&i18n.t("tooltip_content_type")));
    let content_box = Box::new(Orientation::Vertical, 4);
    content_box.append(&content_type_dd);

    // Animation 4: Content-type animated stack transition
    let content_stack = Stack::new();
    content_stack.set_transition_type(StackTransitionType::SlideLeftRight);
    content_stack.set_transition_duration(200);
    content_box.append(&content_stack);

    // Text view
    let text_buffer = TextBuffer::new(None::<&gtk4::TextTagTable>);
    text_buffer.set_text("https://github.com/mrinfinidy/qrcode-pretty");
    let text_view = TextView::with_buffer(&text_buffer);
    text_view.set_wrap_mode(gtk4::WrapMode::WordChar);
    text_view.set_tooltip_text(Some(&i18n.t("tooltip_qr_content")));
    text_view.set_margin_start(6);
    text_view.set_margin_end(6);
    text_view.set_margin_top(6);
    text_view.set_margin_bottom(6);
    let text_scroll = ScrolledWindow::new();
    text_scroll.set_min_content_height(120);
    text_scroll.set_max_content_height(200);
    text_scroll.set_propagate_natural_height(true);
    text_scroll.add_css_class("text-input-frame");
    text_scroll.set_child(Some(&text_view));
    content_stack.add_named(&text_scroll, Some("text"));

    // WiFi box
    let wifi_box = Box::new(Orientation::Vertical, 4);
    let wifi_ssid_entry = Entry::new();
    wifi_ssid_entry.set_placeholder_text(Some("SSID"));
    wifi_ssid_entry.set_tooltip_text(Some(&i18n.t("tooltip_wifi_ssid")));
    wifi_box.append(&wifi_ssid_entry);
    let wifi_password_entry = Entry::new();
    wifi_password_entry.set_placeholder_text(Some(&i18n.t("wifi_password")));
    wifi_password_entry.set_tooltip_text(Some(&i18n.t("tooltip_wifi_password")));
    wifi_password_entry.set_visibility(false);
    wifi_box.append(&wifi_password_entry);
    let wifi_enc_types = StringList::new(&[]);
    wifi_enc_types.append(&i18n.t("dd_wifi_wpa"));
    wifi_enc_types.append(&i18n.t("dd_wifi_wep"));
    wifi_enc_types.append(&i18n.t("dd_wifi_none"));
    let wifi_enc_dd = DropDown::new(
        Some(wifi_enc_types.clone().upcast::<gtk4::gio::ListModel>()),
        None::<gtk4::Expression>,
    );
    wifi_enc_dd.set_tooltip_text(Some(&i18n.t("tooltip_wifi_encryption")));
    wifi_box.append(&wifi_enc_dd);
    content_stack.add_named(&wifi_box, Some("wifi"));

    // vCard box
    let vcard_box = Box::new(Orientation::Vertical, 4);
    let vcard_name_entry = Entry::new();
    vcard_name_entry.set_placeholder_text(Some(&i18n.t("vcard_name")));
    vcard_name_entry.set_tooltip_text(Some(&i18n.t("tooltip_vcard_name")));
    vcard_box.append(&vcard_name_entry);
    // vCard Phone with country code selector (intl-tel-input style)
    let vcard_phone_row = Box::new(Orientation::Horizontal, 4);
    let vcard_country_dd = create_country_dropdown();
    vcard_country_dd.set_valign(Align::Center);
    vcard_phone_row.append(&vcard_country_dd);
    let vcard_phone_entry = Entry::new();
    vcard_phone_entry.set_placeholder_text(Some(&i18n.t("vcard_phone")));
    vcard_phone_entry.set_tooltip_text(Some(&i18n.t("tooltip_vcard_phone")));
    vcard_phone_entry.set_hexpand(true);
    vcard_phone_entry.set_valign(Align::Center);
    vcard_phone_row.append(&vcard_phone_entry);
    vcard_box.append(&vcard_phone_row);
    let vcard_email_entry = Entry::new();
    vcard_email_entry.set_placeholder_text(Some(&i18n.t("vcard_email")));
    vcard_email_entry.set_tooltip_text(Some(&i18n.t("tooltip_vcard_email")));
    vcard_box.append(&vcard_email_entry);
    let vcard_email_hint = Label::new(None);
    vcard_email_hint.add_css_class("input-error-hint");
    vcard_email_hint.set_visible(false);
    vcard_email_hint.set_xalign(0.0);
    vcard_box.append(&vcard_email_hint);
    let vcard_org_entry = Entry::new();
    vcard_org_entry.set_placeholder_text(Some(&i18n.t("vcard_org")));
    vcard_org_entry.set_tooltip_text(Some(&i18n.t("tooltip_vcard_org")));
    vcard_box.append(&vcard_org_entry);
    let vcard_url_entry = Entry::new();
    vcard_url_entry.set_placeholder_text(Some(&i18n.t("vcard_url")));
    vcard_url_entry.set_tooltip_text(Some(&i18n.t("tooltip_vcard_url")));
    vcard_box.append(&vcard_url_entry);
    content_stack.add_named(&vcard_box, Some("vcard"));

    // Calendar box
    let calendar_box = Box::new(Orientation::Vertical, 4);
    let cal_title_entry = Entry::new();
    cal_title_entry.set_placeholder_text(Some(&i18n.t("cal_title")));
    cal_title_entry.set_tooltip_text(Some(&i18n.t("tooltip_cal_title")));
    calendar_box.append(&cal_title_entry);

    // Helper: parse YYYYMMDDTHHMMSS and set calendar + spinbutton widgets
    let set_cal_from_string = |cal: &Calendar, hour: &SpinButton, minute: &SpinButton, s: &str| {
        if s.len() >= 8 {
            if let (Some(y), Some(m), Some(d)) = (
                s.get(0..4).and_then(|v| v.parse::<i32>().ok()),
                s.get(4..6).and_then(|v| v.parse::<i32>().ok()),
                s.get(6..8).and_then(|v| v.parse::<i32>().ok()),
            ) {
                let h = s
                    .get(9..11)
                    .and_then(|v| v.parse::<f64>().ok())
                    .unwrap_or(0.0);
                let min = s
                    .get(11..13)
                    .and_then(|v| v.parse::<f64>().ok())
                    .unwrap_or(0.0);
                if let Ok(dt) = glib::DateTime::from_local(y, m, d, h as i32, min as i32, 0.0) {
                    cal.select_day(&dt);
                }
                hour.set_value(h);
                minute.set_value(min);
            }
        }
    };

    // --- Start date ---
    let cal_start_label = Label::new(Some(&i18n.t("label_start_date")));
    cal_start_label.add_css_class("heading");
    cal_start_label.set_halign(Align::Start);
    calendar_box.append(&cal_start_label);

    let cal_start_calendar = Calendar::new();
    cal_start_calendar.add_css_class("calendar-rounded");
    calendar_box.append(&cal_start_calendar);

    let cal_start_time_row = Box::new(Orientation::Horizontal, 4);
    cal_start_time_row.append(&Label::new(Some(&i18n.t("label_time"))));
    let cal_start_hour = SpinButton::with_range(0.0, 23.0, 1.0);
    cal_start_hour.set_tooltip_text(Some(&i18n.t("tooltip_cal_hour")));
    cal_start_time_row.append(&cal_start_hour);
    cal_start_time_row.append(&Label::new(Some(":")));
    let cal_start_minute = SpinButton::with_range(0.0, 59.0, 1.0);
    cal_start_minute.set_tooltip_text(Some(&i18n.t("tooltip_cal_minute")));
    cal_start_time_row.append(&cal_start_minute);
    calendar_box.append(&cal_start_time_row);

    // --- End date ---
    let cal_end_label = Label::new(Some(&i18n.t("label_end_date")));
    cal_end_label.add_css_class("heading");
    cal_end_label.set_halign(Align::Start);
    calendar_box.append(&cal_end_label);

    let cal_end_calendar = Calendar::new();
    cal_end_calendar.add_css_class("calendar-rounded");
    calendar_box.append(&cal_end_calendar);

    let cal_end_time_row = Box::new(Orientation::Horizontal, 4);
    cal_end_time_row.append(&Label::new(Some(&i18n.t("label_time"))));
    let cal_end_hour = SpinButton::with_range(0.0, 23.0, 1.0);
    cal_end_hour.set_tooltip_text(Some(&i18n.t("tooltip_cal_hour")));
    cal_end_time_row.append(&cal_end_hour);
    cal_end_time_row.append(&Label::new(Some(":")));
    let cal_end_minute = SpinButton::with_range(0.0, 59.0, 1.0);
    cal_end_minute.set_tooltip_text(Some(&i18n.t("tooltip_cal_minute")));
    cal_end_time_row.append(&cal_end_minute);
    calendar_box.append(&cal_end_time_row);

    let cal_location_entry = Entry::new();
    cal_location_entry.set_placeholder_text(Some(&i18n.t("cal_location")));
    cal_location_entry.set_tooltip_text(Some(&i18n.t("tooltip_cal_location")));
    calendar_box.append(&cal_location_entry);
    content_stack.add_named(&calendar_box, Some("calendar"));

    // GPS box — embedded OpenStreetMap via libshumate
    let gps_box = Box::new(Orientation::Vertical, 4);
    gps_box.set_vexpand(true);

    // Map widget inside an Overlay (for OSD theme toggle)
    let gps_map = Map::new();
    gps_map.set_size_request(-1, 200);
    gps_map.set_hexpand(true);
    gps_map.set_vexpand(true);
    gps_map.add_css_class("gps-map");

    // OpenFreeMap vector tiles with language-aware labels (fallback: CartoDB raster)
    let gps_map_style: Rc<RefCell<crate::map_styles::MapStyle>> = Rc::new(RefCell::new(
        crate::map_styles::MapStyle::default_for_system(),
    ));
    let style_json = crate::map_styles::get_map_style(*gps_map_style.borrow(), lang);
    let vector_ok: Rc<RefCell<bool>> = Rc::new(RefCell::new(false));
    let gps_map_layer: Rc<RefCell<Option<MapLayer>>> = Rc::new(RefCell::new(None));

    gps_map.center_on(52.0, 10.0);

    // Try vector tiles first, fall back to raster
    match VectorRenderer::new("ofm", &style_json) {
        Ok(source) => {
            *vector_ok.borrow_mut() = true;
            if let Some(vp) = gps_map.viewport() {
                vp.set_zoom_level(3.5);
                vp.set_reference_map_source(Some(&source));
                let map_layer = MapLayer::new(&source, &vp);
                gps_map.add_layer(&map_layer);
                *gps_map_layer.borrow_mut() = Some(map_layer);
            }
        }
        Err(_e) => {
            let url = if gps_map_style.borrow().is_dark_style() {
                "https://basemaps.cartocdn.com/dark_all/{z}/{x}/{y}.png"
            } else {
                "https://basemaps.cartocdn.com/light_all/{z}/{x}/{y}.png"
            };
            let source = RasterRenderer::from_url(url);
            if let Some(vp) = gps_map.viewport() {
                vp.set_zoom_level(3.5);
                vp.set_reference_map_source(Some(&source));
                let map_layer = MapLayer::new(&source, &vp);
                gps_map.add_layer(&map_layer);
                *gps_map_layer.borrow_mut() = Some(map_layer);
            }
        }
    }

    // Marker at current position
    let gps_marker = Marker::new();
    gps_marker.set_location(52.0, 10.0);
    let gps_pin = Label::new(Some("📍"));
    gps_marker.set_child(Some(&gps_pin));
    // Store marker layer so we can re-add it on top after style switches
    let gps_marker_layer: Rc<RefCell<Option<MarkerLayer>>> = Rc::new(RefCell::new(None));
    if let Some(vp) = gps_map.viewport() {
        let ml = MarkerLayer::new(&vp);
        ml.add_marker(&gps_marker);
        gps_map.add_layer(&ml);
        *gps_marker_layer.borrow_mut() = Some(ml);
    }

    // ── OSD style picker: gear icon that expands to pill on hover ──────────
    let style_picker = Box::new(Orientation::Horizontal, 0);
    style_picker.add_css_class("gps-osd-picker");
    style_picker.set_halign(Align::End);
    style_picker.set_valign(Align::End);
    style_picker.set_margin_bottom(6);
    style_picker.set_margin_end(6);

    // Gear icon (always visible)
    let gear_btn = Label::new(Some("⚙️"));
    gear_btn.add_css_class("gps-osd-gear");
    style_picker.append(&gear_btn);

    // Revealer: wraps the buttons and provides a smooth slide animation
    let style_revealer = gtk4::Revealer::new();
    style_revealer.set_transition_type(gtk4::RevealerTransitionType::SlideRight);
    style_revealer.set_transition_duration(300);
    style_revealer.set_reveal_child(false);
    style_revealer.set_overflow(gtk4::Overflow::Hidden);

    // Style name buttons inside the revealer
    let style_btns_box = Box::new(Orientation::Horizontal, 2);
    style_btns_box.add_css_class("gps-osd-style-btns");

    // Build one button per map style
    let all_styles = crate::map_styles::MapStyle::all();
    let mut style_buttons: Vec<Button> = Vec::new();
    for &s in all_styles {
        let btn = Button::with_label(s.label());
        btn.add_css_class("gps-osd-style-item");
        if s == *gps_map_style.borrow() {
            btn.add_css_class("gps-osd-style-active");
        }
        style_buttons.push(btn.clone());
        style_btns_box.append(&btn);
    }
    style_revealer.set_child(Some(&style_btns_box));
    style_picker.append(&style_revealer);

    // Hover detection: reveal / hide the buttons + spin the gear
    {
        let revealer_c = style_revealer.clone();
        let gear_c = gear_btn.clone();
        let motion = gtk4::EventControllerMotion::new();
        motion.connect_enter(move |_, _, _| {
            gear_c.add_css_class("gps-osd-gear-spin");
            revealer_c.set_reveal_child(true);
        });
        let revealer_c2 = style_revealer.clone();
        let gear_c2 = gear_btn.clone();
        motion.connect_leave(move |_| {
            gear_c2.remove_css_class("gps-osd-gear-spin");
            revealer_c2.set_reveal_child(false);
        });
        style_picker.add_controller(motion);
    }

    // Wrap map in Overlay
    let gps_overlay = Overlay::new();
    gps_overlay.set_child(Some(&gps_map));
    gps_overlay.add_overlay(&style_picker);
    gps_box.append(&gps_overlay);

    // OSD picker CSS
    {
        let osd_css = gtk4::CssProvider::new();
        osd_css.load_from_data(
            ".gps-osd-picker { \
               border-radius: 9999px; \
               background: alpha(@card_bg_color, 0.88); \
               color: @card_fg_color; \
               border: 1px solid alpha(currentColor, 0.12); \
               box-shadow: 0 2px 8px alpha(black, 0.25); \
               padding: 5px 9px; \
             } \
             .gps-osd-gear { \
               font-size: 1.15em; \
               padding: 0 1px; \
               transition: transform 0.4s ease-in-out; \
               transform-origin: center; \
             } \
             .gps-osd-gear-spin { \
               transform: rotate(360deg); \
             } \
             .gps-osd-style-item { \
               border-radius: 9999px; \
               padding: 2px 10px; \
               background: transparent; \
               color: @card_fg_color; \
               border: 1px solid transparent; \
               font-size: 0.78em; \
               font-weight: normal; \
               transition: all 150ms ease-out; \
             } \
             .gps-osd-style-item:hover { \
               background: alpha(@accent_color, 0.15); \
               border-color: alpha(@accent_color, 0.3); \
             } \
             .gps-osd-style-active { \
               background: alpha(@accent_color, 0.2); \
               border-color: @accent_color; \
               font-weight: bold; \
             }",
        );
        gtk4::style_context_add_provider_for_display(
            &gtk4::gdk::Display::default().unwrap(),
            &osd_css,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    // Style button click handlers: switch map style
    for (i, btn) in style_buttons.iter().enumerate() {
        let map_c = gps_map.clone();
        let map_layer_c = gps_map_layer.clone();
        let style_c = gps_map_style.clone();
        let lang_c = lang;
        let vector_ok_c = vector_ok.clone();
        let btn_c = btn.clone();
        let all_btns: Vec<Button> = style_buttons.iter().cloned().collect();
        btn.connect_clicked(move |_| {
            let new_style = crate::map_styles::MapStyle::all()[i];
            *style_c.borrow_mut() = new_style;

            // Update active highlighting
            for b in &all_btns {
                b.remove_css_class("gps-osd-style-active");
            }
            btn_c.add_css_class("gps-osd-style-active");

            // Adopt the GNOME Maps approach: insert new layer ABOVE old, then remove old.
            // This ensures there is never a frame without a tile layer.
            let old_layer = map_layer_c.borrow_mut().take();

            if *vector_ok_c.borrow() {
                let style_json = crate::map_styles::get_map_style(new_style, lang_c);
                match VectorRenderer::new("ofm", &style_json) {
                    Ok(new_source) => {
                        if let Some(vp) = map_c.viewport() {
                            vp.set_reference_map_source(Some(&new_source));
                            let new_layer = MapLayer::new(&new_source, &vp);

                            // Insert new layer above old (or just add if no old layer)
                            if let Some(ref old) = old_layer {
                                map_c.insert_layer_above(&new_layer, Some(old));
                                map_c.remove_layer(old);
                            } else {
                                map_c.add_layer(&new_layer);
                            }
                            *map_layer_c.borrow_mut() = Some(new_layer);
                        }
                    }
                    Err(_e) => {
                        // Restore old layer on failure
                        if let Some(ref old) = old_layer {
                            map_c.add_layer(old);
                            *map_layer_c.borrow_mut() = Some(old.clone());
                        }
                    }
                }
            } else {
                // Fallback: CartoDB raster tiles
                let url = if new_style.is_dark_style() {
                    "https://basemaps.cartocdn.com/dark_all/{z}/{x}/{y}.png"
                } else {
                    "https://basemaps.cartocdn.com/light_all/{z}/{x}/{y}.png"
                };
                let new_source = RasterRenderer::from_url(url);
                if let Some(vp) = map_c.viewport() {
                    vp.set_reference_map_source(Some(&new_source));
                    let new_layer = MapLayer::new(&new_source, &vp);

                    if let Some(ref old) = old_layer {
                        map_c.insert_layer_above(&new_layer, Some(old));
                        map_c.remove_layer(old);
                    } else {
                        map_c.add_layer(&new_layer);
                    }
                    *map_layer_c.borrow_mut() = Some(new_layer);
                }
            }
        });
    }

    // Location search entry (Nominatim geocoding)
    let gps_search_entry = Entry::new();
    gps_search_entry.set_placeholder_text(Some(&i18n.t("gps_search")));
    gps_search_entry.set_tooltip_text(Some(&i18n.t("tooltip_gps_search")));
    gps_box.append(&gps_search_entry);

    // Inline suggestion list (below search bar, not a popup)
    let gps_suggestions = gtk4::ListBox::new();
    gps_suggestions.add_css_class("gps-suggestions");
    gps_suggestions.set_selection_mode(gtk4::SelectionMode::None);
    let gps_suggestions_scroll = ScrolledWindow::new();
    gps_suggestions_scroll.set_max_content_height(160);
    gps_suggestions_scroll.set_propagate_natural_height(true);
    gps_suggestions_scroll.set_visible(false);
    gps_suggestions_scroll.set_child(Some(&gps_suggestions));
    gps_box.append(&gps_suggestions_scroll);

    // Parallel Vec with coordinates for each suggestion row
    let gps_suggestion_coords: Rc<RefCell<Vec<(f64, f64)>>> = Rc::new(RefCell::new(Vec::new()));

    let gps_lat_entry = Entry::new();
    gps_lat_entry.set_placeholder_text(Some(&i18n.t("gps_lat")));
    gps_lat_entry.set_tooltip_text(Some(&i18n.t("tooltip_gps_lat")));
    gps_box.append(&gps_lat_entry);
    let gps_lat_hint = Label::new(None);
    gps_lat_hint.add_css_class("input-error-hint");
    gps_lat_hint.set_visible(false);
    gps_lat_hint.set_xalign(0.0);
    gps_box.append(&gps_lat_hint);
    let gps_lon_entry = Entry::new();
    gps_lon_entry.set_placeholder_text(Some(&i18n.t("gps_lon")));
    gps_lon_entry.set_tooltip_text(Some(&i18n.t("tooltip_gps_lon")));
    gps_box.append(&gps_lon_entry);
    let gps_lon_hint = Label::new(None);
    gps_lon_hint.add_css_class("input-error-hint");
    gps_lon_hint.set_visible(false);
    gps_lon_hint.set_xalign(0.0);
    gps_box.append(&gps_lon_hint);

    content_stack.add_named(&gps_box, Some("gps"));

    // SMS box
    let sms_box = Box::new(Orientation::Vertical, 4);
    // SMS Phone with country code selector (intl-tel-input style)
    let sms_phone_row = Box::new(Orientation::Horizontal, 4);
    let sms_country_dd = create_country_dropdown();
    sms_country_dd.set_valign(Align::Center);
    sms_phone_row.append(&sms_country_dd);
    let sms_phone_entry = Entry::new();
    sms_phone_entry.set_placeholder_text(Some(&i18n.t("sms_phone")));
    sms_phone_entry.set_tooltip_text(Some(&i18n.t("tooltip_sms_phone")));
    sms_phone_entry.set_hexpand(true);
    sms_phone_entry.set_valign(Align::Center);
    sms_phone_row.append(&sms_phone_entry);
    sms_box.append(&sms_phone_row);
    let sms_phone_hint = Label::new(None);
    sms_phone_hint.add_css_class("input-error-hint");
    sms_phone_hint.set_visible(false);
    sms_phone_hint.set_xalign(0.0);
    sms_box.append(&sms_phone_hint);
    let sms_message_entry = Entry::new();
    sms_message_entry.set_placeholder_text(Some(&i18n.t("sms_message")));
    sms_message_entry.set_tooltip_text(Some(&i18n.t("tooltip_sms_message")));
    sms_box.append(&sms_message_entry);
    content_stack.add_named(&sms_box, Some("sms"));

    tab_content_box.append(&content_box);

    // ============================================================
    // SECTION: Vorlagen (unified: Style + optional Content)
    // ============================================================
    let template_vbox = Box::new(Orientation::Vertical, 4);
    template_vbox.set_margin_start(12);
    template_vbox.set_margin_end(12);
    template_vbox.set_margin_top(8);
    template_vbox.set_margin_bottom(8);

    // Built-in quick-style presets dropdown
    let preset_list = StringList::new(&[]);
    preset_list.append(&i18n.t("dd_preset_custom"));
    preset_list.append(&i18n.t("dd_preset_classic"));
    preset_list.append(&i18n.t("dd_preset_rounded"));
    preset_list.append(&i18n.t("dd_preset_dots"));
    preset_list.append(&i18n.t("dd_preset_diamond"));
    preset_list.append(&i18n.t("dd_preset_minimal"));
    preset_list.append(&i18n.t("dd_preset_retro"));
    let preset_dd = DropDown::new(
        Some(preset_list.clone().upcast::<gtk4::gio::ListModel>()),
        None::<gtk4::Expression>,
    );
    preset_dd.set_tooltip_text(Some(&i18n.t("tooltip_preset_select")));
    template_vbox.append(&preset_dd);

    // Separator
    template_vbox.append(&Separator::new(Orientation::Horizontal));

    // Saved template name entry + save button
    let template_save_row = Box::new(Orientation::Horizontal, 4);
    let template_name_entry = Entry::new();
    template_name_entry.set_placeholder_text(Some(&i18n.t("placeholder_template_name")));
    template_name_entry.set_hexpand(true);
    template_save_row.append(&template_name_entry);
    let template_save_btn = Button::new();
    template_save_btn.set_child(Some(&Image::from_icon_name("document-save-symbolic")));
    template_save_btn.set_tooltip_text(Some(&i18n.t("tooltip_template_save")));
    template_save_row.append(&template_save_btn);
    template_vbox.append(&template_save_row);

    // Checkbox: include content when saving
    let save_content_check = CheckButton::new();
    save_content_check.set_label(Some(&i18n.t("check_save_content")));
    save_content_check.set_tooltip_text(Some(&i18n.t("tooltip_save_content")));
    template_vbox.append(&save_content_check);

    // Unified saved templates dropdown + delete button
    let template_row = Box::new(Orientation::Horizontal, 4);
    let template_list = StringList::new(&[]);
    let template_dd = DropDown::new(
        Some(template_list.clone().upcast::<gtk4::gio::ListModel>()),
        None::<gtk4::Expression>,
    );
    template_dd.set_tooltip_text(Some(&i18n.t("tooltip_template_load")));
    template_dd.set_hexpand(true);
    template_row.append(&template_dd);
    let template_delete_btn = Button::new();
    template_delete_btn.set_child(Some(&Image::from_icon_name("user-trash-symbolic")));
    template_delete_btn.set_tooltip_text(Some(&i18n.t("tooltip_template_delete")));
    template_row.append(&template_delete_btn);
    template_vbox.append(&template_row);

    style_detail_stack.add_titled(&template_vbox, Some("templates"), &i18n.t("exp_templates"));

    // ============================================================
    // SECTION: Muster & Ecken
    // ============================================================
    let pattern_box = Box::new(Orientation::Vertical, 4);
    pattern_box.set_margin_start(12);
    pattern_box.set_margin_end(12);
    pattern_box.set_margin_top(8);
    pattern_box.set_margin_bottom(8);

    let dot_styles = StringList::new(&[]);
    dot_styles.append(&i18n.t("dd_dot_rounded"));
    dot_styles.append(&i18n.t("dd_dot_square"));
    dot_styles.append(&i18n.t("dd_dot_dots"));
    dot_styles.append(&i18n.t("dd_dot_diamond"));
    dot_styles.append(&i18n.t("dd_dot_custom"));
    let dot_style_dd = DropDown::new(
        Some(dot_styles.clone().upcast::<gtk4::gio::ListModel>()),
        None::<gtk4::Expression>,
    );
    dot_style_dd.set_tooltip_text(Some(&i18n.t("tooltip_dot_style")));
    pattern_box.append(&dot_style_dd);

    let corner_sq_styles = StringList::new(&[]);
    corner_sq_styles.append(&i18n.t("dd_corner_sq_rounded"));
    corner_sq_styles.append(&i18n.t("dd_corner_sq_square"));
    corner_sq_styles.append(&i18n.t("dd_corner_sq_dot"));
    corner_sq_styles.append(&i18n.t("dd_corner_sq_circle"));
    let corner_sq_dd = DropDown::new(
        Some(corner_sq_styles.clone().upcast::<gtk4::gio::ListModel>()),
        None::<gtk4::Expression>,
    );
    corner_sq_dd.set_tooltip_text(Some(&i18n.t("tooltip_corner_sq_style")));
    pattern_box.append(&corner_sq_dd);

    let corner_dot_styles = StringList::new(&[]);
    corner_dot_styles.append(&i18n.t("dd_corner_dot_dot"));
    corner_dot_styles.append(&i18n.t("dd_corner_dot_square"));
    corner_dot_styles.append(&i18n.t("dd_corner_dot_circle"));
    corner_dot_styles.append(&i18n.t("dd_corner_dot_rounded"));
    let corner_dot_dd = DropDown::new(
        Some(corner_dot_styles.clone().upcast::<gtk4::gio::ListModel>()),
        None::<gtk4::Expression>,
    );
    corner_dot_dd.set_tooltip_text(Some(&i18n.t("tooltip_corner_dot_style")));
    pattern_box.append(&corner_dot_dd);

    // Custom dot path entry (visible only when "Benutzerdefiniert" selected)
    let custom_dot_box = Box::new(Orientation::Vertical, 4);
    custom_dot_box.set_visible(false);
    let custom_dot_label = Label::new(Some(&i18n.t("label_svg_path")));
    custom_dot_label.set_xalign(0.0);
    custom_dot_box.append(&custom_dot_label);
    let custom_dot_entry = Entry::new();
    custom_dot_entry.set_placeholder_text(Some(&i18n.t("placeholder_custom_dot")));
    custom_dot_entry.set_tooltip_text(Some(&i18n.t("tooltip_custom_dot_svg")));
    custom_dot_box.append(&custom_dot_entry);
    let custom_dot_hint = Label::new(Some(&i18n.t("label_custom_dot_hint")));
    custom_dot_hint.set_wrap(true);
    custom_dot_hint.set_xalign(0.0);
    custom_dot_hint.add_css_class("caption");
    custom_dot_hint.set_sensitive(false);
    custom_dot_box.append(&custom_dot_hint);
    pattern_box.append(&custom_dot_box);

    style_detail_stack.add_titled(&pattern_box, Some("pattern"), &i18n.t("exp_pattern"));

    // ============================================================
    // SECTION: Farben
    // ============================================================
    let colors_box = Box::new(Orientation::Vertical, 4);
    colors_box.set_margin_start(12);
    colors_box.set_margin_end(12);
    colors_box.set_margin_top(8);
    colors_box.set_margin_bottom(8);

    // Contrast warning label (hidden by default)
    let contrast_warning_label = Label::new(Some(""));
    contrast_warning_label.set_visible(false);
    contrast_warning_label.set_wrap(true);
    contrast_warning_label.set_xalign(0.0);
    contrast_warning_label.add_css_class("warning");
    colors_box.append(&contrast_warning_label);

    let fg_color_row = Box::new(Orientation::Horizontal, 6);
    fg_color_row.set_halign(Align::Fill);
    let fg_color_label = Label::new(Some(&i18n.t("color_fg")));
    fg_color_label.set_xalign(0.0);
    fg_color_label.set_hexpand(true);
    let fg_color_btn = ColorButton::with_rgba(&gdk::RGBA::new(
        15.0 / 255.0,
        23.0 / 255.0,
        42.0 / 255.0,
        1.0,
    ));
    fg_color_btn.set_tooltip_text(Some(&i18n.t("color_fg")));
    fg_color_btn.add_css_class("color-btn-hover");
    fg_color_row.append(&fg_color_label);
    fg_color_row.append(&fg_color_btn);
    colors_box.append(&fg_color_row);

    // --- Farbharmonien ---
    let harmony_label = Label::new(Some(&i18n.t("harmony_title")));
    harmony_label.set_xalign(0.0);
    harmony_label.add_css_class("dim-label");
    colors_box.append(&harmony_label);

    let harmony_box = Box::new(Orientation::Horizontal, 4);
    let initial_harmonies = color_harmonies(Rgba([15, 23, 42, 255]));
    let harmony_buttons: Vec<ColorButton> = initial_harmonies
        .iter()
        .map(|(name, color)| {
            let btn = ColorButton::with_rgba(&gdk::RGBA::new(
                color.0[0] as f32 / 255.0,
                color.0[1] as f32 / 255.0,
                color.0[2] as f32 / 255.0,
                color.0[3] as f32 / 255.0,
            ));
            btn.set_tooltip_text(Some(name));
            btn.add_css_class("color-btn-hover");
            btn
        })
        .collect();
    for btn in &harmony_buttons {
        harmony_box.append(btn);
    }

    colors_box.append(&harmony_box);

    let bg_color_row = Box::new(Orientation::Horizontal, 6);
    bg_color_row.set_halign(Align::Fill);
    let bg_color_label = Label::new(Some(&i18n.t("color_bg")));
    bg_color_label.set_xalign(0.0);
    bg_color_label.set_hexpand(true);
    let bg_color_btn = ColorButton::with_rgba(&gdk::RGBA::new(1.0, 1.0, 1.0, 1.0));
    bg_color_btn.set_tooltip_text(Some(&i18n.t("color_bg")));
    bg_color_btn.add_css_class("color-btn-hover");
    bg_color_row.append(&bg_color_label);
    bg_color_row.append(&bg_color_btn);
    colors_box.append(&bg_color_row);

    let corner_color_row = Box::new(Orientation::Horizontal, 6);
    corner_color_row.set_halign(Align::Fill);
    let corner_color_label = Label::new(Some(&i18n.t("color_corner")));
    corner_color_label.set_xalign(0.0);
    corner_color_label.set_hexpand(true);
    let corner_color_btn = ColorButton::with_rgba(&gdk::RGBA::new(
        15.0 / 255.0,
        23.0 / 255.0,
        42.0 / 255.0,
        1.0,
    ));
    corner_color_btn.set_tooltip_text(Some(&i18n.t("color_corner")));
    corner_color_btn.add_css_class("color-btn-hover");
    corner_color_row.append(&corner_color_label);
    corner_color_row.append(&corner_color_btn);
    colors_box.append(&corner_color_row);

    let transparent_bg_check = CheckButton::with_label(&i18n.t("check_transparent_bg"));
    transparent_bg_check.set_tooltip_text(Some(&i18n.t("tooltip_transparent_bg")));
    colors_box.append(&transparent_bg_check);

    let gradient_check = CheckButton::with_label(&i18n.t("check_gradient"));
    gradient_check.set_tooltip_text(Some(&i18n.t("tooltip_gradient_enable")));
    colors_box.append(&gradient_check);

    let grad_color_row = Box::new(Orientation::Horizontal, 6);
    grad_color_row.set_halign(Align::Fill);
    let grad_color_label = Label::new(Some(&i18n.t("color_gradient")));
    grad_color_label.set_xalign(0.0);
    grad_color_label.set_hexpand(true);
    let grad_color_btn = ColorButton::with_rgba(&gdk::RGBA::new(
        102.0 / 255.0,
        51.0 / 255.0,
        204.0 / 255.0,
        1.0,
    ));
    grad_color_btn.set_tooltip_text(Some(&i18n.t("color_gradient")));
    grad_color_btn.add_css_class("color-btn-hover");
    grad_color_row.append(&grad_color_label);
    grad_color_row.append(&grad_color_btn);
    colors_box.append(&grad_color_row);

    let grad_dirs = StringList::new(&[]);
    grad_dirs.append(&i18n.t("dd_grad_horizontal"));
    grad_dirs.append(&i18n.t("dd_grad_vertical"));
    grad_dirs.append(&i18n.t("dd_grad_diagonal"));
    grad_dirs.append(&i18n.t("dd_grad_radial"));
    let grad_dir_dd = DropDown::new(
        Some(grad_dirs.clone().upcast::<gtk4::gio::ListModel>()),
        None::<gtk4::Expression>,
    );
    grad_dir_dd.set_tooltip_text(Some(&i18n.t("tooltip_gradient_dir")));
    colors_box.append(&grad_dir_dd);

    let palette_list = StringList::new(&[]);
    palette_list.append(&i18n.t("dd_palette_custom"));
    palette_list.append(&i18n.t("dd_palette_classic"));
    palette_list.append(&i18n.t("dd_palette_ocean"));
    palette_list.append(&i18n.t("dd_palette_sunset"));
    palette_list.append(&i18n.t("dd_palette_forest"));
    palette_list.append(&i18n.t("dd_palette_lavender"));
    palette_list.append(&i18n.t("dd_palette_fire"));
    palette_list.append(&i18n.t("dd_palette_aurora"));
    palette_list.append(&i18n.t("dd_palette_pastel"));
    palette_list.append(&i18n.t("dd_palette_neon"));
    let palette_dd = DropDown::new(
        Some(palette_list.clone().upcast::<gtk4::gio::ListModel>()),
        None::<gtk4::Expression>,
    );
    palette_dd.set_tooltip_text(Some(&i18n.t("tooltip_palette")));
    colors_box.append(&palette_dd);

    style_detail_stack.add_titled(&colors_box, Some("colors"), &i18n.t("exp_colors"));

    // ============================================================
    // SECTION: Einstellungen
    // ============================================================
    let settings_box = Box::new(Orientation::Vertical, 4);
    settings_box.set_margin_start(12);
    settings_box.set_margin_end(12);
    settings_box.set_margin_top(8);
    settings_box.set_margin_bottom(8);

    let ec_levels = StringList::new(&[]);
    ec_levels.append(&i18n.t("dd_ec_medium"));
    ec_levels.append(&i18n.t("dd_ec_low"));
    ec_levels.append(&i18n.t("dd_ec_quartile"));
    ec_levels.append(&i18n.t("dd_ec_high"));
    let ec_level_dd = DropDown::new(
        Some(ec_levels.clone().upcast::<gtk4::gio::ListModel>()),
        None::<gtk4::Expression>,
    );
    ec_level_dd.set_tooltip_text(Some(&i18n.t("tooltip_ec_level")));
    settings_box.append(&ec_level_dd);

    let module_sizes = StringList::new(&[]);
    module_sizes.append(&i18n.t("dd_module_medium"));
    module_sizes.append(&i18n.t("dd_module_small"));
    module_sizes.append(&i18n.t("dd_module_large"));
    module_sizes.append(&i18n.t("dd_module_print"));
    let module_size_dd = DropDown::new(
        Some(module_sizes.clone().upcast::<gtk4::gio::ListModel>()),
        None::<gtk4::Expression>,
    );
    module_size_dd.set_tooltip_text(Some(&i18n.t("tooltip_module_size")));
    settings_box.append(&module_size_dd);

    let quiet_zone_label = Label::new(Some(&i18n.t("label_quiet_zone")));
    settings_box.append(&quiet_zone_label);
    let quiet_zone_scale = Scale::with_range(Orientation::Horizontal, 0.0, 10.0, 1.0);
    quiet_zone_scale.set_draw_value(true);
    quiet_zone_scale.set_has_origin(false);
    quiet_zone_scale.set_format_value_func(move |_, v| format!("{:.0}", v));
    quiet_zone_scale.set_value(4.0);
    quiet_zone_scale.set_tooltip_text(Some(&i18n.t("tooltip_quiet_zone")));
    settings_box.append(&quiet_zone_scale);

    let gap_label = Label::new(Some(&i18n.t("label_module_gap")));
    settings_box.append(&gap_label);
    let module_gap_scale = Scale::with_range(Orientation::Horizontal, 0.0, 0.4, 0.05);
    module_gap_scale.set_draw_value(true);
    module_gap_scale.set_has_origin(false);
    module_gap_scale.set_format_value_func(move |_, v| format!("{:.2}", v));
    module_gap_scale.set_value(0.0);
    module_gap_scale.set_tooltip_text(Some(&i18n.t("tooltip_module_gap")));
    settings_box.append(&module_gap_scale);

    style_detail_stack.add_titled(&settings_box, Some("settings"), &i18n.t("exp_settings"));

    // ============================================================
    // SECTION: QR-Info
    // ============================================================
    let qr_info_section_label = Label::new(Some(&i18n.t("exp_qr_info")));
    qr_info_section_label.add_css_class("heading");
    qr_info_section_label.set_halign(Align::Start);
    qr_info_section_label.set_margin_top(8);
    qr_info_section_label.set_margin_bottom(4);
    tab_content_box.append(&qr_info_section_label);
    let qr_info_label = Label::new(Some("—"));
    qr_info_label.set_selectable(true);
    qr_info_label.set_wrap(true);
    qr_info_label.set_xalign(0.0);
    tab_content_box.append(&qr_info_label);

    let qr_capacity_bar = ProgressBar::new();
    qr_capacity_bar.set_hexpand(true);
    qr_capacity_bar.set_show_text(false);
    qr_capacity_bar.set_margin_top(2);
    qr_capacity_bar.set_margin_bottom(4);
    tab_content_box.append(&qr_capacity_bar);

    // ============================================================
    // SECTION: Erweitert
    // ============================================================
    let advanced_box = Box::new(Orientation::Vertical, 4);
    advanced_box.set_margin_start(12);
    advanced_box.set_margin_end(12);
    advanced_box.set_margin_top(8);
    advanced_box.set_margin_bottom(8);

    let shadow_check = CheckButton::with_label(&i18n.t("check_shadow"));
    shadow_check.set_tooltip_text(Some(&i18n.t("tooltip_shadow_enable")));
    advanced_box.append(&shadow_check);

    let shadow_offset_label = Label::new(Some(&i18n.t("label_shadow_offset")));
    advanced_box.append(&shadow_offset_label);
    let shadow_offset_scale = Scale::with_range(Orientation::Horizontal, 1.0, 5.0, 0.5);
    shadow_offset_scale.set_draw_value(true);
    shadow_offset_scale.set_has_origin(false);
    shadow_offset_scale.set_format_value_func(move |_, v| format!("{:.1} px", v));
    shadow_offset_scale.set_value(2.0);
    shadow_offset_scale.set_tooltip_text(Some(&i18n.t("tooltip_shadow_offset")));
    advanced_box.append(&shadow_offset_scale);

    style_detail_stack.add_titled(&advanced_box, Some("advanced"), &i18n.t("exp_advanced"));

    // ============================================================
    // SECTION: Logo (Zentrum)
    // ============================================================
    let logo_box = Box::new(Orientation::Vertical, 4);
    logo_box.set_margin_start(12);
    logo_box.set_margin_end(12);
    logo_box.set_margin_top(8);
    logo_box.set_margin_bottom(8);

    let logo_btn_box = Box::new(Orientation::Horizontal, 4);
    let logo_select_btn = make_icon_btn("image-x-generic-symbolic", &i18n.t("btn_select_image"));
    logo_select_btn.set_tooltip_text(Some(&i18n.t("tooltip_logo_select")));
    let logo_remove_btn = make_icon_btn("edit-clear-symbolic", &i18n.t("btn_remove"));
    logo_remove_btn.set_tooltip_text(Some(&i18n.t("tooltip_logo_remove")));
    logo_btn_box.append(&logo_select_btn);
    logo_btn_box.append(&logo_remove_btn);
    logo_box.append(&logo_btn_box);

    let logo_size_label = Label::new(Some(&i18n.t("label_logo_size")));
    logo_box.append(&logo_size_label);
    let logo_size_scale = Scale::with_range(Orientation::Horizontal, 0.1, 0.6, 0.05);
    logo_size_scale.set_draw_value(true);
    logo_size_scale.set_has_origin(false);
    logo_size_scale.set_format_value_func(move |_, v| format!("{:.0} %", v * 100.0));
    logo_size_scale.set_value(0.4);
    logo_size_scale.set_tooltip_text(Some(&i18n.t("tooltip_logo_size")));
    logo_box.append(&logo_size_scale);

    let logo_shapes = StringList::new(&[]);
    logo_shapes.append(&i18n.t("dd_logo_circle"));
    logo_shapes.append(&i18n.t("dd_logo_rectangle"));
    logo_shapes.append(&i18n.t("dd_logo_rounded"));
    let logo_shape_dd = DropDown::new(
        Some(logo_shapes.clone().upcast::<gtk4::gio::ListModel>()),
        None::<gtk4::Expression>,
    );
    logo_shape_dd.set_tooltip_text(Some(&i18n.t("tooltip_logo_shape")));
    logo_box.append(&logo_shape_dd);

    let logo_outer_radius_box = Box::new(Orientation::Vertical, 2);
    let logo_outer_radius_label = Label::new(Some(&i18n.t("label_outer_radius")));
    let logo_outer_radius_scale = Scale::with_range(Orientation::Horizontal, 0.0, 0.5, 0.01);
    logo_outer_radius_scale.set_draw_value(true);
    logo_outer_radius_scale.set_has_origin(false);
    logo_outer_radius_scale.set_format_value_func(move |_, v| format!("{:.0} %", v * 100.0));
    logo_outer_radius_scale.set_value(0.15);
    logo_outer_radius_scale.set_tooltip_text(Some(&i18n.t("tooltip_outer_radius")));
    logo_outer_radius_box.append(&logo_outer_radius_label);
    logo_outer_radius_box.append(&logo_outer_radius_scale);
    logo_outer_radius_box.set_visible(false);
    logo_box.append(&logo_outer_radius_box);

    let logo_radius_sync_btn = CheckButton::with_label(&i18n.t("check_radius_sync"));
    logo_radius_sync_btn.set_tooltip_text(Some(&i18n.t("tooltip_logo_radius_sync")));
    logo_radius_sync_btn.set_active(true);
    logo_radius_sync_btn.set_visible(false);
    logo_box.append(&logo_radius_sync_btn);

    let logo_inner_radius_box = Box::new(Orientation::Vertical, 2);
    let logo_inner_radius_label = Label::new(Some(&i18n.t("label_inner_radius")));
    let logo_inner_radius_scale = Scale::with_range(Orientation::Horizontal, 0.0, 0.5, 0.01);
    logo_inner_radius_scale.set_draw_value(true);
    logo_inner_radius_scale.set_has_origin(false);
    logo_inner_radius_scale.set_format_value_func(move |_, v| format!("{:.0} %", v * 100.0));
    logo_inner_radius_scale.set_value(0.15);
    logo_inner_radius_scale.set_tooltip_text(Some(&i18n.t("tooltip_inner_radius")));
    logo_inner_radius_box.append(&logo_inner_radius_label);
    logo_inner_radius_box.append(&logo_inner_radius_scale);
    logo_inner_radius_box.set_visible(false);
    logo_box.append(&logo_inner_radius_box);

    let logo_color_btn = ColorButton::with_rgba(&gdk::RGBA::new(0.0, 0.0, 0.0, 0.0));
    logo_color_btn.set_tooltip_text(Some(&i18n.t("tooltip_logo_color")));
    logo_box.append(&logo_color_btn);

    let logo_border_width_label = Label::new(Some(&i18n.t("label_logo_border_width")));
    logo_box.append(&logo_border_width_label);
    let logo_border_width_scale = Scale::with_range(Orientation::Horizontal, 0.0, 20.0, 1.0);
    logo_border_width_scale.set_draw_value(true);
    logo_border_width_scale.set_has_origin(false);
    logo_border_width_scale.set_format_value_func(move |_, v| format!("{:.0} px", v));
    logo_border_width_scale.set_value(0.0);
    logo_border_width_scale.set_tooltip_text(Some(&i18n.t("tooltip_logo_border_width")));
    logo_box.append(&logo_border_width_scale);

    let logo_border_color_btn = ColorButton::with_rgba(&gdk::RGBA::new(1.0, 1.0, 1.0, 1.0));
    logo_border_color_btn.set_tooltip_text(Some(&i18n.t("tooltip_logo_border_color")));
    logo_box.append(&logo_border_color_btn);

    let logo_vectorize_check = CheckButton::with_label(&i18n.t("check_logo_vectorize"));
    logo_vectorize_check.set_tooltip_text(Some(&i18n.t("tooltip_logo_vectorize")));
    logo_vectorize_check.set_active(false);
    logo_box.append(&logo_vectorize_check);

    let logo_vectorize_bg_color_btn = ColorButton::with_rgba(&gdk::RGBA::new(0.0, 0.0, 0.0, 0.0));
    logo_vectorize_bg_color_btn.set_use_alpha(true);
    logo_vectorize_bg_color_btn.set_tooltip_text(Some(&i18n.t("tooltip_logo_vectorize_bg")));
    logo_box.append(&logo_vectorize_bg_color_btn);

    let logo_bg_transparent_check = CheckButton::with_label(&i18n.t("check_logo_bg_transparent"));
    logo_bg_transparent_check.set_tooltip_text(Some(&i18n.t("tooltip_logo_bg_transparent")));
    logo_bg_transparent_check.set_active(false);
    logo_box.append(&logo_bg_transparent_check);

    let logo_clear_area_check = CheckButton::with_label(&i18n.t("check_logo_clear_area"));
    logo_clear_area_check.set_tooltip_text(Some(&i18n.t("tooltip_logo_clear_area")));
    logo_clear_area_check.set_active(true);
    logo_box.append(&logo_clear_area_check);

    let padding_box = Box::new(Orientation::Horizontal, 4);
    let padding_label = Label::new(Some(&i18n.t("label_logo_padding")));
    let logo_clear_padding_spin = SpinButton::with_range(0.0, 3.0, 0.1);
    logo_clear_padding_spin.set_value(0.0);
    logo_clear_padding_spin.set_tooltip_text(Some(&i18n.t("tooltip_logo_padding")));
    logo_clear_padding_spin.set_digits(1);
    padding_box.append(&padding_label);
    padding_box.append(&logo_clear_padding_spin);
    logo_box.append(&padding_box);

    style_detail_stack.add_titled(&logo_box, Some("logo"), &i18n.t("exp_logo"));

    // ============================================================
    // SECTION: Text um den QR-Code
    // ============================================================
    let text_outer_box = Box::new(Orientation::Vertical, 4);
    text_outer_box.set_margin_start(12);
    text_outer_box.set_margin_end(12);
    text_outer_box.set_margin_top(8);
    text_outer_box.set_margin_bottom(8);

    let top_text_entry = Entry::new();
    top_text_entry.set_placeholder_text(Some(&i18n.t("placeholder_top_text")));
    top_text_entry.set_tooltip_text(Some(&i18n.t("tooltip_top_text")));
    text_outer_box.append(&top_text_entry);

    let bottom_text_entry = Entry::new();
    bottom_text_entry.set_placeholder_text(Some(&i18n.t("placeholder_bottom_text")));
    bottom_text_entry.set_tooltip_text(Some(&i18n.t("tooltip_bottom_text")));
    text_outer_box.append(&bottom_text_entry);

    let text_color_btn = ColorButton::with_rgba(&gdk::RGBA::new(0.0, 0.0, 0.0, 1.0));
    text_color_btn.set_tooltip_text(Some(&i18n.t("tooltip_text_color")));
    text_outer_box.append(&text_color_btn);

    // Font family dropdown
    let font_row = Box::new(Orientation::Horizontal, 6);
    font_row.set_halign(Align::Fill);
    let font_label = Label::new(Some(&i18n.t("label_font")));
    font_label.set_xalign(0.0);
    font_label.set_hexpand(true);
    font_row.append(&font_label);
    // Enumerate all system fonts via Pango
    let pango_ctx = font_label.pango_context();
    let mut system_fonts: Vec<String> = match pango_ctx.font_map() {
        Some(fm) => fm
            .list_families()
            .into_iter()
            .map(|f: gtk4::pango::FontFamily| f.name().to_string())
            .collect(),
        None => vec!["Sans".to_string()],
    };
    system_fonts.sort_by(|a: &String, b: &String| a.to_lowercase().cmp(&b.to_lowercase()));
    let font_str_refs: Vec<&str> = system_fonts.iter().map(|s: &String| s.as_str()).collect();
    let font_families = StringList::new(&font_str_refs);
    let font_expression = gtk4::PropertyExpression::new(
        gtk4::StringObject::static_type(),
        None::<&gtk4::Expression>,
        "string",
    );
    let font_dd = DropDown::new(
        Some(font_families.upcast::<gtk4::gio::ListModel>()),
        Some(font_expression.upcast()),
    );
    font_dd.set_enable_search(true);
    // Select "Sans" as default to match AppState default
    for (i, font) in system_fonts.iter().enumerate() {
        if font == "Sans" {
            font_dd.set_selected(i as u32);
            break;
        }
    }
    font_dd.set_tooltip_text(Some(&i18n.t("label_font")));
    font_row.append(&font_dd);
    text_outer_box.append(&font_row);

    // Font size spinner
    let font_size_row = Box::new(Orientation::Horizontal, 6);
    font_size_row.set_halign(Align::Fill);
    let font_size_label = Label::new(Some(&i18n.t("label_font_size")));
    font_size_label.set_xalign(0.0);
    font_size_label.set_hexpand(true);
    font_size_row.append(&font_size_label);
    let font_size_spin = SpinButton::with_range(8.0, 72.0, 1.0);
    font_size_spin.set_value(14.0);
    font_size_spin.set_tooltip_text(Some(&i18n.t("label_font_size")));
    font_size_row.append(&font_size_spin);
    text_outer_box.append(&font_size_row);

    style_detail_stack.add_titled(&text_outer_box, Some("text"), &i18n.t("exp_outer_text"));

    // ============================================================
    // SECTION: Rahmen
    // ============================================================
    let frame_box = Box::new(Orientation::Vertical, 4);
    frame_box.set_margin_start(12);
    frame_box.set_margin_end(12);
    frame_box.set_margin_top(8);
    frame_box.set_margin_bottom(8);

    let frame_styles = StringList::new(&[]);
    frame_styles.append(&i18n.t("dd_frame_none"));
    frame_styles.append(&i18n.t("dd_frame_simple"));
    frame_styles.append(&i18n.t("dd_frame_rounded"));
    frame_styles.append(&i18n.t("dd_frame_banner"));
    let frame_style_dd = DropDown::new(
        Some(frame_styles.clone().upcast::<gtk4::gio::ListModel>()),
        None::<gtk4::Expression>,
    );
    frame_style_dd.set_tooltip_text(Some(&i18n.t("tooltip_frame_style")));
    frame_box.append(&frame_style_dd);

    let frame_color_btn = ColorButton::with_rgba(&gdk::RGBA::new(0.0, 0.0, 0.0, 1.0));
    frame_color_btn.set_tooltip_text(Some(&i18n.t("tooltip_frame_color")));
    frame_box.append(&frame_color_btn);

    let frame_width_label = Label::new(Some(&i18n.t("label_frame_width")));
    frame_box.append(&frame_width_label);
    let frame_width_scale = Scale::with_range(Orientation::Horizontal, 1.0, 10.0, 1.0);
    frame_width_scale.set_draw_value(true);
    frame_width_scale.set_has_origin(false);
    frame_width_scale.set_format_value_func(move |_, v| format!("{:.0} px", v));
    frame_width_scale.set_value(2.0);
    frame_width_scale.set_tooltip_text(Some(&i18n.t("tooltip_frame_width")));
    frame_box.append(&frame_width_scale);

    let frame_outer_radius_box = Box::new(Orientation::Vertical, 2);
    let frame_outer_radius_label = Label::new(Some(&i18n.t("label_frame_outer_radius")));
    let frame_outer_radius_scale = Scale::with_range(Orientation::Horizontal, 0.0, 0.5, 0.01);
    frame_outer_radius_scale.set_draw_value(true);
    frame_outer_radius_scale.set_has_origin(false);
    frame_outer_radius_scale.set_format_value_func(move |_, v| format!("{:.0} %", v * 100.0));
    frame_outer_radius_scale.set_value(0.15);
    frame_outer_radius_scale.set_tooltip_text(Some(&i18n.t("tooltip_frame_outer_radius")));
    frame_outer_radius_box.append(&frame_outer_radius_label);
    frame_outer_radius_box.append(&frame_outer_radius_scale);
    frame_outer_radius_box.set_visible(false);
    frame_box.append(&frame_outer_radius_box);

    style_detail_stack.add_titled(&frame_box, Some("frame"), &i18n.t("exp_frame"));

    // ============================================================
    // SECTION: Hintergrund
    // ============================================================
    let bg_box = Box::new(Orientation::Horizontal, 4);
    bg_box.set_valign(Align::Start);
    bg_box.set_margin_start(12);
    bg_box.set_margin_end(12);
    bg_box.set_margin_top(8);
    bg_box.set_margin_bottom(8);

    let bg_select_btn = make_icon_btn("image-x-generic-symbolic", &i18n.t("btn_bg_select"));
    bg_select_btn.set_tooltip_text(Some(&i18n.t("tooltip_bg_select")));
    let bg_remove_btn = make_icon_btn("edit-clear-symbolic", &i18n.t("btn_remove"));
    bg_remove_btn.set_tooltip_text(Some(&i18n.t("tooltip_bg_remove")));
    bg_box.append(&bg_select_btn);
    bg_box.append(&bg_remove_btn);

    style_detail_stack.add_titled(&bg_box, Some("background"), &i18n.t("exp_bg"));

    // ============================================================
    // SECTION: Import/Export
    // ============================================================
    let import_box = Box::new(Orientation::Vertical, 4);
    import_box.set_margin_start(12);
    import_box.set_margin_end(12);
    import_box.set_margin_top(8);
    import_box.set_margin_bottom(8);

    let export_style_btn = make_icon_btn(
        "document-export-symbolic",
        &i18n.t("btn_export_style_short"),
    );
    export_style_btn.set_tooltip_text(Some(&i18n.t("tooltip_export_style")));
    import_box.append(&export_style_btn);

    let import_style_btn = make_icon_btn(
        "document-import-symbolic",
        &i18n.t("btn_import_style_short"),
    );
    import_style_btn.set_tooltip_text(Some(&i18n.t("tooltip_import_style")));
    import_box.append(&import_style_btn);

    let print_calc_btn =
        make_icon_btn("accessories-calculator-symbolic", &i18n.t("btn_print_calc"));
    print_calc_btn.set_tooltip_text(Some(&i18n.t("tooltip_print_calc")));
    import_box.append(&print_calc_btn);

    style_detail_stack.add_titled(&import_box, Some("import"), &i18n.t("exp_import"));

    // Create sidebar rows for all style sections
    {
        let section_info: [(&str, &str); 10] = [
            ("templates", "exp_templates"),
            ("pattern", "exp_pattern"),
            ("colors", "exp_colors"),
            ("settings", "exp_settings"),
            ("advanced", "exp_advanced"),
            ("logo", "exp_logo"),
            ("text", "exp_outer_text"),
            ("frame", "exp_frame"),
            ("background", "exp_bg"),
            ("import", "exp_import"),
        ];
        for &(_, title_key) in &section_info {
            let row = ListBoxRow::new();
            let lbl = Label::new(Some(&i18n.t(title_key)));
            lbl.set_xalign(0.0);
            row.set_child(Some(&lbl));
            style_sidebar_list.append(&row);
        }
        // Select first row by default
        if let Some(first_row) = style_sidebar_list.row_at_index(0) {
            style_sidebar_list.select_row(Some(&first_row));
        }
        // Row selection → switch detail page
        let stack_for_handler = style_detail_stack.clone();
        style_sidebar_list.connect_row_selected(move |_, row| {
            if let Some(r) = row {
                let pages = [
                    "templates",
                    "pattern",
                    "colors",
                    "settings",
                    "advanced",
                    "logo",
                    "text",
                    "frame",
                    "background",
                    "import",
                ];
                let idx = r.index() as usize;
                if idx < pages.len() {
                    stack_for_handler.set_visible_child_name(pages[idx]);
                }
            }
        });
    }

    paned.set_start_child(Some(&left_box));

    // ============================================================
    // RIGHT SIDE (PREVIEW)
    // ============================================================
    let right_box = Box::new(Orientation::Vertical, 8);
    right_box.set_margin_start(12);
    right_box.set_margin_end(12);
    right_box.set_margin_top(12);
    right_box.set_margin_bottom(12);

    let preview_header = Box::new(Orientation::Horizontal, 6);
    preview_header.set_halign(Align::Fill);
    let preview_label = Label::new(Some(&i18n.t("preview_label")));
    preview_label.add_css_class("title-2");
    preview_label.set_hexpand(true);
    preview_header.append(&preview_label);
    let preview_bg_btn = Button::new();
    preview_bg_btn.set_icon_name("preferences-desktop-wallpaper-symbolic");
    preview_bg_btn.set_tooltip_text(Some(&i18n.t("tooltip_preview_bg")));
    preview_bg_btn.add_css_class("flat");
    preview_header.append(&preview_bg_btn);
    right_box.append(&preview_header);

    let preview_picture = Picture::new();
    preview_picture.set_halign(Align::Center);
    preview_picture.set_valign(Align::Center);
    preview_picture.set_hexpand(true);
    preview_picture.set_vexpand(true);
    preview_picture.set_can_shrink(true);
    preview_picture.set_size_request(400, 400);
    preview_picture.add_css_class("preview-fade");
    preview_picture.add_css_class("preview-morph");
    right_box.append(&preview_picture);

    // Preview background toggle signal
    {
        let preview_picture = preview_picture.clone();
        preview_bg_btn.connect_clicked(move |_| {
            if preview_picture.has_css_class("preview-checkerboard") {
                preview_picture.remove_css_class("preview-checkerboard");
            } else {
                preview_picture.add_css_class("preview-checkerboard");
            }
        });
    }

    // Scan verification button (auto-updated after render, starts hidden)
    let scan_verify_btn = Button::with_label(&i18n.t("btn_verify_scan"));
    scan_verify_btn.set_halign(Align::Center);
    scan_verify_btn.set_tooltip_text(Some(&i18n.t("scan_tooltip")));
    scan_verify_btn.add_css_class("scan-verify-btn");
    scan_verify_btn.set_visible(false);
    right_box.append(&scan_verify_btn);

    right_box.append(&Separator::new(Orientation::Horizontal));

    let export_row = Box::new(Orientation::Horizontal, 6);
    export_row.set_halign(Align::Center);

    // Primary copy button
    let copy_btn = make_icon_btn("edit-copy-symbolic", &i18n.t("btn_copy"));
    copy_btn.set_tooltip_text(Some(&i18n.t("tooltip_copy_png")));
    copy_btn.add_css_class("suggested-action");
    export_row.append(&copy_btn);

    // Export popover with additional options
    let export_popover_box = Box::new(Orientation::Vertical, 2);

    let save_png_btn = make_icon_btn("image-x-generic-symbolic", &i18n.t("btn_save_png"));
    save_png_btn.set_tooltip_text(Some(&i18n.t("tooltip_save_png")));
    save_png_btn.set_hexpand(true);
    export_popover_box.append(&save_png_btn);

    let copy_svg_btn = make_icon_btn("edit-copy-symbolic", &i18n.t("btn_copy_svg"));
    copy_svg_btn.set_tooltip_text(Some(&i18n.t("tooltip_copy_svg")));
    copy_svg_btn.set_hexpand(true);
    export_popover_box.append(&copy_svg_btn);

    let save_svg_btn = make_icon_btn("document-save-symbolic", &i18n.t("btn_save_svg"));
    save_svg_btn.set_tooltip_text(Some(&i18n.t("tooltip_save_svg")));
    save_svg_btn.set_hexpand(true);
    export_popover_box.append(&save_svg_btn);

    let save_gif_btn = make_icon_btn("video-x-generic-symbolic", &i18n.t("btn_save_gif"));
    save_gif_btn.set_tooltip_text(Some(&i18n.t("tooltip_save_gif")));
    save_gif_btn.set_hexpand(true);
    export_popover_box.append(&save_gif_btn);

    let save_pdf_btn = make_icon_btn("x-office-document-symbolic", &i18n.t("btn_save_pdf"));
    save_pdf_btn.set_tooltip_text(Some(&i18n.t("tooltip_save_pdf")));
    save_pdf_btn.set_hexpand(true);
    export_popover_box.append(&save_pdf_btn);

    let label_sheet_btn = make_icon_btn("printer-symbolic", &i18n.t("btn_label_sheet"));
    label_sheet_btn.set_tooltip_text(Some(&i18n.t("tooltip_label_sheet")));
    label_sheet_btn.set_hexpand(true);
    export_popover_box.append(&label_sheet_btn);

    let batch_btn = make_icon_btn("document-multiple-symbolic", &i18n.t("btn_batch"));
    batch_btn.set_tooltip_text(Some(&i18n.t("tooltip_batch")));
    batch_btn.set_hexpand(true);
    export_popover_box.append(&batch_btn);

    let export_more_btn = Button::new();
    let export_more_inner = Box::new(Orientation::Horizontal, 4);
    export_more_inner.append(&Image::from_icon_name("document-export-symbolic"));
    export_more_inner.append(&Label::new(Some(&i18n.t("btn_export_more"))));
    export_more_btn.set_child(Some(&export_more_inner));
    export_more_btn.set_tooltip_text(Some(&i18n.t("tooltip_export_more")));

    let export_popover = Popover::new();
    export_popover.set_child(Some(&export_popover_box));
    export_popover.set_parent(&export_more_btn);
    export_popover.set_autohide(true);
    {
        let pop = export_popover.clone();
        export_more_btn.connect_clicked(move |_| {
            pop.popup();
        });
    }
    export_row.append(&export_more_btn);

    right_box.append(&export_row);
    paned.set_end_child(Some(&right_box));

    // ============================================================
    // APP STATE
    // ============================================================
    let state = Rc::new(RefCell::new(AppState {
        preview_picture: preview_picture.clone(),
        text_buffer: text_buffer.clone(),
        dot_style: RefCell::new(DotStyle::Rounded),
        corner_square_style: RefCell::new(CornerSquareStyle::Square),
        corner_dot_style: RefCell::new(CornerDotStyle::Square),
        fg_color: RefCell::new(Rgba([15, 23, 42, 255])),
        bg_color: RefCell::new(Rgba([255, 255, 255, 255])),
        corner_color: RefCell::new(Rgba([15, 23, 42, 255])),
        logo_path: RefCell::new(None),
        logo_size: RefCell::new(0.4),
        outer_text_top: RefCell::new(String::new()),
        outer_text_bottom: RefCell::new(String::new()),
        outer_text_color: RefCell::new(Rgba([0, 0, 0, 255])),
        logo_shape: RefCell::new(LogoShape::Circle),
        quiet_zone: RefCell::new(4),
        module_gap: RefCell::new(0.0),
        frame_style: RefCell::new(FrameStyle::None),
        frame_color: RefCell::new(Rgba([0, 0, 0, 255])),
        toast_overlay: toast_overlay.clone(),
        qr_info_label: qr_info_label.clone(),
        qr_capacity_bar: qr_capacity_bar.clone(),
        ec_level: RefCell::new(ErrorCorrectionLevel::Medium),
        transparent_bg: RefCell::new(false),
        module_size: RefCell::new(32),
        gradient_enabled: RefCell::new(false),
        gradient_color: RefCell::new(Rgba([102, 51, 204, 255])),
        gradient_direction: RefCell::new(GradientDirection::Horizontal),
        content_type: RefCell::new(ContentType::Text),
        wifi_ssid: RefCell::new(String::new()),
        wifi_password: RefCell::new(String::new()),
        wifi_encryption: RefCell::new(WifiEncryption::Wpa),
        shadow_enabled: RefCell::new(false),
        shadow_offset: RefCell::new(2.0),
        frame_width: RefCell::new(2),
        logo_color: RefCell::new(Rgba([0, 0, 0, 0])),
        logo_border_width: RefCell::new(0.0),
        logo_border_color: RefCell::new(Rgba([255, 255, 255, 255])),
        bg_image_path: RefCell::new(None),
        vcard_name: RefCell::new(String::new()),
        vcard_phone: RefCell::new(String::new()),
        vcard_country_code: RefCell::new("+49".to_string()),
        vcard_email: RefCell::new(String::new()),
        vcard_org: RefCell::new(String::new()),
        vcard_url: RefCell::new(String::new()),
        calendar_title: RefCell::new(String::new()),
        calendar_start: RefCell::new(String::new()),
        calendar_end: RefCell::new(String::new()),
        calendar_location: RefCell::new(String::new()),
        gps_lat: RefCell::new(String::new()),
        gps_lon: RefCell::new(String::new()),
        sms_phone: RefCell::new(String::new()),
        sms_country_code: RefCell::new("+49".to_string()),
        sms_message: RefCell::new(String::new()),
        preview_generation: RefCell::new(0),
        cached_svg: RefCell::new(None),
        cached_rgba: RefCell::new(None),
        cached_qr_data: RefCell::new(None),
        scan_verify_btn: scan_verify_btn.clone(),
        logo_vectorize: RefCell::new(false),
        logo_vectorize_bg_color: RefCell::new(Rgba([0, 0, 0, 0])),
        logo_bg_transparent: RefCell::new(false),
        logo_clear_area: RefCell::new(false),
        logo_clear_padding: RefCell::new(0.0),
        logo_outer_radius: RefCell::new(0.15),
        logo_inner_radius: RefCell::new(0.15),
        frame_outer_radius: RefCell::new(0.15),
        frame_inner_radius: RefCell::new(0.15),
        undo_stack: RefCell::new(Vec::new()),
        redo_stack: RefCell::new(Vec::new()),
        is_restoring: RefCell::new(false),
        custom_dot_path: RefCell::new(String::new()),
        outer_text_font: RefCell::new("Sans".to_string()),
        outer_text_font_size: RefCell::new(14),
        contrast_warning_label: contrast_warning_label.clone(),
        i18n: RefCell::new(i18n),
    }));

    // ============================================================
    // LANGUAGE CHANGE HANDLER — save content, rebuild UI with crossfade
    // ============================================================
    {
        let app_clone = app.clone();
        let tb = text_buffer.clone();
        let ct_dd = content_type_dd.clone();
        let ws = wifi_ssid_entry.clone();
        let wp = wifi_password_entry.clone();
        let we = wifi_enc_dd.clone();
        let vn = vcard_name_entry.clone();
        let vp = vcard_phone_entry.clone();
        let ve = vcard_email_entry.clone();
        let vo = vcard_org_entry.clone();
        let vu = vcard_url_entry.clone();
        let vc = vcard_country_dd.clone();
        let ct = cal_title_entry.clone();
        let cl = cal_location_entry.clone();
        let cs = cal_start_calendar.clone();
        let ce = cal_end_calendar.clone();
        let ch = cal_start_hour.clone();
        let cm = cal_start_minute.clone();
        let gl = gps_lat_entry.clone();
        let gn = gps_lon_entry.clone();
        let gs = gps_search_entry.clone();
        let sp = sms_phone_entry.clone();
        let sm = sms_message_entry.clone();
        let sc = sms_country_dd.clone();
        lang_dd.connect_selected_notify(move |dd| {
            // Save content snapshot before rebuilding
            let start = tb.start_iter();
            let end = tb.end_iter();
            let dt_s = cs.date();
            let dt_e = ce.date();
            CONTENT_SNAPSHOT.with(|snap| {
                *snap.borrow_mut() = Some(ContentSnapshot {
                    text: tb.text(&start, &end, false).to_string(),
                    content_type_idx: ct_dd.selected(),
                    wifi_ssid: ws.text().to_string(),
                    wifi_password: wp.text().to_string(),
                    wifi_enc_idx: we.selected(),
                    vcard_name: vn.text().to_string(),
                    vcard_phone: vp.text().to_string(),
                    vcard_email: ve.text().to_string(),
                    vcard_org: vo.text().to_string(),
                    vcard_url: vu.text().to_string(),
                    vcard_country_idx: vc.selected(),
                    cal_title: ct.text().to_string(),
                    cal_location: cl.text().to_string(),
                    cal_start_date: dt_s,
                    cal_end_date: dt_e,
                    cal_start_hour: ch.value(),
                    cal_start_minute: cm.value(),
                    gps_lat: gl.text().to_string(),
                    gps_lon: gn.text().to_string(),
                    gps_search: gs.text().to_string(),
                    sms_phone: sp.text().to_string(),
                    sms_message: sm.text().to_string(),
                    sms_country_idx: sc.selected(),
                });
            });
            let new_lang = match dd.selected() {
                0 => Lang::De,
                1 => Lang::En,
                2 => Lang::Es,
                3 => Lang::Fr,
                4 => Lang::It,
                5 => Lang::PtBr,
                6 => Lang::Ja,
                7 => Lang::Ko,
                8 => Lang::ZhCn,
                _ => Lang::De,
            };
            save_lang(new_lang);
            build_ui(&app_clone);
        });
    }

    // ============================================================
    // SIGNAL HANDLERS
    // ============================================================

    // Clear All — reset every content input field
    {
        let text_buffer = text_buffer.clone();
        let wifi_ssid_entry = wifi_ssid_entry.clone();
        let wifi_password_entry = wifi_password_entry.clone();
        let vcard_name_entry = vcard_name_entry.clone();
        let vcard_phone_entry = vcard_phone_entry.clone();
        let vcard_email_entry = vcard_email_entry.clone();
        let vcard_org_entry = vcard_org_entry.clone();
        let vcard_url_entry = vcard_url_entry.clone();
        let cal_title_entry = cal_title_entry.clone();
        let cal_location_entry = cal_location_entry.clone();
        let gps_lat_entry = gps_lat_entry.clone();
        let gps_lon_entry = gps_lon_entry.clone();
        let gps_search_entry = gps_search_entry.clone();
        let sms_phone_entry = sms_phone_entry.clone();
        let sms_message_entry = sms_message_entry.clone();
        let vcard_country_dd = vcard_country_dd.clone();
        let sms_country_dd = sms_country_dd.clone();
        let wifi_enc_dd = wifi_enc_dd.clone();
        let cal_start_calendar = cal_start_calendar.clone();
        let cal_start_hour = cal_start_hour.clone();
        let cal_start_minute = cal_start_minute.clone();
        let cal_end_calendar = cal_end_calendar.clone();
        let gps_map = gps_map.clone();
        let gps_marker = gps_marker.clone();
        let gps_suggestions_scroll = gps_suggestions_scroll.clone();
        let state = state.clone();
        clear_all_btn.connect_clicked(move |_| {
            text_buffer.set_text("");
            wifi_ssid_entry.set_text("");
            wifi_password_entry.set_text("");
            vcard_name_entry.set_text("");
            vcard_phone_entry.set_text("");
            vcard_email_entry.set_text("");
            vcard_org_entry.set_text("");
            vcard_url_entry.set_text("");
            cal_title_entry.set_text("");
            cal_location_entry.set_text("");
            gps_lat_entry.set_text("");
            gps_lon_entry.set_text("");
            gps_search_entry.set_text("");
            sms_phone_entry.set_text("");
            sms_message_entry.set_text("");
            vcard_country_dd.set_selected(default_country_index());
            sms_country_dd.set_selected(default_country_index());
            wifi_enc_dd.set_selected(0); // WPA
            if let Ok(now) = glib::DateTime::now_local() {
                cal_start_calendar.select_day(&now);
                cal_end_calendar.select_day(&now);
            }
            cal_start_hour.set_value(0.0);
            cal_start_minute.set_value(0.0);
            gps_suggestions_scroll.set_visible(false);
            gps_marker.set_visible(false);
            gps_map.center_on(52.0, 10.0);
            update_preview(&state);
        });
    }

    // Content type change - show/hide boxes (index-based, language-independent)
    {
        let state = state.clone();
        let content_stack = content_stack.clone();
        content_type_dd.connect_selected_notify(move |dd| {
            let ct = match dd.selected() {
                0 => ContentType::Text,
                1 => ContentType::Wifi,
                2 => ContentType::Vcard,
                3 => ContentType::Calendar,
                4 => ContentType::Gps,
                5 => ContentType::Sms,
                _ => ContentType::Text,
            };
            {
                let s = state.borrow();
                *s.content_type.borrow_mut() = ct;
            }
            content_stack.set_visible_child_name(match ct {
                ContentType::Text => "text",
                ContentType::Wifi => "wifi",
                ContentType::Vcard => "vcard",
                ContentType::Calendar => "calendar",
                ContentType::Gps => "gps",
                ContentType::Sms => "sms",
            });
            schedule_preview(&state);
        });
    }

    // Text buffer changed
    {
        let state = state.clone();
        text_buffer.connect_changed(move |_| {
            schedule_preview(&state);
        });
    }

    // WiFi SSID
    {
        let state = state.clone();
        wifi_ssid_entry.connect_changed(move |e| {
            state.borrow().wifi_ssid.replace(e.text().to_string());
            schedule_preview(&state);
        });
    }

    // WiFi Password
    {
        let state = state.clone();
        wifi_password_entry.connect_changed(move |e| {
            state.borrow().wifi_password.replace(e.text().to_string());
            schedule_preview(&state);
        });
    }

    // WiFi Encryption
    {
        let state = state.clone();
        wifi_enc_dd.connect_selected_notify(move |dd| {
            let enc = match dd.selected() {
                0 => WifiEncryption::Wpa,
                1 => WifiEncryption::Wep,
                2 => WifiEncryption::None,
                _ => WifiEncryption::Wpa,
            };
            state.borrow().wifi_encryption.replace(enc);
            schedule_preview(&state);
        });
    }

    // vCard Name
    {
        let state = state.clone();
        vcard_name_entry.connect_changed(move |e| {
            state.borrow().vcard_name.replace(e.text().to_string());
            schedule_preview(&state);
        });
    }

    // vCard Country Code
    {
        let state = state.clone();
        vcard_country_dd.connect_selected_notify(move |dd| {
            let idx = dd.selected() as usize;
            let c_list = countries();
            if let Some(c) = c_list.get(idx) {
                state
                    .borrow()
                    .vcard_country_code
                    .replace(c.calling_code.to_string());
                schedule_preview(&state);
            }
        });
    }

    // vCard Phone
    {
        let state = state.clone();
        vcard_phone_entry.connect_changed(move |e| {
            state.borrow().vcard_phone.replace(e.text().to_string());
            schedule_preview(&state);
        });
    }

    // vCard Email (with validation)
    {
        let state = state.clone();
        let hint = vcard_email_hint.clone();
        vcard_email_entry.connect_changed(move |e| {
            let text = e.text().to_string();
            state.borrow().vcard_email.replace(text.clone());
            if !text.is_empty() && !text.contains('@') {
                e.add_css_class("input-error");
                trigger_shake(e);
                hint.set_text(&state.borrow().i18n.borrow().t("validation_invalid_email"));
                hint.set_visible(true);
            } else {
                e.remove_css_class("input-error");
                hint.set_visible(false);
            }
            schedule_preview(&state);
        });
    }

    // vCard Org
    {
        let state = state.clone();
        vcard_org_entry.connect_changed(move |e| {
            state.borrow().vcard_org.replace(e.text().to_string());
            schedule_preview(&state);
        });
    }

    // vCard URL
    {
        let state = state.clone();
        vcard_url_entry.connect_changed(move |e| {
            state.borrow().vcard_url.replace(e.text().to_string());
            schedule_preview(&state);
        });
    }

    // Calendar Title
    {
        let state = state.clone();
        cal_title_entry.connect_changed(move |e| {
            state.borrow().calendar_title.replace(e.text().to_string());
            schedule_preview(&state);
        });
    }

    // Calendar Start — day selected
    {
        let state = state.clone();
        let cal_start_calendar = cal_start_calendar.clone();
        let cal_start_hour = cal_start_hour.clone();
        let cal_start_minute = cal_start_minute.clone();
        cal_start_calendar.connect_day_selected(move |cal| {
            let s = format!(
                "{:04}{:02}{:02}T{:02}{:02}00",
                cal.date().year(),
                cal.date().month(),
                cal.date().day_of_month(),
                cal_start_hour.value() as u32,
                cal_start_minute.value() as u32
            );
            state.borrow().calendar_start.replace(s);
            schedule_preview(&state);
        });
    }
    // Calendar Start — time changed
    {
        let state = state.clone();
        let cal_start_calendar = cal_start_calendar.clone();
        let cal_start_minute = cal_start_minute.clone();
        cal_start_hour.connect_value_changed(move |h| {
            let dt = cal_start_calendar.date();
            let s = format!(
                "{:04}{:02}{:02}T{:02}{:02}00",
                dt.year(),
                dt.month(),
                dt.day_of_month(),
                h.value() as u32,
                cal_start_minute.value() as u32
            );
            state.borrow().calendar_start.replace(s);
            schedule_preview(&state);
        });
    }
    {
        let state = state.clone();
        let cal_start_calendar = cal_start_calendar.clone();
        let cal_start_hour = cal_start_hour.clone();
        cal_start_minute.connect_value_changed(move |m| {
            let dt = cal_start_calendar.date();
            let s = format!(
                "{:04}{:02}{:02}T{:02}{:02}00",
                dt.year(),
                dt.month(),
                dt.day_of_month(),
                cal_start_hour.value() as u32,
                m.value() as u32
            );
            state.borrow().calendar_start.replace(s);
            schedule_preview(&state);
        });
    }

    // Calendar End — day selected
    {
        let state = state.clone();
        let cal_end_calendar = cal_end_calendar.clone();
        let cal_end_hour = cal_end_hour.clone();
        let cal_end_minute = cal_end_minute.clone();
        cal_end_calendar.connect_day_selected(move |cal| {
            let s = format!(
                "{:04}{:02}{:02}T{:02}{:02}00",
                cal.date().year(),
                cal.date().month(),
                cal.date().day_of_month(),
                cal_end_hour.value() as u32,
                cal_end_minute.value() as u32
            );
            state.borrow().calendar_end.replace(s);
            schedule_preview(&state);
        });
    }
    // Calendar End — time changed
    {
        let state = state.clone();
        let cal_end_calendar = cal_end_calendar.clone();
        let cal_end_minute = cal_end_minute.clone();
        cal_end_hour.connect_value_changed(move |h| {
            let dt = cal_end_calendar.date();
            let s = format!(
                "{:04}{:02}{:02}T{:02}{:02}00",
                dt.year(),
                dt.month(),
                dt.day_of_month(),
                h.value() as u32,
                cal_end_minute.value() as u32
            );
            state.borrow().calendar_end.replace(s);
            schedule_preview(&state);
        });
    }
    {
        let state = state.clone();
        let cal_end_calendar = cal_end_calendar.clone();
        let cal_end_hour = cal_end_hour.clone();
        cal_end_minute.connect_value_changed(move |m| {
            let dt = cal_end_calendar.date();
            let s = format!(
                "{:04}{:02}{:02}T{:02}{:02}00",
                dt.year(),
                dt.month(),
                dt.day_of_month(),
                cal_end_hour.value() as u32,
                m.value() as u32
            );
            state.borrow().calendar_end.replace(s);
            schedule_preview(&state);
        });
    }

    // Calendar Location
    {
        let state = state.clone();
        cal_location_entry.connect_changed(move |e| {
            state
                .borrow()
                .calendar_location
                .replace(e.text().to_string());
            schedule_preview(&state);
        });
    }

    // GPS Lat (with validation)
    {
        let state = state.clone();
        let hint = gps_lat_hint.clone();
        let mm = gps_marker.clone();
        gps_lat_entry.connect_changed(move |e| {
            let text = e.text().to_string();
            state.borrow().gps_lat.replace(text.clone());
            if let Ok(val) = text.parse::<f64>() {
                if val < -90.0 || val > 90.0 {
                    e.add_css_class("input-error");
                    trigger_shake(e);
                    hint.set_text(&state.borrow().i18n.borrow().t("validation_invalid_lat"));
                    hint.set_visible(true);
                } else {
                    e.remove_css_class("input-error");
                    hint.set_visible(false);
                }
            } else if !text.is_empty() {
                e.add_css_class("input-error");
                trigger_shake(e);
                hint.set_text(&state.borrow().i18n.borrow().t("validation_invalid_lat"));
                hint.set_visible(true);
            } else {
                e.remove_css_class("input-error");
                hint.set_visible(false);
            }
            // Sync marker on map
            if let Ok(lat) = text.parse::<f64>() {
                if let Ok(lon) = state.borrow().gps_lon.borrow().parse::<f64>() {
                    if lat >= -90.0 && lat <= 90.0 && lon >= -180.0 && lon <= 180.0 {
                        mm.set_location(lat, lon);
                        mm.set_visible(true);
                    }
                }
            }
            schedule_preview(&state);
        });
    }

    // Longitude entry change → update marker
    // GPS Lon (with validation)
    {
        let state = state.clone();
        let hint = gps_lon_hint.clone();
        let mm = gps_marker.clone();
        gps_lon_entry.connect_changed(move |e| {
            let text = e.text().to_string();
            state.borrow().gps_lon.replace(text.clone());
            if let Ok(val) = text.parse::<f64>() {
                if val < -180.0 || val > 180.0 {
                    e.add_css_class("input-error");
                    trigger_shake(e);
                    hint.set_text(&state.borrow().i18n.borrow().t("validation_invalid_lon"));
                    hint.set_visible(true);
                } else {
                    e.remove_css_class("input-error");
                    hint.set_visible(false);
                }
            } else if !text.is_empty() {
                e.add_css_class("input-error");
                trigger_shake(e);
                hint.set_text(&state.borrow().i18n.borrow().t("validation_invalid_lon"));
                hint.set_visible(true);
            } else {
                e.remove_css_class("input-error");
                hint.set_visible(false);
            }
            // Sync marker on map
            if let Ok(lon) = text.parse::<f64>() {
                if let Ok(lat) = state.borrow().gps_lat.borrow().parse::<f64>() {
                    if lat >= -90.0 && lat <= 90.0 && lon >= -180.0 && lon <= 180.0 {
                        mm.set_location(lat, lon);
                        mm.set_visible(true);
                    }
                }
            }
            schedule_preview(&state);
        });
    }

    // GPS Map — click to pick location
    {
        let gesture = gtk4::GestureClick::new();
        gesture.set_button(1);
        let press_pos: Rc<RefCell<(f64, f64)>> = Rc::new(RefCell::new((0.0, 0.0)));
        let pp = press_pos.clone();
        gesture.connect_pressed(move |_g, _n, x, y| {
            *pp.borrow_mut() = (x, y);
        });
        let map_c = gps_map.clone();
        let marker_c = gps_marker.clone();
        let lat_ec = gps_lat_entry.clone();
        let lon_ec = gps_lon_entry.clone();
        let pp2 = press_pos.clone();
        gesture.connect_released(move |_g, _n, x, y| {
            let (px, py) = *pp2.borrow();
            let dist = ((x - px).powi(2) + (y - py).powi(2)).sqrt();
            if dist < 5.0 {
                if let Some(vp) = map_c.viewport() {
                    let (lat, lon) = vp.widget_coords_to_location(&map_c, x, y);
                    marker_c.set_location(lat, lon);
                    marker_c.set_visible(true);
                    lat_ec.set_text(&format!("{:.6}", lat));
                    lon_ec.set_text(&format!("{:.6}", lon));
                }
            }
        });
        gps_map.add_controller(gesture);
    }

    // GPS Search — inline suggestions via Nominatim
    {
        let search_version: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
        let suppress_search: Rc<RefCell<bool>> = Rc::new(RefCell::new(false));
        let suggestions_c = gps_suggestions.clone();
        let scroll_c = gps_suggestions_scroll.clone();
        let coords_c = gps_suggestion_coords.clone();
        let version_c = search_version.clone();
        let suppress_c = suppress_search.clone();

        // Live suggestions as the user types (debounced via version counter)
        gps_search_entry.connect_changed(move |entry| {
            // Prevent re-entrant calls when row_activated clears the entry
            if *suppress_c.borrow() {
                return;
            }

            // Bump version to invalidate any pending fetch
            let ver = {
                let mut v = version_c.borrow_mut();
                *v += 1;
                *v
            };

            let query = entry.text().to_string();
            if query.trim().is_empty() {
                coords_c.borrow_mut().clear();
                while let Some(child) = suggestions_c.first_child() {
                    suggestions_c.remove(&child);
                }
                scroll_c.set_visible(false);
                return;
            }

            let query_c = query.clone();
            let suggestions_cc = suggestions_c.clone();
            let scroll_cc = scroll_c.clone();
            let coords_cc = coords_c.clone();
            let version_cc = version_c.clone();

            // Debounce: wait 400 ms, then fetch only if still current
            glib::timeout_add_local(Duration::from_millis(400), move || {
                // Stale — a newer keystroke superseded us
                if *version_cc.borrow() != ver {
                    return glib::ControlFlow::Break;
                }

                let (tx, rx) = std::sync::mpsc::channel::<Vec<(String, f64, f64)>>();
                let query_for_thread = query_c.clone();
                std::thread::spawn(move || {
                    let url = match reqwest::Url::parse_with_params(
                        "https://nominatim.openstreetmap.org/search",
                        &[
                            ("q", query_for_thread.as_str()),
                            ("format", "json"),
                            ("limit", "5"),
                            ("addressdetails", "0"),
                        ],
                    ) {
                        Ok(u) => u,
                        Err(_) => return,
                    };
                    let resp = match reqwest::blocking::Client::new()
                        .get(url)
                        .header("User-Agent", "QR-Studio/1.0")
                        .timeout(Duration::from_secs(8))
                        .send()
                    {
                        Ok(r) => r,
                        Err(_) => return,
                    };
                    let data: serde_json::Value = match resp.json() {
                        Ok(v) => v,
                        Err(_) => return,
                    };
                    let mut results = Vec::new();
                    if let Some(arr) = data.as_array() {
                        for item in arr {
                            if let (Some(name), Some(lat_s), Some(lon_s)) = (
                                item.get("display_name").and_then(|v| v.as_str()),
                                item.get("lat").and_then(|v| v.as_str()),
                                item.get("lon").and_then(|v| v.as_str()),
                            ) {
                                if let (Ok(lat), Ok(lon)) =
                                    (lat_s.parse::<f64>(), lon_s.parse::<f64>())
                                {
                                    results.push((name.to_string(), lat, lon));
                                }
                            }
                        }
                    }
                    let _ = tx.send(results);
                });

                // Poll channel for results on the main thread
                let suggestions_ccc = suggestions_cc.clone();
                let scroll_ccc = scroll_cc.clone();
                let coords_ccc = coords_cc.clone();
                let version_ccc = version_cc.clone();

                glib::timeout_add_local(Duration::from_millis(100), move || {
                    // Discard if a newer search has started
                    if *version_ccc.borrow() != ver {
                        return glib::ControlFlow::Break;
                    }
                    match rx.try_recv() {
                        Ok(results) => {
                            // Clear old rows
                            coords_ccc.borrow_mut().clear();
                            while let Some(child) = suggestions_ccc.first_child() {
                                suggestions_ccc.remove(&child);
                            }

                            if results.is_empty() {
                                scroll_ccc.set_visible(false);
                                return glib::ControlFlow::Break;
                            }

                            for (name, _lat, _lon) in &results {
                                let display: String = name.chars().take(60).collect();
                                let truncated = if name.chars().count() > 60 {
                                    format!("{display}…")
                                } else {
                                    display
                                };
                                let lbl = Label::new(Some(&format!("📍  {truncated}")));
                                lbl.set_xalign(0.0);
                                lbl.set_ellipsize(gtk4::pango::EllipsizeMode::End);
                                lbl.set_max_width_chars(45);
                                lbl.set_margin_start(8);
                                lbl.set_margin_end(8);
                                lbl.set_margin_top(4);
                                lbl.set_margin_bottom(4);
                                let row = gtk4::ListBoxRow::new();
                                row.set_child(Some(&lbl));
                                row.add_css_class("gps-suggestion-row");
                                suggestions_ccc.append(&row);
                            }

                            *coords_ccc.borrow_mut() =
                                results.iter().map(|(_, lat, lon)| (*lat, *lon)).collect();
                            scroll_ccc.set_visible(true);
                            glib::ControlFlow::Break
                        }
                        Err(std::sync::mpsc::TryRecvError::Empty) => glib::ControlFlow::Continue,
                        Err(_) => glib::ControlFlow::Break,
                    }
                });

                glib::ControlFlow::Break
            });
        });

        // Click on a suggestion row → navigate there
        let map_c2 = gps_map.clone();
        let marker_c2 = gps_marker.clone();
        let lat_ec2 = gps_lat_entry.clone();
        let lon_ec2 = gps_lon_entry.clone();
        let scroll_c2 = gps_suggestions_scroll.clone();
        let coords_c2 = gps_suggestion_coords.clone();
        let entry_c = gps_search_entry.clone();
        let suppress_c2 = suppress_search.clone();

        gps_suggestions.connect_row_activated(move |_list, row| {
            let idx = row.index() as usize;
            // Short-lived borrow — dropped before set_text triggers connect_changed
            let result = coords_c2.borrow().get(idx).copied();
            if let Some((lat, lon)) = result {
                map_c2.center_on(lat, lon);
                marker_c2.set_location(lat, lon);
                marker_c2.set_visible(true);
                lat_ec2.set_text(&format!("{:.6}", lat));
                lon_ec2.set_text(&format!("{:.6}", lon));
                if let Some(vp) = map_c2.viewport() {
                    vp.set_zoom_level(14.0);
                }
                scroll_c2.set_visible(false);
                *suppress_c2.borrow_mut() = true;
                entry_c.set_text("");
                *suppress_c2.borrow_mut() = false;
            }
        });
    }

    // SMS Country Code
    {
        let state = state.clone();
        sms_country_dd.connect_selected_notify(move |dd| {
            let idx = dd.selected() as usize;
            let c_list = countries();
            if let Some(c) = c_list.get(idx) {
                state
                    .borrow()
                    .sms_country_code
                    .replace(c.calling_code.to_string());
                schedule_preview(&state);
            }
        });
    }

    // SMS Phone (with validation)
    {
        let state = state.clone();
        let hint = sms_phone_hint.clone();
        sms_phone_entry.connect_changed(move |e| {
            let text = e.text().to_string();
            state.borrow().sms_phone.replace(text.clone());
            let digits: String = text.chars().filter(|c| c.is_ascii_digit()).collect();
            if !text.is_empty() && digits.len() < 3 {
                e.add_css_class("input-error");
                trigger_shake(e);
                hint.set_text(&state.borrow().i18n.borrow().t("validation_invalid_phone"));
                hint.set_visible(true);
            } else {
                e.remove_css_class("input-error");
                hint.set_visible(false);
            }
            schedule_preview(&state);
        });
    }

    // SMS Message
    {
        let state = state.clone();
        sms_message_entry.connect_changed(move |e| {
            state.borrow().sms_message.replace(e.text().to_string());
            schedule_preview(&state);
        });
    }

    // Style preset handler
    {
        let state = state.clone();
        let dot_style_dd = dot_style_dd.clone();
        let corner_sq_dd = corner_sq_dd.clone();
        let corner_dot_dd = corner_dot_dd.clone();
        let fg_color_btn = fg_color_btn.clone();
        let bg_color_btn = bg_color_btn.clone();
        let corner_color_btn = corner_color_btn.clone();
        let gradient_check = gradient_check.clone();
        let grad_color_btn = grad_color_btn.clone();
        preset_dd.connect_selected_notify(move |dd| {
            let idx = dd.selected();
            if idx == 0 {
                return;
            }
            // Animation 5: Preview morph on preset switch
            state
                .borrow()
                .preview_picture
                .add_css_class("preview-morphing");
            save_undo_state(&state.borrow());
            let (ds, cs, cd, fg, bg, cc, grad, gc) = match idx {
                1 => (
                    DotStyle::Square,
                    CornerSquareStyle::Square,
                    CornerDotStyle::Square,
                    [0, 0, 0, 255],
                    [255, 255, 255, 255],
                    [0, 0, 0, 255],
                    false,
                    [100, 100, 100, 255],
                ),
                2 => (
                    DotStyle::Rounded,
                    CornerSquareStyle::ExtraRounded,
                    CornerDotStyle::Dot,
                    [15, 23, 107, 255],
                    [255, 255, 255, 255],
                    [15, 23, 107, 255],
                    false,
                    [100, 100, 100, 255],
                ),
                3 => (
                    DotStyle::Dots,
                    CornerSquareStyle::ExtraRounded,
                    CornerDotStyle::Dot,
                    [15, 23, 107, 255],
                    [255, 255, 255, 255],
                    [15, 23, 107, 255],
                    false,
                    [100, 100, 100, 255],
                ),
                4 => (
                    DotStyle::Diamond,
                    CornerSquareStyle::Circle,
                    CornerDotStyle::Dot,
                    [15, 23, 107, 255],
                    [255, 255, 255, 255],
                    [15, 23, 107, 255],
                    false,
                    [100, 100, 100, 255],
                ),
                5 => (
                    DotStyle::Rounded,
                    CornerSquareStyle::Square,
                    CornerDotStyle::Square,
                    [102, 102, 102, 255],
                    [255, 255, 255, 255],
                    [0, 0, 0, 255],
                    false,
                    [100, 100, 100, 255],
                ),
                6 => (
                    DotStyle::Square,
                    CornerSquareStyle::Dot,
                    CornerDotStyle::Dot,
                    [0, 77, 0, 255],
                    [255, 255, 255, 255],
                    [0, 77, 0, 255],
                    false,
                    [100, 100, 100, 255],
                ),
                _ => return,
            };
            {
                let s = state.borrow();
                *s.dot_style.borrow_mut() = ds;
                *s.corner_square_style.borrow_mut() = cs;
                *s.corner_dot_style.borrow_mut() = cd;
                *s.fg_color.borrow_mut() = Rgba(fg);
                *s.bg_color.borrow_mut() = Rgba(bg);
                *s.corner_color.borrow_mut() = Rgba(cc);
                *s.gradient_enabled.borrow_mut() = grad;
                *s.gradient_color.borrow_mut() = Rgba(gc);
            }
            let ds_idx = match ds {
                DotStyle::Rounded => 0,
                DotStyle::Square => 1,
                DotStyle::Dots => 2,
                DotStyle::Diamond => 3,
                DotStyle::Custom => 4,
            };
            let cs_idx = match cs {
                CornerSquareStyle::ExtraRounded => 0,
                CornerSquareStyle::Square => 1,
                CornerSquareStyle::Dot => 2,
                CornerSquareStyle::Circle => 3,
            };
            let cd_idx = match cd {
                CornerDotStyle::Dot => 0,
                CornerDotStyle::Square => 1,
                CornerDotStyle::Circle => 2,
                CornerDotStyle::ExtraRounded => 3,
            };
            dot_style_dd.set_selected(ds_idx);
            corner_sq_dd.set_selected(cs_idx);
            corner_dot_dd.set_selected(cd_idx);
            fg_color_btn.set_rgba(&gdk::RGBA::new(
                fg[0] as f32 / 255.0,
                fg[1] as f32 / 255.0,
                fg[2] as f32 / 255.0,
                fg[3] as f32 / 255.0,
            ));
            bg_color_btn.set_rgba(&gdk::RGBA::new(
                bg[0] as f32 / 255.0,
                bg[1] as f32 / 255.0,
                bg[2] as f32 / 255.0,
                bg[3] as f32 / 255.0,
            ));
            corner_color_btn.set_rgba(&gdk::RGBA::new(
                cc[0] as f32 / 255.0,
                cc[1] as f32 / 255.0,
                cc[2] as f32 / 255.0,
                cc[3] as f32 / 255.0,
            ));
            gradient_check.set_active(grad);
            grad_color_btn.set_rgba(&gdk::RGBA::new(
                gc[0] as f32 / 255.0,
                gc[1] as f32 / 255.0,
                gc[2] as f32 / 255.0,
                gc[3] as f32 / 255.0,
            ));
            schedule_preview(&state);
        });
    }

    // Color palette handler
    {
        let state = state.clone();
        let fg_color_btn = fg_color_btn.clone();
        let bg_color_btn = bg_color_btn.clone();
        let corner_color_btn = corner_color_btn.clone();
        let grad_color_btn = grad_color_btn.clone();
        let gradient_check = gradient_check.clone();
        palette_dd.connect_selected_notify(move |dd| {
            let idx = dd.selected();
            if idx == 0 {
                return;
            }
            save_undo_state(&state.borrow());
            let (fg, bg, cc, gc, grad) = match idx {
                1 => (
                    [0, 0, 0, 255],
                    [255, 255, 255, 255],
                    [0, 0, 0, 255],
                    [100, 100, 100, 255],
                    false,
                ),
                2 => (
                    [0, 51, 128, 255],
                    [230, 242, 255, 255],
                    [0, 77, 153, 255],
                    [0, 128, 204, 255],
                    true,
                ),
                3 => (
                    [204, 51, 77, 255],
                    [255, 242, 230, 255],
                    [230, 77, 26, 255],
                    [230, 128, 51, 255],
                    true,
                ),
                4 => (
                    [26, 102, 26, 255],
                    [242, 255, 242, 255],
                    [0, 77, 0, 255],
                    [51, 153, 51, 255],
                    true,
                ),
                5 => (
                    [102, 51, 153, 255],
                    [247, 242, 255, 255],
                    [128, 51, 179, 255],
                    [153, 102, 204, 255],
                    true,
                ),
                6 => (
                    [204, 26, 0, 255],
                    [255, 250, 230, 255],
                    [230, 51, 0, 255],
                    [255, 153, 0, 255],
                    true,
                ),
                7 => (
                    [0, 128, 153, 255],
                    [26, 26, 51, 255],
                    [0, 179, 204, 255],
                    [77, 51, 204, 255],
                    true,
                ),
                8 => (
                    [128, 102, 153, 255],
                    [255, 250, 242, 255],
                    [153, 128, 179, 255],
                    [179, 153, 128, 255],
                    true,
                ),
                9 => (
                    [0, 255, 128, 255],
                    [13, 13, 26, 255],
                    [0, 255, 204, 255],
                    [255, 0, 128, 255],
                    true,
                ),
                _ => return,
            };
            {
                let s = state.borrow();
                *s.fg_color.borrow_mut() = Rgba(fg);
                *s.bg_color.borrow_mut() = Rgba(bg);
                *s.corner_color.borrow_mut() = Rgba(cc);
                *s.gradient_color.borrow_mut() = Rgba(gc);
                *s.gradient_enabled.borrow_mut() = grad;
            }
            fg_color_btn.set_rgba(&gdk::RGBA::new(
                fg[0] as f32 / 255.0,
                fg[1] as f32 / 255.0,
                fg[2] as f32 / 255.0,
                fg[3] as f32 / 255.0,
            ));
            bg_color_btn.set_rgba(&gdk::RGBA::new(
                bg[0] as f32 / 255.0,
                bg[1] as f32 / 255.0,
                bg[2] as f32 / 255.0,
                bg[3] as f32 / 255.0,
            ));
            corner_color_btn.set_rgba(&gdk::RGBA::new(
                cc[0] as f32 / 255.0,
                cc[1] as f32 / 255.0,
                cc[2] as f32 / 255.0,
                cc[3] as f32 / 255.0,
            ));
            grad_color_btn.set_rgba(&gdk::RGBA::new(
                gc[0] as f32 / 255.0,
                gc[1] as f32 / 255.0,
                gc[2] as f32 / 255.0,
                gc[3] as f32 / 255.0,
            ));
            gradient_check.set_active(grad);
            schedule_preview(&state);
        });
    }

    // Dot style
    {
        let state = state.clone();
        let custom_dot_box = custom_dot_box.clone();
        dot_style_dd.connect_selected_notify(move |dd| {
            save_undo_state(&state.borrow());
            let style = match dd.selected() {
                0 => DotStyle::Rounded,
                1 => DotStyle::Square,
                2 => DotStyle::Dots,
                3 => DotStyle::Diamond,
                4 => DotStyle::Custom,
                _ => DotStyle::Rounded,
            };
            *state.borrow().dot_style.borrow_mut() = style;
            custom_dot_box.set_visible(matches!(style, DotStyle::Custom));
            schedule_preview(&state);
        });
    }

    // Custom dot path entry
    {
        let state = state.clone();
        custom_dot_entry.connect_changed(move |e| {
            let path = e.text().to_string();
            *state.borrow().custom_dot_path.borrow_mut() = path;
            schedule_preview(&state);
        });
    }

    // Corner square style
    {
        let state = state.clone();
        corner_sq_dd.connect_selected_notify(move |dd| {
            save_undo_state(&state.borrow());
            let style = match dd.selected() {
                0 => CornerSquareStyle::ExtraRounded,
                1 => CornerSquareStyle::Square,
                2 => CornerSquareStyle::Dot,
                3 => CornerSquareStyle::Circle,
                _ => CornerSquareStyle::ExtraRounded,
            };
            *state.borrow().corner_square_style.borrow_mut() = style;
            schedule_preview(&state);
        });
    }

    // Corner dot style
    {
        let state = state.clone();
        corner_dot_dd.connect_selected_notify(move |dd| {
            save_undo_state(&state.borrow());
            let style = match dd.selected() {
                0 => CornerDotStyle::Dot,
                1 => CornerDotStyle::Square,
                2 => CornerDotStyle::Circle,
                3 => CornerDotStyle::ExtraRounded,
                _ => CornerDotStyle::Dot,
            };
            *state.borrow().corner_dot_style.borrow_mut() = style;
            schedule_preview(&state);
        });
    }

    // FG color
    {
        let state = state.clone();
        let hbtns: Vec<ColorButton> = harmony_buttons.iter().cloned().collect();
        fg_color_btn.connect_color_set(move |btn| {
            save_undo_state(&state.borrow());
            // Animation 7: Color pop
            btn.add_css_class("color-btn-pop");
            let b = btn.clone();
            glib::timeout_add_local(Duration::from_millis(220), move || {
                b.remove_css_class("color-btn-pop");
                glib::ControlFlow::Break
            });
            let fg = gdk_to_image_rgba(&btn.rgba());
            *state.borrow().fg_color.borrow_mut() = fg;
            let harmonies = color_harmonies(fg);
            for (i, (_, color)) in harmonies.iter().enumerate() {
                hbtns[i].set_rgba(&gdk::RGBA::new(
                    color.0[0] as f32 / 255.0,
                    color.0[1] as f32 / 255.0,
                    color.0[2] as f32 / 255.0,
                    color.0[3] as f32 / 255.0,
                ));
                // Animation 4: Harmony button pop
                hbtns[i].add_css_class("color-btn-pop");
                let hb = hbtns[i].clone();
                glib::timeout_add_local(Duration::from_millis(220), move || {
                    hb.remove_css_class("color-btn-pop");
                    glib::ControlFlow::Break
                });
            }
            schedule_preview(&state);
        });
    }

    // BG color
    {
        let state = state.clone();
        bg_color_btn.connect_color_set(move |btn| {
            save_undo_state(&state.borrow());
            // Animation 7: Color pop
            btn.add_css_class("color-btn-pop");
            let b = btn.clone();
            glib::timeout_add_local(Duration::from_millis(220), move || {
                b.remove_css_class("color-btn-pop");
                glib::ControlFlow::Break
            });
            *state.borrow().bg_color.borrow_mut() = gdk_to_image_rgba(&btn.rgba());
            schedule_preview(&state);
        });
    }

    // Harmony buttons: set background color on selection
    for hbtn in &harmony_buttons {
        let state = state.clone();
        let bg_btn = bg_color_btn.clone();
        hbtn.connect_color_set(move |btn| {
            save_undo_state(&state.borrow());
            // Animation 7: Color pop
            btn.add_css_class("color-btn-pop");
            let b = btn.clone();
            glib::timeout_add_local(Duration::from_millis(220), move || {
                b.remove_css_class("color-btn-pop");
                glib::ControlFlow::Break
            });
            let rgba = btn.rgba();
            *state.borrow().bg_color.borrow_mut() = gdk_to_image_rgba(&rgba);
            bg_btn.set_rgba(&rgba);
            schedule_preview(&state);
        });
    }

    // Corner color
    {
        let state = state.clone();
        corner_color_btn.connect_color_set(move |btn| {
            save_undo_state(&state.borrow());
            // Animation 7: Color pop
            btn.add_css_class("color-btn-pop");
            let b = btn.clone();
            glib::timeout_add_local(Duration::from_millis(220), move || {
                b.remove_css_class("color-btn-pop");
                glib::ControlFlow::Break
            });
            *state.borrow().corner_color.borrow_mut() = gdk_to_image_rgba(&btn.rgba());
            schedule_preview(&state);
        });
    }

    // Transparent background
    {
        let state = state.clone();
        transparent_bg_check.connect_toggled(move |cb| {
            save_undo_state(&state.borrow());
            *state.borrow().transparent_bg.borrow_mut() = cb.is_active();
            // Auto-show checkerboard when transparent BG is enabled
            if cb.is_active() {
                state
                    .borrow()
                    .preview_picture
                    .add_css_class("preview-checkerboard");
            } else {
                state
                    .borrow()
                    .preview_picture
                    .remove_css_class("preview-checkerboard");
            }
            schedule_preview(&state);
        });
    }

    // Gradient enabled
    {
        let state = state.clone();
        gradient_check.connect_toggled(move |cb| {
            save_undo_state(&state.borrow());
            *state.borrow().gradient_enabled.borrow_mut() = cb.is_active();
            schedule_preview(&state);
        });
    }

    // Gradient color
    {
        let state = state.clone();
        grad_color_btn.connect_color_set(move |btn| {
            save_undo_state(&state.borrow());
            // Animation 7: Color pop
            btn.add_css_class("color-btn-pop");
            let b = btn.clone();
            glib::timeout_add_local(Duration::from_millis(220), move || {
                b.remove_css_class("color-btn-pop");
                glib::ControlFlow::Break
            });
            *state.borrow().gradient_color.borrow_mut() = gdk_to_image_rgba(&btn.rgba());
            schedule_preview(&state);
        });
    }

    // Gradient direction
    {
        let state = state.clone();
        grad_dir_dd.connect_selected_notify(move |dd| {
            save_undo_state(&state.borrow());
            let dir = match dd.selected() {
                0 => GradientDirection::Horizontal,
                1 => GradientDirection::Vertical,
                2 => GradientDirection::Diagonal,
                3 => GradientDirection::Radial,
                _ => GradientDirection::Horizontal,
            };
            *state.borrow().gradient_direction.borrow_mut() = dir;
            schedule_preview(&state);
        });
    }

    // EC level
    {
        let state = state.clone();
        ec_level_dd.connect_selected_notify(move |dd| {
            save_undo_state(&state.borrow());
            let ec = match dd.selected() {
                0 => ErrorCorrectionLevel::Medium,
                1 => ErrorCorrectionLevel::Low,
                2 => ErrorCorrectionLevel::Quartile,
                3 => ErrorCorrectionLevel::High,
                _ => ErrorCorrectionLevel::Medium,
            };
            *state.borrow().ec_level.borrow_mut() = ec;
            schedule_preview(&state);
        });
    }

    // Module size
    {
        let state = state.clone();
        module_size_dd.connect_selected_notify(move |dd| {
            save_undo_state(&state.borrow());
            let size = match dd.selected() {
                0 => 32,
                1 => 16,
                2 => 64,
                3 => 128,
                _ => 32,
            };
            *state.borrow().module_size.borrow_mut() = size;
            schedule_preview(&state);
        });
    }

    // Quiet zone
    {
        let state = state.clone();
        quiet_zone_scale.connect_value_changed(move |s| {
            save_undo_state(&state.borrow());
            *state.borrow().quiet_zone.borrow_mut() = s.value() as u32;
            schedule_preview(&state);
        });
    }

    // Module gap
    {
        let state = state.clone();
        module_gap_scale.connect_value_changed(move |s| {
            save_undo_state(&state.borrow());
            *state.borrow().module_gap.borrow_mut() = s.value();
            schedule_preview(&state);
        });
    }

    // Shadow enabled
    {
        let state = state.clone();
        shadow_check.connect_toggled(move |cb| {
            save_undo_state(&state.borrow());
            *state.borrow().shadow_enabled.borrow_mut() = cb.is_active();
            schedule_preview(&state);
        });
    }

    // Shadow offset
    {
        let state = state.clone();
        shadow_offset_scale.connect_value_changed(move |s| {
            save_undo_state(&state.borrow());
            *state.borrow().shadow_offset.borrow_mut() = s.value();
            schedule_preview(&state);
        });
    }

    // Logo select
    {
        let state = state.clone();
        let ec_level_dd = ec_level_dd.clone();
        logo_select_btn.connect_clicked(move |_| {
            let state_ref = state.borrow();
            let i18n = state_ref.i18n.borrow();
            let dialog = FileChooserDialog::new(
                Some(&i18n.t("dlg_select_logo")),
                None::<&gtk4::Window>,
                gtk4::FileChooserAction::Open,
                &[
                    (&i18n.t("btn_cancel"), gtk4::ResponseType::Cancel),
                    (&i18n.t("btn_open"), gtk4::ResponseType::Accept),
                ],
            );
            let filter = FileFilter::new();
            filter.add_mime_type("image/*");
            filter.set_name(Some(&i18n.t("filter_images")));
            drop(i18n);
            drop(state_ref);
            dialog.set_filter(&filter);
            let state = state.clone();
            let ec_level_dd = ec_level_dd.clone();
            dialog.connect_response(move |dlg, resp| {
                if resp == gtk4::ResponseType::Accept {
                    if let Some(file) = dlg.file() {
                        if let Some(path) = file.path() {
                            save_undo_state(&state.borrow());
                            state.borrow().logo_path.replace(Some(path));
                            if *state.borrow().ec_level.borrow() != ErrorCorrectionLevel::High {
                                *state.borrow().ec_level.borrow_mut() = ErrorCorrectionLevel::High;
                                ec_level_dd.set_selected(3);
                            }
                            schedule_preview(&state);
                            // Animation 8: Logo placement bounce
                            let pic = state.borrow().preview_picture.clone();
                            pic.add_css_class("preview-bounce");
                            glib::timeout_add_local(Duration::from_millis(450), move || {
                                pic.remove_css_class("preview-bounce");
                                glib::ControlFlow::Break
                            });
                        }
                    }
                }
                dlg.close();
            });
            dialog.show();
        });
    }

    // Logo remove
    {
        let state = state.clone();
        logo_remove_btn.connect_clicked(move |_| {
            save_undo_state(&state.borrow());
            state.borrow().logo_path.replace(None);
            schedule_preview(&state);
        });
    }

    // Logo size
    {
        let state = state.clone();
        logo_size_scale.connect_value_changed(move |s| {
            save_undo_state(&state.borrow());
            *state.borrow().logo_size.borrow_mut() = s.value();
            schedule_preview(&state);
        });
    }

    // Logo shape
    {
        let state = state.clone();
        let orb_clone = logo_outer_radius_box.clone();
        let irb_clone = logo_inner_radius_box.clone();
        let sync_clone = logo_radius_sync_btn.clone();
        logo_shape_dd.connect_selected_notify(move |dd| {
            save_undo_state(&state.borrow());
            let shape = match dd.selected() {
                0 => LogoShape::Circle,
                1 => LogoShape::Rectangle,
                2 => LogoShape::RoundedRect,
                _ => LogoShape::Circle,
            };
            *state.borrow().logo_shape.borrow_mut() = shape;
            let is_rounded = matches!(shape, LogoShape::RoundedRect);
            orb_clone.set_visible(is_rounded);
            irb_clone.set_visible(is_rounded);
            sync_clone.set_visible(is_rounded);
            schedule_preview(&state);
        });
    }
    {
        let state = state.clone();
        let sync_btn = logo_radius_sync_btn.clone();
        let inner_scale = logo_inner_radius_scale.clone();
        let syncing = Rc::new(RefCell::new(false));
        let syncing_c = syncing.clone();
        logo_outer_radius_scale.connect_value_changed(move |scale| {
            *state.borrow().logo_outer_radius.borrow_mut() = scale.value();
            if sync_btn.is_active() && !*syncing_c.borrow() {
                *syncing_c.borrow_mut() = true;
                inner_scale.set_value(scale.value());
                *syncing_c.borrow_mut() = false;
            }
            schedule_preview(&state);
        });
    }
    {
        let state = state.clone();
        let sync_btn = logo_radius_sync_btn.clone();
        let outer_scale = logo_outer_radius_scale.clone();
        let syncing = Rc::new(RefCell::new(false));
        let syncing_c = syncing.clone();
        logo_inner_radius_scale.connect_value_changed(move |scale| {
            *state.borrow().logo_inner_radius.borrow_mut() = scale.value();
            if sync_btn.is_active() && !*syncing_c.borrow() {
                *syncing_c.borrow_mut() = true;
                outer_scale.set_value(scale.value());
                *syncing_c.borrow_mut() = false;
            }
            schedule_preview(&state);
        });
    }

    // Top text
    {
        let state = state.clone();
        top_text_entry.connect_changed(move |e| {
            state.borrow().outer_text_top.replace(e.text().to_string());
            schedule_preview(&state);
        });
    }

    // Bottom text
    {
        let state = state.clone();
        bottom_text_entry.connect_changed(move |e| {
            state
                .borrow()
                .outer_text_bottom
                .replace(e.text().to_string());
            schedule_preview(&state);
        });
    }

    // Text color
    {
        let state = state.clone();
        text_color_btn.connect_color_set(move |btn| {
            save_undo_state(&state.borrow());
            // Animation 7: Color pop
            btn.add_css_class("color-btn-pop");
            let b = btn.clone();
            glib::timeout_add_local(Duration::from_millis(220), move || {
                b.remove_css_class("color-btn-pop");
                glib::ControlFlow::Break
            });
            *state.borrow().outer_text_color.borrow_mut() = gdk_to_image_rgba(&btn.rgba());
            schedule_preview(&state);
        });
    }

    // Font family
    {
        let state = state.clone();
        font_dd.connect_selected_notify(move |dd| {
            save_undo_state(&state.borrow());
            let font = get_dropdown_string(dd);
            *state.borrow().outer_text_font.borrow_mut() = font;
            schedule_preview(&state);
        });
    }

    // Font size
    {
        let state = state.clone();
        font_size_spin.connect_value_changed(move |spin| {
            save_undo_state(&state.borrow());
            *state.borrow().outer_text_font_size.borrow_mut() = spin.value() as u32;
            schedule_preview(&state);
        });
    }

    // Frame style
    {
        let state = state.clone();
        let for_clone = frame_outer_radius_box.clone();
        frame_style_dd.connect_selected_notify(move |dd| {
            save_undo_state(&state.borrow());
            let style = match dd.selected() {
                0 => FrameStyle::None,
                1 => FrameStyle::Simple,
                2 => FrameStyle::Rounded,
                3 => FrameStyle::Banner,
                _ => FrameStyle::None,
            };
            *state.borrow().frame_style.borrow_mut() = style;
            let is_rounded = matches!(style, FrameStyle::Rounded);
            for_clone.set_visible(is_rounded);
            schedule_preview(&state);
        });
    }
    {
        let state = state.clone();
        frame_outer_radius_scale.connect_value_changed(move |scale| {
            *state.borrow().frame_outer_radius.borrow_mut() = scale.value();
            schedule_preview(&state);
        });
    }

    // Frame color
    {
        let state = state.clone();
        frame_color_btn.connect_color_set(move |btn| {
            save_undo_state(&state.borrow());
            // Animation 7: Color pop
            btn.add_css_class("color-btn-pop");
            let b = btn.clone();
            glib::timeout_add_local(Duration::from_millis(220), move || {
                b.remove_css_class("color-btn-pop");
                glib::ControlFlow::Break
            });
            *state.borrow().frame_color.borrow_mut() = gdk_to_image_rgba(&btn.rgba());
            schedule_preview(&state);
        });
    }
    {
        let state = state.clone();
        frame_width_scale.connect_value_changed(move |s| {
            save_undo_state(&state.borrow());
            *state.borrow().frame_width.borrow_mut() = s.value() as u32;
            schedule_preview(&state);
        });
    }
    {
        let state = state.clone();
        logo_color_btn.connect_color_set(move |btn| {
            save_undo_state(&state.borrow());
            // Animation 7: Color pop
            btn.add_css_class("color-btn-pop");
            let b = btn.clone();
            glib::timeout_add_local(Duration::from_millis(220), move || {
                b.remove_css_class("color-btn-pop");
                glib::ControlFlow::Break
            });
            *state.borrow().logo_color.borrow_mut() = gdk_to_image_rgba(&btn.rgba());
            schedule_preview(&state);
        });
    }
    {
        let state = state.clone();
        logo_border_width_scale.connect_value_changed(move |s| {
            save_undo_state(&state.borrow());
            *state.borrow().logo_border_width.borrow_mut() = s.value();
            schedule_preview(&state);
        });
    }
    {
        let state = state.clone();
        logo_border_color_btn.connect_color_set(move |btn| {
            save_undo_state(&state.borrow());
            // Animation 7: Color pop
            btn.add_css_class("color-btn-pop");
            let b = btn.clone();
            glib::timeout_add_local(Duration::from_millis(220), move || {
                b.remove_css_class("color-btn-pop");
                glib::ControlFlow::Break
            });
            *state.borrow().logo_border_color.borrow_mut() = gdk_to_image_rgba(&btn.rgba());
            schedule_preview(&state);
        });
    }

    {
        let state = state.clone();
        logo_vectorize_check.connect_toggled(move |btn| {
            save_undo_state(&state.borrow());
            *state.borrow().logo_vectorize.borrow_mut() = btn.is_active();
            schedule_preview(&state);
        });
    }

    {
        let state = state.clone();
        logo_vectorize_bg_color_btn.connect_color_set(move |btn| {
            save_undo_state(&state.borrow());
            // Animation 7: Color pop
            btn.add_css_class("color-btn-pop");
            let b = btn.clone();
            glib::timeout_add_local(Duration::from_millis(220), move || {
                b.remove_css_class("color-btn-pop");
                glib::ControlFlow::Break
            });
            *state.borrow().logo_vectorize_bg_color.borrow_mut() = gdk_to_image_rgba(&btn.rgba());
            schedule_preview(&state);
        });
    }

    // Logo background transparent toggle
    {
        let state = state.clone();
        logo_bg_transparent_check.connect_toggled(move |btn| {
            save_undo_state(&state.borrow());
            *state.borrow().logo_bg_transparent.borrow_mut() = btn.is_active();
            schedule_preview(&state);
        });
    }

    // Logo clear area toggle
    {
        let state = state.clone();
        logo_clear_area_check.connect_toggled(move |btn| {
            save_undo_state(&state.borrow());
            *state.borrow().logo_clear_area.borrow_mut() = btn.is_active();
            schedule_preview(&state);
        });
    }

    // Logo clear padding
    {
        let state = state.clone();
        logo_clear_padding_spin.connect_value_changed(move |spin| {
            save_undo_state(&state.borrow());
            *state.borrow().logo_clear_padding.borrow_mut() = spin.value();
            schedule_preview(&state);
        });
    }

    // Background image select
    {
        let state = state.clone();
        bg_select_btn.connect_clicked(move |_| {
            let state_ref = state.borrow();
            let i18n = state_ref.i18n.borrow();
            let dialog = FileChooserDialog::new(
                Some(&i18n.t("dlg_select_bg")),
                None::<&gtk4::Window>,
                gtk4::FileChooserAction::Open,
                &[
                    (&i18n.t("btn_cancel"), gtk4::ResponseType::Cancel),
                    (&i18n.t("btn_open"), gtk4::ResponseType::Accept),
                ],
            );
            let filter = FileFilter::new();
            filter.add_mime_type("image/*");
            filter.set_name(Some(&i18n.t("filter_images")));
            drop(i18n);
            drop(state_ref);
            dialog.set_filter(&filter);
            let state = state.clone();
            dialog.connect_response(move |dlg, resp| {
                if resp == gtk4::ResponseType::Accept {
                    if let Some(file) = dlg.file() {
                        if let Some(path) = file.path() {
                            save_undo_state(&state.borrow());
                            state.borrow().bg_image_path.replace(Some(path));
                            schedule_preview(&state);
                        }
                    }
                }
                dlg.close();
            });
            dialog.show();
        });
    }

    // Background image remove
    {
        let state = state.clone();
        bg_remove_btn.connect_clicked(move |_| {
            save_undo_state(&state.borrow());
            state.borrow().bg_image_path.replace(None);
            schedule_preview(&state);
        });
    }

    // Export style
    {
        let state = state.clone();
        export_style_btn.connect_clicked(move |_| {
            let state_ref = state.borrow();
            let i18n = state_ref.i18n.borrow();
            let dialog = FileChooserDialog::new(
                Some(&i18n.t("dlg_export_style")),
                None::<&gtk4::Window>,
                gtk4::FileChooserAction::Save,
                &[
                    (&i18n.t("btn_cancel"), gtk4::ResponseType::Cancel),
                    (&i18n.t("btn_save"), gtk4::ResponseType::Accept),
                ],
            );
            drop(i18n);
            drop(state_ref);
            dialog.set_current_name("qr_style.json");
            let state = state.clone();
            dialog.connect_response(move |dlg, resp| {
                if resp == gtk4::ResponseType::Accept {
                    if let Some(file) = dlg.file() {
                        if let Some(path) = file.path() {
                            let settings = current_style_settings(&state.borrow());
                            if let Ok(json) = serde_json::to_string_pretty(&settings) {
                                let _ = std::fs::write(path, json);
                                state.borrow().update_status_typed(
                                    &state.borrow().i18n.borrow().t("status_style_exported"),
                                    ToastType::Success,
                                );
                            }
                        }
                    }
                }
                dlg.close();
            });
            dialog.show();
        });
    }

    // Import style
    {
        let state = state.clone();
        let dot_style_dd = dot_style_dd.clone();
        let corner_sq_dd = corner_sq_dd.clone();
        let corner_dot_dd = corner_dot_dd.clone();
        let fg_color_btn = fg_color_btn.clone();
        let bg_color_btn = bg_color_btn.clone();
        let corner_color_btn = corner_color_btn.clone();
        let transparent_bg_check = transparent_bg_check.clone();
        let gradient_check = gradient_check.clone();
        let grad_color_btn = grad_color_btn.clone();
        let grad_dir_dd = grad_dir_dd.clone();
        let ec_level_dd = ec_level_dd.clone();
        let module_size_dd = module_size_dd.clone();
        let quiet_zone_scale = quiet_zone_scale.clone();
        let module_gap_scale = module_gap_scale.clone();
        let shadow_check = shadow_check.clone();
        let shadow_offset_scale = shadow_offset_scale.clone();
        let logo_shape_dd = logo_shape_dd.clone();
        let logo_color_btn = logo_color_btn.clone();
        let logo_border_width_scale = logo_border_width_scale.clone();
        let logo_border_color_btn = logo_border_color_btn.clone();
        let logo_vectorize_bg_color_btn = logo_vectorize_bg_color_btn.clone();
        let logo_bg_transparent_check = logo_bg_transparent_check.clone();
        let logo_clear_area_check = logo_clear_area_check.clone();
        let logo_clear_padding_spin = logo_clear_padding_spin.clone();
        let frame_style_dd = frame_style_dd.clone();
        let frame_color_btn = frame_color_btn.clone();
        let frame_width_scale = frame_width_scale.clone();
        let logo_outer_radius_scale = logo_outer_radius_scale.clone();
        let logo_inner_radius_scale = logo_inner_radius_scale.clone();
        let logo_outer_radius_box = logo_outer_radius_box.clone();
        let logo_inner_radius_box = logo_inner_radius_box.clone();
        let frame_outer_radius_scale = frame_outer_radius_scale.clone();
        let font_dd = font_dd.clone();
        let font_size_spin = font_size_spin.clone();
        let frame_outer_radius_box = frame_outer_radius_box.clone();
        let logo_radius_sync_btn = logo_radius_sync_btn.clone();
        let custom_dot_box = custom_dot_box.clone();
        import_style_btn.connect_clicked(move |_| {
            let state_ref = state.borrow();
            let i18n = state_ref.i18n.borrow();
            let dialog = FileChooserDialog::new(
                Some(&i18n.t("dlg_import_style")),
                None::<&gtk4::Window>,
                gtk4::FileChooserAction::Open,
                &[
                    (&i18n.t("btn_cancel"), gtk4::ResponseType::Cancel),
                    (&i18n.t("btn_open"), gtk4::ResponseType::Accept),
                ],
            );
            let filter = FileFilter::new();
            filter.add_mime_type("application/json");
            filter.add_pattern("*.json");
            filter.set_name(Some(&i18n.t("filter_json")));
            drop(i18n);
            drop(state_ref);
            dialog.set_filter(&filter);
            let state = state.clone();
            let dot_style_dd = dot_style_dd.clone();
            let corner_sq_dd = corner_sq_dd.clone();
            let corner_dot_dd = corner_dot_dd.clone();
            let fg_color_btn = fg_color_btn.clone();
            let bg_color_btn = bg_color_btn.clone();
            let corner_color_btn = corner_color_btn.clone();
            let transparent_bg_check = transparent_bg_check.clone();
            let gradient_check = gradient_check.clone();
            let grad_color_btn = grad_color_btn.clone();
            let grad_dir_dd = grad_dir_dd.clone();
            let ec_level_dd = ec_level_dd.clone();
            let module_size_dd = module_size_dd.clone();
            let quiet_zone_scale = quiet_zone_scale.clone();
            let module_gap_scale = module_gap_scale.clone();
            let shadow_check = shadow_check.clone();
            let shadow_offset_scale = shadow_offset_scale.clone();
            let logo_shape_dd = logo_shape_dd.clone();
            let logo_color_btn = logo_color_btn.clone();
            let logo_border_width_scale = logo_border_width_scale.clone();
            let logo_border_color_btn = logo_border_color_btn.clone();
            let logo_vectorize_bg_color_btn = logo_vectorize_bg_color_btn.clone();
            let logo_bg_transparent_check = logo_bg_transparent_check.clone();
            let logo_clear_area_check = logo_clear_area_check.clone();
            let logo_clear_padding_spin = logo_clear_padding_spin.clone();
            let frame_style_dd = frame_style_dd.clone();
            let frame_color_btn = frame_color_btn.clone();
            let frame_width_scale = frame_width_scale.clone();
            let logo_outer_radius_scale = logo_outer_radius_scale.clone();
            let logo_inner_radius_scale = logo_inner_radius_scale.clone();
            let logo_outer_radius_box = logo_outer_radius_box.clone();
            let logo_inner_radius_box = logo_inner_radius_box.clone();
            let frame_outer_radius_scale = frame_outer_radius_scale.clone();
            let font_dd = font_dd.clone();
            let font_size_spin = font_size_spin.clone();
            let frame_outer_radius_box = frame_outer_radius_box.clone();
            let logo_radius_sync_btn = logo_radius_sync_btn.clone();
            let custom_dot_box = custom_dot_box.clone();
            dialog.connect_response(move |dlg, resp| {
                if resp == gtk4::ResponseType::Accept {
                    if let Some(file) = dlg.file() {
                        if let Some(path) = file.path() {
                            if let Ok(data) = std::fs::read_to_string(&path) {
                                if let Ok(settings) = serde_json::from_str::<StyleSettings>(&data) {
                                    save_undo_state(&state.borrow());
                                    apply_style_to_state(&state.borrow(), &settings);
                                    let s = state.borrow();
                                    let ds = *s.dot_style.borrow();
                                    let cs = *s.corner_square_style.borrow();
                                    let cd = *s.corner_dot_style.borrow();
                                    let fg = s.fg_color.borrow().0;
                                    let bg = s.bg_color.borrow().0;
                                    let cc = s.corner_color.borrow().0;
                                    let gc = s.gradient_color.borrow().0;
                                    let fc = s.frame_color.borrow().0;
                                    let transparent_bg = *s.transparent_bg.borrow();
                                    let gradient_enabled = *s.gradient_enabled.borrow();
                                    let gradient_direction = *s.gradient_direction.borrow();
                                    let ec_level = *s.ec_level.borrow();
                                    let module_size = *s.module_size.borrow();
                                    let quiet_zone = *s.quiet_zone.borrow();
                                    let module_gap = *s.module_gap.borrow();
                                    let shadow_enabled = *s.shadow_enabled.borrow();
                                    let shadow_offset = *s.shadow_offset.borrow();
                                    let logo_shape = *s.logo_shape.borrow();
                                    let logo_color = s.logo_color.borrow().0;
                                    let logo_border_width = *s.logo_border_width.borrow();
                                    let logo_border_color = s.logo_border_color.borrow().0;
                                    let logo_vectorize_bg_color =
                                        s.logo_vectorize_bg_color.borrow().0;
                                    let logo_bg_transparent = *s.logo_bg_transparent.borrow();
                                    let logo_clear_area = *s.logo_clear_area.borrow();
                                    let logo_clear_padding = *s.logo_clear_padding.borrow();
                                    let logo_outer_radius = *s.logo_outer_radius.borrow();
                                    let logo_inner_radius = *s.logo_inner_radius.borrow();
                                    let frame_style = *s.frame_style.borrow();
                                    let frame_width = *s.frame_width.borrow();
                                    let frame_outer_radius = *s.frame_outer_radius.borrow();
                                    let outer_text_font = s.outer_text_font.borrow().clone();
                                    let outer_text_font_size = *s.outer_text_font_size.borrow();
                                    drop(s);
                                    let ds_idx = match ds {
                                        DotStyle::Rounded => 0,
                                        DotStyle::Square => 1,
                                        DotStyle::Dots => 2,
                                        DotStyle::Diamond => 3,
                                        DotStyle::Custom => 4,
                                    };
                                    dot_style_dd.set_selected(ds_idx);
                                    custom_dot_box.set_visible(matches!(ds, DotStyle::Custom));
                                    corner_sq_dd.set_selected(match cs {
                                        CornerSquareStyle::ExtraRounded => 0,
                                        CornerSquareStyle::Square => 1,
                                        CornerSquareStyle::Dot => 2,
                                        CornerSquareStyle::Circle => 3,
                                    });
                                    corner_dot_dd.set_selected(match cd {
                                        CornerDotStyle::Dot => 0,
                                        CornerDotStyle::Square => 1,
                                        CornerDotStyle::Circle => 2,
                                        CornerDotStyle::ExtraRounded => 3,
                                    });
                                    fg_color_btn.set_rgba(&gdk::RGBA::new(
                                        fg[0] as f32 / 255.0,
                                        fg[1] as f32 / 255.0,
                                        fg[2] as f32 / 255.0,
                                        fg[3] as f32 / 255.0,
                                    ));
                                    bg_color_btn.set_rgba(&gdk::RGBA::new(
                                        bg[0] as f32 / 255.0,
                                        bg[1] as f32 / 255.0,
                                        bg[2] as f32 / 255.0,
                                        bg[3] as f32 / 255.0,
                                    ));
                                    corner_color_btn.set_rgba(&gdk::RGBA::new(
                                        cc[0] as f32 / 255.0,
                                        cc[1] as f32 / 255.0,
                                        cc[2] as f32 / 255.0,
                                        cc[3] as f32 / 255.0,
                                    ));
                                    grad_color_btn.set_rgba(&gdk::RGBA::new(
                                        gc[0] as f32 / 255.0,
                                        gc[1] as f32 / 255.0,
                                        gc[2] as f32 / 255.0,
                                        gc[3] as f32 / 255.0,
                                    ));
                                    frame_color_btn.set_rgba(&gdk::RGBA::new(
                                        fc[0] as f32 / 255.0,
                                        fc[1] as f32 / 255.0,
                                        fc[2] as f32 / 255.0,
                                        fc[3] as f32 / 255.0,
                                    ));
                                    transparent_bg_check.set_active(transparent_bg);
                                    gradient_check.set_active(gradient_enabled);
                                    grad_dir_dd.set_selected(match gradient_direction {
                                        GradientDirection::Horizontal => 0,
                                        GradientDirection::Vertical => 1,
                                        GradientDirection::Diagonal => 2,
                                        GradientDirection::Radial => 3,
                                    });
                                    ec_level_dd.set_selected(match ec_level {
                                        ErrorCorrectionLevel::Medium => 0,
                                        ErrorCorrectionLevel::Low => 1,
                                        ErrorCorrectionLevel::Quartile => 2,
                                        ErrorCorrectionLevel::High => 3,
                                    });
                                    module_size_dd.set_selected(match module_size {
                                        32 => 0,
                                        16 => 1,
                                        64 => 2,
                                        128 => 3,
                                        _ => 0,
                                    });
                                    quiet_zone_scale.set_value(quiet_zone as f64);
                                    module_gap_scale.set_value(module_gap);
                                    shadow_check.set_active(shadow_enabled);
                                    shadow_offset_scale.set_value(shadow_offset);
                                    logo_shape_dd.set_selected(match logo_shape {
                                        LogoShape::Circle => 0,
                                        LogoShape::Rectangle => 1,
                                        LogoShape::RoundedRect => 2,
                                    });
                                    logo_color_btn.set_rgba(&gdk::RGBA::new(
                                        logo_color[0] as f32 / 255.0,
                                        logo_color[1] as f32 / 255.0,
                                        logo_color[2] as f32 / 255.0,
                                        logo_color[3] as f32 / 255.0,
                                    ));
                                    logo_border_width_scale.set_value(logo_border_width);
                                    logo_border_color_btn.set_rgba(&gdk::RGBA::new(
                                        logo_border_color[0] as f32 / 255.0,
                                        logo_border_color[1] as f32 / 255.0,
                                        logo_border_color[2] as f32 / 255.0,
                                        logo_border_color[3] as f32 / 255.0,
                                    ));
                                    logo_vectorize_bg_color_btn.set_rgba(&gdk::RGBA::new(
                                        logo_vectorize_bg_color[0] as f32 / 255.0,
                                        logo_vectorize_bg_color[1] as f32 / 255.0,
                                        logo_vectorize_bg_color[2] as f32 / 255.0,
                                        logo_vectorize_bg_color[3] as f32 / 255.0,
                                    ));
                                    logo_bg_transparent_check.set_active(logo_bg_transparent);
                                    logo_clear_area_check.set_active(logo_clear_area);
                                    logo_clear_padding_spin.set_value(logo_clear_padding);
                                    logo_outer_radius_scale.set_value(logo_outer_radius);
                                    logo_inner_radius_scale.set_value(logo_inner_radius);
                                    logo_outer_radius_box
                                        .set_visible(matches!(logo_shape, LogoShape::RoundedRect));
                                    logo_inner_radius_box
                                        .set_visible(matches!(logo_shape, LogoShape::RoundedRect));
                                    logo_radius_sync_btn
                                        .set_visible(matches!(logo_shape, LogoShape::RoundedRect));
                                    frame_outer_radius_scale.set_value(frame_outer_radius);
                                    frame_outer_radius_box
                                        .set_visible(matches!(frame_style, FrameStyle::Rounded));
                                    frame_style_dd.set_selected(match frame_style {
                                        FrameStyle::None => 0,
                                        FrameStyle::Simple => 1,
                                        FrameStyle::Rounded => 2,
                                        FrameStyle::Banner => 3,
                                    });
                                    frame_width_scale.set_value(frame_width as f64);
                                    set_dropdown_by_string(&font_dd, &outer_text_font);
                                    font_size_spin.set_value(outer_text_font_size as f64);
                                    update_preview(&state);
                                }
                            }
                        }
                    }
                }
                dlg.close();
            });
            dialog.show();
        });
    }

    // Print size calculator
    {
        let state = state.clone();
        let module_size_dd = module_size_dd.clone();
        print_calc_btn.connect_clicked(move |_| {
            let state_ref = state.borrow();
            let i18n_ref = state_ref.i18n.borrow();
            let dlg_title = i18n_ref.t("dlg_print_calc").to_string();
            let cancel_btn = i18n_ref.t("btn_cancel").to_string();
            let apply_btn = i18n_ref.t("btn_apply").to_string();
            drop(i18n_ref);
            drop(state_ref);
            let dialog = gtk4::Dialog::with_buttons(
                Some(&dlg_title),
                None::<&gtk4::Window>,
                gtk4::DialogFlags::MODAL,
                &[
                    (&cancel_btn, gtk4::ResponseType::Cancel),
                    (&apply_btn, gtk4::ResponseType::Apply),
                ],
            );
            dialog.set_default_size(350, 250);
            let content = dialog.content_area();
            let box_v = Box::new(Orientation::Vertical, 8);
            box_v.set_margin_start(12);
            box_v.set_margin_end(12);
            box_v.set_margin_top(12);
            box_v.set_margin_bottom(12);

            let width_label =
                Label::new(Some(&state.borrow().i18n.borrow().t("label_print_width")));
            box_v.append(&width_label);
            let width_spin = SpinButton::with_range(1.0, 100.0, 0.5);
            box_v.append(&width_spin);

            let height_label =
                Label::new(Some(&state.borrow().i18n.borrow().t("label_print_height")));
            box_v.append(&height_label);
            let height_spin = SpinButton::with_range(1.0, 100.0, 0.5);
            height_spin.set_value(10.0);
            box_v.append(&height_spin);

            let dpi_label = Label::new(Some(&state.borrow().i18n.borrow().t("label_dpi")));
            box_v.append(&dpi_label);
            let dpi_spin = SpinButton::with_range(72.0, 600.0, 1.0);
            dpi_spin.set_value(300.0);
            box_v.append(&dpi_spin);

            let result_label = Label::new(Some(""));
            box_v.append(&result_label);

            {
                let tmpl = state
                    .borrow()
                    .i18n
                    .borrow()
                    .t("print_calc_result")
                    .to_string();
                let w_px = (10.0 * 300.0 / 2.54) as u32;
                let h_px = (10.0 * 300.0 / 2.54) as u32;
                let rec = (w_px.min(h_px) / 25).max(8);
                result_label.set_text(
                    &tmpl
                        .replacen("{}", &w_px.to_string(), 1)
                        .replacen("{}", &h_px.to_string(), 1)
                        .replacen("{}", &rec.to_string(), 1),
                );
            }

            {
                let state = state.clone();
                let result_label = result_label.clone();
                let w_clone = width_spin.clone();
                let h_clone = height_spin.clone();
                let d_clone = dpi_spin.clone();
                width_spin.connect_value_changed(move |_| {
                    let tmpl = state
                        .borrow()
                        .i18n
                        .borrow()
                        .t("print_calc_result")
                        .to_string();
                    let w_px = (w_clone.value() * d_clone.value() / 2.54) as u32;
                    let h_px = (h_clone.value() * d_clone.value() / 2.54) as u32;
                    let rec = (w_px.min(h_px) / 25).max(8);
                    result_label.set_text(
                        &tmpl
                            .replacen("{}", &w_px.to_string(), 1)
                            .replacen("{}", &h_px.to_string(), 1)
                            .replacen("{}", &rec.to_string(), 1),
                    );
                });
            }
            {
                let state = state.clone();
                let result_label = result_label.clone();
                let w_clone = width_spin.clone();
                let h_clone = height_spin.clone();
                let d_clone = dpi_spin.clone();
                height_spin.connect_value_changed(move |_| {
                    let tmpl = state
                        .borrow()
                        .i18n
                        .borrow()
                        .t("print_calc_result")
                        .to_string();
                    let w_px = (w_clone.value() * d_clone.value() / 2.54) as u32;
                    let h_px = (h_clone.value() * d_clone.value() / 2.54) as u32;
                    let rec = (w_px.min(h_px) / 25).max(8);
                    result_label.set_text(
                        &tmpl
                            .replacen("{}", &w_px.to_string(), 1)
                            .replacen("{}", &h_px.to_string(), 1)
                            .replacen("{}", &rec.to_string(), 1),
                    );
                });
            }
            {
                let state = state.clone();
                let result_label = result_label.clone();
                let w_clone = width_spin.clone();
                let h_clone = height_spin.clone();
                let d_clone = dpi_spin.clone();
                dpi_spin.connect_value_changed(move |_| {
                    let tmpl = state
                        .borrow()
                        .i18n
                        .borrow()
                        .t("print_calc_result")
                        .to_string();
                    let w_px = (w_clone.value() * d_clone.value() / 2.54) as u32;
                    let h_px = (h_clone.value() * d_clone.value() / 2.54) as u32;
                    let rec = (w_px.min(h_px) / 25).max(8);
                    result_label.set_text(
                        &tmpl
                            .replacen("{}", &w_px.to_string(), 1)
                            .replacen("{}", &h_px.to_string(), 1)
                            .replacen("{}", &rec.to_string(), 1),
                    );
                });
            }

            content.append(&box_v);

            let state = state.clone();
            let module_size_dd = module_size_dd.clone();
            dialog.connect_response(move |dlg, resp| {
                if resp == gtk4::ResponseType::Apply {
                    let w_cm = width_spin.value();
                    let dpi = dpi_spin.value();
                    let w_px = (w_cm * dpi / 2.54) as u32;
                    let rec_size = (w_px / 25).max(8).min(128);
                    let sizes = [16u32, 32, 64, 128];
                    let mut best = 32;
                    let mut best_diff = (rec_size as i32 - 32).abs();
                    for &s in &sizes {
                        let diff = (rec_size as i32 - s as i32).abs();
                        if diff < best_diff {
                            best_diff = diff;
                            best = s;
                        }
                    }
                    let idx = match best {
                        16 => 1,
                        64 => 2,
                        128 => 3,
                        _ => 0,
                    };
                    module_size_dd.set_selected(idx);
                    *state.borrow().module_size.borrow_mut() = best;
                    schedule_preview(&state);
                }
                dlg.close();
            });
            dialog.show();
        });
    }

    // ============================================================
    // UNIFIED TEMPLATE HANDLERS
    // ============================================================
    // Populate unified dropdown (from both presets/ and templates/ dirs)
    {
        let template_list = template_list.clone();
        for name in load_all_saved_names() {
            template_list.append(&name);
        }
    }

    // Save (style-only or style+content depending on checkbox)
    {
        let state = state.clone();
        let template_name_entry = template_name_entry.clone();
        let template_list = template_list.clone();
        let template_dd = template_dd.clone();
        let save_content_check = save_content_check.clone();
        let save_btn_anim = template_save_btn.clone();
        template_save_btn.connect_clicked(move |_| {
            let name = template_name_entry.text().trim().to_string();
            if name.is_empty() {
                state.borrow().update_status_typed(
                    &state.borrow().i18n.borrow().t("status_enter_template_name"),
                    ToastType::Error,
                );
                return;
            }

            let include_content = save_content_check.is_active();
            let saved = if include_content {
                // Save as TemplateSettings (style + content) in templates/ dir
                if let Some(dir) = get_templates_dir() {
                    let path = dir.join(format!("{}.json", name));
                    let tmpl = current_template_settings(&state.borrow());
                    if let Ok(json) = serde_json::to_string_pretty(&tmpl) {
                        if std::fs::write(&path, json).is_ok() {
                            // Remove stale style-only version from presets/
                            if let Some(pdir) = get_presets_dir() {
                                let _ = std::fs::remove_file(pdir.join(format!("{}.json", name)));
                            }
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                // Save as StyleSettings (style only) in presets/ dir
                if let Some(dir) = get_presets_dir() {
                    let path = dir.join(format!("{}.json", name));
                    let settings = current_style_settings(&state.borrow());
                    if let Ok(json) = serde_json::to_string_pretty(&settings) {
                        if std::fs::write(&path, json).is_ok() {
                            // Remove stale full version from templates/
                            if let Some(tdir) = get_templates_dir() {
                                let _ = std::fs::remove_file(tdir.join(format!("{}.json", name)));
                            }
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            };

            if saved {
                // Refresh dropdown: remove and re-add all names
                while template_list.n_items() > 0 {
                    template_list.remove(0);
                }
                for n in load_all_saved_names() {
                    template_list.append(&n);
                }
                // Select the saved name
                for i in 0..template_list.n_items() {
                    if let Some(item) = template_list.item(i) {
                        if let Some(obj) = item.downcast_ref::<gtk4::StringObject>() {
                            if obj.string() == name {
                                template_dd.set_selected(i);
                                break;
                            }
                        }
                    }
                }
                let status = {
                    let s = state.borrow();
                    let i18n = s.i18n.borrow();
                    if include_content {
                        i18n.t("status_template_saved_full").replace("{}", &name)
                    } else {
                        i18n.t("status_template_saved_style").replace("{}", &name)
                    }
                };
                state
                    .borrow()
                    .update_status_typed(&status, ToastType::Success);
                // Animation 2: Save confirmation flash
                save_btn_anim.add_css_class("save-confirmed");
                {
                    let btn = save_btn_anim.clone();
                    glib::timeout_add_local(Duration::from_millis(600), move || {
                        btn.remove_css_class("save-confirmed");
                        glib::ControlFlow::Break
                    });
                }
            }
        });
    }

    // Load saved item (unified: detects style-only vs style+content)
    {
        let state = state.clone();
        let dot_style_dd = dot_style_dd.clone();
        let corner_sq_dd = corner_sq_dd.clone();
        let corner_dot_dd = corner_dot_dd.clone();
        let fg_color_btn = fg_color_btn.clone();
        let bg_color_btn = bg_color_btn.clone();
        let corner_color_btn = corner_color_btn.clone();
        let transparent_bg_check = transparent_bg_check.clone();
        let gradient_check = gradient_check.clone();
        let grad_color_btn = grad_color_btn.clone();
        let grad_dir_dd = grad_dir_dd.clone();
        let ec_level_dd = ec_level_dd.clone();
        let module_size_dd = module_size_dd.clone();
        let quiet_zone_scale = quiet_zone_scale.clone();
        let module_gap_scale = module_gap_scale.clone();
        let shadow_check = shadow_check.clone();
        let shadow_offset_scale = shadow_offset_scale.clone();
        let logo_shape_dd = logo_shape_dd.clone();
        let logo_color_btn = logo_color_btn.clone();
        let logo_border_width_scale = logo_border_width_scale.clone();
        let logo_border_color_btn = logo_border_color_btn.clone();
        let logo_vectorize_check = logo_vectorize_check.clone();
        let logo_vectorize_bg_color_btn = logo_vectorize_bg_color_btn.clone();
        let logo_bg_transparent_check = logo_bg_transparent_check.clone();
        let logo_clear_area_check = logo_clear_area_check.clone();
        let logo_clear_padding_spin = logo_clear_padding_spin.clone();
        let logo_outer_radius_scale = logo_outer_radius_scale.clone();
        let logo_inner_radius_scale = logo_inner_radius_scale.clone();
        let logo_outer_radius_box = logo_outer_radius_box.clone();
        let logo_inner_radius_box = logo_inner_radius_box.clone();
        let logo_radius_sync_btn = logo_radius_sync_btn.clone();
        let frame_style_dd = frame_style_dd.clone();
        let frame_color_btn = frame_color_btn.clone();
        let frame_width_scale = frame_width_scale.clone();
        let frame_outer_radius_scale = frame_outer_radius_scale.clone();
        let font_dd = font_dd.clone();
        let font_size_spin = font_size_spin.clone();
        let frame_outer_radius_box = frame_outer_radius_box.clone();
        let content_type_dd = content_type_dd.clone();
        let content_stack = content_stack.clone();

        let wifi_ssid_entry = wifi_ssid_entry.clone();
        let wifi_password_entry = wifi_password_entry.clone();
        let wifi_enc_dd = wifi_enc_dd.clone();
        let vcard_name_entry = vcard_name_entry.clone();
        let vcard_phone_entry = vcard_phone_entry.clone();
        let vcard_email_entry = vcard_email_entry.clone();
        let vcard_org_entry = vcard_org_entry.clone();
        let vcard_url_entry = vcard_url_entry.clone();
        let vcard_country_dd = vcard_country_dd.clone();
        let cal_title_entry = cal_title_entry.clone();
        let cal_start_calendar = cal_start_calendar.clone();
        let cal_start_hour = cal_start_hour.clone();
        let cal_start_minute = cal_start_minute.clone();
        let cal_end_calendar = cal_end_calendar.clone();
        let cal_end_hour = cal_end_hour.clone();
        let cal_end_minute = cal_end_minute.clone();
        let cal_location_entry = cal_location_entry.clone();
        let gps_lat_entry = gps_lat_entry.clone();
        let gps_lon_entry = gps_lon_entry.clone();
        let sms_phone_entry = sms_phone_entry.clone();
        let sms_country_dd = sms_country_dd.clone();
        let sms_message_entry = sms_message_entry.clone();
        let top_text_entry = top_text_entry.clone();
        let bottom_text_entry = bottom_text_entry.clone();
        let text_color_btn = text_color_btn.clone();
        let custom_dot_box = custom_dot_box.clone();
        template_dd.connect_selected_notify(move |dd| {
            if dd.selected_item().is_none() {
                return;
            }
            let name = get_dropdown_string(dd);
            if name.is_empty() {
                return;
            }

            if let Some((data, source)) = load_saved_item_json(&name) {
                let is_full_template = source == "templates";

                if is_full_template {
                    // Full template (style + content)
                    if let Ok(tmpl) = serde_json::from_str::<TemplateSettings>(&data) {
                        save_undo_state(&state.borrow());
                        apply_template_to_state(&state.borrow(), &tmpl);

                        // Update content UI
                        let ct = *state.borrow().content_type.borrow();
                        content_type_dd.set_selected(match ct {
                            ContentType::Text => 0,
                            ContentType::Wifi => 1,
                            ContentType::Vcard => 2,
                            ContentType::Calendar => 3,
                            ContentType::Gps => 4,
                            ContentType::Sms => 5,
                        });
                        content_stack.set_visible_child_name(match ct {
                            ContentType::Text => "text",
                            ContentType::Wifi => "wifi",
                            ContentType::Vcard => "vcard",
                            ContentType::Calendar => "calendar",
                            ContentType::Gps => "gps",
                            ContentType::Sms => "sms",
                        });
                        let ws = state.borrow().wifi_ssid.borrow().clone();
                        let wp = state.borrow().wifi_password.borrow().clone();
                        wifi_ssid_entry.set_text(&ws);
                        wifi_password_entry.set_text(&wp);
                        let we = *state.borrow().wifi_encryption.borrow();
                        wifi_enc_dd.set_selected(match we {
                            WifiEncryption::Wpa => 0,
                            WifiEncryption::Wep => 1,
                            WifiEncryption::None => 2,
                        });
                        let vn = state.borrow().vcard_name.borrow().clone();
                        let vp = state.borrow().vcard_phone.borrow().clone();
                        let ve = state.borrow().vcard_email.borrow().clone();
                        let vo = state.borrow().vcard_org.borrow().clone();
                        let vu = state.borrow().vcard_url.borrow().clone();
                        vcard_name_entry.set_text(&vn);
                        vcard_phone_entry.set_text(&vp);
                        vcard_email_entry.set_text(&ve);
                        vcard_org_entry.set_text(&vo);
                        vcard_url_entry.set_text(&vu);
                        {
                            let cc = state.borrow().vcard_country_code.borrow().clone();
                            vcard_country_dd.set_selected(country_index_by_code(&cc));
                        }
                        let ct_title = state.borrow().calendar_title.borrow().clone();
                        cal_title_entry.set_text(&ct_title);
                        let cs_start = state.borrow().calendar_start.borrow().clone();
                        set_cal_from_string(
                            &cal_start_calendar,
                            &cal_start_hour,
                            &cal_start_minute,
                            &cs_start,
                        );
                        let cs_end = state.borrow().calendar_end.borrow().clone();
                        set_cal_from_string(
                            &cal_end_calendar,
                            &cal_end_hour,
                            &cal_end_minute,
                            &cs_end,
                        );
                        let ct_loc = state.borrow().calendar_location.borrow().clone();
                        cal_location_entry.set_text(&ct_loc);
                        let glat = state.borrow().gps_lat.borrow().clone();
                        let glon = state.borrow().gps_lon.borrow().clone();
                        gps_lat_entry.set_text(&glat);
                        gps_lon_entry.set_text(&glon);
                        let sp = state.borrow().sms_phone.borrow().clone();
                        let sm = state.borrow().sms_message.borrow().clone();
                        sms_phone_entry.set_text(&sp);
                        sms_message_entry.set_text(&sm);
                        {
                            let cc = state.borrow().sms_country_code.borrow().clone();
                            sms_country_dd.set_selected(country_index_by_code(&cc));
                        }
                        let ot_top = state.borrow().outer_text_top.borrow().clone();
                        let ot_bottom = state.borrow().outer_text_bottom.borrow().clone();
                        top_text_entry.set_text(&ot_top);
                        bottom_text_entry.set_text(&ot_bottom);
                    } else {
                        return;
                    }
                } else {
                    // Style-only preset
                    if let Ok(settings) = serde_json::from_str::<StyleSettings>(&data) {
                        save_undo_state(&state.borrow());
                        apply_style_to_state(&state.borrow(), &settings);
                    } else {
                        return;
                    }
                }

                // Common: update style UI (always needed)
                let s = state.borrow();
                let ds = *s.dot_style.borrow();
                let cs = *s.corner_square_style.borrow();
                let cd = *s.corner_dot_style.borrow();
                let fg = s.fg_color.borrow().0;
                let bg = s.bg_color.borrow().0;
                let cc = s.corner_color.borrow().0;
                let gc = s.gradient_color.borrow().0;
                let fc = s.frame_color.borrow().0;
                let transparent_bg = *s.transparent_bg.borrow();
                let gradient_enabled = *s.gradient_enabled.borrow();
                let gradient_direction = *s.gradient_direction.borrow();
                let ec_level = *s.ec_level.borrow();
                let module_size = *s.module_size.borrow();
                let quiet_zone = *s.quiet_zone.borrow();
                let module_gap = *s.module_gap.borrow();
                let shadow_enabled = *s.shadow_enabled.borrow();
                let shadow_offset = *s.shadow_offset.borrow();
                let logo_shape = *s.logo_shape.borrow();
                let logo_color = s.logo_color.borrow().0;
                let logo_border_width = *s.logo_border_width.borrow();
                let logo_border_color = s.logo_border_color.borrow().0;
                let logo_vectorize = *s.logo_vectorize.borrow();
                let logo_vectorize_bg_color = s.logo_vectorize_bg_color.borrow().0;
                let logo_bg_transparent = *s.logo_bg_transparent.borrow();
                let logo_clear_area = *s.logo_clear_area.borrow();
                let logo_clear_padding = *s.logo_clear_padding.borrow();
                let logo_outer_radius = *s.logo_outer_radius.borrow();
                let logo_inner_radius = *s.logo_inner_radius.borrow();
                let frame_style = *s.frame_style.borrow();
                let frame_width = *s.frame_width.borrow();
                let frame_outer_radius = *s.frame_outer_radius.borrow();
                let outer_text_color = s.outer_text_color.borrow().0;
                let outer_text_font = s.outer_text_font.borrow().clone();
                let outer_text_font_size = *s.outer_text_font_size.borrow();
                drop(s);

                dot_style_dd.set_selected(match ds {
                    DotStyle::Rounded => 0,
                    DotStyle::Square => 1,
                    DotStyle::Dots => 2,
                    DotStyle::Diamond => 3,
                    DotStyle::Custom => 4,
                });
                custom_dot_box.set_visible(matches!(ds, DotStyle::Custom));
                corner_sq_dd.set_selected(match cs {
                    CornerSquareStyle::ExtraRounded => 0,
                    CornerSquareStyle::Square => 1,
                    CornerSquareStyle::Dot => 2,
                    CornerSquareStyle::Circle => 3,
                });
                corner_dot_dd.set_selected(match cd {
                    CornerDotStyle::Dot => 0,
                    CornerDotStyle::Square => 1,
                    CornerDotStyle::Circle => 2,
                    CornerDotStyle::ExtraRounded => 3,
                });
                fg_color_btn.set_rgba(&gdk::RGBA::new(
                    fg[0] as f32 / 255.0,
                    fg[1] as f32 / 255.0,
                    fg[2] as f32 / 255.0,
                    fg[3] as f32 / 255.0,
                ));
                bg_color_btn.set_rgba(&gdk::RGBA::new(
                    bg[0] as f32 / 255.0,
                    bg[1] as f32 / 255.0,
                    bg[2] as f32 / 255.0,
                    bg[3] as f32 / 255.0,
                ));
                corner_color_btn.set_rgba(&gdk::RGBA::new(
                    cc[0] as f32 / 255.0,
                    cc[1] as f32 / 255.0,
                    cc[2] as f32 / 255.0,
                    cc[3] as f32 / 255.0,
                ));
                grad_color_btn.set_rgba(&gdk::RGBA::new(
                    gc[0] as f32 / 255.0,
                    gc[1] as f32 / 255.0,
                    gc[2] as f32 / 255.0,
                    gc[3] as f32 / 255.0,
                ));
                frame_color_btn.set_rgba(&gdk::RGBA::new(
                    fc[0] as f32 / 255.0,
                    fc[1] as f32 / 255.0,
                    fc[2] as f32 / 255.0,
                    fc[3] as f32 / 255.0,
                ));
                text_color_btn.set_rgba(&gdk::RGBA::new(
                    outer_text_color[0] as f32 / 255.0,
                    outer_text_color[1] as f32 / 255.0,
                    outer_text_color[2] as f32 / 255.0,
                    outer_text_color[3] as f32 / 255.0,
                ));
                transparent_bg_check.set_active(transparent_bg);
                gradient_check.set_active(gradient_enabled);
                grad_dir_dd.set_selected(match gradient_direction {
                    GradientDirection::Horizontal => 0,
                    GradientDirection::Vertical => 1,
                    GradientDirection::Diagonal => 2,
                    GradientDirection::Radial => 3,
                });
                ec_level_dd.set_selected(match ec_level {
                    ErrorCorrectionLevel::Medium => 0,
                    ErrorCorrectionLevel::Low => 1,
                    ErrorCorrectionLevel::Quartile => 2,
                    ErrorCorrectionLevel::High => 3,
                });
                module_size_dd.set_selected(match module_size {
                    32 => 0,
                    16 => 1,
                    64 => 2,
                    128 => 3,
                    _ => 0,
                });
                quiet_zone_scale.set_value(quiet_zone as f64);
                module_gap_scale.set_value(module_gap);
                shadow_check.set_active(shadow_enabled);
                shadow_offset_scale.set_value(shadow_offset);
                logo_shape_dd.set_selected(match logo_shape {
                    LogoShape::Circle => 0,
                    LogoShape::Rectangle => 1,
                    LogoShape::RoundedRect => 2,
                });
                logo_color_btn.set_rgba(&gdk::RGBA::new(
                    logo_color[0] as f32 / 255.0,
                    logo_color[1] as f32 / 255.0,
                    logo_color[2] as f32 / 255.0,
                    logo_color[3] as f32 / 255.0,
                ));
                logo_border_width_scale.set_value(logo_border_width);
                logo_border_color_btn.set_rgba(&gdk::RGBA::new(
                    logo_border_color[0] as f32 / 255.0,
                    logo_border_color[1] as f32 / 255.0,
                    logo_border_color[2] as f32 / 255.0,
                    logo_border_color[3] as f32 / 255.0,
                ));
                logo_vectorize_check.set_active(logo_vectorize);
                logo_vectorize_bg_color_btn.set_rgba(&gdk::RGBA::new(
                    logo_vectorize_bg_color[0] as f32 / 255.0,
                    logo_vectorize_bg_color[1] as f32 / 255.0,
                    logo_vectorize_bg_color[2] as f32 / 255.0,
                    logo_vectorize_bg_color[3] as f32 / 255.0,
                ));
                logo_bg_transparent_check.set_active(logo_bg_transparent);
                logo_clear_area_check.set_active(logo_clear_area);
                logo_clear_padding_spin.set_value(logo_clear_padding);
                logo_outer_radius_scale.set_value(logo_outer_radius);
                logo_inner_radius_scale.set_value(logo_inner_radius);
                logo_outer_radius_box.set_visible(matches!(logo_shape, LogoShape::RoundedRect));
                logo_inner_radius_box.set_visible(matches!(logo_shape, LogoShape::RoundedRect));
                logo_radius_sync_btn.set_visible(matches!(logo_shape, LogoShape::RoundedRect));
                frame_style_dd.set_selected(match frame_style {
                    FrameStyle::None => 0,
                    FrameStyle::Simple => 1,
                    FrameStyle::Rounded => 2,
                    FrameStyle::Banner => 3,
                });
                frame_width_scale.set_value(frame_width as f64);
                set_dropdown_by_string(&font_dd, &outer_text_font);
                font_size_spin.set_value(outer_text_font_size as f64);
                frame_outer_radius_scale.set_value(frame_outer_radius);
                frame_outer_radius_box.set_visible(matches!(frame_style, FrameStyle::Rounded));
                update_preview(&state);
                let status = {
                    let s = state.borrow();
                    let i18n = s.i18n.borrow();
                    if is_full_template {
                        i18n.t("status_template_loaded_full").replace("{}", &name)
                    } else {
                        i18n.t("status_template_loaded_style").replace("{}", &name)
                    }
                };
                state
                    .borrow()
                    .update_status_typed(&status, ToastType::Success);
            }
        });
    }

    // Delete saved item (unified: removes from both dirs)
    {
        let state = state.clone();
        let template_list = template_list.clone();
        let template_dd = template_dd.clone();
        template_delete_btn.connect_clicked(move |_| {
            let name = get_dropdown_string(&template_dd);
            if name.is_empty() {
                return;
            }
            if delete_saved_item(&name) {
                for i in 0..template_list.n_items() {
                    if let Some(item) = template_list.item(i) {
                        if let Some(obj) = item.downcast_ref::<gtk4::StringObject>() {
                            if obj.string() == name {
                                template_list.remove(i);
                                break;
                            }
                        }
                    }
                }
                template_dd.set_selected(0);
                state.borrow().update_status_typed(
                    &state
                        .borrow()
                        .i18n
                        .borrow()
                        .t("status_template_deleted_fmt")
                        .replace("{}", &name),
                    ToastType::Info,
                );
            }
        });
    }

    // ============================================================
    // PDF EXPORT
    // ============================================================
    {
        let state = state.clone();
        save_pdf_btn.connect_clicked(move |_| {
            let state_ref = state.borrow();
            let i18n = state_ref.i18n.borrow();
            let dialog = FileChooserDialog::new(
                Some(&i18n.t("dlg_save_pdf")),
                None::<&gtk4::Window>,
                gtk4::FileChooserAction::Save,
                &[
                    (&i18n.t("btn_cancel"), gtk4::ResponseType::Cancel),
                    (&i18n.t("btn_save"), gtk4::ResponseType::Accept),
                ],
            );
            drop(i18n);
            drop(state_ref);
            dialog.set_current_name("qrcode.pdf");
            let state = state.clone();
            dialog.connect_response(move |dlg, resp| {
                if resp == gtk4::ResponseType::Accept {
                    if let Some(file) = dlg.file() {
                        if let Some(path) = file.path() {
                            let s = state.borrow();
                            if let Some(pdf_data) = render_pdf_from_state(&s) {
                                let _ = std::fs::write(path, pdf_data);
                                s.update_status_typed(
                                    &s.i18n.borrow().t("status_pdf_saved"),
                                    ToastType::Success,
                                );
                            } else {
                                s.update_status_typed(
                                    &s.i18n.borrow().t("status_pdf_error"),
                                    ToastType::Error,
                                );
                            }
                        }
                    }
                }
                dlg.close();
            });
            dialog.show();
        });
    }

    // ============================================================
    // LABEL SHEET EXPORT
    // ============================================================
    {
        let state = state.clone();
        label_sheet_btn.connect_clicked(move |_| {
            let state_ref = state.borrow();
            let i18n = state_ref.i18n.borrow();
            let dialog = gtk4::Dialog::with_buttons(
                Some(&i18n.t("dlg_label_sheet")),
                None::<&gtk4::Window>,
                gtk4::DialogFlags::MODAL,
                &[
                    (&i18n.t("btn_cancel"), gtk4::ResponseType::Cancel),
                    (&i18n.t("btn_export"), gtk4::ResponseType::Accept),
                ],
            );
            dialog.set_default_size(400, 350);
            let content = dialog.content_area();
            let box_v = Box::new(Orientation::Vertical, 8);
            box_v.set_margin_start(12);
            box_v.set_margin_end(12);
            box_v.set_margin_top(12);
            box_v.set_margin_bottom(12);

            let cols_label = Label::new(Some(&format!("{}:", i18n.t("lbl_columns"))));
            box_v.append(&cols_label);
            let cols_spin = SpinButton::with_range(1.0, 10.0, 1.0);
            cols_spin.set_value(4.0);
            box_v.append(&cols_spin);

            let rows_label = Label::new(Some(&format!("{}:", i18n.t("lbl_rows"))));
            box_v.append(&rows_label);
            let rows_spin = SpinButton::with_range(1.0, 15.0, 1.0);
            rows_spin.set_value(10.0);
            box_v.append(&rows_spin);

            let margin_label = Label::new(Some(&format!("{}:", i18n.t("lbl_margin_mm"))));
            box_v.append(&margin_label);
            let margin_spin = SpinButton::with_range(0.0, 30.0, 1.0);
            margin_spin.set_value(5.0);
            box_v.append(&margin_spin);

            let spacing_label = Label::new(Some(&format!("{}:", i18n.t("lbl_spacing_mm"))));
            box_v.append(&spacing_label);
            let spacing_spin = SpinButton::with_range(0.0, 20.0, 0.5);
            spacing_spin.set_value(2.0);
            box_v.append(&spacing_spin);

            let info_label = Label::new(Some(&i18n.t("label_sheet_a4_info")));
            drop(i18n);
            drop(state_ref);
            box_v.append(&info_label);

            content.append(&box_v);

            let state = state.clone();
            dialog.connect_response(move |dlg, resp| {
                if resp == gtk4::ResponseType::Accept {
                    let cols = cols_spin.value() as u32;
                    let rows = rows_spin.value() as u32;
                    let margin = margin_spin.value();
                    let spacing = spacing_spin.value();

                    // Now ask for save location
                    let save_dialog = FileChooserDialog::new(
                        Some(&state.borrow().i18n.borrow().t("dlg_save_label_sheet")),
                        None::<&gtk4::Window>,
                        gtk4::FileChooserAction::Save,
                        &[
                            (
                                &state.borrow().i18n.borrow().t("btn_cancel"),
                                gtk4::ResponseType::Cancel,
                            ),
                            (
                                &state.borrow().i18n.borrow().t("btn_save"),
                                gtk4::ResponseType::Accept,
                            ),
                        ],
                    );
                    save_dialog.set_current_name("etiketten.pdf");
                    let state = state.clone();
                    save_dialog.connect_response(move |save_dlg, save_resp| {
                        if save_resp == gtk4::ResponseType::Accept {
                            if let Some(file) = save_dlg.file() {
                                if let Some(path) = file.path() {
                                    let s = state.borrow();
                                    if let Some(pdf_data) =
                                        render_label_sheet(&s, cols, rows, margin, spacing)
                                    {
                                        let _ = std::fs::write(path, pdf_data);
                                        s.update_status_typed(
                                            &s.i18n.borrow().t("status_label_sheet_saved"),
                                            ToastType::Success,
                                        );
                                    } else {
                                        s.update_status_typed(
                                            &s.i18n.borrow().t("status_label_sheet_error"),
                                            ToastType::Error,
                                        );
                                    }
                                }
                            }
                        }
                        save_dlg.close();
                    });
                    save_dialog.show();
                }
                dlg.close();
            });
            dialog.show();
        });
    }

    // Auto-save session (style + content) on window close
    {
        let state = state.clone();
        window.connect_close_request(move |_| {
            let tmpl = current_template_settings(&state.borrow());
            if let Some(path) = get_session_path() {
                if let Ok(json) = serde_json::to_string_pretty(&tmpl) {
                    let _ = std::fs::write(path, json);
                }
            }
            glib::Propagation::Proceed
        });
    }

    // Auto-load session (style + content) on startup
    // Try new session format first, fall back to legacy style-only format
    let session_loaded = if let Some(path) = get_session_path() {
        if let Ok(data) = std::fs::read_to_string(&path) {
            if let Ok(tmpl) = serde_json::from_str::<TemplateSettings>(&data) {
                apply_template_to_state(&state.borrow(), &tmpl);
                // Update content UI
                let ct = *state.borrow().content_type.borrow();
                let content_stack = content_stack.clone();
                content_type_dd.set_selected(match ct {
                    ContentType::Text => 0,
                    ContentType::Wifi => 1,
                    ContentType::Vcard => 2,
                    ContentType::Calendar => 3,
                    ContentType::Gps => 4,
                    ContentType::Sms => 5,
                });
                content_stack.set_visible_child_name(match ct {
                    ContentType::Text => "text",
                    ContentType::Wifi => "wifi",
                    ContentType::Vcard => "vcard",
                    ContentType::Calendar => "calendar",
                    ContentType::Gps => "gps",
                    ContentType::Sms => "sms",
                });
                let ws = state.borrow().wifi_ssid.borrow().clone();
                let wp = state.borrow().wifi_password.borrow().clone();
                wifi_ssid_entry.set_text(&ws);
                wifi_password_entry.set_text(&wp);
                wifi_enc_dd.set_selected(match *state.borrow().wifi_encryption.borrow() {
                    WifiEncryption::Wpa => 0,
                    WifiEncryption::Wep => 1,
                    WifiEncryption::None => 2,
                });
                let vn = state.borrow().vcard_name.borrow().clone();
                let vp = state.borrow().vcard_phone.borrow().clone();
                let ve = state.borrow().vcard_email.borrow().clone();
                let vo = state.borrow().vcard_org.borrow().clone();
                let vu = state.borrow().vcard_url.borrow().clone();
                vcard_name_entry.set_text(&vn);
                vcard_phone_entry.set_text(&vp);
                vcard_email_entry.set_text(&ve);
                vcard_org_entry.set_text(&vo);
                vcard_url_entry.set_text(&vu);
                // Restore vCard country code selection
                {
                    let cc = state.borrow().vcard_country_code.borrow().clone();
                    vcard_country_dd.set_selected(country_index_by_code(&cc));
                }
                let ct_title = state.borrow().calendar_title.borrow().clone();
                cal_title_entry.set_text(&ct_title);
                let cal_start_str = state.borrow().calendar_start.borrow().clone();
                set_cal_from_string(
                    &cal_start_calendar,
                    &cal_start_hour,
                    &cal_start_minute,
                    &cal_start_str,
                );
                let cal_end_str = state.borrow().calendar_end.borrow().clone();
                set_cal_from_string(
                    &cal_end_calendar,
                    &cal_end_hour,
                    &cal_end_minute,
                    &cal_end_str,
                );
                let ct_loc = state.borrow().calendar_location.borrow().clone();
                cal_location_entry.set_text(&ct_loc);
                let glat = state.borrow().gps_lat.borrow().clone();
                let glon = state.borrow().gps_lon.borrow().clone();
                gps_lat_entry.set_text(&glat);
                gps_lon_entry.set_text(&glon);
                let sp = state.borrow().sms_phone.borrow().clone();
                let sm = state.borrow().sms_message.borrow().clone();
                sms_phone_entry.set_text(&sp);
                sms_message_entry.set_text(&sm);
                // Restore SMS country code selection
                {
                    let cc = state.borrow().sms_country_code.borrow().clone();
                    sms_country_dd.set_selected(country_index_by_code(&cc));
                }
                let ot_top = state.borrow().outer_text_top.borrow().clone();
                let ot_bottom = state.borrow().outer_text_bottom.borrow().clone();
                top_text_entry.set_text(&ot_top);
                bottom_text_entry.set_text(&ot_bottom);

                // Update style UI
                let s = state.borrow();
                let ds = *s.dot_style.borrow();
                let cs = *s.corner_square_style.borrow();
                let cd = *s.corner_dot_style.borrow();
                let fg = s.fg_color.borrow().0;
                let bg = s.bg_color.borrow().0;
                let cc = s.corner_color.borrow().0;
                let gc = s.gradient_color.borrow().0;
                let fc = s.frame_color.borrow().0;
                let tc = s.outer_text_color.borrow().0;
                let transparent_bg = *s.transparent_bg.borrow();
                let gradient_enabled = *s.gradient_enabled.borrow();
                let gradient_direction = *s.gradient_direction.borrow();
                let ec_level = *s.ec_level.borrow();
                let module_size = *s.module_size.borrow();
                let quiet_zone = *s.quiet_zone.borrow();
                let module_gap = *s.module_gap.borrow();
                let shadow_enabled = *s.shadow_enabled.borrow();
                let shadow_offset = *s.shadow_offset.borrow();
                let logo_shape = *s.logo_shape.borrow();
                let logo_color = s.logo_color.borrow().0;
                let logo_border_width = *s.logo_border_width.borrow();
                let logo_border_color = s.logo_border_color.borrow().0;
                let logo_vectorize = *s.logo_vectorize.borrow();
                let logo_vectorize_bg_color = s.logo_vectorize_bg_color.borrow().0;
                let logo_bg_transparent = *s.logo_bg_transparent.borrow();
                let logo_clear_area = *s.logo_clear_area.borrow();
                let logo_clear_padding = *s.logo_clear_padding.borrow();
                let logo_outer_radius = *s.logo_outer_radius.borrow();
                let logo_inner_radius = *s.logo_inner_radius.borrow();
                let frame_style = *s.frame_style.borrow();
                let frame_width = *s.frame_width.borrow();
                let frame_outer_radius = *s.frame_outer_radius.borrow();
                let outer_text_font = s.outer_text_font.borrow().clone();
                let outer_text_font_size = *s.outer_text_font_size.borrow();
                drop(s);
                dot_style_dd.set_selected(match ds {
                    DotStyle::Rounded => 0,
                    DotStyle::Square => 1,
                    DotStyle::Dots => 2,
                    DotStyle::Diamond => 3,
                    DotStyle::Custom => 4,
                });
                custom_dot_box.set_visible(matches!(ds, DotStyle::Custom));
                corner_sq_dd.set_selected(match cs {
                    CornerSquareStyle::ExtraRounded => 0,
                    CornerSquareStyle::Square => 1,
                    CornerSquareStyle::Dot => 2,
                    CornerSquareStyle::Circle => 3,
                });
                corner_dot_dd.set_selected(match cd {
                    CornerDotStyle::Dot => 0,
                    CornerDotStyle::Square => 1,
                    CornerDotStyle::Circle => 2,
                    CornerDotStyle::ExtraRounded => 3,
                });
                fg_color_btn.set_rgba(&gdk::RGBA::new(
                    fg[0] as f32 / 255.0,
                    fg[1] as f32 / 255.0,
                    fg[2] as f32 / 255.0,
                    fg[3] as f32 / 255.0,
                ));
                bg_color_btn.set_rgba(&gdk::RGBA::new(
                    bg[0] as f32 / 255.0,
                    bg[1] as f32 / 255.0,
                    bg[2] as f32 / 255.0,
                    bg[3] as f32 / 255.0,
                ));
                corner_color_btn.set_rgba(&gdk::RGBA::new(
                    cc[0] as f32 / 255.0,
                    cc[1] as f32 / 255.0,
                    cc[2] as f32 / 255.0,
                    cc[3] as f32 / 255.0,
                ));
                grad_color_btn.set_rgba(&gdk::RGBA::new(
                    gc[0] as f32 / 255.0,
                    gc[1] as f32 / 255.0,
                    gc[2] as f32 / 255.0,
                    gc[3] as f32 / 255.0,
                ));
                frame_color_btn.set_rgba(&gdk::RGBA::new(
                    fc[0] as f32 / 255.0,
                    fc[1] as f32 / 255.0,
                    fc[2] as f32 / 255.0,
                    fc[3] as f32 / 255.0,
                ));
                text_color_btn.set_rgba(&gdk::RGBA::new(
                    tc[0] as f32 / 255.0,
                    tc[1] as f32 / 255.0,
                    tc[2] as f32 / 255.0,
                    tc[3] as f32 / 255.0,
                ));
                transparent_bg_check.set_active(transparent_bg);
                gradient_check.set_active(gradient_enabled);
                grad_dir_dd.set_selected(match gradient_direction {
                    GradientDirection::Horizontal => 0,
                    GradientDirection::Vertical => 1,
                    GradientDirection::Diagonal => 2,
                    GradientDirection::Radial => 3,
                });
                ec_level_dd.set_selected(match ec_level {
                    ErrorCorrectionLevel::Medium => 0,
                    ErrorCorrectionLevel::Low => 1,
                    ErrorCorrectionLevel::Quartile => 2,
                    ErrorCorrectionLevel::High => 3,
                });
                module_size_dd.set_selected(match module_size {
                    32 => 0,
                    16 => 1,
                    64 => 2,
                    128 => 3,
                    _ => 0,
                });
                quiet_zone_scale.set_value(quiet_zone as f64);
                module_gap_scale.set_value(module_gap);
                shadow_check.set_active(shadow_enabled);
                shadow_offset_scale.set_value(shadow_offset);
                logo_shape_dd.set_selected(match logo_shape {
                    LogoShape::Circle => 0,
                    LogoShape::Rectangle => 1,
                    LogoShape::RoundedRect => 2,
                });
                logo_color_btn.set_rgba(&gdk::RGBA::new(
                    logo_color[0] as f32 / 255.0,
                    logo_color[1] as f32 / 255.0,
                    logo_color[2] as f32 / 255.0,
                    logo_color[3] as f32 / 255.0,
                ));
                logo_border_width_scale.set_value(logo_border_width);
                logo_border_color_btn.set_rgba(&gdk::RGBA::new(
                    logo_border_color[0] as f32 / 255.0,
                    logo_border_color[1] as f32 / 255.0,
                    logo_border_color[2] as f32 / 255.0,
                    logo_border_color[3] as f32 / 255.0,
                ));
                logo_vectorize_check.set_active(logo_vectorize);
                logo_vectorize_bg_color_btn.set_rgba(&gdk::RGBA::new(
                    logo_vectorize_bg_color[0] as f32 / 255.0,
                    logo_vectorize_bg_color[1] as f32 / 255.0,
                    logo_vectorize_bg_color[2] as f32 / 255.0,
                    logo_vectorize_bg_color[3] as f32 / 255.0,
                ));
                logo_bg_transparent_check.set_active(logo_bg_transparent);
                logo_clear_area_check.set_active(logo_clear_area);
                logo_clear_padding_spin.set_value(logo_clear_padding);
                logo_outer_radius_scale.set_value(logo_outer_radius);
                logo_inner_radius_scale.set_value(logo_inner_radius);
                logo_outer_radius_box.set_visible(matches!(logo_shape, LogoShape::RoundedRect));
                logo_inner_radius_box.set_visible(matches!(logo_shape, LogoShape::RoundedRect));
                logo_radius_sync_btn.set_visible(matches!(logo_shape, LogoShape::RoundedRect));
                frame_outer_radius_scale.set_value(frame_outer_radius);
                frame_outer_radius_box.set_visible(matches!(frame_style, FrameStyle::Rounded));
                frame_style_dd.set_selected(match frame_style {
                    FrameStyle::None => 0,
                    FrameStyle::Simple => 1,
                    FrameStyle::Rounded => 2,
                    FrameStyle::Banner => 3,
                });
                frame_width_scale.set_value(frame_width as f64);
                set_dropdown_by_string(&font_dd, &outer_text_font);
                font_size_spin.set_value(outer_text_font_size as f64);
                update_preview(&state);
                true
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    };

    // Fallback: load legacy style-only settings
    if !session_loaded {
        if let Some(path) = get_settings_path() {
            if let Ok(data) = std::fs::read_to_string(&path) {
                if let Ok(settings) = serde_json::from_str::<StyleSettings>(&data) {
                    apply_style_to_state(&state.borrow(), &settings);
                    let s = state.borrow();
                    let ds = *s.dot_style.borrow();
                    let cs = *s.corner_square_style.borrow();
                    let cd = *s.corner_dot_style.borrow();
                    let fg = s.fg_color.borrow().0;
                    let bg = s.bg_color.borrow().0;
                    let cc = s.corner_color.borrow().0;
                    let gc = s.gradient_color.borrow().0;
                    let fc = s.frame_color.borrow().0;
                    let transparent_bg = *s.transparent_bg.borrow();
                    let gradient_enabled = *s.gradient_enabled.borrow();
                    let gradient_direction = *s.gradient_direction.borrow();
                    let ec_level = *s.ec_level.borrow();
                    let module_size = *s.module_size.borrow();
                    let quiet_zone = *s.quiet_zone.borrow();
                    let module_gap = *s.module_gap.borrow();
                    let shadow_enabled = *s.shadow_enabled.borrow();
                    let shadow_offset = *s.shadow_offset.borrow();
                    let logo_shape = *s.logo_shape.borrow();
                    let logo_color = s.logo_color.borrow().0;
                    let logo_border_width = *s.logo_border_width.borrow();
                    let logo_border_color = s.logo_border_color.borrow().0;
                    let logo_vectorize = *s.logo_vectorize.borrow();
                    let logo_vectorize_bg_color = s.logo_vectorize_bg_color.borrow().0;
                    let logo_bg_transparent = *s.logo_bg_transparent.borrow();
                    let logo_clear_area = *s.logo_clear_area.borrow();
                    let logo_clear_padding = *s.logo_clear_padding.borrow();
                    let logo_outer_radius = *s.logo_outer_radius.borrow();
                    let logo_inner_radius = *s.logo_inner_radius.borrow();
                    let frame_outer_radius = *s.frame_outer_radius.borrow();
                    let frame_style = *s.frame_style.borrow();
                    let frame_width = *s.frame_width.borrow();
                    let outer_text_font = s.outer_text_font.borrow().clone();
                    let outer_text_font_size = *s.outer_text_font_size.borrow();
                    drop(s);
                    dot_style_dd.set_selected(match ds {
                        DotStyle::Rounded => 0,
                        DotStyle::Square => 1,
                        DotStyle::Dots => 2,
                        DotStyle::Diamond => 3,
                        DotStyle::Custom => 4,
                    });
                    custom_dot_box.set_visible(matches!(ds, DotStyle::Custom));
                    corner_sq_dd.set_selected(match cs {
                        CornerSquareStyle::ExtraRounded => 0,
                        CornerSquareStyle::Square => 1,
                        CornerSquareStyle::Dot => 2,
                        CornerSquareStyle::Circle => 3,
                    });
                    corner_dot_dd.set_selected(match cd {
                        CornerDotStyle::Dot => 0,
                        CornerDotStyle::Square => 1,
                        CornerDotStyle::Circle => 2,
                        CornerDotStyle::ExtraRounded => 3,
                    });
                    fg_color_btn.set_rgba(&gdk::RGBA::new(
                        fg[0] as f32 / 255.0,
                        fg[1] as f32 / 255.0,
                        fg[2] as f32 / 255.0,
                        fg[3] as f32 / 255.0,
                    ));
                    bg_color_btn.set_rgba(&gdk::RGBA::new(
                        bg[0] as f32 / 255.0,
                        bg[1] as f32 / 255.0,
                        bg[2] as f32 / 255.0,
                        bg[3] as f32 / 255.0,
                    ));
                    corner_color_btn.set_rgba(&gdk::RGBA::new(
                        cc[0] as f32 / 255.0,
                        cc[1] as f32 / 255.0,
                        cc[2] as f32 / 255.0,
                        cc[3] as f32 / 255.0,
                    ));
                    grad_color_btn.set_rgba(&gdk::RGBA::new(
                        gc[0] as f32 / 255.0,
                        gc[1] as f32 / 255.0,
                        gc[2] as f32 / 255.0,
                        gc[3] as f32 / 255.0,
                    ));
                    frame_color_btn.set_rgba(&gdk::RGBA::new(
                        fc[0] as f32 / 255.0,
                        fc[1] as f32 / 255.0,
                        fc[2] as f32 / 255.0,
                        fc[3] as f32 / 255.0,
                    ));
                    transparent_bg_check.set_active(transparent_bg);
                    gradient_check.set_active(gradient_enabled);
                    grad_dir_dd.set_selected(match gradient_direction {
                        GradientDirection::Horizontal => 0,
                        GradientDirection::Vertical => 1,
                        GradientDirection::Diagonal => 2,
                        GradientDirection::Radial => 3,
                    });
                    ec_level_dd.set_selected(match ec_level {
                        ErrorCorrectionLevel::Medium => 0,
                        ErrorCorrectionLevel::Low => 1,
                        ErrorCorrectionLevel::Quartile => 2,
                        ErrorCorrectionLevel::High => 3,
                    });
                    module_size_dd.set_selected(match module_size {
                        32 => 0,
                        16 => 1,
                        64 => 2,
                        128 => 3,
                        _ => 0,
                    });
                    quiet_zone_scale.set_value(quiet_zone as f64);
                    module_gap_scale.set_value(module_gap);
                    shadow_check.set_active(shadow_enabled);
                    shadow_offset_scale.set_value(shadow_offset);
                    logo_shape_dd.set_selected(match logo_shape {
                        LogoShape::Circle => 0,
                        LogoShape::Rectangle => 1,
                        LogoShape::RoundedRect => 2,
                    });
                    logo_color_btn.set_rgba(&gdk::RGBA::new(
                        logo_color[0] as f32 / 255.0,
                        logo_color[1] as f32 / 255.0,
                        logo_color[2] as f32 / 255.0,
                        logo_color[3] as f32 / 255.0,
                    ));
                    logo_border_width_scale.set_value(logo_border_width);
                    logo_border_color_btn.set_rgba(&gdk::RGBA::new(
                        logo_border_color[0] as f32 / 255.0,
                        logo_border_color[1] as f32 / 255.0,
                        logo_border_color[2] as f32 / 255.0,
                        logo_border_color[3] as f32 / 255.0,
                    ));
                    logo_vectorize_check.set_active(logo_vectorize);
                    logo_vectorize_bg_color_btn.set_rgba(&gdk::RGBA::new(
                        logo_vectorize_bg_color[0] as f32 / 255.0,
                        logo_vectorize_bg_color[1] as f32 / 255.0,
                        logo_vectorize_bg_color[2] as f32 / 255.0,
                        logo_vectorize_bg_color[3] as f32 / 255.0,
                    ));
                    logo_bg_transparent_check.set_active(logo_bg_transparent);
                    logo_clear_area_check.set_active(logo_clear_area);
                    logo_clear_padding_spin.set_value(logo_clear_padding);
                    logo_outer_radius_scale.set_value(logo_outer_radius);
                    logo_inner_radius_scale.set_value(logo_inner_radius);
                    logo_outer_radius_box.set_visible(matches!(logo_shape, LogoShape::RoundedRect));
                    logo_inner_radius_box.set_visible(matches!(logo_shape, LogoShape::RoundedRect));
                    logo_radius_sync_btn.set_visible(matches!(logo_shape, LogoShape::RoundedRect));
                    frame_outer_radius_scale.set_value(frame_outer_radius);
                    frame_outer_radius_box.set_visible(matches!(frame_style, FrameStyle::Rounded));
                    frame_style_dd.set_selected(match frame_style {
                        FrameStyle::None => 0,
                        FrameStyle::Simple => 1,
                        FrameStyle::Rounded => 2,
                        FrameStyle::Banner => 3,
                    });
                    frame_width_scale.set_value(frame_width as f64);
                    set_dropdown_by_string(&font_dd, &outer_text_font);
                    font_size_spin.set_value(outer_text_font_size as f64);
                    update_preview(&state);
                }
            }
        }
    }

    // Helper closure to sync all widgets to current state (shared by undo/redo)
    let sync_widgets = Rc::new({
        let state = state.clone();
        let dot_style_dd = dot_style_dd.clone();
        let corner_sq_dd = corner_sq_dd.clone();
        let corner_dot_dd = corner_dot_dd.clone();
        let fg_color_btn = fg_color_btn.clone();
        let bg_color_btn = bg_color_btn.clone();
        let corner_color_btn = corner_color_btn.clone();
        let transparent_bg_check = transparent_bg_check.clone();
        let gradient_check = gradient_check.clone();
        let grad_color_btn = grad_color_btn.clone();
        let grad_dir_dd = grad_dir_dd.clone();
        let ec_level_dd = ec_level_dd.clone();
        let module_size_dd = module_size_dd.clone();
        let quiet_zone_scale = quiet_zone_scale.clone();
        let module_gap_scale = module_gap_scale.clone();
        let shadow_check = shadow_check.clone();
        let shadow_offset_scale = shadow_offset_scale.clone();
        let logo_shape_dd = logo_shape_dd.clone();
        let logo_size_scale = logo_size_scale.clone();
        let logo_color_btn = logo_color_btn.clone();
        let logo_border_width_scale = logo_border_width_scale.clone();
        let logo_border_color_btn = logo_border_color_btn.clone();
        let logo_vectorize_check = logo_vectorize_check.clone();
        let logo_vectorize_bg_color_btn = logo_vectorize_bg_color_btn.clone();
        let logo_bg_transparent_check = logo_bg_transparent_check.clone();
        let logo_clear_area_check = logo_clear_area_check.clone();
        let logo_clear_padding_spin = logo_clear_padding_spin.clone();
        let logo_outer_radius_scale = logo_outer_radius_scale.clone();
        let logo_inner_radius_scale = logo_inner_radius_scale.clone();
        let logo_radius_sync_btn = logo_radius_sync_btn.clone();
        let frame_style_dd = frame_style_dd.clone();
        let frame_color_btn = frame_color_btn.clone();
        let frame_width_scale = frame_width_scale.clone();
        let frame_outer_radius_scale = frame_outer_radius_scale.clone();
        let font_dd = font_dd.clone();
        let font_size_spin = font_size_spin.clone();
        let preset_dd = preset_dd.clone();
        let palette_dd = palette_dd.clone();
        let custom_dot_box = custom_dot_box.clone();
        move || {
            // Read all style values from state (single borrow, then drop)
            let (
                ds,
                cs,
                cd,
                fg,
                bg,
                cc,
                tr_bg,
                g_en,
                g_col,
                g_dir,
                ec_lev,
                m_size,
                qz,
                m_gap,
                sh_en,
                sh_off,
                l_shape,
                l_size,
                l_col,
                l_bw,
                l_bc,
                l_vec,
                l_vbg,
                l_bg_tr,
                l_ca,
                l_cp,
                l_or,
                l_ir,
                f_style,
                f_col,
                f_w,
                f_or,
                o_text_font,
                o_text_font_size,
            ) = {
                let s = state.borrow();
                let x = (
                    *s.dot_style.borrow(),
                    *s.corner_square_style.borrow(),
                    *s.corner_dot_style.borrow(),
                    *s.fg_color.borrow(),
                    *s.bg_color.borrow(),
                    *s.corner_color.borrow(),
                    *s.transparent_bg.borrow(),
                    *s.gradient_enabled.borrow(),
                    *s.gradient_color.borrow(),
                    *s.gradient_direction.borrow(),
                    *s.ec_level.borrow(),
                    *s.module_size.borrow(),
                    *s.quiet_zone.borrow(),
                    *s.module_gap.borrow(),
                    *s.shadow_enabled.borrow(),
                    *s.shadow_offset.borrow(),
                    *s.logo_shape.borrow(),
                    *s.logo_size.borrow(),
                    *s.logo_color.borrow(),
                    *s.logo_border_width.borrow(),
                    *s.logo_border_color.borrow(),
                    *s.logo_vectorize.borrow(),
                    *s.logo_vectorize_bg_color.borrow(),
                    *s.logo_bg_transparent.borrow(),
                    *s.logo_clear_area.borrow(),
                    *s.logo_clear_padding.borrow(),
                    *s.logo_outer_radius.borrow(),
                    *s.logo_inner_radius.borrow(),
                    *s.frame_style.borrow(),
                    *s.frame_color.borrow(),
                    *s.frame_width.borrow(),
                    *s.frame_outer_radius.borrow(),
                    s.outer_text_font.borrow().clone(),
                    *s.outer_text_font_size.borrow(),
                );
                x
            };

            // Prevent signal handlers from pushing to undo stack during restore
            *state.borrow().is_restoring.borrow_mut() = true;

            // Dropdowns
            dot_style_dd.set_selected(match ds {
                DotStyle::Rounded => 0,
                DotStyle::Square => 1,
                DotStyle::Dots => 2,
                DotStyle::Diamond => 3,
                DotStyle::Custom => 4,
            });
            custom_dot_box.set_visible(matches!(ds, DotStyle::Custom));
            corner_sq_dd.set_selected(match cs {
                CornerSquareStyle::ExtraRounded => 0,
                CornerSquareStyle::Square => 1,
                CornerSquareStyle::Dot => 2,
                CornerSquareStyle::Circle => 3,
            });
            corner_dot_dd.set_selected(match cd {
                CornerDotStyle::Dot => 0,
                CornerDotStyle::Square => 1,
                CornerDotStyle::Circle => 2,
                CornerDotStyle::ExtraRounded => 3,
            });
            grad_dir_dd.set_selected(match g_dir {
                GradientDirection::Horizontal => 0,
                GradientDirection::Vertical => 1,
                GradientDirection::Diagonal => 2,
                GradientDirection::Radial => 3,
            });
            ec_level_dd.set_selected(match ec_lev {
                ErrorCorrectionLevel::Medium => 0,
                ErrorCorrectionLevel::Low => 1,
                ErrorCorrectionLevel::Quartile => 2,
                ErrorCorrectionLevel::High => 3,
            });
            module_size_dd.set_selected(match m_size {
                32 => 0,
                16 => 1,
                64 => 2,
                128 => 3,
                _ => 0,
            });
            logo_shape_dd.set_selected(match l_shape {
                LogoShape::Circle => 0,
                LogoShape::Rectangle => 1,
                LogoShape::RoundedRect => 2,
            });
            frame_style_dd.set_selected(match f_style {
                FrameStyle::None => 0,
                FrameStyle::Simple => 1,
                FrameStyle::Rounded => 2,
                FrameStyle::Banner => 3,
            });
            preset_dd.set_selected(0);
            palette_dd.set_selected(0);

            // Color buttons
            fg_color_btn.set_rgba(&rgba_to_gdk(&fg));
            bg_color_btn.set_rgba(&rgba_to_gdk(&bg));
            corner_color_btn.set_rgba(&rgba_to_gdk(&cc));
            grad_color_btn.set_rgba(&rgba_to_gdk(&g_col));
            logo_color_btn.set_rgba(&rgba_to_gdk(&l_col));
            logo_border_color_btn.set_rgba(&rgba_to_gdk(&l_bc));
            logo_vectorize_bg_color_btn.set_rgba(&rgba_to_gdk(&l_vbg));
            frame_color_btn.set_rgba(&rgba_to_gdk(&f_col));

            // Check buttons
            transparent_bg_check.set_active(tr_bg);
            gradient_check.set_active(g_en);
            shadow_check.set_active(sh_en);
            logo_vectorize_check.set_active(l_vec);
            logo_bg_transparent_check.set_active(l_bg_tr);
            logo_clear_area_check.set_active(l_ca);
            logo_radius_sync_btn.set_active((l_or - l_ir).abs() < 0.001);

            // Scales & spin buttons
            quiet_zone_scale.set_value(qz as f64);
            module_gap_scale.set_value(m_gap);
            shadow_offset_scale.set_value(sh_off);
            logo_size_scale.set_value(l_size);
            logo_outer_radius_scale.set_value(l_or);
            logo_inner_radius_scale.set_value(l_ir);
            logo_border_width_scale.set_value(l_bw);
            logo_clear_padding_spin.set_value(l_cp);
            frame_width_scale.set_value(f_w as f64);
            frame_outer_radius_scale.set_value(f_or);
            set_dropdown_by_string(&font_dd, &o_text_font);
            font_size_spin.set_value(o_text_font_size as f64);

            *state.borrow().is_restoring.borrow_mut() = false;
        }
    });

    // Undo button
    {
        let state = state.clone();
        let sync_widgets = sync_widgets.clone();
        undo_btn.connect_clicked(move |_| {
            if let Some(prev) = state.borrow().undo_stack.borrow_mut().pop() {
                let current = current_style_settings(&state.borrow());
                state.borrow().redo_stack.borrow_mut().push(current);
                apply_style_to_state(&state.borrow(), &prev);
                sync_widgets();
                update_preview(&state);
                // Animation 3: Undo pulse
                state.borrow().preview_picture.add_css_class("undo-pulse");
                let pic = state.borrow().preview_picture.clone();
                glib::timeout_add_local(Duration::from_millis(500), move || {
                    pic.remove_css_class("undo-pulse");
                    glib::ControlFlow::Break
                });
            }
        });
    }

    // Redo button
    {
        let state = state.clone();
        let sync_widgets = sync_widgets.clone();
        redo_btn.connect_clicked(move |_| {
            if let Some(next) = state.borrow().redo_stack.borrow_mut().pop() {
                let current = current_style_settings(&state.borrow());
                state.borrow().undo_stack.borrow_mut().push(current);
                apply_style_to_state(&state.borrow(), &next);
                sync_widgets();
                update_preview(&state);
                // Animation 3: Redo pulse
                state.borrow().preview_picture.add_css_class("undo-pulse");
                let pic = state.borrow().preview_picture.clone();
                glib::timeout_add_local(Duration::from_millis(500), move || {
                    pic.remove_css_class("undo-pulse");
                    glib::ControlFlow::Break
                });
            }
        });
    }

    // ============================================================
    // Copy SVG to clipboard
    // ============================================================
    {
        let state = state.clone();
        copy_svg_btn.connect_clicked(move |_| {
            let s = state.borrow();
            if let Some(svg) = render_svg_from_state(&s) {
                if let Some(display) = gdk::Display::default() {
                    let clipboard = display.clipboard();
                    let bytes = glib::Bytes::from(svg.as_bytes());
                    let provider = gdk::ContentProvider::for_bytes("image/svg+xml", &bytes);
                    let _ = clipboard.set_content(Some(&provider));
                }
                s.update_status_typed(&s.i18n.borrow().t("status_copied_svg"), ToastType::Success);
            } else {
                s.update_status_typed(&s.i18n.borrow().t("status_render_error"), ToastType::Error);
            }
        });
    }

    // ============================================================
    // Copy to clipboard (PNG, capped at 1024px)
    // ============================================================
    {
        let state = state.clone();
        copy_btn.connect_clicked(move |_| {
            let s = state.borrow();
            if let Some(img) = render_qr_from_state(&s) {
                let mut w = img.width();
                let mut h = img.height();
                // Cap at 1024px for clipboard
                let img = if w > 1024 || h > 1024 {
                    let scale = 1024.0 / w.max(h) as f64;
                    w = (w as f64 * scale) as u32;
                    h = (h as f64 * scale) as u32;
                    image::imageops::resize(
                        &img,
                        w.max(1),
                        h.max(1),
                        image::imageops::FilterType::Lanczos3,
                    )
                } else {
                    img
                };
                let stride = (w as usize) * 4;
                let bytes = glib::Bytes::from(&img.into_raw());
                let texture = gdk::MemoryTexture::new(
                    w as i32,
                    h as i32,
                    gdk::MemoryFormat::R8g8b8a8,
                    &bytes,
                    stride,
                );
                let clipboard = gdk::Display::default().map(|d| d.clipboard());
                if let Some(cb) = clipboard {
                    cb.set_texture(&texture);
                }
                s.update_status_typed(&s.i18n.borrow().t("status_copied"), ToastType::Success);
            } else {
                s.update_status_typed(&s.i18n.borrow().t("status_render_error"), ToastType::Error);
            }
        });
    }

    // Save PNG
    {
        let state = state.clone();
        save_png_btn.connect_clicked(move |_| {
            let state_ref = state.borrow();
            let i18n = state_ref.i18n.borrow();
            let dialog = FileChooserDialog::new(
                Some(&i18n.t("dlg_save_png")),
                None::<&gtk4::Window>,
                gtk4::FileChooserAction::Save,
                &[
                    (&i18n.t("btn_cancel"), gtk4::ResponseType::Cancel),
                    (&i18n.t("btn_save"), gtk4::ResponseType::Accept),
                ],
            );
            drop(i18n);
            drop(state_ref);
            dialog.set_current_name("qrcode.png");
            let state = state.clone();
            dialog.connect_response(move |dlg, resp| {
                if resp == gtk4::ResponseType::Accept {
                    if let Some(file) = dlg.file() {
                        if let Some(path) = file.path() {
                            let s = state.borrow();
                            if let Some(img) = render_qr_from_state(&s) {
                                let _ = img.save(&path);
                                s.update_status_typed(
                                    &s.i18n.borrow().t("status_png_saved"),
                                    ToastType::Success,
                                );
                            }
                        }
                    }
                }
                dlg.close();
            });
            dialog.show();
        });
    }

    // Save SVG
    {
        let state = state.clone();
        save_svg_btn.connect_clicked(move |_| {
            let state_ref = state.borrow();
            let i18n = state_ref.i18n.borrow();
            let dialog = FileChooserDialog::new(
                Some(&i18n.t("dlg_save_svg")),
                None::<&gtk4::Window>,
                gtk4::FileChooserAction::Save,
                &[
                    (&i18n.t("btn_cancel"), gtk4::ResponseType::Cancel),
                    (&i18n.t("btn_save"), gtk4::ResponseType::Accept),
                ],
            );
            drop(i18n);
            drop(state_ref);
            dialog.set_current_name("qrcode.svg");
            let state = state.clone();
            dialog.connect_response(move |dlg, resp| {
                if resp == gtk4::ResponseType::Accept {
                    if let Some(file) = dlg.file() {
                        if let Some(path) = file.path() {
                            let s = state.borrow();
                            if let Some(svg) = render_svg_from_state(&s) {
                                let _ = std::fs::write(path, svg);
                                s.update_status_typed(
                                    &s.i18n.borrow().t("status_svg_saved"),
                                    ToastType::Success,
                                );
                            }
                        }
                    }
                }
                dlg.close();
            });
            dialog.show();
        });
    }

    // Save GIF
    {
        let state = state.clone();
        save_gif_btn.connect_clicked(move |_| {
            let state_ref = state.borrow();
            let i18n = state_ref.i18n.borrow();
            let dialog = FileChooserDialog::new(
                Some(&i18n.t("dlg_save_gif")),
                None::<&gtk4::Window>,
                gtk4::FileChooserAction::Save,
                &[
                    (&i18n.t("btn_cancel"), gtk4::ResponseType::Cancel),
                    (&i18n.t("btn_save"), gtk4::ResponseType::Accept),
                ],
            );
            drop(i18n);
            drop(state_ref);
            dialog.set_current_name("qrcode.gif");
            let state = state.clone();
            dialog.connect_response(move |dlg, resp| {
                if resp == gtk4::ResponseType::Accept {
                    if let Some(file) = dlg.file() {
                        if let Some(path) = file.path() {
                            let s = state.borrow();
                            if let Some(gif_data) = render_gif_from_state(&s) {
                                let _ = std::fs::write(path, gif_data);
                                s.update_status_typed(
                                    &s.i18n.borrow().t("status_gif_saved"),
                                    ToastType::Success,
                                );
                            } else {
                                s.update_status_typed(
                                    &s.i18n.borrow().t("status_gif_gradient_only"),
                                    ToastType::Error,
                                );
                            }
                        }
                    }
                }
                dlg.close();
            });
            dialog.show();
        });
    }

    // Batch export
    {
        let state = state.clone();
        batch_btn.connect_clicked(move |_| {
            let state_ref = state.borrow();
            let i18n = state_ref.i18n.borrow();
            let dialog = gtk4::Dialog::with_buttons(
                Some(&i18n.t("dlg_batch_export")),
                None::<&gtk4::Window>,
                gtk4::DialogFlags::MODAL,
                &[
                    (&i18n.t("btn_cancel"), gtk4::ResponseType::Cancel),
                    (&i18n.t("btn_export"), gtk4::ResponseType::Accept),
                ],
            );
            dialog.set_default_size(400, 350);
            let content = dialog.content_area();
            let box_v = Box::new(Orientation::Vertical, 8);
            box_v.set_margin_start(12);
            box_v.set_margin_end(12);
            box_v.set_margin_top(12);
            box_v.set_margin_bottom(12);

            let info_label = Label::new(Some(&i18n.t("batch_data_label")));
            box_v.append(&info_label);

            let batch_buffer = TextBuffer::new(None::<&gtk4::TextTagTable>);
            let batch_view = TextView::with_buffer(&batch_buffer);
            batch_view.set_wrap_mode(gtk4::WrapMode::WordChar);
            let batch_scroll = ScrolledWindow::new();
            batch_scroll.set_min_content_height(150);
            batch_scroll.set_child(Some(&batch_view));
            box_v.append(&batch_scroll);

            // --- CSV Import ---
            let csv_btn = make_icon_btn("text-x-csv-symbolic", &i18n.t("btn_import_style_short"));
            let csv_hint = Label::new(Some(&i18n.t("batch_csv_hint")));
            csv_hint.add_css_class("dim-label");
            let csv_batch_buffer = batch_buffer.clone();
            let i18n_for_csv = state.borrow().i18n.borrow().t("dlg_select_csv").to_string();
            let i18n_btn_cancel_csv = state.borrow().i18n.borrow().t("btn_cancel").to_string();
            let i18n_btn_open_csv = state.borrow().i18n.borrow().t("btn_open").to_string();
            let i18n_filter_csv = state.borrow().i18n.borrow().t("filter_csv_txt").to_string();
            csv_btn.connect_clicked(move |_| {
                let filter = FileFilter::new();
                filter.add_pattern("*.csv");
                filter.add_pattern("*.txt");
                filter.set_name(Some(&i18n_filter_csv));
                let dlg = FileChooserDialog::new(
                    Some(&i18n_for_csv),
                    None::<&gtk4::Window>,
                    gtk4::FileChooserAction::Open,
                    &[
                        (&i18n_btn_cancel_csv, gtk4::ResponseType::Cancel),
                        (&i18n_btn_open_csv, gtk4::ResponseType::Accept),
                    ],
                );
                dlg.set_filter(&filter);
                let buf = csv_batch_buffer.clone();
                dlg.connect_response(move |d, resp| {
                    if resp == gtk4::ResponseType::Accept {
                        if let Some(file) = d.file() {
                            if let Some(path) = file.path() {
                                if let Ok(content) = std::fs::read_to_string(&path) {
                                    let first_line = content.lines().next().unwrap_or("");
                                    let delim = if first_line.contains(';') {
                                        ';'
                                    } else if first_line.contains('\t') {
                                        '\t'
                                    } else {
                                        ','
                                    };
                                    let entries: Vec<String> = content
                                        .lines()
                                        .skip(1) // skip header row
                                        .filter_map(|line| {
                                            line.split(delim).next().map(|s| s.trim().to_string())
                                        })
                                        .filter(|s| !s.is_empty())
                                        .collect();
                                    buf.set_text(&entries.join("\n"));
                                }
                            }
                        }
                    }
                    d.close();
                });
                dlg.show();
            });
            box_v.append(&csv_btn);
            box_v.append(&csv_hint);

            let fmt_label = Label::new(Some(&i18n.t("batch_format")));
            box_v.append(&fmt_label);
            let fmt_list = StringList::new(&["PNG", "SVG", "GIF"]);
            let fmt_dd = DropDown::new(
                Some(fmt_list.upcast::<gtk4::gio::ListModel>()),
                None::<gtk4::Expression>,
            );
            box_v.append(&fmt_dd);

            let folder_label = Label::new(Some(&i18n.t("batch_folder_label")));
            box_v.append(&folder_label);
            let folder_btn = make_icon_btn("folder-open-symbolic", &i18n.t("dlg_select_folder"));
            let folder_path: Rc<RefCell<Option<PathBuf>>> = Rc::new(RefCell::new(None));
            let folder_path_clone = folder_path.clone();
            let folder_label_clone = folder_label.clone();
            let i18n_folder_selected = state
                .borrow()
                .i18n
                .borrow()
                .t("batch_folder_selected")
                .to_string();
            let i18n_dlg_select_folder = state
                .borrow()
                .i18n
                .borrow()
                .t("dlg_select_folder")
                .to_string();
            let i18n_btn_cancel_folder = state.borrow().i18n.borrow().t("btn_cancel").to_string();
            let i18n_btn_select_folder = state.borrow().i18n.borrow().t("btn_select").to_string();
            folder_btn.connect_clicked(move |_| {
                let dialog = FileChooserDialog::new(
                    Some(&i18n_dlg_select_folder),
                    None::<&gtk4::Window>,
                    gtk4::FileChooserAction::SelectFolder,
                    &[
                        (&i18n_btn_cancel_folder, gtk4::ResponseType::Cancel),
                        (&i18n_btn_select_folder, gtk4::ResponseType::Accept),
                    ],
                );
                let folder_path = folder_path_clone.clone();
                let folder_label = folder_label_clone.clone();
                let i18n_folder_fmt = i18n_folder_selected.clone();
                dialog.connect_response(move |dlg, resp| {
                    if resp == gtk4::ResponseType::Accept {
                        if let Some(file) = dlg.file() {
                            if let Some(path) = file.path() {
                                folder_path.replace(Some(path.clone()));
                                folder_label.set_text(
                                    &i18n_folder_fmt.replace("{}", &path.display().to_string()),
                                );
                            }
                        }
                    }
                    dlg.close();
                });
                dialog.show();
            });
            box_v.append(&folder_btn);
            drop(i18n);
            drop(state_ref);

            content.append(&box_v);

            let state = state.clone();
            let batch_buffer = batch_buffer.clone();
            let fmt_dd = fmt_dd.clone();
            let folder_path = folder_path.clone();
            dialog.connect_response(move |dlg, resp| {
                if resp == gtk4::ResponseType::Accept {
                    let text = batch_buffer.text(
                        &batch_buffer.start_iter(),
                        &batch_buffer.end_iter(),
                        false,
                    );
                    let lines: Vec<&str> = text.lines().filter(|l| !l.trim().is_empty()).collect();
                    let fmt_idx = fmt_dd.selected();
                    let folder = folder_path.borrow().clone();
                    if let Some(folder) = folder {
                        let s = state.borrow();
                        for (i, line) in lines.iter().enumerate() {
                            let data = line.trim().to_string();
                            if data.is_empty() {
                                continue;
                            }
                            let suffix = match fmt_idx {
                                1 => "svg",
                                2 => "gif",
                                _ => "png",
                            };
                            let filename = format!("qr_{:03}.{}", i + 1, suffix);
                            let path = folder.join(&filename);
                            match fmt_idx {
                                1 => {
                                    if let Some(svg) = render_svg_from_state(&s) {
                                        let _ = std::fs::write(&path, svg);
                                    }
                                }
                                2 => {
                                    if let Some(gif) = render_gif_from_state(&s) {
                                        let _ = std::fs::write(&path, gif);
                                    }
                                }
                                _ => {
                                    if let Some(img) = render_qr_from_state(&s) {
                                        let _ = img.save(&path);
                                    }
                                }
                            }
                        }
                        s.update_status_typed(
                            &s.i18n
                                .borrow()
                                .t("status_batch_exported")
                                .replace("{}", &lines.len().to_string()),
                            ToastType::Success,
                        );
                    }
                }
                dlg.close();
            });
            dialog.show();
        });
    }

    // ============================================================
    // DRAG & DROP on preview (logo import)
    // ============================================================
    {
        let target = gtk4::DropTarget::new(glib::Type::STRING, gdk::DragAction::COPY);
        target.set_types(&[glib::Type::STRING, gtk4::gio::File::static_type()]);
        preview_picture.add_controller(target.clone());

        // DnD zone highlight: glow when dragging over preview
        {
            let pic = preview_picture.clone();
            target.connect_enter(move |_, _, _| {
                pic.add_css_class("drop-active");
                gdk::DragAction::COPY
            });
            let pic = preview_picture.clone();
            target.connect_leave(move |_| {
                pic.remove_css_class("drop-active");
            });
        }

        let state = state.clone();
        let ec_level_dd = ec_level_dd.clone();
        target.connect_drop(move |_tgt, val, _x, _y| {
            if let Ok(file) = val.get::<gtk4::gio::File>() {
                if let Some(path) = file.path() {
                    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                    let is_image =
                        matches!(ext, "png" | "jpg" | "jpeg" | "gif" | "svg" | "bmp" | "webp");
                    if is_image && path.exists() {
                        save_undo_state(&state.borrow());
                        state.borrow().logo_path.replace(Some(path));
                        if *state.borrow().ec_level.borrow() != ErrorCorrectionLevel::High {
                            *state.borrow().ec_level.borrow_mut() = ErrorCorrectionLevel::High;
                            ec_level_dd.set_selected(3);
                        }
                        schedule_preview(&state);
                        let msg = state.borrow().i18n.borrow().t("dnd_logo_imported");
                        state.borrow().update_status_typed(&msg, ToastType::Success);
                        // Animation 8: Logo drop bounce
                        {
                            let pic = state.borrow().preview_picture.clone();
                            pic.add_css_class("preview-bounce");
                            glib::timeout_add_local(Duration::from_millis(450), move || {
                                pic.remove_css_class("preview-bounce");
                                glib::ControlFlow::Break
                            });
                        }
                        return true;
                    }
                }
            }
            if let Ok(uri) = val.get::<String>() {
                let path_str = uri.trim();
                let path = if path_str.starts_with("file://") {
                    PathBuf::from(path_str.trim_start_matches("file://"))
                } else if path_str.starts_with('/') {
                    PathBuf::from(path_str)
                } else {
                    return false;
                };
                if path.exists() {
                    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                    let is_image =
                        matches!(ext, "png" | "jpg" | "jpeg" | "gif" | "svg" | "bmp" | "webp");
                    if is_image {
                        save_undo_state(&state.borrow());
                        state.borrow().logo_path.replace(Some(path));
                        if *state.borrow().ec_level.borrow() != ErrorCorrectionLevel::High {
                            *state.borrow().ec_level.borrow_mut() = ErrorCorrectionLevel::High;
                            ec_level_dd.set_selected(3);
                        }
                        schedule_preview(&state);
                        let msg = state.borrow().i18n.borrow().t("dnd_logo_imported");
                        state.borrow().update_status_typed(&msg, ToastType::Success);
                        // Animation 8: Logo drop bounce
                        {
                            let pic = state.borrow().preview_picture.clone();
                            pic.add_css_class("preview-bounce");
                            glib::timeout_add_local(Duration::from_millis(450), move || {
                                pic.remove_css_class("preview-bounce");
                                glib::ControlFlow::Break
                            });
                        }
                        return true;
                    }
                }
            }
            false
        });
    }

    // ============================================================
    // DRAG & DROP on logo area (direct logo import)
    // ============================================================
    {
        let logo_target = gtk4::DropTarget::new(glib::Type::STRING, gdk::DragAction::COPY);
        logo_target.set_types(&[glib::Type::STRING, gtk4::gio::File::static_type()]);
        style_split_pane.add_controller(logo_target.clone());

        // DnD zone highlight: glow when dragging over style area
        {
            let pane = style_split_pane.clone();
            logo_target.connect_enter(move |_, _, _| {
                pane.add_css_class("drop-active");
                gdk::DragAction::COPY
            });
            let pane = style_split_pane.clone();
            logo_target.connect_leave(move |_| {
                pane.remove_css_class("drop-active");
            });
        }

        let state = state.clone();
        let ec_level_dd = ec_level_dd.clone();
        logo_target.connect_drop(move |_tgt, val, _x, _y| {
            if let Ok(file) = val.get::<gtk4::gio::File>() {
                if let Some(path) = file.path() {
                    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                    let is_image =
                        matches!(ext, "png" | "jpg" | "jpeg" | "gif" | "svg" | "bmp" | "webp");
                    if is_image && path.exists() {
                        save_undo_state(&state.borrow());
                        state.borrow().logo_path.replace(Some(path));
                        if *state.borrow().ec_level.borrow() != ErrorCorrectionLevel::High {
                            *state.borrow().ec_level.borrow_mut() = ErrorCorrectionLevel::High;
                            ec_level_dd.set_selected(3);
                        }
                        schedule_preview(&state);
                        let msg = state.borrow().i18n.borrow().t("dnd_logo_imported");
                        state.borrow().update_status_typed(&msg, ToastType::Success);
                        // Animation 8: Logo drop bounce
                        {
                            let pic = state.borrow().preview_picture.clone();
                            pic.add_css_class("preview-bounce");
                            glib::timeout_add_local(Duration::from_millis(450), move || {
                                pic.remove_css_class("preview-bounce");
                                glib::ControlFlow::Break
                            });
                        }
                        return true;
                    }
                }
            }
            if let Ok(uri) = val.get::<String>() {
                let path_str = uri.trim();
                let path = if path_str.starts_with("file://") {
                    PathBuf::from(path_str.trim_start_matches("file://"))
                } else if path_str.starts_with('/') {
                    PathBuf::from(path_str)
                } else {
                    return false;
                };
                if path.exists() {
                    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                    let is_image =
                        matches!(ext, "png" | "jpg" | "jpeg" | "gif" | "svg" | "bmp" | "webp");
                    if is_image {
                        save_undo_state(&state.borrow());
                        state.borrow().logo_path.replace(Some(path));
                        if *state.borrow().ec_level.borrow() != ErrorCorrectionLevel::High {
                            *state.borrow().ec_level.borrow_mut() = ErrorCorrectionLevel::High;
                            ec_level_dd.set_selected(3);
                        }
                        schedule_preview(&state);
                        let msg = state.borrow().i18n.borrow().t("dnd_logo_imported");
                        state.borrow().update_status_typed(&msg, ToastType::Success);
                        // Animation 8: Logo drop bounce
                        {
                            let pic = state.borrow().preview_picture.clone();
                            pic.add_css_class("preview-bounce");
                            glib::timeout_add_local(Duration::from_millis(450), move || {
                                pic.remove_css_class("preview-bounce");
                                glib::ControlFlow::Break
                            });
                        }
                        return true;
                    }
                }
            }
            false
        });
    }

    // ============================================================
    // KEYBOARD SHORTCUTS
    // ============================================================
    let ctrl_z = gtk4::Shortcut::new(
        Some(gtk4::ShortcutTrigger::parse_string("<Control>z").unwrap()),
        Some(gtk4::CallbackAction::new({
            let state = state.clone();
            let sync_widgets = sync_widgets.clone();
            move |_, _| {
                let settings = {
                    let s = state.borrow();
                    let x = s.undo_stack.borrow_mut().pop();
                    x
                };
                if let Some(prev) = settings {
                    let current = current_style_settings(&state.borrow());
                    state.borrow().redo_stack.borrow_mut().push(current);
                    apply_style_to_state(&state.borrow(), &prev);
                    sync_widgets();
                    update_preview(&state);
                    // Animation 3: Undo pulse
                    state.borrow().preview_picture.add_css_class("undo-pulse");
                    let pic = state.borrow().preview_picture.clone();
                    glib::timeout_add_local(Duration::from_millis(500), move || {
                        pic.remove_css_class("undo-pulse");
                        glib::ControlFlow::Break
                    });
                }
                glib::Propagation::Proceed
            }
        })),
    );
    let ctrl_y = gtk4::Shortcut::new(
        Some(gtk4::ShortcutTrigger::parse_string("<Control>y").unwrap()),
        Some(gtk4::CallbackAction::new({
            let state = state.clone();
            let sync_widgets = sync_widgets.clone();
            move |_, _| {
                let settings = {
                    let s = state.borrow();
                    let x = s.redo_stack.borrow_mut().pop();
                    x
                };
                if let Some(next) = settings {
                    let current = current_style_settings(&state.borrow());
                    state.borrow().undo_stack.borrow_mut().push(current);
                    apply_style_to_state(&state.borrow(), &next);
                    sync_widgets();
                    update_preview(&state);
                    // Animation 3: Redo pulse
                    state.borrow().preview_picture.add_css_class("undo-pulse");
                    let pic = state.borrow().preview_picture.clone();
                    glib::timeout_add_local(Duration::from_millis(500), move || {
                        pic.remove_css_class("undo-pulse");
                        glib::ControlFlow::Break
                    });
                }
                glib::Propagation::Proceed
            }
        })),
    );
    let ctrl_c = gtk4::Shortcut::new(
        Some(gtk4::ShortcutTrigger::parse_string("<Control>c").unwrap()),
        Some(gtk4::CallbackAction::new({
            let state = state.clone();
            move |_, _| {
                let s = state.borrow();
                if let Some(img) = render_qr_from_state(&s) {
                    let w = img.width();
                    let h = img.height();
                    let stride = (w as usize) * 4;
                    let bytes = glib::Bytes::from(&img.into_raw());
                    let texture = gdk::MemoryTexture::new(
                        w as i32,
                        h as i32,
                        gdk::MemoryFormat::R8g8b8a8,
                        &bytes,
                        stride,
                    );
                    if let Some(display) = gdk::Display::default() {
                        display.clipboard().set_texture(&texture);
                    }
                }
                glib::Propagation::Proceed
            }
        })),
    );
    let ctrl_s = gtk4::Shortcut::new(
        Some(gtk4::ShortcutTrigger::parse_string("<Control>s").unwrap()),
        Some(gtk4::CallbackAction::new({
            let state = state.clone();
            move |_, _| {
                let s = state.borrow();
                if let Some(img) = render_qr_from_state(&s) {
                    let _ = img.save("qrcode.png");
                    s.update_status_typed(
                        &s.i18n
                            .borrow()
                            .t("status_saved_as")
                            .replace("{}", "qrcode.png"),
                        ToastType::Success,
                    );
                }
                glib::Propagation::Proceed
            }
        })),
    );
    let ctrl_shift_s = gtk4::Shortcut::new(
        Some(gtk4::ShortcutTrigger::parse_string("<Control><Shift>s").unwrap()),
        Some(gtk4::CallbackAction::new({
            let state = state.clone();
            move |_, _| {
                let s = state.borrow();
                if let Some(svg) = render_svg_from_state(&s) {
                    let _ = std::fs::write("qrcode.svg", svg);
                    s.update_status_typed(
                        &s.i18n
                            .borrow()
                            .t("status_saved_as")
                            .replace("{}", "qrcode.svg"),
                        ToastType::Success,
                    );
                }
                glib::Propagation::Proceed
            }
        })),
    );

    let shortcut_controller = gtk4::ShortcutController::new();
    shortcut_controller.add_shortcut(ctrl_z);
    shortcut_controller.add_shortcut(ctrl_y);
    shortcut_controller.add_shortcut(ctrl_c);
    shortcut_controller.add_shortcut(ctrl_s);
    shortcut_controller.add_shortcut(ctrl_shift_s);
    shortcut_controller.set_scope(gtk4::ShortcutScope::Global);
    window.add_controller(shortcut_controller);

    // ============================================================
    // RESTORE CONTENT from snapshot (language switch preserves values)
    // ============================================================
    CONTENT_SNAPSHOT.with(|snap| {
        if let Some(s) = snap.borrow_mut().take() {
            text_buffer.set_text(&s.text);
            content_type_dd.set_selected(s.content_type_idx);
            wifi_ssid_entry.set_text(&s.wifi_ssid);
            wifi_password_entry.set_text(&s.wifi_password);
            wifi_enc_dd.set_selected(s.wifi_enc_idx);
            vcard_name_entry.set_text(&s.vcard_name);
            vcard_phone_entry.set_text(&s.vcard_phone);
            vcard_email_entry.set_text(&s.vcard_email);
            vcard_org_entry.set_text(&s.vcard_org);
            vcard_url_entry.set_text(&s.vcard_url);
            vcard_country_dd.set_selected(s.vcard_country_idx);
            cal_title_entry.set_text(&s.cal_title);
            cal_location_entry.set_text(&s.cal_location);
            cal_start_calendar.select_day(&s.cal_start_date);
            cal_end_calendar.select_day(&s.cal_end_date);
            cal_start_hour.set_value(s.cal_start_hour);
            cal_start_minute.set_value(s.cal_start_minute);
            gps_lat_entry.set_text(&s.gps_lat);
            gps_lon_entry.set_text(&s.gps_lon);
            gps_search_entry.set_text(&s.gps_search);
            sms_phone_entry.set_text(&s.sms_phone);
            sms_message_entry.set_text(&s.sms_message);
            sms_country_dd.set_selected(s.sms_country_idx);
        }
    });

    // ============================================================
    // INITIAL PREVIEW
    // ============================================================
    update_preview(&state);

    // ============================================================
    // KEYBOARD SHORTCUTS
    // ============================================================
    {
        let sc = ShortcutController::new();
        sc.set_scope(gtk4::ShortcutScope::Global);

        // Ctrl+Z → Undo
        let state_undo = state.clone();
        let sync_undo = sync_widgets.clone();
        sc.add_shortcut(Shortcut::new(
            Some(ShortcutTrigger::parse_string("<Control>z").unwrap()),
            Some(CallbackAction::new(move |_, _| {
                if let Some(prev) = state_undo.borrow().undo_stack.borrow_mut().pop() {
                    let current = current_style_settings(&state_undo.borrow());
                    state_undo.borrow().redo_stack.borrow_mut().push(current);
                    apply_style_to_state(&state_undo.borrow(), &prev);
                    sync_undo();
                    update_preview(&state_undo);
                }
                glib::Propagation::Proceed
            })),
        ));

        // Ctrl+Shift+Z / Ctrl+Y → Redo
        let state_redo = state.clone();
        let sync_redo = sync_widgets.clone();
        sc.add_shortcut(Shortcut::new(
            Some(ShortcutTrigger::parse_string("<Control><Shift>z").unwrap()),
            Some(CallbackAction::new(move |_, _| {
                if let Some(next) = state_redo.borrow().redo_stack.borrow_mut().pop() {
                    let current = current_style_settings(&state_redo.borrow());
                    state_redo.borrow().undo_stack.borrow_mut().push(current);
                    apply_style_to_state(&state_redo.borrow(), &next);
                    sync_redo();
                    update_preview(&state_redo);
                }
                glib::Propagation::Proceed
            })),
        ));
        let state_redo_y = state.clone();
        let sync_redo_y = sync_widgets.clone();
        sc.add_shortcut(Shortcut::new(
            Some(ShortcutTrigger::parse_string("<Control>y").unwrap()),
            Some(CallbackAction::new(move |_, _| {
                if let Some(next) = state_redo_y.borrow().redo_stack.borrow_mut().pop() {
                    let current = current_style_settings(&state_redo_y.borrow());
                    state_redo_y.borrow().undo_stack.borrow_mut().push(current);
                    apply_style_to_state(&state_redo_y.borrow(), &next);
                    sync_redo_y();
                    update_preview(&state_redo_y);
                }
                glib::Propagation::Proceed
            })),
        ));

        // Ctrl+S → Save PNG
        let save_png = save_png_btn.clone();
        sc.add_shortcut(Shortcut::new(
            Some(ShortcutTrigger::parse_string("<Control>s").unwrap()),
            Some(CallbackAction::new(move |_, _| {
                save_png.emit_clicked();
                glib::Propagation::Proceed
            })),
        ));

        // Ctrl+Shift+S → Save SVG
        let save_svg = save_svg_btn.clone();
        sc.add_shortcut(Shortcut::new(
            Some(ShortcutTrigger::parse_string("<Control><Shift>s").unwrap()),
            Some(CallbackAction::new(move |_, _| {
                save_svg.emit_clicked();
                glib::Propagation::Proceed
            })),
        ));

        // Ctrl+C → Copy PNG to clipboard
        let copy_png = copy_btn.clone();
        sc.add_shortcut(Shortcut::new(
            Some(ShortcutTrigger::parse_string("<Control>c").unwrap()),
            Some(CallbackAction::new(move |_, _| {
                copy_png.emit_clicked();
                glib::Propagation::Proceed
            })),
        ));

        // Ctrl+Shift+C → Copy SVG to clipboard
        let copy_svg = copy_svg_btn.clone();
        sc.add_shortcut(Shortcut::new(
            Some(ShortcutTrigger::parse_string("<Control><Shift>c").unwrap()),
            Some(CallbackAction::new(move |_, _| {
                copy_svg.emit_clicked();
                glib::Propagation::Proceed
            })),
        ));

        window.add_controller(sc);
    }

    window.present();
}
