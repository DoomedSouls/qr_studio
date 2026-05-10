//! QR Studio — styled QR code generator
//!
//! On Windows GUI builds, hide the console window and show panics
//! in a message box so errors are visible even without a terminal.

// Hide console window on Windows GUI builds (prevents the brief black CMD flash)
#![cfg_attr(all(windows, feature = "gui"), windows_subsystem = "windows")]

mod cli;
#[cfg_attr(not(feature = "gui"), allow(dead_code))]
mod country_codes;
#[cfg_attr(not(feature = "gui"), allow(dead_code))]
mod helpers;
#[cfg_attr(not(feature = "gui"), allow(dead_code))]
mod i18n;
#[cfg(feature = "gui")]
mod map_styles;
#[cfg(feature = "gui")]
mod render;
#[cfg_attr(not(feature = "gui"), allow(dead_code))]
mod svg;
#[cfg_attr(not(feature = "gui"), allow(dead_code))]
mod tests;
#[cfg_attr(not(feature = "gui"), allow(dead_code))]
mod types;
#[cfg(feature = "gui")]
mod ui;

#[cfg(feature = "gui")]
use adw::prelude::*;

#[cfg(not(feature = "gui"))]
fn main() {
    // Headless CLI-only binary — no GTK4, no display server needed
    let cli = match clap::Parser::try_parse() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(e.exit_code());
        }
    };
    let exit_code = cli::run_cli(&cli);
    std::process::exit(exit_code);
}

// ── Windows GUI: error diagnostics ─────────────────────────────────
//
// With `windows_subsystem = "windows"` there is no console window.
// GTK/GLib errors go to C stderr which is invisible.
// If GTK fails to open a display it calls exit(1), bypassing
// Rust's panic hook entirely — so no log file is written either.
//
// Solution: redirect C-level stderr (fd 2) to qr_studio.log BEFORE
// GTK initialises. This captures every GTK/GLib warning and error,
// even when GTK calls exit().

#[cfg(all(windows, feature = "gui"))]
fn init_windows_stderr_log() {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let log_path = dir.join("qr_studio.log");
            if let Ok(file) = std::fs::File::create(&log_path) {
                use std::os::windows::io::AsRawHandle;
                let raw_handle = file.as_raw_handle();

                unsafe {
                    // MSVCRT: convert Win32 HANDLE → C file descriptor,
                    // then redirect fd 2 (C stderr) to the log file.
                    #[link(name = "msvcrt")]
                    unsafe extern "C" {
                        fn _open_osfhandle(osfhandle: isize, flags: i32) -> i32;
                        fn _dup2(fildes1: i32, fildes2: i32) -> i32;
                    }
                    const _O_WRONLY: i32 = 1;
                    const _O_BINARY: i32 = 0x8000;

                    let fd = _open_osfhandle(raw_handle as isize, _O_WRONLY | _O_BINARY);
                    if fd >= 0 {
                        _dup2(fd, 2); // redirect C stderr (fd 2) → log file
                    }
                }

                // Leak the File — it must outlive the process so the handle stays open
                std::mem::forget(file);
            }
        }
    }
}

#[cfg(not(all(windows, feature = "gui")))]
#[allow(dead_code)]
fn init_windows_stderr_log() {}

/// Show a Win32 error MessageBox (used by panic hook and app.run() failure).
#[cfg(all(windows, feature = "gui"))]
fn show_windows_error_message(msg: &str) {
    unsafe {
        #[link(name = "user32")]
        unsafe extern "system" {
            fn MessageBoxA(hwnd: usize, text: *const u8, caption: *const u8, mb: u32) -> i32;
        }
        let caption = b"QR Studio - Error\0";
        // MB_ICONERROR = 0x10, MB_OK = 0
        MessageBoxA(0, msg.as_ptr(), caption.as_ptr(), 0x10);
    }
}

#[cfg(all(windows, feature = "gui"))]
fn init_windows_panic_hook() {
    // Without a console window, panics are invisible on Windows.
    // Install a hook that shows a MessageBox and appends to qr_studio.log.
    std::panic::set_hook(Box::new(|info| {
        let msg = format!("QR Studio crashed:\n\n{}", info);
        eprintln!("{}", msg);

        // Also write directly to qr_studio.log (append) in case
        // the stderr redirect hasn't captured this yet
        if let Ok(exe) = std::env::current_exe() {
            if let Some(dir) = exe.parent() {
                let log_path = dir.join("qr_studio.log");
                let _ = std::fs::OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(&log_path)
                    .and_then(|mut f| std::io::Write::write_all(&mut f, msg.as_bytes()));
            }
        }

        show_windows_error_message(&msg);
    }));
}

#[cfg(not(all(windows, feature = "gui")))]
#[allow(dead_code)]
fn init_windows_panic_hook() {
    // No-op on non-Windows or non-GUI builds
}

#[cfg(feature = "gui")]
#[cfg_attr(feature = "hotpath", hotpath::main)]
fn main() {
    // ── Windows: early diagnostics (before any GTK init) ────────────
    // Redirect C stderr → qr_studio.log so GTK/GLib errors are captured
    // even when GTK calls exit(1) (bypasses Rust's panic hook).
    init_windows_stderr_log();
    // Install panic handler (on Windows GUI: shows MessageBox on crash)
    init_windows_panic_hook();

    // On Windows, force the Win32 GDK backend to avoid "No such backend:
    // wayland/x11" warnings in non-standard environments (Wine, RDP…)
    #[cfg(all(windows, feature = "gui"))]
    if std::env::var("GDK_BACKEND").is_err() {
        // SAFETY: setting GDK_BACKEND before GTK init is safe —
        // no other thread is accessing the environment yet.
        unsafe {
            std::env::set_var("GDK_BACKEND", "win32");
        }
    }

    // Check for CLI mode BEFORE GTK initialization
    // This allows headless QR generation without a display server
    let cli_args: Vec<String> = std::env::args().collect();
    if cli_args.iter().any(|a| a == "--cli") {
        let cli = match clap::Parser::try_parse_from(&cli_args) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(e.exit_code());
            }
        };
        let exit_code = cli::run_cli(&cli);
        std::process::exit(exit_code);
    }

    // Register compiled GResources (includes shortcuts overlay)
    gtk4::gio::resources_register_include!("io.github.SlobCoder.qr_studio.gresource")
        .expect("Failed to register GResource");

    let app = adw::Application::builder()
        .application_id("io.github.SlobCoder.qr_studio")
        .build();
    app.connect_activate(|app| {
        // Pre-fetch OpenFreeMap TileJSON in background so style switching is instant
        map_styles::prefetch_tilejson();

        let provider = gtk4::CssProvider::new();
        provider.load_from_data(
            /* ---- text-input-frame ---- */
            ".text-input-frame { border: 1px solid alpha(currentColor,0.3); border-radius: 6px; background: alpha(currentColor,0.05); }

            /* ---- progress bar: green ---- */
            progressbar.progress-ok trough { background: alpha(#2ec27e, 0.15); }
            progressbar.progress-ok progress { background: #2ec27e; transition: all 300ms ease-out; }

            /* ---- progress bar: yellow ---- */
            progressbar.progress-warn trough { background: alpha(#e5a50a, 0.15); }
            progressbar.progress-warn progress { background: #e5a50a; transition: all 300ms ease-out; }

            /* ---- progress bar: red + pulse ---- */
            progressbar.progress-critical trough { background: alpha(#e01b24, 0.15); }
            progressbar.progress-critical progress { background: #e01b24; animation: pulse-warning 1.5s ease-in-out infinite; }
            @keyframes pulse-warning {
                0%, 100% { opacity: 1.0; }
                50% { opacity: 0.55; }
            }

            /* ---- DnD: dim normal content when drop-zone overlay is visible ---- */
            paned.dnd-dim { opacity: 0.12; transition: opacity 150ms ease-out; }

            /* ---- DnD drop zone container (overlay) ---- */
            .drop-zone-container { background: alpha(@window_bg_color, 0.92); padding: 24px; }
            .drop-zone { border: 2px dashed alpha(@window_fg_color, 0.3); border-radius: 16px; margin: 12px; padding: 32px 24px; transition: all 180ms ease-out; }
            .drop-zone-hover { border-color: @accent_color; background: alpha(@accent_color, 0.1); border-style: solid; box-shadow: 0 0 20px alpha(@accent_color, 0.15); }
            .drop-zone-icon { color: alpha(@window_fg_color, 0.4); transition: color 180ms ease-out; -gtk-icon-size: 48px; }
            .drop-zone-hover .drop-zone-icon { color: @accent_color; }
            .drop-zone-label { color: alpha(@window_fg_color, 0.6); font-size: 1.15em; font-weight: 600; transition: color 180ms ease-out; }
            .drop-zone-hover .drop-zone-label { color: @accent_color; }

            /* ---- DnD drop zone highlight (legacy) ---- */
            .drop-active { border: 2px dashed @accent_color; background: alpha(@accent_color, 0.08); border-radius: 8px; transition: all 200ms ease-out; }

            /* ---- preview fade-in ---- */
            picture.preview-fade { transition: opacity 200ms ease-out, transform 200ms ease-out; }
            picture.preview-updating { opacity: 0.3; transition: opacity 150ms ease-in; }

            /* ---- skeleton pulse while rendering ---- */
            .preview-skeleton { background: alpha(currentColor, 0.06); border-radius: 12px; animation: skeleton-pulse 1.2s ease-in-out infinite; }
            @keyframes skeleton-pulse {
                0%, 100% { opacity: 0.4; }
                50% { opacity: 0.15; }
            }

            /* ---- toast color accents ---- */
            toast.toast-success { border-left: 3px solid #2ec27e; }
            toast.toast-error   { border-left: 3px solid #e01b24; }
            toast.toast-info    { border-left: 3px solid #3584e4; }

            /* ---- color button hover highlight ---- */
            .color-btn-hover { transition: all 150ms ease-out; }
            .color-btn-hover:hover { filter: brightness(1.15); outline: 2px solid alpha(@accent_color, 0.5); outline-offset: 2px; border-radius: 6px; }

            /* ---- input validation error ---- */
            .input-error { outline: 2px solid #e01b24; outline-offset: -1px; border-radius: 6px; }
            .input-error-hint { color: #e01b24; font-size: 0.85em; }

            /* ---- transparency checkerboard preview ---- */
            picture.preview-checkerboard {
                background-color: #ffffff;
                background-image:
                    linear-gradient(45deg, #cccccc 25%, transparent 25%, transparent 75%, #cccccc 75%),
                    linear-gradient(45deg, #cccccc 25%, transparent 25%, transparent 75%, #cccccc 75%);
                background-size: 20px 20px;
                background-position: 0 0, 10px 10px;
                border-radius: 12px;
            }

            /* ============================================================
               ANIMATION 1: Sidebar Slide (animated Paned position)
               — handled purely in Rust via glib::timeout_add_local
               ============================================================ */

            /* ============================================================
               ANIMATION 2: QR Code Appear (scale + fade after render)
               ============================================================ */
            @keyframes qr-appear {
                0% { opacity: 0.3; transform: scale(0.94); }
                100% { opacity: 1.0; transform: scale(1.0); }
            }
            picture.preview-appear { animation: qr-appear 300ms cubic-bezier(0.22, 1, 0.36, 1) forwards; }

            /* ============================================================
               ANIMATION 3: Error Shake on validation failure
               ============================================================ */
            @keyframes shake {
                0%, 100% { transform: translateX(0); }
                15% { transform: translateX(-6px); }
                30% { transform: translateX(5px); }
                45% { transform: translateX(-4px); }
                60% { transform: translateX(3px); }
                75% { transform: translateX(-2px); }
                90% { transform: translateX(1px); }
            }
            .input-error-shake { animation: shake 0.45s ease-out; }

            /* ============================================================
               ANIMATION 5: Section Expand/Collapse smooth height
               ============================================================ */
            expander > box > box { transition: margin 200ms ease-out, opacity 200ms ease-out; }

            /* ============================================================
               ANIMATION 6: Toast Slide-in enhanced (scale + slide)
               ============================================================ */
            @keyframes toast-slide-in {
                0% { transform: translateY(-16px) scale(0.96); opacity: 0; }
                100% { transform: translateY(0) scale(1.0); opacity: 1; }
            }
            toast { animation: toast-slide-in 250ms cubic-bezier(0.22, 1, 0.36, 1); }

            /* ============================================================
               ANIMATION 7: Color Button Pop on color-set
               ============================================================ */
            @keyframes color-pop {
                0% { transform: scale(1.0); }
                40% { transform: scale(1.18); }
                100% { transform: scale(1.0); }
            }
            .color-btn-pop { animation: color-pop 220ms cubic-bezier(0.22, 1, 0.36, 1); }

            /* ============================================================
               ANIMATION 8: Logo Drop Bounce on DnD
               ============================================================ */
            @keyframes bounce-in {
                0% { transform: scale(0.82); opacity: 0.4; }
                50% { transform: scale(1.06); opacity: 0.9; }
                70% { transform: scale(0.97); }
                100% { transform: scale(1.0); opacity: 1.0; }
            }
            picture.preview-bounce { animation: bounce-in 450ms cubic-bezier(0.22, 1, 0.36, 1) forwards; }

            /* ============================================================
               ANIMATION 9: QR Preview Morph (subtle transition between renders)
               ============================================================ */
            picture.preview-morph { transition: opacity 120ms ease-out, transform 120ms ease-out; }
            picture.preview-morphing { opacity: 0.55; transform: scale(0.985); }

            /* ============================================================
               ANIMATION 10: Export Popover Entrance
               ============================================================ */
            @keyframes popover-appear {
                0% { transform: scale(0.92); opacity: 0; }
                100% { transform: scale(1.0); opacity: 1; }
            }
            popover > contents { animation: popover-appear 180ms cubic-bezier(0.22, 1, 0.36, 1); }

            /* ---- SpinButton value flash (subtle highlight on change) ---- */
            @keyframes value-flash {
                0% { background: alpha(@accent_color, 0.15); }
                100% { background: transparent; }
            }
            spinbutton.value-flash { animation: value-flash 400ms ease-out; }

            /* ---- Calendar rounded corners ---- */
            calendar.calendar-rounded { border-radius: 8px; overflow: hidden; }

            /* ---- GPS map rounded corners ---- */
            .gps-map { border-radius: 8px; overflow: hidden; }

            /* ---- GPS suggestion list ---- */
            .gps-suggestions { background: @card_bg_color; border: 1px solid alpha(currentColor, 0.15); border-radius: 8px; }
            .gps-suggestions:empty { border: none; }
            .gps-suggestion-row { border-radius: 6px; margin: 1px 2px; transition: background 120ms ease-out; }
            .gps-suggestion-row:hover { background: alpha(@accent_color, 0.1); }
            .gps-suggestion-row:active { background: alpha(@accent_color, 0.18); }

            /* ---- Scan verification button ---- */
            .scan-verify-btn { border-radius: 8px; padding: 6px 18px; font-weight: 600; transition: all 200ms ease-out; color: white; }
            .scan-verify-btn.scan-good { background: #2ec27e; border: 1px solid #26a269; }
            .scan-verify-btn.scan-good:hover { background: #33d68a; }
            .scan-verify-btn.scan-limited { background: #e5a50a; border: 1px solid #c88800; }
            .scan-verify-btn.scan-limited:hover { background: #f0b820; }
            .scan-verify-btn.scan-bad { background: #e01b24; border: 1px solid #c01c28; }
            .scan-verify-btn.scan-bad:hover { background: #ed333b; }

            /* ============================================================
               ANIMATION 1: Detail page slide transition
               (handled by StackTransitionType::SlideLeftRight in Rust)
               ============================================================ */

            /* ============================================================
               ANIMATION 2: Save button confirmation flash
               ============================================================ */
            @keyframes save-confirm {
                0% { background: alpha(#2ec27e, 0.35); box-shadow: 0 0 8px alpha(#2ec27e, 0.3); }
                100% { background: transparent; box-shadow: none; }
            }
            .save-confirmed { animation: save-confirm 600ms ease-out; }

            /* ============================================================
               ANIMATION 3: Undo/Redo preview pulse
               ============================================================ */
            @keyframes undo-pulse {
                0% { box-shadow: 0 0 0 0 alpha(@accent_color, 0.4); }
                50% { box-shadow: 0 0 16px 4px alpha(@accent_color, 0.12); }
                100% { box-shadow: none; }
            }
            .undo-pulse { animation: undo-pulse 500ms ease-out; }

            /* ============================================================
               ANIMATION 4: Color harmony button pop on update
               (reuses existing @keyframes color-pop)
               ============================================================ */

            /* ============================================================
               ANIMATION 5: Schnellstil-Morph
               (reuses existing preview-morph / preview-morphing classes)
               ============================================================ */

            /* ============================================================
               ANIMATION 6: Sidebar row hover glow
               ============================================================ */
            .navigation-sidebar row {
                transition: background 150ms ease-out, box-shadow 150ms ease-out;
                border-radius: 8px;
                outline-radius: 8px;
                margin: 1px 4px;
                padding: 6px 10px;
            }
            .navigation-sidebar row:hover {
                background: alpha(@window_fg_color, 0.06);
            }
            .navigation-sidebar row:selected {
                background: alpha(@window_fg_color, 0.1);
                font-weight: 600;
            }

            /* ============================================================
               ANIMATION 7: Module-Gap live preview
               (already handled by schedule_preview on value_changed)
               ============================================================ */

            /* ============================================================
               ANIMATION 8: Logo placement bounce
               (reuses existing @keyframes bounce-in / preview-bounce)
               ============================================================ */

            /* ============================================================
               ANIMATION 9: QR code particle assembly effect
               ============================================================ */
            @keyframes qr-assemble {
                0% { filter: blur(5px) brightness(0.6); opacity: 0; transform: scale(0.97); }
                35% { filter: blur(2px); opacity: 0.5; }
                70% { filter: blur(0.5px); opacity: 0.85; }
                100% { filter: none; opacity: 1; transform: scale(1); }
            }
            .qr-assemble { animation: qr-assemble 550ms cubic-bezier(0.22, 1, 0.36, 1) forwards; }

            /* ============================================================
               ANIMATION 10: Contrast warning shake
               ============================================================ */
            .contrast-shake { animation: shake 0.45s ease-out; }

            /* ============================================================
               ANIMATION 11: Dark/Light mode smooth transition
               ============================================================ */
            window, headerbar, .navigation-sidebar, scrolledwindow, stack {
                transition: background-color 350ms ease-out, color 350ms ease-out;
            }
            ",
        );
        gtk4::style_context_add_provider_for_display(
            &gtk4::gdk::Display::default().unwrap(),
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        ui::build_ui(app);
    });
    #[allow(unused_variables)]
    let exit_status = app.run();

    // On Windows GUI, a non-zero exit code usually means GTK failed
    // to open a display (e.g. missing GPU drivers, RDP session, Wine).
    // Show a MessageBox so the user knows what happened.
    #[cfg(all(windows, feature = "gui"))]
    if exit_status != gtk4::glib::ExitCode::SUCCESS {
        let code = match exit_status {
            gtk4::glib::ExitCode::FAILURE => 1u8,
            _ => 2u8,
        };
        let log_hint = if let Ok(exe) = std::env::current_exe() {
            if let Some(dir) = exe.parent() {
                format!("\n\nDetails: {}", dir.join("qr_studio.log").display())
            } else {
                String::new()
            }
        } else {
            String::new()
        };
        show_windows_error_message(&format!(
            "QR Studio exited with error code {}.\n\
             This usually means GTK4 could not open a display.\n\
             Make sure you are running on a real Windows desktop\n\
             (Wine / headless RDP are not supported).{}",
            code, log_hint
        ));
    }
}
