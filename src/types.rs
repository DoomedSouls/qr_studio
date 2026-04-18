use crate::i18n::I18n;
use adw::ToastOverlay;
use gtk4::{Button, Label, Picture, ProgressBar, TextBuffer};
use image::{Rgba, RgbaImage};
use std::cell::RefCell;
use std::path::PathBuf;

// ============================================================
// ENUMS
// ============================================================

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DotStyle {
    Rounded,
    Square,
    Dots,
    Diamond,
    Custom,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CornerSquareStyle {
    Square,
    ExtraRounded,
    Dot,
    Circle,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CornerDotStyle {
    Square,
    Dot,
    Circle,
    ExtraRounded,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ModuleType {
    Data,
    CornerSquare,
    CornerDot,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ErrorCorrectionLevel {
    Low,
    Medium,
    Quartile,
    High,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GradientDirection {
    Horizontal,
    Vertical,
    Diagonal,
    Radial,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ScanQuality {
    Good,    // 🟢 All checks pass — reliably scannable
    Limited, // 🟡 Decodable but with warnings (low contrast, large logo, etc.)
    Bad,     // 🔴 Cannot be decoded by rqrr
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ContentType {
    Text,
    Wifi,
    Vcard,
    Calendar,
    Gps,
    Sms,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WifiEncryption {
    Wpa,
    Wep,
    None,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LogoShape {
    Rectangle,
    RoundedRect,
    Circle,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FrameStyle {
    None,
    Simple,
    Rounded,
    Banner,
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
#[serde(default)]
pub struct StyleSettings {
    pub dot_style: String,
    pub corner_square_style: String,
    pub corner_dot_style: String,
    pub fg_color: [u8; 4],
    pub bg_color: [u8; 4],
    pub corner_color: [u8; 4],
    pub logo_shape: String,
    pub ec_level: String,
    pub module_size: u32,
    pub quiet_zone: u32,
    pub module_gap: f64,
    pub transparent_bg: bool,
    pub gradient_enabled: bool,
    pub gradient_color: [u8; 4],
    pub gradient_direction: String,
    pub frame_style: String,
    pub frame_color: [u8; 4],
    pub shadow_enabled: bool,
    pub shadow_offset: f64,
    pub frame_width: u32,
    pub logo_color: [u8; 4],
    pub logo_border_width: f64,
    pub logo_border_color: [u8; 4],
    pub logo_vectorize: bool,
    pub logo_vectorize_bg_color: [u8; 4],
    pub logo_clear_area: bool,
    pub logo_clear_padding: f64,
    pub logo_outer_radius: f64,
    pub logo_inner_radius: f64,
    pub frame_outer_radius: f64,
    pub frame_inner_radius: f64,
    pub outer_text_font: String,
    pub outer_text_font_size: u32,
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
#[serde(default)]
pub struct TemplateSettings {
    // Content
    pub content_type: String,
    pub text_content: String,
    pub wifi_ssid: String,
    pub wifi_password: String,
    pub wifi_encryption: String,
    pub vcard_name: String,
    pub vcard_phone: String,
    pub vcard_country_code: String,
    pub vcard_email: String,
    pub vcard_org: String,
    pub vcard_url: String,
    pub calendar_title: String,
    pub calendar_start: String,
    pub calendar_end: String,
    pub calendar_location: String,
    pub gps_lat: String,
    pub gps_lon: String,
    pub sms_phone: String,
    pub sms_country_code: String,
    pub sms_message: String,
    // Style (same as StyleSettings but embedded)
    pub style: StyleSettings,
}

// ============================================================
// APP STATE
// ============================================================

pub struct AppState {
    pub preview_picture: Picture,
    pub text_buffer: TextBuffer,
    pub dot_style: RefCell<DotStyle>,
    pub corner_square_style: RefCell<CornerSquareStyle>,
    pub corner_dot_style: RefCell<CornerDotStyle>,
    pub fg_color: RefCell<Rgba<u8>>,
    pub bg_color: RefCell<Rgba<u8>>,
    pub corner_color: RefCell<Rgba<u8>>,
    pub logo_path: RefCell<Option<PathBuf>>,
    pub logo_size: RefCell<f64>,
    pub outer_text_top: RefCell<String>,
    pub outer_text_bottom: RefCell<String>,
    pub outer_text_color: RefCell<Rgba<u8>>,
    pub logo_shape: RefCell<LogoShape>,
    pub logo_color: RefCell<Rgba<u8>>,
    pub logo_border_width: RefCell<f64>,
    pub logo_border_color: RefCell<Rgba<u8>>,
    pub logo_vectorize: RefCell<bool>,
    pub logo_vectorize_bg_color: RefCell<Rgba<u8>>,
    pub logo_clear_area: RefCell<bool>,
    pub logo_clear_padding: RefCell<f64>,
    pub logo_outer_radius: RefCell<f64>,
    pub logo_inner_radius: RefCell<f64>,
    pub quiet_zone: RefCell<u32>,
    pub module_gap: RefCell<f64>,
    pub frame_style: RefCell<FrameStyle>,
    pub frame_color: RefCell<Rgba<u8>>,
    pub frame_width: RefCell<u32>,
    pub frame_outer_radius: RefCell<f64>,
    pub frame_inner_radius: RefCell<f64>,
    pub toast_overlay: ToastOverlay,
    pub qr_info_label: Label,
    pub qr_capacity_bar: ProgressBar,
    pub ec_level: RefCell<ErrorCorrectionLevel>,
    pub transparent_bg: RefCell<bool>,
    pub module_size: RefCell<u32>,
    pub gradient_enabled: RefCell<bool>,
    pub gradient_color: RefCell<Rgba<u8>>,
    pub gradient_direction: RefCell<GradientDirection>,
    pub content_type: RefCell<ContentType>,
    pub wifi_ssid: RefCell<String>,
    pub wifi_password: RefCell<String>,
    pub wifi_encryption: RefCell<WifiEncryption>,
    pub shadow_enabled: RefCell<bool>,
    pub shadow_offset: RefCell<f64>,
    pub bg_image_path: RefCell<Option<PathBuf>>,
    pub vcard_name: RefCell<String>,
    pub vcard_phone: RefCell<String>,
    pub vcard_country_code: RefCell<String>,
    pub vcard_email: RefCell<String>,
    pub vcard_org: RefCell<String>,
    pub vcard_url: RefCell<String>,
    pub calendar_title: RefCell<String>,
    pub calendar_start: RefCell<String>,
    pub calendar_end: RefCell<String>,
    pub calendar_location: RefCell<String>,
    pub gps_lat: RefCell<String>,
    pub gps_lon: RefCell<String>,
    pub sms_phone: RefCell<String>,
    pub sms_country_code: RefCell<String>,
    pub sms_message: RefCell<String>,
    pub preview_generation: RefCell<u32>,
    pub cached_svg: RefCell<Option<String>>,
    pub cached_rgba: RefCell<Option<RgbaImage>>,
    pub cached_qr_data: RefCell<Option<String>>,
    pub scan_verify_btn: Button,
    pub undo_stack: RefCell<Vec<StyleSettings>>,
    pub redo_stack: RefCell<Vec<StyleSettings>>,
    pub is_restoring: RefCell<bool>,
    pub custom_dot_path: RefCell<String>,
    pub outer_text_font: RefCell<String>,
    pub outer_text_font_size: RefCell<u32>,
    pub contrast_warning_label: Label,
    #[allow(dead_code)]
    pub i18n: RefCell<I18n>,
}

// ============================================================
// TOAST TYPE (visual feedback)
// ============================================================

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
pub enum ToastType {
    Success,
    Error,
    Info,
}

impl AppState {
    /// Show a toast with a colored accent based on type.
    /// Success = green, Error = red, Info = blue.
    pub fn update_status_typed(&self, msg: &str, toast_type: ToastType) {
        let prefix = match toast_type {
            ToastType::Success => "✅  ",
            ToastType::Error => "❌  ",
            ToastType::Info => "ℹ️  ",
        };
        let title = format!("{}{}", prefix, msg);
        let toast = adw::Toast::builder().title(&title).timeout(3).build();
        self.toast_overlay.add_toast(toast);
    }

    /// Convenience: show an info toast (blue accent).
    pub fn update_status(&self, msg: &str) {
        self.update_status_typed(msg, ToastType::Info);
    }
}
