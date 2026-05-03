use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq)]
pub enum Lang {
    De,
    En,
    Es,
    Fr,
    It,
    PtBr,
    Ja,
    Ko,
    ZhCn,
}

#[allow(dead_code)]
pub struct I18n {
    #[allow(dead_code)]
    lang: Lang,
    strings: HashMap<&'static str, &'static str>,
}

impl I18n {
    pub fn new(lang: Lang) -> Self {
        let strings = match lang {
            Lang::De => Self::german(),
            Lang::En => Self::english(),
            Lang::Es => Self::spanish(),
            Lang::Fr => Self::french(),
            Lang::It => Self::italian(),
            Lang::PtBr => Self::portuguese_br(),
            Lang::Ja => Self::japanese(),
            Lang::Ko => Self::korean(),
            Lang::ZhCn => Self::chinese_cn(),
        };
        Self { lang, strings }
    }

    pub fn t(&self, key: &str) -> String {
        self.strings.get(key).unwrap_or(&key).to_string()
    }

    #[allow(dead_code)]
    pub fn lang(&self) -> Lang {
        self.lang
    }

    fn german() -> HashMap<&'static str, &'static str> {
        let mut m = HashMap::new();
        // Tab labels
        m.insert("tab_content", "Inhalt");
        m.insert("tab_style", "Stil");
        m.insert("btn_export_more", "Exportieren ▾");
        // Section headers
        // Content types
        // Buttons
        m.insert("btn_copy", "Kopieren");
        m.insert("btn_save_png", "Als PNG speichern");
        m.insert("btn_save_svg", "Als SVG speichern");
        m.insert("btn_copy_svg", "SVG kopieren");
        m.insert("btn_save_gif", "Als GIF speichern");
        m.insert("btn_save_pdf", "Als PDF speichern");
        m.insert("btn_label_sheet", "Etiketten-Druckbogen...");
        m.insert("btn_batch", "Batch-Export...");
        m.insert("btn_select_image", "Bild auswählen");
        m.insert("btn_remove", "Entfernen");
        m.insert("btn_clear_all", "Alles löschen");
        m.insert("tooltip_clear_all", "Alle Eingabefelder zurücksetzen");
        // Labels
        // WiFi fields
        m.insert("wifi_password", "Passwort");
        // vCard fields
        m.insert("vcard_name", "Name");
        m.insert("vcard_phone", "Telefonnummer");
        m.insert("vcard_email", "E-Mail");
        m.insert("vcard_org", "Organisation");
        m.insert("vcard_url", "Website URL");
        // Calendar fields
        m.insert("cal_title", "Titel");
        m.insert("cal_location", "Ort");
        // GPS fields
        m.insert("gps_lat", "Breitengrad");
        m.insert("gps_lon", "Längengrad");
        m.insert("gps_search", "Ort suchen…");

        // SMS fields
        m.insert("sms_phone", "Telefonnummer");
        m.insert("sms_message", "Nachricht");
        // Frame styles
        // Gradient directions
        // Status messages
        m.insert(
            "status_template_saved_full",
            "Vorlage '{}' gespeichert (Stil + Inhalt)",
        );
        m.insert(
            "status_template_saved_style",
            "Vorlage '{}' gespeichert (nur Stil)",
        );
        m.insert(
            "status_template_loaded_full",
            "Vorlage '{}' geladen (Stil + Inhalt)",
        );
        m.insert(
            "status_template_loaded_style",
            "Vorlage '{}' geladen (nur Stil)",
        );
        // Dialog titles
        // Batch dialog
        // Label sheet dialog
        // EC levels
        // Dot styles
        // Corner square styles
        // Logo shapes
        // Harmonies
        m.insert("harmony_title", "Farbharmonien (als Hintergrund)");
        m.insert("color_fg", "Vordergrundfarbe");
        m.insert("color_bg", "Hintergrundfarbe");
        m.insert("color_corner", "Eckfarbe");
        m.insert("color_gradient", "Farbverlauf-Ziel");
        m.insert("dnd_logo_imported", "Logo per Drag & Drop importiert");
        // QR Info
        m.insert("qrinfo_version", "Version");
        m.insert("qrinfo_modules", "Module");
        m.insert("qrinfo_ec", "Fehlerkorrektur");
        m.insert("qrinfo_capacity", "Datenkapazität");
        m.insert("qrinfo_data_loss", "Datenverlust");
        m.insert("qrinfo_bytes", "Bytes");
        m.insert("qrinfo_scan_dist", "Scan-Distanz");
        m.insert("qrinfo_at_dpi", "bei 300 DPI");
        m.insert("qrinfo_no_data", "Keine Daten");
        m.insert("qrinfo_data_too_long", "Daten zu lang für QR-Code");
        // Expander sections (as used in build_ui)
        m.insert("exp_content", "Inhalt");
        m.insert("exp_presets", "Stil-Vorlagen");
        m.insert("exp_templates", "Vorlagen");
        m.insert("check_save_content", "inkl. Inhalt speichern");
        m.insert("tooltip_save_content", "Wenn aktiviert, wird der aktuelle Inhalt (Text, WiFi, vCard usw.) zusammen mit dem Stil gespeichert");
        m.insert("exp_pattern", "Muster & Ecken");
        m.insert("exp_colors", "Farben");
        m.insert("exp_settings", "Einstellungen");
        m.insert("exp_qr_info", "QR-Info");
        m.insert("exp_advanced", "Erweitert");
        m.insert("exp_logo", "Logo (Zentrum)");
        m.insert("exp_outer_text", "Text um den QR-Code");
        m.insert("exp_frame", "Rahmen");
        m.insert("exp_bg", "Hintergrund");
        m.insert("exp_import", "Import/Export");
        // Preview
        m.insert("preview_label", "Live-Vorschau");
        // Generate button
        // Dropdown items as used in build_ui
        m.insert("dd_content_text", "Text");
        m.insert("dd_content_wifi", "WiFi");
        m.insert("dd_content_vcard", "vCard/Kontakt");
        m.insert("dd_content_calendar", "Kalenderereignis");
        m.insert("dd_content_gps", "GPS-Standort");
        m.insert("dd_content_sms", "SMS");
        m.insert("dd_dot_rounded", "Abgerundet");
        m.insert("dd_dot_square", "Quadratisch");
        m.insert("dd_dot_dots", "Punkte");
        m.insert("dd_dot_diamond", "Raute");
        m.insert("dd_dot_custom", "Benutzerdefiniert");
        m.insert("dd_corner_sq_rounded", "Abgerundet");
        m.insert("dd_corner_sq_square", "Quadratisch");
        m.insert("dd_corner_sq_dot", "Punkt");
        m.insert("dd_corner_sq_circle", "Kreis");
        m.insert("dd_corner_dot_dot", "Punkt");
        m.insert("dd_corner_dot_square", "Quadratisch");
        m.insert("dd_corner_dot_circle", "Kreis");
        m.insert("dd_corner_dot_rounded", "Abgerundet");
        m.insert("dd_wifi_wpa", "WPA");
        m.insert("dd_wifi_wep", "WEP");
        m.insert("dd_wifi_none", "Keine");
        m.insert("dd_ec_medium", "Mittel (M)");
        m.insert("dd_ec_low", "Niedrig (L)");
        m.insert("dd_ec_quartile", "Quartil (Q)");
        m.insert("dd_ec_high", "Hoch (H)");
        m.insert("dd_module_medium", "Mittel (32px)");
        m.insert("dd_module_small", "Klein (16px)");
        m.insert("dd_module_large", "Groß (64px)");
        m.insert("dd_module_print", "Druck (128px)");
        m.insert("dd_grad_horizontal", "Horizontal");
        m.insert("dd_grad_vertical", "Vertikal");
        m.insert("dd_grad_diagonal", "Diagonal");
        m.insert("dd_grad_radial", "Radial");
        m.insert("dd_logo_circle", "Kreis");
        m.insert("dd_logo_rectangle", "Rechteck");
        m.insert("dd_logo_rounded", "Abgerundet");
        m.insert("dd_frame_none", "Keiner");
        m.insert("dd_frame_simple", "Einfach");
        m.insert("dd_frame_rounded", "Abgerundet");
        m.insert("dd_frame_banner", "Banner");
        m.insert("dd_palette_custom", "Benutzerdefiniert");
        m.insert("dd_palette_classic", "Klassisch");
        m.insert("dd_palette_ocean", "Ozean");
        m.insert("dd_palette_sunset", "Sonnenuntergang");
        m.insert("dd_palette_forest", "Wald");
        m.insert("dd_palette_lavender", "Lavendel");
        m.insert("dd_palette_fire", "Feuer");
        m.insert("dd_palette_aurora", "Nordlicht");
        m.insert("dd_palette_pastel", "Pastell");
        m.insert("dd_palette_neon", "Neon");
        m.insert("dd_preset_custom", "Benutzerdefiniert");
        m.insert("dd_preset_classic", "Klassisch");
        m.insert("dd_preset_rounded", "Abgerundet");
        m.insert("dd_preset_dots", "Punkte");
        m.insert("dd_preset_diamond", "Raute");
        m.insert("dd_preset_minimal", "Minimalistisch");
        m.insert("dd_preset_retro", "Retro");
        // Misc
        m.insert("btn_bg_select", "Hintergrundbild auswählen");
        m.insert("btn_export_style_short", "Stil exportieren");
        m.insert("btn_import_style_short", "Stil importieren");
        m.insert("btn_print_calc", "Druckgrößenrechner");
        m.insert("dlg_print_calc", "Druckgrößenrechner");
        m.insert("btn_apply", "Anwenden");
        m.insert("print_calc_result", "{} x {} Pixel (empfohlene Modulgröße: ~{}px)");
        m.insert("label_dpi", "DPI:");
        m.insert("check_transparent_bg", "Transparenter Hintergrund");
        m.insert("check_gradient", "Farbverlauf aktivieren");
        m.insert("check_shadow", "Schatten aktivieren");
        m.insert("check_logo_vectorize", "Logo vektorisieren");
        m.insert("check_logo_bg_transparent", "Transparenter Hintergrund");
        m.insert("check_logo_clear_area", "Bereich freihalten");
        m.insert("check_radius_sync", "Radius synchronisieren");
        m.insert("placeholder_template_name", "Vorlagenname...");
        m.insert("placeholder_top_text", "Text oben");
        m.insert("placeholder_bottom_text", "Text unten");
        m.insert("tooltip_undo", "Rückgängig (Ctrl+Z)");
        m.insert("tooltip_redo", "Wiederholen (Ctrl+Y)");
        // Feature 2: Transparency Checkerboard
        m.insert("tooltip_preview_bg", "Vorschau-Hintergrund wechseln");
        // Feature 3: i18n for hardcoded strings - tooltips
        m.insert("tooltip_content_type", "Inhaltstyp auswählen");
        m.insert("tooltip_qr_content", "QR-Code Inhalt");
        m.insert("tooltip_wifi_ssid", "WiFi Netzwerkname");
        m.insert("tooltip_wifi_password", "WiFi Passwort");
        m.insert("tooltip_wifi_encryption", "Verschlüsselung");
        m.insert("tooltip_vcard_name", "Kontaktname");
        m.insert("tooltip_vcard_phone", "Telefonnummer");
        m.insert("tooltip_vcard_email", "E-Mail Adresse");
        m.insert("tooltip_vcard_org", "Organisation/Firma");
        m.insert("tooltip_vcard_url", "Website URL");
        m.insert("tooltip_cal_title", "Termintitel");
        m.insert("tooltip_cal_hour", "Stunde");
        m.insert("tooltip_cal_minute", "Minute");
        m.insert("tooltip_cal_location", "Veranstaltungsort");
        m.insert("tooltip_gps_lat", "Breitengrad (Latitude)");
        m.insert("tooltip_gps_lon", "Längengrad (Longitude)");
        m.insert("tooltip_gps_search", "Ort eingeben und Enter drücken");

        m.insert("tooltip_sms_phone", "SMS Empfängernummer");
        m.insert("tooltip_sms_message", "SMS Nachrichtentext");
        m.insert("tooltip_preset_select", "Stil-Vorlage auswählen");
        m.insert(
            "tooltip_template_save",
            "Aktuellen Stil + Inhalt als Vorlage speichern",
        );
        m.insert(
            "tooltip_template_load",
            "Gespeicherte Vorlage laden (Stil + Inhalt)",
        );
        m.insert("tooltip_template_delete", "Ausgewählte Vorlage löschen");
        m.insert("tooltip_dot_style", "Datenpunkt-Stil");
        m.insert("tooltip_corner_sq_style", "Eckquadrat-Stil");
        m.insert("tooltip_corner_dot_style", "Eckpunkt-Stil");
        m.insert(
            "tooltip_custom_dot_svg",
            "SVG-Pfaddaten für benutzerdefinierte Punktform (Koordinaten 0..1)",
        );
        m.insert("tooltip_transparent_bg", "Hintergrund transparent machen");
        m.insert("tooltip_gradient_enable", "Farbverlauf aktivieren");
        m.insert("tooltip_gradient_dir", "Farbverlauf-Richtung");
        m.insert("tooltip_palette", "Farbpalette auswählen");
        m.insert("tooltip_ec_level", "Fehlerkorrektur-Level");
        m.insert("tooltip_module_size", "Modulgröße");
        m.insert("tooltip_quiet_zone", "Ruhezone um den QR-Code");
        m.insert("tooltip_module_gap", "Abstand zwischen Modulen");
        m.insert("tooltip_shadow_enable", "Schatteneffekt aktivieren");
        m.insert("tooltip_shadow_offset", "Schattenversatz");
        m.insert("tooltip_logo_select", "Logo-Bild auswählen");
        m.insert("tooltip_logo_remove", "Logo entfernen");
        m.insert("tooltip_logo_size", "Logo-Größe relativ zum QR-Code");
        m.insert("tooltip_logo_shape", "Logo-Form");
        m.insert(
            "tooltip_logo_radius_sync",
            "Inneren und äußeren Radius gekoppelt einstellen",
        );
        m.insert("tooltip_logo_color", "Logo-Farbton (Alpha = Stärke)");
        m.insert("tooltip_logo_border_width", "Rahmendicke um das Logo");
        m.insert("tooltip_logo_border_color", "Logo-Rahmenfarbe");
        m.insert(
            "tooltip_logo_vectorize",
            "Wandelt Raster-Logos (PNG/JPG) in Vektorpfade um",
        );
        m.insert("tooltip_logo_vectorize_bg", "HG-Farbe des vektorisierten Logos: Alpha=0 entfernt den Hintergrund, Alpha>0 ersetzt ihn durch diese Farbe");
        m.insert(
            "tooltip_logo_bg_transparent",
            "Logo-Hintergrund transparent machen, unabhängig von der QR-Hintergrundfarbe",
        );
        m.insert(
            "tooltip_logo_clear_area",
            "QR-Module um das Logo herum fließen lassen (erfordert Fehlerkorrektur)",
        );
        m.insert(
            "tooltip_logo_padding",
            "Zusätzlicher Abstand um das Logo (in Modulen) für besseren Reflow-Effekt",
        );
        m.insert(
            "tooltip_outer_radius",
            "Äußerer Rahmen-Radius (0 = eckig, 0.5 = maximal abgerundet)",
        );
        m.insert(
            "tooltip_inner_radius",
            "Innerer Rahmen-Radius (0 = eckig, 0.5 = maximal abgerundet)",
        );
        m.insert("tooltip_top_text", "Text über dem QR-Code");
        m.insert("tooltip_bottom_text", "Text unter dem QR-Code");
        m.insert("tooltip_text_color", "Textfarbe");
        m.insert("tooltip_frame_style", "Rahmenstil");
        m.insert("tooltip_frame_color", "Rahmenfarbe");
        m.insert("tooltip_frame_width", "Rahmendicke in Moduleinheiten");
        m.insert(
            "tooltip_frame_outer_radius",
            "Äußerer Rahmen-Radius (0 = eckig, 0.5 = maximal abgerundet)",
        );
        m.insert("tooltip_bg_select", "Hintergrundbild auswählen");
        m.insert("tooltip_bg_remove", "Hintergrundbild entfernen");
        m.insert(
            "tooltip_export_style",
            "Aktuelle Stileinstellungen als JSON exportieren",
        );
        m.insert(
            "tooltip_import_style",
            "Stileinstellungen aus JSON importieren",
        );
        m.insert("tooltip_print_calc", "Pixelgröße für Druck berechnen");
        m.insert("tooltip_copy_png", "QR-Code in Zwischenablage kopieren");
        m.insert("tooltip_save_png", "QR-Code als PNG speichern");
        m.insert(
            "tooltip_copy_svg",
            "QR-Code als SVG in Zwischenablage kopieren",
        );
        m.insert("tooltip_save_svg", "QR-Code als SVG speichern");
        m.insert("tooltip_save_gif", "Animierten QR-Code als GIF speichern");
        m.insert(
            "tooltip_save_pdf",
            "QR-Code als PDF exportieren (A4, druckfertig)",
        );
        m.insert(
            "tooltip_label_sheet",
            "Mehrere QR-Codes als Etiketten auf A4 anordnen",
        );
        m.insert("tooltip_batch", "Mehrere QR-Codes gleichzeitig exportieren");
        m.insert("tooltip_export_more", "Weitere Export-Optionen");
        m.insert("tooltip_sidebar_toggle", "Seitenleiste ein-/ausblenden");
        // Feature 3: labels
        m.insert("label_start_date", "Startdatum");
        m.insert("label_end_date", "Enddatum");
        m.insert("label_time", "Uhrzeit:");
        m.insert("label_quiet_zone", "Ruhezone (0-10)");
        m.insert("label_module_gap", "Modulabstand (0-0.4)");
        m.insert("label_shadow_offset", "Schattenversatz (1.0-5.0)");
        m.insert("label_logo_size", "Logo-Größe (0.1-0.6)");
        m.insert("label_outer_radius", "Äußerer Radius");
        m.insert("label_inner_radius", "Innerer Radius");
        m.insert("label_logo_border_width", "Logo-Rahmendicke (0-20)");
        m.insert("label_logo_padding", "Logo-Abstand:");
        m.insert("label_frame_width", "Rahmendicke (1-10)");
        m.insert("label_frame_outer_radius", "Äußerer Radius");
        m.insert("label_svg_path", "SVG-Pfad (d-Attribut):");
        m.insert("label_custom_dot_hint", "Tipp: Koordinaten im Bereich 0 bis 1. Beispiele:\n• Stern: M0.5,0 L0.62,0.38 L1,0.38 L0.69,0.62 L0.81,1 L0.5,0.76 L0.19,1 L0.31,0.62 L0,0.38 L0.38,0.38 Z\n• Herz: M0.5,0.9 L0.1,0.5 C0.1,0.1 0.5,0.1 0.5,0.4 C0.5,0.1 0.9,0.1 0.9,0.5 Z");
        m.insert("placeholder_custom_dot", "z.B. M0,0 L1,0 L1,1 L0,1 Z");
        m.insert("label_print_width", "Breite (cm):");
        m.insert("label_print_height", "Höhe (cm):");
        // Feature 6: Content validation
        m.insert("validation_invalid_email", "Ungültige E-Mail-Adresse");
        m.insert(
            "validation_invalid_lat",
            "Breitengrad muss zwischen -90 und 90 liegen",
        );
        m.insert(
            "validation_invalid_lon",
            "Längengrad muss zwischen -180 und 180 liegen",
        );
        m.insert("validation_invalid_phone", "Ungültige Telefonnummer");
        // Feature 9: Font selection
        m.insert("label_font", "Schriftart");
        m.insert("label_font_size", "Schriftgröße");
        // Scan verification
        m.insert("btn_verify_scan", "Wird geprüft…");
        m.insert("scan_status_good", "Scanbar — Alle Prüfungen bestanden");
        m.insert("scan_status_limited", "Eingeschränkt scanbar");
        m.insert(
            "scan_status_bad",
            "Nicht scanbar — Code konnte nicht gelesen werden",
        );
        m.insert(
            "scan_tooltip",
            "Überprüft Kontrast, Logo-Überdeckung und ob der Code dekodiert werden kann",
        );
        m.insert(
            "scan_detail_low_contrast",
            "Niedriger Kontrast ({:.1}:1, empfohlen ≥ 4.5:1)",
        );
        m.insert(
            "scan_detail_logo_ec",
            "Logo zu groß für Fehlerkorrektur-Level",
        );
        m.insert("scan_detail_large_gap", "Modulabstand sehr groß");
        m.insert(
            "scan_detail_styled_corners",
            "Stilisierte Ecken — Smartphone-Scanner erkennen dies zuverlässig",
        );

        // Dialog titles for file choosers
        m.insert("dlg_select_logo", "Logo auswählen");
        m.insert("dlg_select_bg", "Hintergrundbild auswählen");
        m.insert("dlg_save_label_sheet", "Etiketten-Druckbogen speichern");
        m.insert("dlg_select_csv", "CSV-Datei auswählen");
        m.insert("dlg_select_folder", "Ordner auswählen");

        // Dialog buttons
        m.insert("btn_open", "Öffnen");
        m.insert("btn_select", "Auswählen");

        // File filter names
        m.insert("filter_images", "Bilddateien");
        m.insert("filter_json", "JSON-Dateien");
        m.insert("filter_csv_txt", "CSV/TXT");

        // Status messages (file operations)
        m.insert("status_style_exported", "Stil exportiert");
        m.insert("status_pdf_saved", "PDF gespeichert");
        m.insert("status_pdf_error", "Fehler beim PDF-Export");
        m.insert(
            "status_label_sheet_saved",
            "Etiketten-Druckbogen gespeichert",
        );
        m.insert("status_label_sheet_error", "Fehler beim Etiketten-Export");
        m.insert("status_png_saved", "PNG gespeichert");
        m.insert("status_svg_saved", "SVG gespeichert");
        m.insert("status_gif_saved", "GIF gespeichert");
        m.insert(
            "status_gif_gradient_only",
            "GIF nur mit Farbverlauf verfügbar",
        );
        m.insert("status_batch_exported", "{} QR-Codes exportiert");
        m.insert("status_saved_as", "Als {} gespeichert");
        m.insert("status_enter_template_name", "Bitte Vorlagenname eingeben");
        m.insert("status_template_deleted_fmt", "Vorlage '{}' gelöscht");
        m.insert(
            "status_render_error",
            "Fehler: QR-Code konnte nicht gerendert werden",
        );
        m.insert("status_copied", "In Zwischenablage kopiert");
        m.insert("status_copied_svg", "SVG in Zwischenablage kopiert");

        // Batch/Label dialog labels
        m.insert("batch_data_label", "QR-Daten (eine pro Zeile):");
        m.insert(
            "batch_csv_hint",
            "(Erste Spalte wird als QR-Daten verwendet, Kopfzeile übersprungen)",
        );
        m.insert("batch_format", "Format:");
        m.insert("batch_csv_filter", "CSV/TXT");
        m.insert("batch_folder_label", "Ordner:");
        m.insert("batch_folder_selected", "Ordner: {}");

        // Label sheet dialog
        m.insert("lbl_columns", "Spalten");
        m.insert("lbl_rows", "Zeilen");
        m.insert("lbl_margin_mm", "Rand (mm)");
        m.insert("lbl_spacing_mm", "Abstand (mm)");
        m.insert(
            "lbl_sheet_info",
            "Mehrere QR-Codes auf einer A4-Seite zum Ausdrucken",
        );
        m.insert(
            "label_sheet_a4_info",
            "Die QR-Codes werden auf einer A4-Seite angeordnet.",
        );

        // Dialog buttons for batch/label
        m.insert("btn_cancel", "Abbrechen");
        m.insert("btn_export", "Exportieren");
        m.insert("btn_save", "Speichern");

        // Dialog titles (FileChooserDialog)
        m.insert("dlg_save_pdf", "Als PDF speichern");
        m.insert("dlg_save_png", "Als PNG speichern");
        m.insert("dlg_save_svg", "Als SVG speichern");
        m.insert("dlg_save_gif", "Als GIF speichern");
        m.insert("dlg_import_style", "Stil importieren");
        m.insert("dlg_export_style", "Stil exportieren");
        m.insert("dlg_batch_export", "Batch-Export");
        m.insert("dlg_label_sheet", "Etiketten-Druckbogen");

        m
    }

    fn english() -> HashMap<&'static str, &'static str> {
        let mut m = HashMap::new();
        // Tab labels
        m.insert("tab_content", "Content");
        m.insert("tab_style", "Style");
        m.insert("btn_export_more", "Export ▾");
        // Section headers
        // Content types
        // Buttons
        m.insert("btn_copy", "Copy");
        m.insert("btn_save_png", "Save as PNG");
        m.insert("btn_save_svg", "Save as SVG");
        m.insert("btn_copy_svg", "Copy SVG");
        m.insert("btn_save_gif", "Save as GIF");
        m.insert("btn_save_pdf", "Save as PDF");
        m.insert("btn_label_sheet", "Label Sheet...");
        m.insert("btn_batch", "Batch Export...");
        m.insert("btn_select_image", "Select Image");
        m.insert("btn_remove", "Remove");
        m.insert("btn_clear_all", "Clear All");
        m.insert("tooltip_clear_all", "Reset all input fields");
        // Labels
        // WiFi fields
        m.insert("wifi_password", "Password");
        // vCard fields
        m.insert("vcard_name", "Name");
        m.insert("vcard_phone", "Phone Number");
        m.insert("vcard_email", "Email");
        m.insert("vcard_org", "Organization");
        m.insert("vcard_url", "Website URL");
        // Calendar fields
        m.insert("cal_title", "Title");
        m.insert("cal_location", "Location");
        // GPS fields
        m.insert("gps_lat", "Latitude");
        m.insert("gps_lon", "Longitude");
        m.insert("gps_search", "Search location…");

        // SMS fields
        m.insert("sms_phone", "Phone Number");
        m.insert("sms_message", "Message");
        // Frame styles
        // Gradient directions
        // Status messages
        m.insert(
            "status_template_saved_full",
            "Template '{}' saved (style + content)",
        );
        m.insert(
            "status_template_saved_style",
            "Template '{}' saved (style only)",
        );
        m.insert(
            "status_template_loaded_full",
            "Template '{}' loaded (style + content)",
        );
        m.insert(
            "status_template_loaded_style",
            "Template '{}' loaded (style only)",
        );
        // Dialog titles
        // Batch dialog
        // Label sheet dialog
        // EC levels
        // Dot styles
        // Corner square styles
        // Logo shapes
        // Harmonies
        m.insert("harmony_title", "Color Harmonies (as background)");
        m.insert("color_fg", "Foreground");
        m.insert("color_bg", "Background");
        m.insert("color_corner", "Corner Color");
        m.insert("color_gradient", "Gradient Target");
        m.insert("dnd_logo_imported", "Logo imported via Drag & Drop");
        // QR Info
        m.insert("qrinfo_version", "Version");
        m.insert("qrinfo_modules", "modules");
        m.insert("qrinfo_ec", "Error Correction");
        m.insert("qrinfo_capacity", "Data Capacity");
        m.insert("qrinfo_data_loss", "data loss");
        m.insert("qrinfo_bytes", "Bytes");
        m.insert("qrinfo_scan_dist", "Scan Distance");
        m.insert("qrinfo_at_dpi", "at 300 DPI");
        m.insert("qrinfo_no_data", "No data");
        m.insert("qrinfo_data_too_long", "Data too long for QR code");
        // Expander sections (as used in build_ui)
        m.insert("exp_content", "Content");
        m.insert("exp_presets", "Style Presets");
        m.insert("exp_templates", "Templates");
        m.insert("check_save_content", "Include content");
        m.insert("tooltip_save_content", "When enabled, the current content (text, WiFi, vCard, etc.) is saved together with the style");
        m.insert("exp_pattern", "Pattern & Corners");
        m.insert("exp_colors", "Colors");
        m.insert("exp_settings", "Settings");
        m.insert("exp_qr_info", "QR Info");
        m.insert("exp_advanced", "Advanced");
        m.insert("exp_logo", "Logo (Center)");
        m.insert("exp_outer_text", "Text around QR Code");
        m.insert("exp_frame", "Frame");
        m.insert("exp_bg", "Background");
        m.insert("exp_import", "Import/Export");
        // Preview
        m.insert("preview_label", "Live Preview");
        // Generate button
        // Dropdown items as used in build_ui
        m.insert("dd_content_text", "Text");
        m.insert("dd_content_wifi", "WiFi");
        m.insert("dd_content_vcard", "vCard/Contact");
        m.insert("dd_content_calendar", "Calendar Event");
        m.insert("dd_content_gps", "GPS Location");
        m.insert("dd_content_sms", "SMS");
        m.insert("dd_dot_rounded", "Rounded");
        m.insert("dd_dot_square", "Square");
        m.insert("dd_dot_dots", "Dots");
        m.insert("dd_dot_diamond", "Diamond");
        m.insert("dd_dot_custom", "Custom");
        m.insert("dd_corner_sq_rounded", "Rounded");
        m.insert("dd_corner_sq_square", "Square");
        m.insert("dd_corner_sq_dot", "Dot");
        m.insert("dd_corner_sq_circle", "Circle");
        m.insert("dd_corner_dot_dot", "Dot");
        m.insert("dd_corner_dot_square", "Square");
        m.insert("dd_corner_dot_circle", "Circle");
        m.insert("dd_corner_dot_rounded", "Rounded");
        m.insert("dd_wifi_wpa", "WPA");
        m.insert("dd_wifi_wep", "WEP");
        m.insert("dd_wifi_none", "None");
        m.insert("dd_ec_medium", "Medium (M)");
        m.insert("dd_ec_low", "Low (L)");
        m.insert("dd_ec_quartile", "Quartile (Q)");
        m.insert("dd_ec_high", "High (H)");
        m.insert("dd_module_medium", "Medium (32px)");
        m.insert("dd_module_small", "Small (16px)");
        m.insert("dd_module_large", "Large (64px)");
        m.insert("dd_module_print", "Print (128px)");
        m.insert("dd_grad_horizontal", "Horizontal");
        m.insert("dd_grad_vertical", "Vertical");
        m.insert("dd_grad_diagonal", "Diagonal");
        m.insert("dd_grad_radial", "Radial");
        m.insert("dd_logo_circle", "Circle");
        m.insert("dd_logo_rectangle", "Rectangle");
        m.insert("dd_logo_rounded", "Rounded");
        m.insert("dd_frame_none", "None");
        m.insert("dd_frame_simple", "Simple");
        m.insert("dd_frame_rounded", "Rounded");
        m.insert("dd_frame_banner", "Banner");
        m.insert("dd_palette_custom", "Custom");
        m.insert("dd_palette_classic", "Classic");
        m.insert("dd_palette_ocean", "Ocean");
        m.insert("dd_palette_sunset", "Sunset");
        m.insert("dd_palette_forest", "Forest");
        m.insert("dd_palette_lavender", "Lavender");
        m.insert("dd_palette_fire", "Fire");
        m.insert("dd_palette_aurora", "Aurora");
        m.insert("dd_palette_pastel", "Pastel");
        m.insert("dd_palette_neon", "Neon");
        m.insert("dd_preset_custom", "Custom");
        m.insert("dd_preset_classic", "Classic");
        m.insert("dd_preset_rounded", "Rounded");
        m.insert("dd_preset_dots", "Dots");
        m.insert("dd_preset_diamond", "Diamond");
        m.insert("dd_preset_minimal", "Minimalist");
        m.insert("dd_preset_retro", "Retro");
        // Misc
        m.insert("btn_bg_select", "Select Background Image");
        m.insert("btn_export_style_short", "Export Style");
        m.insert("btn_import_style_short", "Import Style");
        m.insert("btn_print_calc", "Print Size Calculator");
        m.insert("dlg_print_calc", "Print Size Calculator");
        m.insert("btn_apply", "Apply");
        m.insert("print_calc_result", "{} x {} pixels (recommended module size: ~{}px)");
        m.insert("label_dpi", "DPI:");
        m.insert("check_transparent_bg", "Transparent Background");
        m.insert("check_gradient", "Enable Gradient");
        m.insert("check_shadow", "Enable Shadow");
        m.insert("check_logo_vectorize", "Vectorize Logo");
        m.insert("check_logo_bg_transparent", "Transparent Background");
        m.insert("check_logo_clear_area", "Clear Area");
        m.insert("check_radius_sync", "Synchronize Radii");
        m.insert("placeholder_template_name", "Template name...");
        m.insert("placeholder_top_text", "Top text");
        m.insert("placeholder_bottom_text", "Bottom text");
        m.insert("tooltip_undo", "Undo (Ctrl+Z)");
        m.insert("tooltip_redo", "Redo (Ctrl+Y)");
        // Feature 2: Transparency Checkerboard
        m.insert("tooltip_preview_bg", "Toggle preview background");
        // Feature 3: i18n for hardcoded strings - tooltips
        m.insert("tooltip_content_type", "Select content type");
        m.insert("tooltip_qr_content", "QR Code content");
        m.insert("tooltip_wifi_ssid", "WiFi network name");
        m.insert("tooltip_wifi_password", "WiFi password");
        m.insert("tooltip_wifi_encryption", "Encryption");
        m.insert("tooltip_vcard_name", "Contact name");
        m.insert("tooltip_vcard_phone", "Phone number");
        m.insert("tooltip_vcard_email", "Email address");
        m.insert("tooltip_vcard_org", "Organization/Company");
        m.insert("tooltip_vcard_url", "Website URL");
        m.insert("tooltip_cal_title", "Event title");
        m.insert("tooltip_cal_hour", "Hour");
        m.insert("tooltip_cal_minute", "Minute");
        m.insert("tooltip_cal_location", "Event location");
        m.insert("tooltip_gps_lat", "Latitude");
        m.insert("tooltip_gps_lon", "Longitude");
        m.insert("tooltip_gps_search", "Type a location and press Enter");

        m.insert("tooltip_sms_phone", "SMS recipient number");
        m.insert("tooltip_sms_message", "SMS message text");
        m.insert("tooltip_preset_select", "Select style preset");
        m.insert(
            "tooltip_template_save",
            "Save current style + content as template",
        );
        m.insert(
            "tooltip_template_load",
            "Load saved template (style + content)",
        );
        m.insert("tooltip_template_delete", "Delete selected template");
        m.insert("tooltip_dot_style", "Data dot style");
        m.insert("tooltip_corner_sq_style", "Corner square style");
        m.insert("tooltip_corner_dot_style", "Corner dot style");
        m.insert(
            "tooltip_custom_dot_svg",
            "SVG path data for custom dot shape (coordinates 0..1)",
        );
        m.insert("tooltip_transparent_bg", "Make background transparent");
        m.insert("tooltip_gradient_enable", "Enable gradient");
        m.insert("tooltip_gradient_dir", "Gradient direction");
        m.insert("tooltip_palette", "Select color palette");
        m.insert("tooltip_ec_level", "Error correction level");
        m.insert("tooltip_module_size", "Module size");
        m.insert("tooltip_quiet_zone", "Quiet zone around QR code");
        m.insert("tooltip_module_gap", "Gap between modules");
        m.insert("tooltip_shadow_enable", "Enable shadow effect");
        m.insert("tooltip_shadow_offset", "Shadow offset");
        m.insert("tooltip_logo_select", "Select logo image");
        m.insert("tooltip_logo_remove", "Remove logo");
        m.insert("tooltip_logo_size", "Logo size relative to QR code");
        m.insert("tooltip_logo_shape", "Logo shape");
        m.insert("tooltip_logo_radius_sync", "Link inner and outer radius");
        m.insert("tooltip_logo_color", "Logo tint (Alpha = strength)");
        m.insert("tooltip_logo_border_width", "Border width around logo");
        m.insert("tooltip_logo_border_color", "Logo border color");
        m.insert(
            "tooltip_logo_vectorize",
            "Convert raster logos (PNG/JPG) to vector paths",
        );
        m.insert("tooltip_logo_vectorize_bg", "Vectorized logo BG color: Alpha=0 removes background, Alpha>0 replaces with this color");
        m.insert(
            "tooltip_logo_bg_transparent",
            "Make logo area background transparent, independent of QR background color",
        );
        m.insert(
            "tooltip_logo_clear_area",
            "Reflow QR modules around logo (requires error correction)",
        );
        m.insert(
            "tooltip_logo_padding",
            "Extra padding around logo (in modules) for better reflow",
        );
        m.insert(
            "tooltip_outer_radius",
            "Outer frame radius (0 = square, 0.5 = maximally rounded)",
        );
        m.insert(
            "tooltip_inner_radius",
            "Inner frame radius (0 = square, 0.5 = maximally rounded)",
        );
        m.insert("tooltip_top_text", "Text above QR code");
        m.insert("tooltip_bottom_text", "Text below QR code");
        m.insert("tooltip_text_color", "Text color");
        m.insert("tooltip_frame_style", "Frame style");
        m.insert("tooltip_frame_color", "Frame color");
        m.insert("tooltip_frame_width", "Frame width in module units");
        m.insert(
            "tooltip_frame_outer_radius",
            "Outer frame radius (0 = square, 0.5 = maximally rounded)",
        );
        m.insert("tooltip_bg_select", "Select background image");
        m.insert("tooltip_bg_remove", "Remove background image");
        m.insert(
            "tooltip_export_style",
            "Export current style settings as JSON",
        );
        m.insert("tooltip_import_style", "Import style settings from JSON");
        m.insert("tooltip_print_calc", "Calculate pixel size for printing");
        m.insert("tooltip_copy_png", "Copy QR code to clipboard");
        m.insert("tooltip_save_png", "Save QR code as PNG");
        m.insert("tooltip_copy_svg", "Copy QR code as SVG to clipboard");
        m.insert("tooltip_save_svg", "Save QR code as SVG");
        m.insert("tooltip_save_gif", "Save animated QR code as GIF");
        m.insert(
            "tooltip_save_pdf",
            "Export QR code as PDF (A4, print-ready)",
        );
        m.insert(
            "tooltip_label_sheet",
            "Arrange multiple QR codes as labels on A4",
        );
        m.insert("tooltip_batch", "Export multiple QR codes at once");
        m.insert("tooltip_export_more", "More export options");
        m.insert("tooltip_sidebar_toggle", "Toggle sidebar");
        // Feature 3: labels
        m.insert("label_start_date", "Start date");
        m.insert("label_end_date", "End date");
        m.insert("label_time", "Time:");
        m.insert("label_quiet_zone", "Quiet zone (0-10)");
        m.insert("label_module_gap", "Module gap (0-0.4)");
        m.insert("label_shadow_offset", "Shadow offset (1.0-5.0)");
        m.insert("label_logo_size", "Logo size (0.1-0.6)");
        m.insert("label_outer_radius", "Outer Radius");
        m.insert("label_inner_radius", "Inner Radius");
        m.insert("label_logo_border_width", "Logo border width (0-20)");
        m.insert("label_logo_padding", "Logo padding:");
        m.insert("label_frame_width", "Frame width (1-10)");
        m.insert("label_frame_outer_radius", "Outer Radius");
        m.insert("label_svg_path", "SVG path (d-attribute):");
        m.insert("label_custom_dot_hint", "Tip: Coordinates in range 0 to 1. Examples:\n• Star: M0.5,0 L0.62,0.38 L1,0.38 L0.69,0.62 L0.81,1 L0.5,0.76 L0.19,1 L0.31,0.62 L0,0.38 L0.38,0.38 Z\n• Heart: M0.5,0.9 L0.1,0.5 C0.1,0.1 0.5,0.1 0.5,0.4 C0.5,0.1 0.9,0.1 0.9,0.5 Z");
        m.insert("placeholder_custom_dot", "e.g. M0,0 L1,0 L1,1 L0,1 Z");
        m.insert("label_print_width", "Width (cm):");
        m.insert("label_print_height", "Height (cm):");
        // Feature 6: Content validation
        m.insert("validation_invalid_email", "Invalid email address");
        m.insert(
            "validation_invalid_lat",
            "Latitude must be between -90 and 90",
        );
        m.insert(
            "validation_invalid_lon",
            "Longitude must be between -180 and 180",
        );
        m.insert("validation_invalid_phone", "Invalid phone number");
        // Feature 9: Font selection
        m.insert("label_font", "Font");
        m.insert("label_font_size", "Font size");
        // Scan verification
        m.insert("btn_verify_scan", "Checking…");
        m.insert("scan_status_good", "Scannable — All checks passed");
        m.insert("scan_status_limited", "Limited scannability");
        m.insert(
            "scan_status_bad",
            "Not scannable — Code could not be decoded",
        );
        m.insert(
            "scan_tooltip",
            "Verifies contrast, logo coverage and whether the code can be decoded",
        );
        m.insert(
            "scan_detail_low_contrast",
            "Low contrast ({:.1}:1, recommended ≥ 4.5:1)",
        );
        m.insert(
            "scan_detail_logo_ec",
            "Logo too large for error correction level",
        );
        m.insert("scan_detail_large_gap", "Module gap is very large");
        m.insert(
            "scan_detail_styled_corners",
            "Styled corners — smartphone scanners handle these reliably",
        );

        // Dialog titles for file choosers
        m.insert("dlg_select_logo", "Select Logo");
        m.insert("dlg_select_bg", "Select Background Image");
        m.insert("dlg_save_label_sheet", "Save Label Sheet");
        m.insert("dlg_select_csv", "Select CSV File");
        m.insert("dlg_select_folder", "Select Folder");

        // Dialog buttons
        m.insert("btn_open", "Open");
        m.insert("btn_select", "Select");

        // File filter names
        m.insert("filter_images", "Image Files");
        m.insert("filter_json", "JSON Files");
        m.insert("filter_csv_txt", "CSV/TXT");

        // Status messages (file operations)
        m.insert("status_style_exported", "Style exported");
        m.insert("status_pdf_saved", "PDF saved");
        m.insert("status_pdf_error", "PDF export error");
        m.insert("status_label_sheet_saved", "Label sheet saved");
        m.insert("status_label_sheet_error", "Label sheet export error");
        m.insert("status_png_saved", "PNG saved");
        m.insert("status_svg_saved", "SVG saved");
        m.insert("status_gif_saved", "GIF saved");
        m.insert(
            "status_gif_gradient_only",
            "GIF only available with gradient",
        );
        m.insert("status_batch_exported", "{} QR codes exported");
        m.insert("status_saved_as", "Saved as {}");
        m.insert("status_enter_template_name", "Please enter a template name");
        m.insert("status_template_deleted_fmt", "Template '{}' deleted");
        m.insert(
            "status_render_error",
            "Error: QR code could not be rendered",
        );
        m.insert("status_copied", "Copied to clipboard");
        m.insert("status_copied_svg", "SVG copied to clipboard");

        // Batch/Label dialog labels
        m.insert("batch_data_label", "QR Data (one per line):");
        m.insert(
            "batch_csv_hint",
            "(First column used as QR data, header row skipped)",
        );
        m.insert("batch_format", "Format:");
        m.insert("batch_csv_filter", "CSV/TXT");
        m.insert("batch_folder_label", "Folder:");
        m.insert("batch_folder_selected", "Folder: {}");

        // Label sheet dialog
        m.insert("lbl_columns", "Columns");
        m.insert("lbl_rows", "Rows");
        m.insert("lbl_margin_mm", "Margin (mm)");
        m.insert("lbl_spacing_mm", "Spacing (mm)");
        m.insert(
            "lbl_sheet_info",
            "Multiple QR codes on an A4 page for printing",
        );
        m.insert(
            "label_sheet_a4_info",
            "QR codes will be arranged on an A4 page.",
        );

        // Dialog buttons for batch/label
        m.insert("btn_cancel", "Cancel");
        m.insert("btn_export", "Export");
        m.insert("btn_save", "Save");

        // Dialog titles (FileChooserDialog)
        m.insert("dlg_save_pdf", "Save as PDF");
        m.insert("dlg_save_png", "Save as PNG");
        m.insert("dlg_save_svg", "Save as SVG");
        m.insert("dlg_save_gif", "Save as GIF");
        m.insert("dlg_import_style", "Import Style");
        m.insert("dlg_export_style", "Export Style");
        m.insert("dlg_batch_export", "Batch Export");
        m.insert("dlg_label_sheet", "Label Sheet");

        m
    }

    fn spanish() -> HashMap<&'static str, &'static str> {
        let mut m = HashMap::new();
        // Tab labels
        m.insert("tab_content", "Contenido");
        m.insert("tab_style", "Estilo");
        m.insert("btn_export_more", "Exportar ▾");
        // Section headers
        // Content types
        // Buttons
        m.insert("btn_copy", "Copiar");
        m.insert("btn_save_png", "Guardar como PNG");
        m.insert("btn_save_svg", "Guardar como SVG");
        m.insert("btn_copy_svg", "Copiar SVG");
        m.insert("btn_save_gif", "Guardar como GIF");
        m.insert("btn_save_pdf", "Guardar como PDF");
        m.insert("btn_label_sheet", "Hoja de etiquetas...");
        m.insert("btn_batch", "Exportación por lotes...");
        m.insert("btn_select_image", "Seleccionar imagen");
        m.insert("btn_remove", "Eliminar");
        m.insert("btn_clear_all", "Borrar todo");
        m.insert(
            "tooltip_clear_all",
            "Restablecer todos los campos de entrada",
        );
        // Labels
        // WiFi fields
        m.insert("wifi_password", "Contraseña");
        // vCard fields
        m.insert("vcard_name", "Nombre");
        m.insert("vcard_phone", "Número de teléfono");
        m.insert("vcard_email", "Correo electrónico");
        m.insert("vcard_org", "Organización");
        m.insert("vcard_url", "URL del sitio web");
        // Calendar fields
        m.insert("cal_title", "Título");
        m.insert("cal_location", "Ubicación");
        // GPS fields
        m.insert("gps_lat", "Latitud");
        m.insert("gps_lon", "Longitud");
        m.insert("gps_search", "Buscar ubicación…");

        // SMS fields
        m.insert("sms_phone", "Número de teléfono");
        m.insert("sms_message", "Mensaje");
        // Frame styles
        // Gradient directions
        // Status messages
        m.insert(
            "status_template_saved_full",
            "Plantilla '{}' guardada (estilo + contenido)",
        );
        m.insert(
            "status_template_saved_style",
            "Plantilla '{}' guardada (solo estilo)",
        );
        m.insert(
            "status_template_loaded_full",
            "Plantilla '{}' cargada (estilo + contenido)",
        );
        m.insert(
            "status_template_loaded_style",
            "Plantilla '{}' cargada (solo estilo)",
        );
        // Dialog titles
        // Batch dialog
        // Label sheet dialog
        // EC levels
        // Dot styles
        // Corner square styles
        // Logo shapes
        // Harmonies
        m.insert("harmony_title", "Armonías de color (como fondo)");
        m.insert("color_fg", "Primer plano");
        m.insert("color_bg", "Fondo");
        m.insert("color_corner", "Color de esquina");
        m.insert("color_gradient", "Destino de degradado");
        m.insert(
            "dnd_logo_imported",
            "Logotipo importado mediante arrastrar y soltar",
        );
        // QR Info
        m.insert("qrinfo_version", "Versión");
        m.insert("qrinfo_modules", "módulos");
        m.insert("qrinfo_ec", "Corrección de errores");
        m.insert("qrinfo_capacity", "Capacidad de datos");
        m.insert("qrinfo_data_loss", "pérdida de datos");
        m.insert("qrinfo_bytes", "Bytes");
        m.insert("qrinfo_scan_dist", "Distancia de escaneo");
        m.insert("qrinfo_at_dpi", "a 300 DPI");
        m.insert("qrinfo_no_data", "Sin datos");
        m.insert(
            "qrinfo_data_too_long",
            "Datos demasiado largos para el código QR",
        );
        // Expander sections (as used in build_ui)
        m.insert("exp_content", "Contenido");
        m.insert("exp_presets", "Preajustes de estilo");
        m.insert("exp_templates", "Plantillas");
        m.insert("check_save_content", "Incluir contenido");
        m.insert("tooltip_save_content", "Cuando está activado, el contenido actual (texto, WiFi, vCard, etc.) se guarda junto con el estilo");
        m.insert("exp_pattern", "Patrón y esquinas");
        m.insert("exp_colors", "Colores");
        m.insert("exp_settings", "Configuración");
        m.insert("exp_qr_info", "Información QR");
        m.insert("exp_advanced", "Avanzado");
        m.insert("exp_logo", "Logotipo (centro)");
        m.insert("exp_outer_text", "Texto alrededor del código QR");
        m.insert("exp_frame", "Marco");
        m.insert("exp_bg", "Fondo");
        m.insert("exp_import", "Importar/Exportar");
        // Preview
        m.insert("preview_label", "Vista previa en vivo");
        // Generate button
        // Dropdown items as used in build_ui
        m.insert("dd_content_text", "Texto");
        m.insert("dd_content_wifi", "WiFi");
        m.insert("dd_content_vcard", "vCard/Contacto");
        m.insert("dd_content_calendar", "Evento de calendario");
        m.insert("dd_content_gps", "Ubicación GPS");
        m.insert("dd_content_sms", "SMS");
        m.insert("dd_dot_rounded", "Redondeado");
        m.insert("dd_dot_square", "Cuadrado");
        m.insert("dd_dot_dots", "Puntos");
        m.insert("dd_dot_diamond", "Diamante");
        m.insert("dd_dot_custom", "Personalizado");
        m.insert("dd_corner_sq_rounded", "Redondeado");
        m.insert("dd_corner_sq_square", "Cuadrado");
        m.insert("dd_corner_sq_dot", "Punto");
        m.insert("dd_corner_sq_circle", "Círculo");
        m.insert("dd_corner_dot_dot", "Punto");
        m.insert("dd_corner_dot_square", "Cuadrado");
        m.insert("dd_corner_dot_circle", "Círculo");
        m.insert("dd_corner_dot_rounded", "Redondeado");
        m.insert("dd_wifi_wpa", "WPA");
        m.insert("dd_wifi_wep", "WEP");
        m.insert("dd_wifi_none", "Ninguno");
        m.insert("dd_ec_medium", "Medio (M)");
        m.insert("dd_ec_low", "Bajo (L)");
        m.insert("dd_ec_quartile", "Cuarto (Q)");
        m.insert("dd_ec_high", "Alto (H)");
        m.insert("dd_module_medium", "Mediano (32px)");
        m.insert("dd_module_small", "Pequeño (16px)");
        m.insert("dd_module_large", "Grande (64px)");
        m.insert("dd_module_print", "Impresión (128px)");
        m.insert("dd_grad_horizontal", "Horizontal");
        m.insert("dd_grad_vertical", "Vertical");
        m.insert("dd_grad_diagonal", "Diagonal");
        m.insert("dd_grad_radial", "Radial");
        m.insert("dd_logo_circle", "Círculo");
        m.insert("dd_logo_rectangle", "Rectángulo");
        m.insert("dd_logo_rounded", "Redondeado");
        m.insert("dd_frame_none", "Ninguno");
        m.insert("dd_frame_simple", "Simple");
        m.insert("dd_frame_rounded", "Redondeado");
        m.insert("dd_frame_banner", "Banner");
        m.insert("dd_palette_custom", "Personalizado");
        m.insert("dd_palette_classic", "Clásico");
        m.insert("dd_palette_ocean", "Océano");
        m.insert("dd_palette_sunset", "Atardecer");
        m.insert("dd_palette_forest", "Bosque");
        m.insert("dd_palette_lavender", "Lavanda");
        m.insert("dd_palette_fire", "Fuego");
        m.insert("dd_palette_aurora", "Aurora");
        m.insert("dd_palette_pastel", "Pastel");
        m.insert("dd_palette_neon", "Neón");
        m.insert("dd_preset_custom", "Personalizado");
        m.insert("dd_preset_classic", "Clásico");
        m.insert("dd_preset_rounded", "Redondeado");
        m.insert("dd_preset_dots", "Puntos");
        m.insert("dd_preset_diamond", "Diamante");
        m.insert("dd_preset_minimal", "Minimalista");
        m.insert("dd_preset_retro", "Retro");
        // Misc
        m.insert("btn_bg_select", "Seleccionar imagen de fondo");
        m.insert("btn_export_style_short", "Exportar estilo");
        m.insert("btn_import_style_short", "Importar estilo");
        m.insert("btn_print_calc", "Calculadora de tamaño de impresión");
        m.insert("dlg_print_calc", "Calculadora de tamaño de impresión");
        m.insert("btn_apply", "Aplicar");
        m.insert("print_calc_result", "{} x {} píxeles (tamaño de módulo recomendado: ~{}px)");
        m.insert("label_dpi", "DPI:");
        m.insert("check_transparent_bg", "Fondo transparente");
        m.insert("check_gradient", "Activar degradado");
        m.insert("check_shadow", "Activar sombra");
        m.insert("check_logo_vectorize", "Vectorizar logotipo");
        m.insert("check_logo_bg_transparent", "Fondo transparente");
        m.insert("check_logo_clear_area", "Limpiar área");
        m.insert("check_radius_sync", "Sincronizar radios");
        m.insert("placeholder_template_name", "Nombre de plantilla...");
        m.insert("placeholder_top_text", "Texto superior");
        m.insert("placeholder_bottom_text", "Texto inferior");
        m.insert("tooltip_undo", "Deshacer (Ctrl+Z)");
        m.insert("tooltip_redo", "Rehacer (Ctrl+Y)");
        // Feature 2: Transparency Checkerboard
        m.insert("tooltip_preview_bg", "Alternar fondo de vista previa");
        // Feature 3: i18n for hardcoded strings - tooltips
        m.insert("tooltip_content_type", "Seleccionar tipo de contenido");
        m.insert("tooltip_qr_content", "Contenido del código QR");
        m.insert("tooltip_wifi_ssid", "Nombre de la red WiFi");
        m.insert("tooltip_wifi_password", "Contraseña WiFi");
        m.insert("tooltip_wifi_encryption", "Cifrado");
        m.insert("tooltip_vcard_name", "Nombre del contacto");
        m.insert("tooltip_vcard_phone", "Número de teléfono");
        m.insert("tooltip_vcard_email", "Dirección de correo electrónico");
        m.insert("tooltip_vcard_org", "Organización/Empresa");
        m.insert("tooltip_vcard_url", "URL del sitio web");
        m.insert("tooltip_cal_title", "Título del evento");
        m.insert("tooltip_cal_hour", "Hora");
        m.insert("tooltip_cal_minute", "Minuto");
        m.insert("tooltip_cal_location", "Ubicación del evento");
        m.insert("tooltip_gps_lat", "Latitud");
        m.insert("tooltip_gps_lon", "Longitud");
        m.insert("tooltip_gps_search", "Escriba una ubicación y pulse Enter");

        m.insert("tooltip_sms_phone", "Número de destinatario SMS");
        m.insert("tooltip_sms_message", "Texto del mensaje SMS");
        m.insert("tooltip_preset_select", "Seleccionar preajuste de estilo");
        m.insert(
            "tooltip_template_save",
            "Guardar estilo actual + contenido como plantilla",
        );
        m.insert(
            "tooltip_template_load",
            "Cargar plantilla guardada (estilo + contenido)",
        );
        m.insert("tooltip_template_delete", "Eliminar plantilla seleccionada");
        m.insert("tooltip_dot_style", "Estilo de punto de datos");
        m.insert("tooltip_corner_sq_style", "Estilo de esquina cuadrada");
        m.insert("tooltip_corner_dot_style", "Estilo de punto de esquina");
        m.insert(
            "tooltip_custom_dot_svg",
            "Datos de ruta SVG para forma de punto personalizada (coordenadas 0..1)",
        );
        m.insert("tooltip_transparent_bg", "Hacer el fondo transparente");
        m.insert("tooltip_gradient_enable", "Activar degradado");
        m.insert("tooltip_gradient_dir", "Dirección del degradado");
        m.insert("tooltip_palette", "Seleccionar paleta de colores");
        m.insert("tooltip_ec_level", "Nivel de corrección de errores");
        m.insert("tooltip_module_size", "Tamaño de módulo");
        m.insert(
            "tooltip_quiet_zone",
            "Zona de silencio alrededor del código QR",
        );
        m.insert("tooltip_module_gap", "Separación entre módulos");
        m.insert("tooltip_shadow_enable", "Activar efecto de sombra");
        m.insert("tooltip_shadow_offset", "Desplazamiento de sombra");
        m.insert("tooltip_logo_select", "Seleccionar imagen de logotipo");
        m.insert("tooltip_logo_remove", "Eliminar logotipo");
        m.insert(
            "tooltip_logo_size",
            "Tamaño del logotipo relativo al código QR",
        );
        m.insert("tooltip_logo_shape", "Forma del logotipo");
        m.insert(
            "tooltip_logo_radius_sync",
            "Vincular radio interior y exterior",
        );
        m.insert(
            "tooltip_logo_color",
            "Tinte del logotipo (Alfa = intensidad)",
        );
        m.insert(
            "tooltip_logo_border_width",
            "Ancho del borde alrededor del logotipo",
        );
        m.insert("tooltip_logo_border_color", "Color del borde del logotipo");
        m.insert(
            "tooltip_logo_vectorize",
            "Convertir logotipos rasterizados (PNG/JPG) a trazados vectoriales",
        );
        m.insert("tooltip_logo_vectorize_bg", "Color de fondo del logotipo vectorizado: Alfa=0 elimina el fondo, Alfa>0 lo reemplaza con este color");
        m.insert("tooltip_logo_bg_transparent", "Hacer transparente el fondo del área del logo, independientemente del color de fondo del QR");
        m.insert(
            "tooltip_logo_clear_area",
            "Redistribuir módulos QR alrededor del logotipo (requiere corrección de errores)",
        );
        m.insert(
            "tooltip_logo_padding",
            "Relleno extra alrededor del logotipo (en módulos) para mejor redistribución",
        );
        m.insert(
            "tooltip_outer_radius",
            "Radio del marco exterior (0 = cuadrado, 0.5 = máximo redondeo)",
        );
        m.insert(
            "tooltip_inner_radius",
            "Radio del marco interior (0 = cuadrado, 0.5 = máximo redondeo)",
        );
        m.insert("tooltip_top_text", "Texto sobre el código QR");
        m.insert("tooltip_bottom_text", "Texto debajo del código QR");
        m.insert("tooltip_text_color", "Color del texto");
        m.insert("tooltip_frame_style", "Estilo de marco");
        m.insert("tooltip_frame_color", "Color del marco");
        m.insert(
            "tooltip_frame_width",
            "Ancho del marco en unidades de módulo",
        );
        m.insert(
            "tooltip_frame_outer_radius",
            "Radio del marco exterior (0 = cuadrado, 0.5 = máximo redondeo)",
        );
        m.insert("tooltip_bg_select", "Seleccionar imagen de fondo");
        m.insert("tooltip_bg_remove", "Eliminar imagen de fondo");
        m.insert(
            "tooltip_export_style",
            "Exportar ajustes de estilo actuales como JSON",
        );
        m.insert(
            "tooltip_import_style",
            "Importar ajustes de estilo desde JSON",
        );
        m.insert(
            "tooltip_print_calc",
            "Calcular tamaño en píxeles para impresión",
        );
        m.insert("tooltip_copy_png", "Copiar código QR al portapapeles");
        m.insert("tooltip_save_png", "Guardar código QR como PNG");
        m.insert(
            "tooltip_copy_svg",
            "Copiar código QR como SVG al portapapeles",
        );
        m.insert("tooltip_save_svg", "Guardar código QR como SVG");
        m.insert("tooltip_save_gif", "Guardar código QR animado como GIF");
        m.insert(
            "tooltip_save_pdf",
            "Exportar código QR como PDF (A4, listo para imprimir)",
        );
        m.insert(
            "tooltip_label_sheet",
            "Organizar múltiples códigos QR como etiquetas en A4",
        );
        m.insert("tooltip_batch", "Exportar múltiples códigos QR a la vez");
        m.insert("tooltip_export_more", "Más opciones de exportación");
        m.insert("tooltip_sidebar_toggle", "Alternar barra lateral");
        // Feature 3: labels
        m.insert("label_start_date", "Fecha de inicio");
        m.insert("label_end_date", "Fecha de fin");
        m.insert("label_time", "Hora:");
        m.insert("label_quiet_zone", "Zona de silencio (0-10)");
        m.insert("label_module_gap", "Separación de módulo (0-0.4)");
        m.insert("label_shadow_offset", "Desplazamiento de sombra (1.0-5.0)");
        m.insert("label_logo_size", "Tamaño del logotipo (0.1-0.6)");
        m.insert("label_outer_radius", "Radio exterior");
        m.insert("label_inner_radius", "Radio interior");
        m.insert(
            "label_logo_border_width",
            "Ancho del borde del logotipo (0-20)",
        );
        m.insert("label_logo_padding", "Relleno del logotipo:");
        m.insert("label_frame_width", "Ancho del marco (1-10)");
        m.insert("label_frame_outer_radius", "Radio exterior");
        m.insert("label_svg_path", "Ruta SVG (atributo d):");
        m.insert("label_custom_dot_hint", "Consejo: Coordenadas en el rango de 0 a 1. Ejemplos:
• Estrella: M0.5,0 L0.62,0.38 L1,0.38 L0.69,0.62 L0.81,1 L0.5,0.76 L0.19,1 L0.31,0.62 L0,0.38 L0.38,0.38 Z
• Corazón: M0.5,0.9 L0.1,0.5 C0.1,0.1 0.5,0.1 0.5,0.4 C0.5,0.1 0.9,0.1 0.9,0.5 Z");
        m.insert("placeholder_custom_dot", "ej. M0,0 L1,0 L1,1 L0,1 Z");
        m.insert("label_print_width", "Ancho (cm):");
        m.insert("label_print_height", "Alto (cm):");
        // Feature 6: Content validation
        m.insert(
            "validation_invalid_email",
            "Dirección de correo electrónico no válida",
        );
        m.insert(
            "validation_invalid_lat",
            "La latitud debe estar entre -90 y 90",
        );
        m.insert(
            "validation_invalid_lon",
            "La longitud debe estar entre -180 y 180",
        );
        m.insert("validation_invalid_phone", "Número de teléfono no válido");
        // Feature 9: Font selection
        m.insert("label_font", "Fuente");
        m.insert("label_font_size", "Tamaño de fuente");
        // Scan verification
        m.insert("btn_verify_scan", "Verificando…");
        m.insert(
            "scan_status_good",
            "Escaneable — Todas las comprobaciones superadas",
        );
        m.insert("scan_status_limited", "Escaneabilidad limitada");
        m.insert(
            "scan_status_bad",
            "No escaneable — No se pudo decodificar el código",
        );
        m.insert(
            "scan_tooltip",
            "Verifica el contraste, la cobertura del logotipo y si el código puede ser decodificado",
        );
        m.insert(
            "scan_detail_low_contrast",
            "Bajo contraste ({:.1}:1, recomendado ≥ 4.5:1)",
        );
        m.insert(
            "scan_detail_logo_ec",
            "Logotipo demasiado grande para el nivel de corrección de errores",
        );
        m.insert(
            "scan_detail_large_gap",
            "La separación de módulo es muy grande",
        );
        m.insert(
            "scan_detail_styled_corners",
            "Esquinas con estilo — los escáneres de smartphone las manejan de forma fiable",
        );

        // Dialog titles for file choosers
        m.insert("dlg_select_logo", "Seleccionar logo");
        m.insert("dlg_select_bg", "Seleccionar imagen de fondo");
        m.insert("dlg_save_label_sheet", "Guardar hoja de etiquetas");
        m.insert("dlg_select_csv", "Seleccionar archivo CSV");
        m.insert("dlg_select_folder", "Seleccionar carpeta");

        // Dialog buttons
        m.insert("btn_open", "Abrir");
        m.insert("btn_select", "Seleccionar");

        // File filter names
        m.insert("filter_images", "Archivos de imagen");
        m.insert("filter_json", "Archivos JSON");
        m.insert("filter_csv_txt", "CSV/TXT");

        // Status messages (file operations)
        m.insert("status_style_exported", "Estilo exportado");
        m.insert("status_pdf_saved", "PDF guardado");
        m.insert("status_pdf_error", "Error al exportar PDF");
        m.insert("status_label_sheet_saved", "Hoja de etiquetas guardada");
        m.insert(
            "status_label_sheet_error",
            "Error al exportar hoja de etiquetas",
        );
        m.insert("status_png_saved", "PNG guardado");
        m.insert("status_svg_saved", "SVG guardado");
        m.insert("status_gif_saved", "GIF guardado");
        m.insert(
            "status_gif_gradient_only",
            "GIF solo disponible con degradado",
        );
        m.insert("status_batch_exported", "{} códigos QR exportados");
        m.insert("status_saved_as", "Guardado como {}");
        m.insert(
            "status_enter_template_name",
            "Introduzca nombre de plantilla",
        );
        m.insert("status_template_deleted_fmt", "Plantilla '{}' eliminada");
        m.insert(
            "status_render_error",
            "Error: No se pudo renderizar el código QR",
        );
        m.insert("status_copied", "Copiado al portapapeles");
        m.insert("status_copied_svg", "SVG copiado al portapapeles");

        // Batch/Label dialog labels
        m.insert("batch_data_label", "Datos QR (uno por línea):");
        m.insert(
            "batch_csv_hint",
            "(Primera columna como datos QR, cabecera omitida)",
        );
        m.insert("batch_format", "Formato:");
        m.insert("batch_csv_filter", "CSV/TXT");
        m.insert("batch_folder_label", "Carpeta:");
        m.insert("batch_folder_selected", "Carpeta: {}");

        // Label sheet dialog
        m.insert("lbl_columns", "Columnas");
        m.insert("lbl_rows", "Filas");
        m.insert("lbl_margin_mm", "Margen (mm)");
        m.insert("lbl_spacing_mm", "Espaciado (mm)");
        m.insert(
            "lbl_sheet_info",
            "Varios códigos QR en una página A4 para imprimir",
        );
        m.insert(
            "label_sheet_a4_info",
            "Los códigos QR se organizarán en una página A4.",
        );

        // Dialog buttons for batch/label
        m.insert("btn_cancel", "Cancelar");
        m.insert("btn_export", "Exportar");
        m.insert("btn_save", "Guardar");

        // Dialog titles (FileChooserDialog)
        m.insert("dlg_save_pdf", "Guardar como PDF");
        m.insert("dlg_save_png", "Guardar como PNG");
        m.insert("dlg_save_svg", "Guardar como SVG");
        m.insert("dlg_save_gif", "Guardar como GIF");
        m.insert("dlg_import_style", "Importar estilo");
        m.insert("dlg_export_style", "Exportar estilo");
        m.insert("dlg_batch_export", "Exportación por lotes");
        m.insert("dlg_label_sheet", "Hoja de etiquetas");

        m
    }

    fn french() -> HashMap<&'static str, &'static str> {
        let mut m = HashMap::new();
        // Tab labels
        m.insert("tab_content", "Contenu");
        m.insert("tab_style", "Style");
        m.insert("btn_export_more", "Exporter ▾");
        // Section headers
        // Content types
        // Buttons
        m.insert("btn_copy", "Copier");
        m.insert("btn_save_png", "Enregistrer en PNG");
        m.insert("btn_save_svg", "Enregistrer en SVG");
        m.insert("btn_copy_svg", "Copier SVG");
        m.insert("btn_save_gif", "Enregistrer en GIF");
        m.insert("btn_save_pdf", "Enregistrer en PDF");
        m.insert("btn_label_sheet", "Feuille d'étiquettes…");
        m.insert("btn_batch", "Export par lot…");
        m.insert("btn_select_image", "Sélectionner une image");
        m.insert("btn_remove", "Supprimer");
        m.insert("btn_clear_all", "Tout effacer");
        m.insert(
            "tooltip_clear_all",
            "Réinitialiser tous les champs de saisie",
        );
        // Labels
        // WiFi fields
        m.insert("wifi_password", "Mot de passe");
        // vCard fields
        m.insert("vcard_name", "Nom");
        m.insert("vcard_phone", "Numéro de téléphone");
        m.insert("vcard_email", "E-mail");
        m.insert("vcard_org", "Organisation");
        m.insert("vcard_url", "URL du site web");
        // Calendar fields
        m.insert("cal_title", "Titre");
        m.insert("cal_location", "Lieu");
        // GPS fields
        m.insert("gps_lat", "Latitude");
        m.insert("gps_lon", "Longitude");
        m.insert("gps_search", "Rechercher un lieu…");

        // SMS fields
        m.insert("sms_phone", "Numéro de téléphone");
        m.insert("sms_message", "Message");
        // Frame styles
        // Gradient directions
        // Status messages
        m.insert(
            "status_template_saved_full",
            "Modèle '{}' enregistré (style + contenu)",
        );
        m.insert(
            "status_template_saved_style",
            "Modèle '{}' enregistré (style uniquement)",
        );
        m.insert(
            "status_template_loaded_full",
            "Modèle '{}' chargé (style + contenu)",
        );
        m.insert(
            "status_template_loaded_style",
            "Modèle '{}' chargé (style uniquement)",
        );
        // Dialog titles
        // Batch dialog
        // Label sheet dialog
        // EC levels
        // Dot styles
        // Corner square styles
        // Logo shapes
        // Harmonies
        m.insert("harmony_title", "Harmonies de couleurs (en arrière-plan)");
        m.insert("color_fg", "Premier plan");
        m.insert("color_bg", "Arrière-plan");
        m.insert("color_corner", "Couleur des angles");
        m.insert("color_gradient", "Cible du dégradé");
        m.insert("dnd_logo_imported", "Logo importé par glisser-déposer");
        // QR Info
        m.insert("qrinfo_version", "Version");
        m.insert("qrinfo_modules", "modules");
        m.insert("qrinfo_ec", "Correction d'erreur");
        m.insert("qrinfo_capacity", "Capacité des données");
        m.insert("qrinfo_data_loss", "perte de données");
        m.insert("qrinfo_bytes", "Octets");
        m.insert("qrinfo_scan_dist", "Distance de scan");
        m.insert("qrinfo_at_dpi", "à 300 PPP");
        m.insert("qrinfo_no_data", "Aucune donnée");
        m.insert(
            "qrinfo_data_too_long",
            "Données trop longues pour un code QR",
        );
        // Expander sections (as used in build_ui)
        m.insert("exp_content", "Contenu");
        m.insert("exp_presets", "Préréglages de style");
        m.insert("exp_templates", "Modèles");
        m.insert("check_save_content", "Inclure le contenu");
        m.insert(
            "tooltip_save_content",
            "Si activé, le contenu actuel (texte, WiFi, vCard, etc.) est enregistré avec le style",
        );
        m.insert("exp_pattern", "Motif et angles");
        m.insert("exp_colors", "Couleurs");
        m.insert("exp_settings", "Paramètres");
        m.insert("exp_qr_info", "Infos QR");
        m.insert("exp_advanced", "Avancé");
        m.insert("exp_logo", "Logo (Centre)");
        m.insert("exp_outer_text", "Texte autour du code QR");
        m.insert("exp_frame", "Cadre");
        m.insert("exp_bg", "Arrière-plan");
        m.insert("exp_import", "Importer/Exporter");
        // Preview
        m.insert("preview_label", "Aperçu en direct");
        // Generate button
        // Dropdown items as used in build_ui
        m.insert("dd_content_text", "Texte");
        m.insert("dd_content_wifi", "WiFi");
        m.insert("dd_content_vcard", "vCard/Contact");
        m.insert("dd_content_calendar", "Événement calendrier");
        m.insert("dd_content_gps", "Localisation GPS");
        m.insert("dd_content_sms", "SMS");
        m.insert("dd_dot_rounded", "Arrondi");
        m.insert("dd_dot_square", "Carré");
        m.insert("dd_dot_dots", "Points");
        m.insert("dd_dot_diamond", "Diamant");
        m.insert("dd_dot_custom", "Personnalisé");
        m.insert("dd_corner_sq_rounded", "Arrondi");
        m.insert("dd_corner_sq_square", "Carré");
        m.insert("dd_corner_sq_dot", "Point");
        m.insert("dd_corner_sq_circle", "Cercle");
        m.insert("dd_corner_dot_dot", "Point");
        m.insert("dd_corner_dot_square", "Carré");
        m.insert("dd_corner_dot_circle", "Cercle");
        m.insert("dd_corner_dot_rounded", "Arrondi");
        m.insert("dd_wifi_wpa", "WPA");
        m.insert("dd_wifi_wep", "WEP");
        m.insert("dd_wifi_none", "Aucun");
        m.insert("dd_ec_medium", "Moyen (M)");
        m.insert("dd_ec_low", "Bas (L)");
        m.insert("dd_ec_quartile", "Quartile (Q)");
        m.insert("dd_ec_high", "Élevé (H)");
        m.insert("dd_module_medium", "Moyen (32 px)");
        m.insert("dd_module_small", "Petit (16 px)");
        m.insert("dd_module_large", "Grand (64 px)");
        m.insert("dd_module_print", "Impression (128 px)");
        m.insert("dd_grad_horizontal", "Horizontal");
        m.insert("dd_grad_vertical", "Vertical");
        m.insert("dd_grad_diagonal", "Diagonal");
        m.insert("dd_grad_radial", "Radial");
        m.insert("dd_logo_circle", "Cercle");
        m.insert("dd_logo_rectangle", "Rectangle");
        m.insert("dd_logo_rounded", "Arrondi");
        m.insert("dd_frame_none", "Aucun");
        m.insert("dd_frame_simple", "Simple");
        m.insert("dd_frame_rounded", "Arrondi");
        m.insert("dd_frame_banner", "Bannière");
        m.insert("dd_palette_custom", "Personnalisée");
        m.insert("dd_palette_classic", "Classique");
        m.insert("dd_palette_ocean", "Océan");
        m.insert("dd_palette_sunset", "Coucher de soleil");
        m.insert("dd_palette_forest", "Forêt");
        m.insert("dd_palette_lavender", "Lavande");
        m.insert("dd_palette_fire", "Feu");
        m.insert("dd_palette_aurora", "Aurore");
        m.insert("dd_palette_pastel", "Pastel");
        m.insert("dd_palette_neon", "Néon");
        m.insert("dd_preset_custom", "Personnalisé");
        m.insert("dd_preset_classic", "Classique");
        m.insert("dd_preset_rounded", "Arrondi");
        m.insert("dd_preset_dots", "Points");
        m.insert("dd_preset_diamond", "Diamant");
        m.insert("dd_preset_minimal", "Minimaliste");
        m.insert("dd_preset_retro", "Rétro");
        // Misc
        m.insert("btn_bg_select", "Sélectionner une image d'arrière-plan");
        m.insert("btn_export_style_short", "Exporter le style");
        m.insert("btn_import_style_short", "Importer le style");
        m.insert("btn_print_calc", "Calculateur de taille d'impression");
        m.insert("dlg_print_calc", "Calculateur de taille d'impression");
        m.insert("btn_apply", "Appliquer");
        m.insert("print_calc_result", "{} x {} pixels (taille de module recommandée : ~{}px)");
        m.insert("label_dpi", "DPI :");
        m.insert("check_transparent_bg", "Arrière-plan transparent");
        m.insert("check_gradient", "Activer le dégradé");
        m.insert("check_shadow", "Activer l'ombre");
        m.insert("check_logo_vectorize", "Vectoriser le logo");
        m.insert("check_logo_bg_transparent", "Fond transparent");
        m.insert("check_logo_clear_area", "Effacer la zone");
        m.insert("check_radius_sync", "Synchroniser les rayons");
        m.insert("placeholder_template_name", "Nom du modèle…");
        m.insert("placeholder_top_text", "Texte supérieur");
        m.insert("placeholder_bottom_text", "Texte inférieur");
        m.insert("tooltip_undo", "Annuler (Ctrl+Z)");
        m.insert("tooltip_redo", "Rétablir (Ctrl+Y)");
        // Feature 2: Transparency Checkerboard
        m.insert("tooltip_preview_bg", "Basculer l'arrière-plan de l'aperçu");
        // Feature 3: i18n for hardcoded strings - tooltips
        m.insert("tooltip_content_type", "Sélectionner le type de contenu");
        m.insert("tooltip_qr_content", "Contenu du code QR");
        m.insert("tooltip_wifi_ssid", "Nom du réseau WiFi");
        m.insert("tooltip_wifi_password", "Mot de passe WiFi");
        m.insert("tooltip_wifi_encryption", "Chiffrement");
        m.insert("tooltip_vcard_name", "Nom du contact");
        m.insert("tooltip_vcard_phone", "Numéro de téléphone");
        m.insert("tooltip_vcard_email", "Adresse e-mail");
        m.insert("tooltip_vcard_org", "Organisation/Entreprise");
        m.insert("tooltip_vcard_url", "URL du site web");
        m.insert("tooltip_cal_title", "Titre de l'événement");
        m.insert("tooltip_cal_hour", "Heure");
        m.insert("tooltip_cal_minute", "Minute");
        m.insert("tooltip_cal_location", "Lieu de l'événement");
        m.insert("tooltip_gps_lat", "Latitude");
        m.insert("tooltip_gps_lon", "Longitude");
        m.insert(
            "tooltip_gps_search",
            "Saisissez un lieu et appuyez sur Entrée",
        );

        m.insert("tooltip_sms_phone", "Numéro du destinataire SMS");
        m.insert("tooltip_sms_message", "Texte du message SMS");
        m.insert(
            "tooltip_preset_select",
            "Sélectionner un préréglage de style",
        );
        m.insert(
            "tooltip_template_save",
            "Enregistrer le style et le contenu actuels comme modèle",
        );
        m.insert(
            "tooltip_template_load",
            "Charger le modèle enregistré (style et contenu)",
        );
        m.insert("tooltip_template_delete", "Supprimer le modèle sélectionné");
        m.insert("tooltip_dot_style", "Style des points de données");
        m.insert("tooltip_corner_sq_style", "Style des carrés d'angle");
        m.insert("tooltip_corner_dot_style", "Style des points d'angle");
        m.insert(
            "tooltip_custom_dot_svg",
            "Données de chemin SVG pour la forme de point personnalisée (coordonnées 0..1)",
        );
        m.insert(
            "tooltip_transparent_bg",
            "Rendre l'arrière-plan transparent",
        );
        m.insert("tooltip_gradient_enable", "Activer le dégradé");
        m.insert("tooltip_gradient_dir", "Direction du dégradé");
        m.insert("tooltip_palette", "Sélectionner la palette de couleurs");
        m.insert("tooltip_ec_level", "Niveau de correction d'erreur");
        m.insert("tooltip_module_size", "Taille du module");
        m.insert("tooltip_quiet_zone", "Zone de silence autour du code QR");
        m.insert("tooltip_module_gap", "Espace entre les modules");
        m.insert("tooltip_shadow_enable", "Activer l'effet d'ombre");
        m.insert("tooltip_shadow_offset", "Décalage de l'ombre");
        m.insert("tooltip_logo_select", "Sélectionner une image de logo");
        m.insert("tooltip_logo_remove", "Supprimer le logo");
        m.insert("tooltip_logo_size", "Taille du logo par rapport au code QR");
        m.insert("tooltip_logo_shape", "Forme du logo");
        m.insert(
            "tooltip_logo_radius_sync",
            "Lier le rayon intérieur et extérieur",
        );
        m.insert("tooltip_logo_color", "Teinte du logo (Alpha = intensité)");
        m.insert(
            "tooltip_logo_border_width",
            "Largeur de bordure autour du logo",
        );
        m.insert("tooltip_logo_border_color", "Couleur de bordure du logo");
        m.insert(
            "tooltip_logo_vectorize",
            "Convertir les logos matriciels (PNG/JPG) en chemins vectoriels",
        );
        m.insert("tooltip_logo_vectorize_bg", "Couleur d'arrière-plan du logo vectorisé : Alpha=0 supprime l'arrière-plan, Alpha>0 le remplace par cette couleur");
        m.insert("tooltip_logo_bg_transparent", "Rendre le fond de la zone logo transparent, indépendamment de la couleur de fond du QR");
        m.insert(
            "tooltip_logo_clear_area",
            "Réorganiser les modules QR autour du logo (nécessite une correction d'erreur)",
        );
        m.insert(
            "tooltip_logo_padding",
            "Marge supplémentaire autour du logo (en modules) pour un meilleur réagencement",
        );
        m.insert(
            "tooltip_outer_radius",
            "Rayon du cadre extérieur (0 = carré, 0,5 = maximum arrondi)",
        );
        m.insert(
            "tooltip_inner_radius",
            "Rayon du cadre intérieur (0 = carré, 0,5 = maximum arrondi)",
        );
        m.insert("tooltip_top_text", "Texte au-dessus du code QR");
        m.insert("tooltip_bottom_text", "Texte en dessous du code QR");
        m.insert("tooltip_text_color", "Couleur du texte");
        m.insert("tooltip_frame_style", "Style du cadre");
        m.insert("tooltip_frame_color", "Couleur du cadre");
        m.insert(
            "tooltip_frame_width",
            "Largeur du cadre en unités de module",
        );
        m.insert(
            "tooltip_frame_outer_radius",
            "Rayon du cadre extérieur (0 = carré, 0,5 = maximum arrondi)",
        );
        m.insert("tooltip_bg_select", "Sélectionner une image d'arrière-plan");
        m.insert("tooltip_bg_remove", "Supprimer l'image d'arrière-plan");
        m.insert(
            "tooltip_export_style",
            "Exporter les paramètres de style actuels en JSON",
        );
        m.insert(
            "tooltip_import_style",
            "Importer les paramètres de style depuis JSON",
        );
        m.insert(
            "tooltip_print_calc",
            "Calculer la taille en pixels pour l'impression",
        );
        m.insert(
            "tooltip_copy_png",
            "Copier le code QR dans le presse-papiers",
        );
        m.insert("tooltip_save_png", "Enregistrer le code QR en PNG");
        m.insert(
            "tooltip_copy_svg",
            "Copier le code QR en SVG dans le presse-papiers",
        );
        m.insert("tooltip_save_svg", "Enregistrer le code QR en SVG");
        m.insert("tooltip_save_gif", "Enregistrer le code QR animé en GIF");
        m.insert(
            "tooltip_save_pdf",
            "Exporter le code QR en PDF (A4, prêt à imprimer)",
        );
        m.insert(
            "tooltip_label_sheet",
            "Disposer plusieurs codes QR comme étiquettes sur A4",
        );
        m.insert("tooltip_batch", "Exporter plusieurs codes QR à la fois");
        m.insert("tooltip_export_more", "Plus d'options d'exportation");
        m.insert("tooltip_sidebar_toggle", "Basculer la barre latérale");
        // Feature 3: labels
        m.insert("label_start_date", "Date de début");
        m.insert("label_end_date", "Date de fin");
        m.insert("label_time", "Heure :");
        m.insert("label_quiet_zone", "Zone de silence (0-10)");
        m.insert("label_module_gap", "Espacement des modules (0-0,4)");
        m.insert("label_shadow_offset", "Décalage de l'ombre (1,0-5,0)");
        m.insert("label_logo_size", "Taille du logo (0,1-0,6)");
        m.insert("label_outer_radius", "Rayon extérieur");
        m.insert("label_inner_radius", "Rayon intérieur");
        m.insert(
            "label_logo_border_width",
            "Largeur de bordure du logo (0-20)",
        );
        m.insert("label_logo_padding", "Marge du logo :");
        m.insert("label_frame_width", "Largeur du cadre (1-10)");
        m.insert("label_frame_outer_radius", "Rayon extérieur");
        m.insert("label_svg_path", "Chemin SVG (attribut d) :");
        m.insert("label_custom_dot_hint", "Astuce : Coordonnées dans la plage 0 à 1. Exemples :\n• Étoile : M0.5,0 L0.62,0.38 L1,0.38 L0.69,0.62 L0.81,1 L0.5,0.76 L0.19,1 L0.31,0.62 L0,0.38 L0.38,0.38 Z\n• Cœur : M0.5,0.9 L0.1,0.5 C0.1,0.1 0.5,0.1 0.5,0.4 C0.5,0.1 0.9,0.1 0.9,0.5 Z");
        m.insert("placeholder_custom_dot", "ex. M0,0 L1,0 L1,1 L0,1 Z");
        m.insert("label_print_width", "Largeur (cm) :");
        m.insert("label_print_height", "Hauteur (cm) :");
        // Feature 6: Content validation
        m.insert("validation_invalid_email", "Adresse e-mail invalide");
        m.insert(
            "validation_invalid_lat",
            "La latitude doit être comprise entre -90 et 90",
        );
        m.insert(
            "validation_invalid_lon",
            "La longitude doit être comprise entre -180 et 180",
        );
        m.insert("validation_invalid_phone", "Numéro de téléphone invalide");
        // Feature 9: Font selection
        m.insert("label_font", "Police");
        m.insert("label_font_size", "Taille de police");
        // Scan verification
        m.insert("btn_verify_scan", "Vérification…");
        m.insert(
            "scan_status_good",
            "Lisible — Toutes les vérifications réussies",
        );
        m.insert("scan_status_limited", "Lisibilité limitée");
        m.insert(
            "scan_status_bad",
            "Non lisible — Le code n'a pas pu être décodé",
        );
        m.insert(
            "scan_tooltip",
            "Vérifie le contraste, la couverture du logo et si le code peut être décodé",
        );
        m.insert(
            "scan_detail_low_contrast",
            "Contraste faible ({:.1}:1, recommandé ≥ 4,5:1)",
        );
        m.insert(
            "scan_detail_logo_ec",
            "Logo trop grand pour le niveau de correction d'erreur",
        );
        m.insert(
            "scan_detail_large_gap",
            "L'espace entre les modules est très grand",
        );
        m.insert(
            "scan_detail_styled_corners",
            "Angles stylisés — les scanneurs de smartphones les lisent de manière fiable",
        );

        // Dialog titles for file choosers
        m.insert("dlg_select_logo", "Sélectionner un logo");
        m.insert("dlg_select_bg", "Sélectionner une image de fond");
        m.insert(
            "dlg_save_label_sheet",
            "Enregistrer la feuille d'étiquettes",
        );
        m.insert("dlg_select_csv", "Sélectionner un fichier CSV");
        m.insert("dlg_select_folder", "Sélectionner un dossier");

        // Dialog buttons
        m.insert("btn_open", "Ouvrir");
        m.insert("btn_select", "Sélectionner");

        // File filter names
        m.insert("filter_images", "Fichiers image");
        m.insert("filter_json", "Fichiers JSON");
        m.insert("filter_csv_txt", "CSV/TXT");

        // Status messages (file operations)
        m.insert("status_style_exported", "Style exporté");
        m.insert("status_pdf_saved", "PDF enregistré");
        m.insert("status_pdf_error", "Erreur lors de l'export PDF");
        m.insert(
            "status_label_sheet_saved",
            "Feuille d'étiquettes enregistrée",
        );
        m.insert(
            "status_label_sheet_error",
            "Erreur lors de l'export des étiquettes",
        );
        m.insert("status_png_saved", "PNG enregistré");
        m.insert("status_svg_saved", "SVG enregistré");
        m.insert("status_gif_saved", "GIF enregistré");
        m.insert(
            "status_gif_gradient_only",
            "GIF disponible uniquement avec dégradé",
        );
        m.insert("status_batch_exported", "{} codes QR exportés");
        m.insert("status_saved_as", "Enregistré sous {}");
        m.insert(
            "status_enter_template_name",
            "Veuillez entrer un nom de modèle",
        );
        m.insert("status_template_deleted_fmt", "Modèle '{}' supprimé");
        m.insert(
            "status_render_error",
            "Erreur : Impossible de rendre le code QR",
        );
        m.insert("status_copied", "Copié dans le presse-papiers");
        m.insert("status_copied_svg", "SVG copié dans le presse-papiers");

        // Batch/Label dialog labels
        m.insert("batch_data_label", "Données QR (une par ligne) :");
        m.insert(
            "batch_csv_hint",
            "(Première colonne utilisée, en-tête ignoré)",
        );
        m.insert("batch_format", "Format :");
        m.insert("batch_csv_filter", "CSV/TXT");
        m.insert("batch_folder_label", "Dossier :");
        m.insert("batch_folder_selected", "Dossier : {}");

        // Label sheet dialog
        m.insert("lbl_columns", "Colonnes");
        m.insert("lbl_rows", "Lignes");
        m.insert("lbl_margin_mm", "Marge (mm)");
        m.insert("lbl_spacing_mm", "Espacement (mm)");
        m.insert(
            "lbl_sheet_info",
            "Plusieurs codes QR sur une page A4 pour impression",
        );
        m.insert(
            "label_sheet_a4_info",
            "Les codes QR seront disposés sur une page A4.",
        );

        // Dialog buttons for batch/label
        m.insert("btn_cancel", "Annuler");
        m.insert("btn_export", "Exporter");
        m.insert("btn_save", "Enregistrer");

        // Dialog titles (FileChooserDialog)
        m.insert("dlg_save_pdf", "Enregistrer en PDF");
        m.insert("dlg_save_png", "Enregistrer en PNG");
        m.insert("dlg_save_svg", "Enregistrer en SVG");
        m.insert("dlg_save_gif", "Enregistrer en GIF");
        m.insert("dlg_import_style", "Importer un style");
        m.insert("dlg_export_style", "Exporter le style");
        m.insert("dlg_batch_export", "Export par lots");
        m.insert("dlg_label_sheet", "Feuille d'étiquettes");

        m
    }

    fn italian() -> HashMap<&'static str, &'static str> {
        let mut m = HashMap::new();
        // Tab labels
        m.insert("tab_content", "Contenuto");
        m.insert("tab_style", "Stile");
        m.insert("btn_export_more", "Esporta ▾");
        // Section headers
        // Content types
        // Buttons
        m.insert("btn_copy", "Copia");
        m.insert("btn_save_png", "Salva come PNG");
        m.insert("btn_save_svg", "Salva come SVG");
        m.insert("btn_copy_svg", "Copia SVG");
        m.insert("btn_save_gif", "Salva come GIF");
        m.insert("btn_save_pdf", "Salva come PDF");
        m.insert("btn_label_sheet", "Foglio etichette...");
        m.insert("btn_batch", "Esportazione batch...");
        m.insert("btn_select_image", "Seleziona immagine");
        m.insert("btn_remove", "Rimuovi");
        m.insert("btn_clear_all", "Cancella tutto");
        m.insert("tooltip_clear_all", "Cancella tutti i campi di input");
        // Labels
        // WiFi fields
        m.insert("wifi_password", "Password");
        // vCard fields
        m.insert("vcard_name", "Nome");
        m.insert("vcard_phone", "Numero di telefono");
        m.insert("vcard_email", "Email");
        m.insert("vcard_org", "Organizzazione");
        m.insert("vcard_url", "Sito web URL");
        // Calendar fields
        m.insert("cal_title", "Titolo");
        m.insert("cal_location", "Luogo");
        // GPS fields
        m.insert("gps_lat", "Latitudine");
        m.insert("gps_lon", "Longitudine");
        m.insert("gps_search", "Cerca luogo…");

        // SMS fields
        m.insert("sms_phone", "Numero di telefono");
        m.insert("sms_message", "Messaggio");
        // Frame styles
        // Gradient directions
        // Status messages
        m.insert(
            "status_template_saved_full",
            "Modello '{}' salvato (stile + contenuto)",
        );
        m.insert(
            "status_template_saved_style",
            "Modello '{}' salvato (solo stile)",
        );
        m.insert(
            "status_template_loaded_full",
            "Modello '{}' caricato (stile + contenuto)",
        );
        m.insert(
            "status_template_loaded_style",
            "Modello '{}' caricato (solo stile)",
        );
        // Dialog titles
        // Batch dialog
        // Label sheet dialog
        // EC levels
        // Dot styles
        // Corner square styles
        // Logo shapes
        // Harmonies
        m.insert("harmony_title", "Armonie cromatiche (come sfondo)");
        m.insert("color_fg", "Primo piano");
        m.insert("color_bg", "Sfondo");
        m.insert("color_corner", "Colore angoli");
        m.insert("color_gradient", "Destinazione gradiente");
        m.insert("dnd_logo_imported", "Logo importato tramite drag & drop");
        // QR Info
        m.insert("qrinfo_version", "Versione");
        m.insert("qrinfo_modules", "moduli");
        m.insert("qrinfo_ec", "Correzione errore");
        m.insert("qrinfo_capacity", "Capacità dati");
        m.insert("qrinfo_data_loss", "perdita dati");
        m.insert("qrinfo_bytes", "Byte");
        m.insert("qrinfo_scan_dist", "Distanza di scansione");
        m.insert("qrinfo_at_dpi", "a 300 DPI");
        m.insert("qrinfo_no_data", "Nessun dato");
        m.insert(
            "qrinfo_data_too_long",
            "Dati troppo lunghi per il codice QR",
        );
        // Expander sections (as used in build_ui)
        m.insert("exp_content", "Contenuto");
        m.insert("exp_presets", "Preimpostazioni stile");
        m.insert("exp_templates", "Modelli");
        m.insert("check_save_content", "Includi contenuto");
        m.insert("tooltip_save_content", "Se abilitato, il contenuto attuale (testo, WiFi, vCard, ecc.) viene salvato insieme allo stile");
        m.insert("exp_pattern", "Motivo e angoli");
        m.insert("exp_colors", "Colori");
        m.insert("exp_settings", "Impostazioni");
        m.insert("exp_qr_info", "Info QR");
        m.insert("exp_advanced", "Avanzate");
        m.insert("exp_logo", "Logo (centro)");
        m.insert("exp_outer_text", "Testo attorno al codice QR");
        m.insert("exp_frame", "Cornice");
        m.insert("exp_bg", "Sfondo");
        m.insert("exp_import", "Importa/Esporta");
        // Preview
        m.insert("preview_label", "Anteprima dal vivo");
        // Generate button
        // Dropdown items as used in build_ui
        m.insert("dd_content_text", "Testo");
        m.insert("dd_content_wifi", "WiFi");
        m.insert("dd_content_vcard", "vCard/Contatto");
        m.insert("dd_content_calendar", "Evento calendario");
        m.insert("dd_content_gps", "Posizione GPS");
        m.insert("dd_content_sms", "SMS");
        m.insert("dd_dot_rounded", "Arrotondato");
        m.insert("dd_dot_square", "Quadrato");
        m.insert("dd_dot_dots", "Punti");
        m.insert("dd_dot_diamond", "Diamante");
        m.insert("dd_dot_custom", "Personalizzato");
        m.insert("dd_corner_sq_rounded", "Arrotondato");
        m.insert("dd_corner_sq_square", "Quadrato");
        m.insert("dd_corner_sq_dot", "Punto");
        m.insert("dd_corner_sq_circle", "Cerchio");
        m.insert("dd_corner_dot_dot", "Punto");
        m.insert("dd_corner_dot_square", "Quadrato");
        m.insert("dd_corner_dot_circle", "Cerchio");
        m.insert("dd_corner_dot_rounded", "Arrotondato");
        m.insert("dd_wifi_wpa", "WPA");
        m.insert("dd_wifi_wep", "WEP");
        m.insert("dd_wifi_none", "Nessuna");
        m.insert("dd_ec_medium", "Medio (M)");
        m.insert("dd_ec_low", "Basso (L)");
        m.insert("dd_ec_quartile", "Quartile (Q)");
        m.insert("dd_ec_high", "Alto (H)");
        m.insert("dd_module_medium", "Medio (32px)");
        m.insert("dd_module_small", "Piccolo (16px)");
        m.insert("dd_module_large", "Grande (64px)");
        m.insert("dd_module_print", "Stampa (128px)");
        m.insert("dd_grad_horizontal", "Orizzontale");
        m.insert("dd_grad_vertical", "Verticale");
        m.insert("dd_grad_diagonal", "Diagonale");
        m.insert("dd_grad_radial", "Radiale");
        m.insert("dd_logo_circle", "Cerchio");
        m.insert("dd_logo_rectangle", "Rettangolo");
        m.insert("dd_logo_rounded", "Arrotondato");
        m.insert("dd_frame_none", "Nessuna");
        m.insert("dd_frame_simple", "Semplice");
        m.insert("dd_frame_rounded", "Arrotondata");
        m.insert("dd_frame_banner", "Banner");
        m.insert("dd_palette_custom", "Personalizzata");
        m.insert("dd_palette_classic", "Classica");
        m.insert("dd_palette_ocean", "Oceano");
        m.insert("dd_palette_sunset", "Tramonto");
        m.insert("dd_palette_forest", "Foresta");
        m.insert("dd_palette_lavender", "Lavanda");
        m.insert("dd_palette_fire", "Fuoco");
        m.insert("dd_palette_aurora", "Aurora");
        m.insert("dd_palette_pastel", "Pastello");
        m.insert("dd_palette_neon", "Neon");
        m.insert("dd_preset_custom", "Personalizzato");
        m.insert("dd_preset_classic", "Classico");
        m.insert("dd_preset_rounded", "Arrotondato");
        m.insert("dd_preset_dots", "Punti");
        m.insert("dd_preset_diamond", "Diamante");
        m.insert("dd_preset_minimal", "Minimalista");
        m.insert("dd_preset_retro", "Retro");
        // Misc
        m.insert("btn_bg_select", "Seleziona immagine di sfondo");
        m.insert("btn_export_style_short", "Esporta stile");
        m.insert("btn_import_style_short", "Importa stile");
        m.insert("btn_print_calc", "Calcolatore dimensione stampa");
        m.insert("dlg_print_calc", "Calcolatore dimensione stampa");
        m.insert("btn_apply", "Applica");
        m.insert("print_calc_result", "{} x {} pixel (dimensione modulo consigliata: ~{}px)");
        m.insert("label_dpi", "DPI:");
        m.insert("check_transparent_bg", "Sfondo trasparente");
        m.insert("check_gradient", "Abilita gradiente");
        m.insert("check_shadow", "Abilita ombra");
        m.insert("check_logo_vectorize", "Vettorizza logo");
        m.insert("check_logo_bg_transparent", "Sfondo trasparente");
        m.insert("check_logo_clear_area", "Cancella area");
        m.insert("check_radius_sync", "Sincronizza raggi");
        m.insert("placeholder_template_name", "Nome modello...");
        m.insert("placeholder_top_text", "Testo superiore");
        m.insert("placeholder_bottom_text", "Testo inferiore");
        m.insert("tooltip_undo", "Annulla (Ctrl+Z)");
        m.insert("tooltip_redo", "Ripristina (Ctrl+Y)");
        // Feature 2: Transparency Checkerboard
        m.insert("tooltip_preview_bg", "Attiva/disattiva sfondo anteprima");
        // Feature 3: i18n for hardcoded strings - tooltips
        m.insert("tooltip_content_type", "Seleziona tipo di contenuto");
        m.insert("tooltip_qr_content", "Contenuto codice QR");
        m.insert("tooltip_wifi_ssid", "Nome rete WiFi");
        m.insert("tooltip_wifi_password", "Password WiFi");
        m.insert("tooltip_wifi_encryption", "Crittografia");
        m.insert("tooltip_vcard_name", "Nome contatto");
        m.insert("tooltip_vcard_phone", "Numero di telefono");
        m.insert("tooltip_vcard_email", "Indirizzo email");
        m.insert("tooltip_vcard_org", "Organizzazione/Azienda");
        m.insert("tooltip_vcard_url", "Sito web URL");
        m.insert("tooltip_cal_title", "Titolo evento");
        m.insert("tooltip_cal_hour", "Ora");
        m.insert("tooltip_cal_minute", "Minuto");
        m.insert("tooltip_cal_location", "Luogo evento");
        m.insert("tooltip_gps_lat", "Latitudine");
        m.insert("tooltip_gps_lon", "Longitudine");
        m.insert("tooltip_gps_search", "Digita un luogo e premi Invio");

        m.insert("tooltip_sms_phone", "Numero destinatario SMS");
        m.insert("tooltip_sms_message", "Testo messaggio SMS");
        m.insert("tooltip_preset_select", "Seleziona preimpostazione stile");
        m.insert(
            "tooltip_template_save",
            "Salva stile e contenuto correnti come modello",
        );
        m.insert(
            "tooltip_template_load",
            "Carica modello salvato (stile + contenuto)",
        );
        m.insert("tooltip_template_delete", "Elimina modello selezionato");
        m.insert("tooltip_dot_style", "Stile punto dati");
        m.insert("tooltip_corner_sq_style", "Stile angolo quadrato");
        m.insert("tooltip_corner_dot_style", "Stile punto angolo");
        m.insert(
            "tooltip_custom_dot_svg",
            "Dati percorso SVG per forma punto personalizzata (coordinate 0..1)",
        );
        m.insert("tooltip_transparent_bg", "Rendi trasparente lo sfondo");
        m.insert("tooltip_gradient_enable", "Abilita gradiente");
        m.insert("tooltip_gradient_dir", "Direzione gradiente");
        m.insert("tooltip_palette", "Seleziona tavolozza colori");
        m.insert("tooltip_ec_level", "Livello correzione errore");
        m.insert("tooltip_module_size", "Dimensione modulo");
        m.insert("tooltip_quiet_zone", "Zona di quiete attorno al codice QR");
        m.insert("tooltip_module_gap", "Spaziatura tra moduli");
        m.insert("tooltip_shadow_enable", "Abilita effetto ombra");
        m.insert("tooltip_shadow_offset", "Scostamento ombra");
        m.insert("tooltip_logo_select", "Seleziona immagine logo");
        m.insert("tooltip_logo_remove", "Rimuovi logo");
        m.insert("tooltip_logo_size", "Dimensione logo rispetto al codice QR");
        m.insert("tooltip_logo_shape", "Forma logo");
        m.insert(
            "tooltip_logo_radius_sync",
            "Collega raggio interno ed esterno",
        );
        m.insert("tooltip_logo_color", "Tinta logo (Alfa = intensità)");
        m.insert(
            "tooltip_logo_border_width",
            "Spessore bordo attorno al logo",
        );
        m.insert("tooltip_logo_border_color", "Colore bordo logo");
        m.insert(
            "tooltip_logo_vectorize",
            "Converti logo raster (PNG/JPG) in percorsi vettoriali",
        );
        m.insert("tooltip_logo_vectorize_bg", "Colore sfondo logo vettorizzato: Alfa=0 rimuove lo sfondo, Alfa>0 lo sostituisce con questo colore");
        m.insert("tooltip_logo_bg_transparent", "Rendi trasparente lo sfondo dell'area logo, indipendentemente dal colore di sfondo del QR");
        m.insert(
            "tooltip_logo_clear_area",
            "Riorganizza i moduli QR attorno al logo (richiede correzione errore)",
        );
        m.insert(
            "tooltip_logo_padding",
            "Padding extra attorno al logo (in moduli) per migliore riorganizzazione",
        );
        m.insert(
            "tooltip_outer_radius",
            "Raggio cornice esterna (0 = quadrato, 0.5 = massimamente arrotondato)",
        );
        m.insert(
            "tooltip_inner_radius",
            "Raggio cornice interna (0 = quadrato, 0.5 = massimamente arrotondato)",
        );
        m.insert("tooltip_top_text", "Testo sopra il codice QR");
        m.insert("tooltip_bottom_text", "Testo sotto il codice QR");
        m.insert("tooltip_text_color", "Colore testo");
        m.insert("tooltip_frame_style", "Stile cornice");
        m.insert("tooltip_frame_color", "Colore cornice");
        m.insert("tooltip_frame_width", "Larghezza cornice in unità modulo");
        m.insert(
            "tooltip_frame_outer_radius",
            "Raggio cornice esterna (0 = quadrato, 0.5 = massimamente arrotondato)",
        );
        m.insert("tooltip_bg_select", "Seleziona immagine di sfondo");
        m.insert("tooltip_bg_remove", "Rimuovi immagine di sfondo");
        m.insert(
            "tooltip_export_style",
            "Esporta impostazioni stile correnti come JSON",
        );
        m.insert("tooltip_import_style", "Importa impostazioni stile da JSON");
        m.insert(
            "tooltip_print_calc",
            "Calcola dimensione pixel per la stampa",
        );
        m.insert("tooltip_copy_png", "Copia codice QR negli appunti");
        m.insert("tooltip_save_png", "Salva codice QR come PNG");
        m.insert("tooltip_copy_svg", "Copia codice QR come SVG negli appunti");
        m.insert("tooltip_save_svg", "Salva codice QR come SVG");
        m.insert("tooltip_save_gif", "Salva codice QR animato come GIF");
        m.insert(
            "tooltip_save_pdf",
            "Esporta codice QR come PDF (A4, pronto per la stampa)",
        );
        m.insert(
            "tooltip_label_sheet",
            "Disponi più codici QR come etichette su A4",
        );
        m.insert("tooltip_batch", "Esporta più codici QR contemporaneamente");
        m.insert("tooltip_export_more", "Altre opzioni di esportazione");
        m.insert("tooltip_sidebar_toggle", "Attiva/disattiva barra laterale");
        // Feature 3: labels
        m.insert("label_start_date", "Data inizio");
        m.insert("label_end_date", "Data fine");
        m.insert("label_time", "Ora:");
        m.insert("label_quiet_zone", "Zona di quiete (0-10)");
        m.insert("label_module_gap", "Spaziatura modulo (0-0.4)");
        m.insert("label_shadow_offset", "Scostamento ombra (1.0-5.0)");
        m.insert("label_logo_size", "Dimensione logo (0.1-0.6)");
        m.insert("label_outer_radius", "Raggio esterno");
        m.insert("label_inner_radius", "Raggio interno");
        m.insert("label_logo_border_width", "Spessore bordo logo (0-20)");
        m.insert("label_logo_padding", "Padding logo:");
        m.insert("label_frame_width", "Larghezza cornice (1-10)");
        m.insert("label_frame_outer_radius", "Raggio esterno");
        m.insert("label_svg_path", "Percorso SVG (attributo d):");
        m.insert("label_custom_dot_hint", "Suggerimento: coordinate nell'intervallo da 0 a 1. Esempi:
• Stella: M0.5,0 L0.62,0.38 L1,0.38 L0.69,0.62 L0.81,1 L0.5,0.76 L0.19,1 L0.31,0.62 L0,0.38 L0.38,0.38 Z
• Cuore: M0.5,0.9 L0.1,0.5 C0.1,0.1 0.5,0.1 0.5,0.4 C0.5,0.1 0.9,0.1 0.9,0.5 Z");
        m.insert("placeholder_custom_dot", "es. M0,0 L1,0 L1,1 L0,1 Z");
        m.insert("label_print_width", "Larghezza (cm):");
        m.insert("label_print_height", "Altezza (cm):");
        // Feature 6: Content validation
        m.insert("validation_invalid_email", "Indirizzo email non valido");
        m.insert(
            "validation_invalid_lat",
            "La latitudine deve essere compresa tra -90 e 90",
        );
        m.insert(
            "validation_invalid_lon",
            "La longitudine deve essere compresa tra -180 e 180",
        );
        m.insert("validation_invalid_phone", "Numero di telefono non valido");
        // Feature 9: Font selection
        m.insert("label_font", "Carattere");
        m.insert("label_font_size", "Dimensione carattere");
        // Scan verification
        m.insert("btn_verify_scan", "Verifica in corso…");
        m.insert(
            "scan_status_good",
            "Scansionabile — Tutti i controlli superati",
        );
        m.insert("scan_status_limited", "Scansionabilità limitata");
        m.insert(
            "scan_status_bad",
            "Non scansionabile — Impossibile decodificare il codice",
        );
        m.insert(
            "scan_tooltip",
            "Verifica contrasto, copertura del logo e se il codice può essere decodificato",
        );
        m.insert(
            "scan_detail_low_contrast",
            "Contrasto basso ({:.1}:1, consigliato ≥ 4.5:1)",
        );
        m.insert(
            "scan_detail_logo_ec",
            "Logo troppo grande per il livello di correzione errore",
        );
        m.insert(
            "scan_detail_large_gap",
            "La spaziatura modulo è molto ampia",
        );
        m.insert(
            "scan_detail_styled_corners",
            "Angoli stilizzati — gli scanner degli smartphone li gestiscono in modo affidabile",
        );

        // Dialog titles for file choosers
        m.insert("dlg_select_logo", "Seleziona logo");
        m.insert("dlg_select_bg", "Seleziona immagine di sfondo");
        m.insert("dlg_save_label_sheet", "Salva foglio etichette");
        m.insert("dlg_select_csv", "Seleziona file CSV");
        m.insert("dlg_select_folder", "Seleziona cartella");

        // Dialog buttons
        m.insert("btn_open", "Apri");
        m.insert("btn_select", "Seleziona");

        // File filter names
        m.insert("filter_images", "File immagine");
        m.insert("filter_json", "File JSON");
        m.insert("filter_csv_txt", "CSV/TXT");

        // Status messages (file operations)
        m.insert("status_style_exported", "Stile esportato");
        m.insert("status_pdf_saved", "PDF salvato");
        m.insert("status_pdf_error", "Errore nell'esportazione PDF");
        m.insert("status_label_sheet_saved", "Foglio etichette salvato");
        m.insert(
            "status_label_sheet_error",
            "Errore nell'esportazione etichette",
        );
        m.insert("status_png_saved", "PNG salvato");
        m.insert("status_svg_saved", "SVG salvato");
        m.insert("status_gif_saved", "GIF salvato");
        m.insert(
            "status_gif_gradient_only",
            "GIF disponibile solo con gradiente",
        );
        m.insert("status_batch_exported", "{} codici QR esportati");
        m.insert("status_saved_as", "Salvato come {}");
        m.insert(
            "status_enter_template_name",
            "Inserisci il nome del modello",
        );
        m.insert("status_template_deleted_fmt", "Modello '{}' eliminato");
        m.insert(
            "status_render_error",
            "Errore: Impossibile renderizzare il codice QR",
        );
        m.insert("status_copied", "Copiato negli appunti");
        m.insert("status_copied_svg", "SVG copiato negli appunti");

        // Batch/Label dialog labels
        m.insert("batch_data_label", "Dati QR (uno per riga):");
        m.insert(
            "batch_csv_hint",
            "(Prima colonna come dati QR, intestazione saltata)",
        );
        m.insert("batch_format", "Formato:");
        m.insert("batch_csv_filter", "CSV/TXT");
        m.insert("batch_folder_label", "Cartella:");
        m.insert("batch_folder_selected", "Cartella: {}");

        // Label sheet dialog
        m.insert("lbl_columns", "Colonne");
        m.insert("lbl_rows", "Righe");
        m.insert("lbl_margin_mm", "Margine (mm)");
        m.insert("lbl_spacing_mm", "Spaziatura (mm)");
        m.insert(
            "lbl_sheet_info",
            "Più codici QR su una pagina A4 per la stampa",
        );
        m.insert(
            "label_sheet_a4_info",
            "I codici QR verranno disposti su una pagina A4.",
        );

        // Dialog buttons for batch/label
        m.insert("btn_cancel", "Annulla");
        m.insert("btn_export", "Esporta");
        m.insert("btn_save", "Salva");

        // Dialog titles (FileChooserDialog)
        m.insert("dlg_save_pdf", "Salva come PDF");
        m.insert("dlg_save_png", "Salva come PNG");
        m.insert("dlg_save_svg", "Salva come SVG");
        m.insert("dlg_save_gif", "Salva come GIF");
        m.insert("dlg_import_style", "Importa stile");
        m.insert("dlg_export_style", "Esporta stile");
        m.insert("dlg_batch_export", "Esportazione batch");
        m.insert("dlg_label_sheet", "Foglio etichette");

        m
    }

    fn portuguese_br() -> HashMap<&'static str, &'static str> {
        let mut m = HashMap::new();
        // Tab labels
        m.insert("tab_content", "Conteúdo");
        m.insert("tab_style", "Estilo");
        m.insert("btn_export_more", "Exportar ▾");
        // Section headers
        // Content types
        // Buttons
        m.insert("btn_copy", "Copiar");
        m.insert("btn_save_png", "Salvar como PNG");
        m.insert("btn_save_svg", "Salvar como SVG");
        m.insert("btn_copy_svg", "Copiar SVG");
        m.insert("btn_save_gif", "Salvar como GIF");
        m.insert("btn_save_pdf", "Salvar como PDF");
        m.insert("btn_label_sheet", "Folha de Etiquetas...");
        m.insert("btn_batch", "Exportação em Lote...");
        m.insert("btn_select_image", "Selecionar Imagem");
        m.insert("btn_remove", "Remover");
        m.insert("btn_clear_all", "Limpar tudo");
        m.insert("tooltip_clear_all", "Limpar todos os campos de entrada");
        // Labels
        // WiFi fields
        m.insert("wifi_password", "Senha");
        // vCard fields
        m.insert("vcard_name", "Nome");
        m.insert("vcard_phone", "Número de Telefone");
        m.insert("vcard_email", "E-mail");
        m.insert("vcard_org", "Organização");
        m.insert("vcard_url", "URL do Site");
        // Calendar fields
        m.insert("cal_title", "Título");
        m.insert("cal_location", "Local");
        // GPS fields
        m.insert("gps_lat", "Latitude");
        m.insert("gps_lon", "Longitude");
        m.insert("gps_search", "Buscar localização…");

        // SMS fields
        m.insert("sms_phone", "Número de Telefone");
        m.insert("sms_message", "Mensagem");
        // Frame styles
        // Gradient directions
        // Status messages
        m.insert(
            "status_template_saved_full",
            "Modelo '{}' salvo (estilo + conteúdo)",
        );
        m.insert(
            "status_template_saved_style",
            "Modelo '{}' salvo (apenas estilo)",
        );
        m.insert(
            "status_template_loaded_full",
            "Modelo '{}' carregado (estilo + conteúdo)",
        );
        m.insert(
            "status_template_loaded_style",
            "Modelo '{}' carregado (apenas estilo)",
        );
        // Dialog titles
        // Batch dialog
        // Label sheet dialog
        // EC levels
        // Dot styles
        // Corner square styles
        // Logo shapes
        // Harmonies
        m.insert("harmony_title", "Harmonias de Cores (como plano de fundo)");
        m.insert("color_fg", "Primeiro Plano");
        m.insert("color_bg", "Plano de Fundo");
        m.insert("color_corner", "Cor do Canto");
        m.insert("color_gradient", "Alvo do Gradiente");
        m.insert("dnd_logo_imported", "Logo importada via Arrastar e Soltar");
        // QR Info
        m.insert("qrinfo_version", "Versão");
        m.insert("qrinfo_modules", "módulos");
        m.insert("qrinfo_ec", "Correção de Erro");
        m.insert("qrinfo_capacity", "Capacidade de Dados");
        m.insert("qrinfo_data_loss", "perda de dados");
        m.insert("qrinfo_bytes", "Bytes");
        m.insert("qrinfo_scan_dist", "Distância de Escaneamento");
        m.insert("qrinfo_at_dpi", "a 300 DPI");
        m.insert("qrinfo_no_data", "Sem dados");
        m.insert(
            "qrinfo_data_too_long",
            "Dados muito longos para o código QR",
        );
        // Expander sections (as used in build_ui)
        m.insert("exp_content", "Conteúdo");
        m.insert("exp_presets", "Predefinições de Estilo");
        m.insert("exp_templates", "Modelos");
        m.insert("check_save_content", "Incluir conteúdo");
        m.insert("tooltip_save_content", "Quando ativado, o conteúdo atual (texto, WiFi, vCard, etc.) é salvo junto com o estilo");
        m.insert("exp_pattern", "Padrão e Cantos");
        m.insert("exp_colors", "Cores");
        m.insert("exp_settings", "Configurações");
        m.insert("exp_qr_info", "Informações QR");
        m.insert("exp_advanced", "Avançado");
        m.insert("exp_logo", "Logo (Centro)");
        m.insert("exp_outer_text", "Texto ao redor do Código QR");
        m.insert("exp_frame", "Moldura");
        m.insert("exp_bg", "Plano de Fundo");
        m.insert("exp_import", "Importar/Exportar");
        // Preview
        m.insert("preview_label", "Pré-visualização em Tempo Real");
        // Generate button
        // Dropdown items as used in build_ui
        m.insert("dd_content_text", "Texto");
        m.insert("dd_content_wifi", "WiFi");
        m.insert("dd_content_vcard", "vCard/Contato");
        m.insert("dd_content_calendar", "Evento de Calendário");
        m.insert("dd_content_gps", "Localização GPS");
        m.insert("dd_content_sms", "SMS");
        m.insert("dd_dot_rounded", "Arredondado");
        m.insert("dd_dot_square", "Quadrado");
        m.insert("dd_dot_dots", "Pontos");
        m.insert("dd_dot_diamond", "Diamante");
        m.insert("dd_dot_custom", "Personalizado");
        m.insert("dd_corner_sq_rounded", "Arredondado");
        m.insert("dd_corner_sq_square", "Quadrado");
        m.insert("dd_corner_sq_dot", "Ponto");
        m.insert("dd_corner_sq_circle", "Círculo");
        m.insert("dd_corner_dot_dot", "Ponto");
        m.insert("dd_corner_dot_square", "Quadrado");
        m.insert("dd_corner_dot_circle", "Círculo");
        m.insert("dd_corner_dot_rounded", "Arredondado");
        m.insert("dd_wifi_wpa", "WPA");
        m.insert("dd_wifi_wep", "WEP");
        m.insert("dd_wifi_none", "Nenhum");
        m.insert("dd_ec_medium", "Médio (M)");
        m.insert("dd_ec_low", "Baixo (L)");
        m.insert("dd_ec_quartile", "Quartil (Q)");
        m.insert("dd_ec_high", "Alto (H)");
        m.insert("dd_module_medium", "Médio (32px)");
        m.insert("dd_module_small", "Pequeno (16px)");
        m.insert("dd_module_large", "Grande (64px)");
        m.insert("dd_module_print", "Impressão (128px)");
        m.insert("dd_grad_horizontal", "Horizontal");
        m.insert("dd_grad_vertical", "Vertical");
        m.insert("dd_grad_diagonal", "Diagonal");
        m.insert("dd_grad_radial", "Radial");
        m.insert("dd_logo_circle", "Círculo");
        m.insert("dd_logo_rectangle", "Retângulo");
        m.insert("dd_logo_rounded", "Arredondado");
        m.insert("dd_frame_none", "Nenhum");
        m.insert("dd_frame_simple", "Simples");
        m.insert("dd_frame_rounded", "Arredondado");
        m.insert("dd_frame_banner", "Faixa");
        m.insert("dd_palette_custom", "Personalizada");
        m.insert("dd_palette_classic", "Clássica");
        m.insert("dd_palette_ocean", "Oceano");
        m.insert("dd_palette_sunset", "Pôr do Sol");
        m.insert("dd_palette_forest", "Floresta");
        m.insert("dd_palette_lavender", "Lavanda");
        m.insert("dd_palette_fire", "Fogo");
        m.insert("dd_palette_aurora", "Aurora");
        m.insert("dd_palette_pastel", "Pastel");
        m.insert("dd_palette_neon", "Neon");
        m.insert("dd_preset_custom", "Personalizado");
        m.insert("dd_preset_classic", "Clássico");
        m.insert("dd_preset_rounded", "Arredondado");
        m.insert("dd_preset_dots", "Pontos");
        m.insert("dd_preset_diamond", "Diamante");
        m.insert("dd_preset_minimal", "Minimalista");
        m.insert("dd_preset_retro", "Retrô");
        // Misc
        m.insert("btn_bg_select", "Selecionar Imagem de Plano de Fundo");
        m.insert("btn_export_style_short", "Exportar Estilo");
        m.insert("btn_import_style_short", "Importar Estilo");
        m.insert("btn_print_calc", "Calculadora de Tamanho de Impressão");
        m.insert("dlg_print_calc", "Calculadora de Tamanho de Impressão");
        m.insert("btn_apply", "Aplicar");
        m.insert("print_calc_result", "{} x {} pixels (tamanho de módulo recomendado: ~{}px)");
        m.insert("label_dpi", "DPI:");
        m.insert("check_transparent_bg", "Plano de Fundo Transparente");
        m.insert("check_gradient", "Ativar Gradiente");
        m.insert("check_shadow", "Ativar Sombra");
        m.insert("check_logo_vectorize", "Vetorizar Logo");
        m.insert("check_logo_bg_transparent", "Fundo Transparente");
        m.insert("check_logo_clear_area", "Limpar Área");
        m.insert("check_radius_sync", "Sincronizar Raios");
        m.insert("placeholder_template_name", "Nome do modelo...");
        m.insert("placeholder_top_text", "Texto superior");
        m.insert("placeholder_bottom_text", "Texto inferior");
        m.insert("tooltip_undo", "Desfazer (Ctrl+Z)");
        m.insert("tooltip_redo", "Refazer (Ctrl+Y)");
        // Feature 2: Transparency Checkerboard
        m.insert(
            "tooltip_preview_bg",
            "Alternar plano de fundo da pré-visualização",
        );
        // Feature 3: i18n for hardcoded strings - tooltips
        m.insert("tooltip_content_type", "Selecionar tipo de conteúdo");
        m.insert("tooltip_qr_content", "Conteúdo do Código QR");
        m.insert("tooltip_wifi_ssid", "Nome da rede WiFi");
        m.insert("tooltip_wifi_password", "Senha WiFi");
        m.insert("tooltip_wifi_encryption", "Criptografia");
        m.insert("tooltip_vcard_name", "Nome do contato");
        m.insert("tooltip_vcard_phone", "Número de telefone");
        m.insert("tooltip_vcard_email", "Endereço de e-mail");
        m.insert("tooltip_vcard_org", "Organização/Empresa");
        m.insert("tooltip_vcard_url", "URL do site");
        m.insert("tooltip_cal_title", "Título do evento");
        m.insert("tooltip_cal_hour", "Hora");
        m.insert("tooltip_cal_minute", "Minuto");
        m.insert("tooltip_cal_location", "Local do evento");
        m.insert("tooltip_gps_lat", "Latitude");
        m.insert("tooltip_gps_lon", "Longitude");
        m.insert("tooltip_gps_search", "Digite um local e pressione Enter");

        m.insert("tooltip_sms_phone", "Número do destinatário SMS");
        m.insert("tooltip_sms_message", "Texto da mensagem SMS");
        m.insert("tooltip_preset_select", "Selecionar predefinição de estilo");
        m.insert(
            "tooltip_template_save",
            "Salvar estilo atual + conteúdo como modelo",
        );
        m.insert(
            "tooltip_template_load",
            "Carregar modelo salvo (estilo + conteúdo)",
        );
        m.insert("tooltip_template_delete", "Excluir modelo selecionado");
        m.insert("tooltip_dot_style", "Estilo do ponto de dados");
        m.insert("tooltip_corner_sq_style", "Estilo do quadrado de canto");
        m.insert("tooltip_corner_dot_style", "Estilo do ponto de canto");
        m.insert(
            "tooltip_custom_dot_svg",
            "Dados de caminho SVG para formato de ponto personalizado (coordenadas 0..1)",
        );
        m.insert(
            "tooltip_transparent_bg",
            "Tornar o plano de fundo transparente",
        );
        m.insert("tooltip_gradient_enable", "Ativar gradiente");
        m.insert("tooltip_gradient_dir", "Direção do gradiente");
        m.insert("tooltip_palette", "Selecionar paleta de cores");
        m.insert("tooltip_ec_level", "Nível de correção de erro");
        m.insert("tooltip_module_size", "Tamanho do módulo");
        m.insert(
            "tooltip_quiet_zone",
            "Zona de silêncio ao redor do código QR",
        );
        m.insert("tooltip_module_gap", "Espaço entre módulos");
        m.insert("tooltip_shadow_enable", "Ativar efeito de sombra");
        m.insert("tooltip_shadow_offset", "Deslocamento da sombra");
        m.insert("tooltip_logo_select", "Selecionar imagem da logo");
        m.insert("tooltip_logo_remove", "Remover logo");
        m.insert("tooltip_logo_size", "Tamanho da logo relativo ao código QR");
        m.insert("tooltip_logo_shape", "Forma da logo");
        m.insert(
            "tooltip_logo_radius_sync",
            "Vincular raio interno e externo",
        );
        m.insert(
            "tooltip_logo_color",
            "Coloração da logo (Alfa = intensidade)",
        );
        m.insert(
            "tooltip_logo_border_width",
            "Largura da borda ao redor da logo",
        );
        m.insert("tooltip_logo_border_color", "Cor da borda da logo");
        m.insert(
            "tooltip_logo_vectorize",
            "Converter logos rasterizadas (PNG/JPG) em caminhos vetoriais",
        );
        m.insert("tooltip_logo_vectorize_bg", "Cor do plano de fundo da logo vetorizada: Alfa=0 remove o fundo, Alfa>0 substitui por esta cor");
        m.insert(
            "tooltip_logo_bg_transparent",
            "Tornar o fundo da área do logo transparente, independente da cor de fundo do QR",
        );
        m.insert(
            "tooltip_logo_clear_area",
            "Reorganizar módulos QR ao redor da logo (requer correção de erro)",
        );
        m.insert(
            "tooltip_logo_padding",
            "Espaçamento extra ao redor da logo (em módulos) para melhor reorganização",
        );
        m.insert(
            "tooltip_outer_radius",
            "Raio externo da moldura (0 = quadrado, 0.5 = maximamente arredondado)",
        );
        m.insert(
            "tooltip_inner_radius",
            "Raio interno da moldura (0 = quadrado, 0.5 = maximamente arredondado)",
        );
        m.insert("tooltip_top_text", "Texto acima do código QR");
        m.insert("tooltip_bottom_text", "Texto abaixo do código QR");
        m.insert("tooltip_text_color", "Cor do texto");
        m.insert("tooltip_frame_style", "Estilo da moldura");
        m.insert("tooltip_frame_color", "Cor da moldura");
        m.insert(
            "tooltip_frame_width",
            "Largura da moldura em unidades de módulo",
        );
        m.insert(
            "tooltip_frame_outer_radius",
            "Raio externo da moldura (0 = quadrado, 0.5 = maximamente arredondado)",
        );
        m.insert("tooltip_bg_select", "Selecionar imagem de plano de fundo");
        m.insert("tooltip_bg_remove", "Remover imagem de plano de fundo");
        m.insert(
            "tooltip_export_style",
            "Exportar configurações de estilo atuais como JSON",
        );
        m.insert(
            "tooltip_import_style",
            "Importar configurações de estilo de JSON",
        );
        m.insert(
            "tooltip_print_calc",
            "Calcular tamanho em pixels para impressão",
        );
        m.insert(
            "tooltip_copy_png",
            "Copiar código QR para a área de transferência",
        );
        m.insert("tooltip_save_png", "Salvar código QR como PNG");
        m.insert(
            "tooltip_copy_svg",
            "Copiar código QR como SVG para a área de transferência",
        );
        m.insert("tooltip_save_svg", "Salvar código QR como SVG");
        m.insert("tooltip_save_gif", "Salvar código QR animado como GIF");
        m.insert(
            "tooltip_save_pdf",
            "Exportar código QR como PDF (A4, pronto para impressão)",
        );
        m.insert(
            "tooltip_label_sheet",
            "Organizar múltiplos códigos QR como etiquetas em A4",
        );
        m.insert("tooltip_batch", "Exportar múltiplos códigos QR de uma vez");
        m.insert("tooltip_export_more", "Mais opções de exportação");
        m.insert("tooltip_sidebar_toggle", "Alternar barra lateral");
        // Feature 3: labels
        m.insert("label_start_date", "Data de início");
        m.insert("label_end_date", "Data de término");
        m.insert("label_time", "Hora:");
        m.insert("label_quiet_zone", "Zona de silêncio (0-10)");
        m.insert("label_module_gap", "Espaçamento do módulo (0-0.4)");
        m.insert("label_shadow_offset", "Deslocamento da sombra (1.0-5.0)");
        m.insert("label_logo_size", "Tamanho da logo (0.1-0.6)");
        m.insert("label_outer_radius", "Raio Externo");
        m.insert("label_inner_radius", "Raio Interno");
        m.insert("label_logo_border_width", "Largura da borda da logo (0-20)");
        m.insert("label_logo_padding", "Espaçamento da logo:");
        m.insert("label_frame_width", "Largura da moldura (1-10)");
        m.insert("label_frame_outer_radius", "Raio Externo");
        m.insert("label_svg_path", "Caminho SVG (atributo d):");
        m.insert("label_custom_dot_hint", "Dica: Coordenadas no intervalo de 0 a 1. Exemplos:\n• Estrela: M0.5,0 L0.62,0.38 L1,0.38 L0.69,0.62 L0.81,1 L0.5,0.76 L0.19,1 L0.31,0.62 L0,0.38 L0.38,0.38 Z\n• Coração: M0.5,0.9 L0.1,0.5 C0.1,0.1 0.5,0.1 0.5,0.4 C0.5,0.1 0.9,0.1 0.9,0.5 Z");
        m.insert("placeholder_custom_dot", "ex. M0,0 L1,0 L1,1 L0,1 Z");
        m.insert("label_print_width", "Largura (cm):");
        m.insert("label_print_height", "Altura (cm):");
        // Feature 6: Content validation
        m.insert("validation_invalid_email", "Endereço de e-mail inválido");
        m.insert(
            "validation_invalid_lat",
            "Latitude deve estar entre -90 e 90",
        );
        m.insert(
            "validation_invalid_lon",
            "Longitude deve estar entre -180 e 180",
        );
        m.insert("validation_invalid_phone", "Número de telefone inválido");
        // Feature 9: Font selection
        m.insert("label_font", "Fonte");
        m.insert("label_font_size", "Tamanho da fonte");
        // Scan verification
        m.insert("btn_verify_scan", "Verificando…");
        m.insert(
            "scan_status_good",
            "Escaneável — Todas as verificações passaram",
        );
        m.insert("scan_status_limited", "Escaneabilidade limitada");
        m.insert(
            "scan_status_bad",
            "Não escaneável — O código não pôde ser decodificado",
        );
        m.insert(
            "scan_tooltip",
            "Verifica contraste, cobertura da logo e se o código pode ser decodificado",
        );
        m.insert(
            "scan_detail_low_contrast",
            "Baixo contraste ({:.1}:1, recomendado ≥ 4.5:1)",
        );
        m.insert(
            "scan_detail_logo_ec",
            "Logo grande demais para o nível de correção de erro",
        );
        m.insert(
            "scan_detail_large_gap",
            "Espaçamento do módulo é muito grande",
        );
        m.insert(
            "scan_detail_styled_corners",
            "Cantos estilizados — escâneres de smartphones lidam com estes de forma confiável",
        );

        // Dialog titles for file choosers
        m.insert("dlg_select_logo", "Selecionar logo");
        m.insert("dlg_select_bg", "Selecionar imagem de fundo");
        m.insert("dlg_save_label_sheet", "Salvar folha de etiquetas");
        m.insert("dlg_select_csv", "Selecionar arquivo CSV");
        m.insert("dlg_select_folder", "Selecionar pasta");

        // Dialog buttons
        m.insert("btn_open", "Abrir");
        m.insert("btn_select", "Selecionar");

        // File filter names
        m.insert("filter_images", "Arquivos de imagem");
        m.insert("filter_json", "Arquivos JSON");
        m.insert("filter_csv_txt", "CSV/TXT");

        // Status messages (file operations)
        m.insert("status_style_exported", "Estilo exportado");
        m.insert("status_pdf_saved", "PDF salvo");
        m.insert("status_pdf_error", "Erro ao exportar PDF");
        m.insert("status_label_sheet_saved", "Folha de etiquetas salva");
        m.insert("status_label_sheet_error", "Erro ao exportar etiquetas");
        m.insert("status_png_saved", "PNG salvo");
        m.insert("status_svg_saved", "SVG salvo");
        m.insert("status_gif_saved", "GIF salvo");
        m.insert(
            "status_gif_gradient_only",
            "GIF disponível apenas com gradiente",
        );
        m.insert("status_batch_exported", "{} códigos QR exportados");
        m.insert("status_saved_as", "Salvo como {}");
        m.insert("status_enter_template_name", "Digite o nome do modelo");
        m.insert("status_template_deleted_fmt", "Modelo '{}' excluído");
        m.insert(
            "status_render_error",
            "Erro: Não foi possível renderizar o código QR",
        );
        m.insert("status_copied", "Copiado para a área de transferência");
        m.insert(
            "status_copied_svg",
            "SVG copiado para a área de transferência",
        );

        // Batch/Label dialog labels
        m.insert("batch_data_label", "Dados QR (um por linha):");
        m.insert(
            "batch_csv_hint",
            "(Primeira coluna como dados QR, cabeçalho ignorado)",
        );
        m.insert("batch_format", "Formato:");
        m.insert("batch_csv_filter", "CSV/TXT");
        m.insert("batch_folder_label", "Pasta:");
        m.insert("batch_folder_selected", "Pasta: {}");

        // Label sheet dialog
        m.insert("lbl_columns", "Colunas");
        m.insert("lbl_rows", "Linhas");
        m.insert("lbl_margin_mm", "Margem (mm)");
        m.insert("lbl_spacing_mm", "Espaçamento (mm)");
        m.insert(
            "lbl_sheet_info",
            "Vários códigos QR em uma página A4 para impressão",
        );
        m.insert(
            "label_sheet_a4_info",
            "Os códigos QR serão organizados em uma página A4.",
        );

        // Dialog buttons for batch/label
        m.insert("btn_cancel", "Cancelar");
        m.insert("btn_export", "Exportar");
        m.insert("btn_save", "Salvar");

        // Dialog titles (FileChooserDialog)
        m.insert("dlg_save_pdf", "Salvar como PDF");
        m.insert("dlg_save_png", "Salvar como PNG");
        m.insert("dlg_save_svg", "Salvar como SVG");
        m.insert("dlg_save_gif", "Salvar como GIF");
        m.insert("dlg_import_style", "Importar estilo");
        m.insert("dlg_export_style", "Exportar estilo");
        m.insert("dlg_batch_export", "Exportação em lote");
        m.insert("dlg_label_sheet", "Folha de etiquetas");

        m
    }

    fn japanese() -> HashMap<&'static str, &'static str> {
        let mut m = HashMap::new();
        // Tab labels
        m.insert("tab_content", "コンテンツ");
        m.insert("tab_style", "スタイル");
        m.insert("btn_export_more", "エクスポート ▾");
        // Section headers
        // Content types
        // Buttons
        m.insert("btn_copy", "コピー");
        m.insert("btn_save_png", "PNG で保存");
        m.insert("btn_save_svg", "SVG で保存");
        m.insert("btn_copy_svg", "SVG をコピー");
        m.insert("btn_save_gif", "GIF で保存");
        m.insert("btn_save_pdf", "PDF で保存");
        m.insert("btn_label_sheet", "ラベルシート...");
        m.insert("btn_batch", "一括エクスポート...");
        m.insert("btn_select_image", "画像を選択");
        m.insert("btn_remove", "削除");
        m.insert("btn_clear_all", "すべてクリア");
        m.insert("tooltip_clear_all", "すべての入力フィールドをリセット");
        // Labels
        // WiFi fields
        m.insert("wifi_password", "パスワード");
        // vCard fields
        m.insert("vcard_name", "名前");
        m.insert("vcard_phone", "電話番号");
        m.insert("vcard_email", "メール");
        m.insert("vcard_org", "組織");
        m.insert("vcard_url", "ウェブサイト URL");
        // Calendar fields
        m.insert("cal_title", "タイトル");
        m.insert("cal_location", "場所");
        // GPS fields
        m.insert("gps_lat", "緯度");
        m.insert("gps_lon", "経度");
        m.insert("gps_search", "場所を検索…");

        // SMS fields
        m.insert("sms_phone", "電話番号");
        m.insert("sms_message", "メッセージ");
        // Frame styles
        // Gradient directions
        // Status messages
        m.insert(
            "status_template_saved_full",
            "テンプレート '{}' を保存しました (スタイル + コンテンツ)",
        );
        m.insert(
            "status_template_saved_style",
            "テンプレート '{}' を保存しました (スタイルのみ)",
        );
        m.insert(
            "status_template_loaded_full",
            "テンプレート '{}' を読み込みました (スタイル + コンテンツ)",
        );
        m.insert(
            "status_template_loaded_style",
            "テンプレート '{}' を読み込みました (スタイルのみ)",
        );
        // Dialog titles
        // Batch dialog
        // Label sheet dialog
        // EC levels
        // Dot styles
        // Corner square styles
        // Logo shapes
        // Harmonies
        m.insert("harmony_title", "カラーハーモニー（背景として）");
        m.insert("color_fg", "前景色");
        m.insert("color_bg", "背景色");
        m.insert("color_corner", "角の色");
        m.insert("color_gradient", "グラデーション対象");
        m.insert(
            "dnd_logo_imported",
            "ドラッグ＆ドロップでロゴをインポートしました",
        );
        // QR Info
        m.insert("qrinfo_version", "バージョン");
        m.insert("qrinfo_modules", "モジュール");
        m.insert("qrinfo_ec", "誤り訂正");
        m.insert("qrinfo_capacity", "データ容量");
        m.insert("qrinfo_data_loss", "データ損失");
        m.insert("qrinfo_bytes", "バイト");
        m.insert("qrinfo_scan_dist", "スキャン距離");
        m.insert("qrinfo_at_dpi", "300 DPI で");
        m.insert("qrinfo_no_data", "データなし");
        m.insert("qrinfo_data_too_long", "QR コードにはデータが長すぎます");
        // Expander sections (as used in build_ui)
        m.insert("exp_content", "コンテンツ");
        m.insert("exp_presets", "スタイルプリセット");
        m.insert("exp_templates", "テンプレート");
        m.insert("check_save_content", "コンテンツを含める");
        m.insert("tooltip_save_content", "有効にすると、現在のコンテンツ（テキスト、WiFi、vCard など）がスタイルと一緒に保存されます");
        m.insert("exp_pattern", "パターンとコーナー");
        m.insert("exp_colors", "色");
        m.insert("exp_settings", "設定");
        m.insert("exp_qr_info", "QR 情報");
        m.insert("exp_advanced", "詳細");
        m.insert("exp_logo", "ロゴ（中央）");
        m.insert("exp_outer_text", "QR コード周辺のテキスト");
        m.insert("exp_frame", "フレーム");
        m.insert("exp_bg", "背景");
        m.insert("exp_import", "インポート/エクスポート");
        // Preview
        m.insert("preview_label", "ライブプレビュー");
        // Generate button
        // Dropdown items as used in build_ui
        m.insert("dd_content_text", "テキスト");
        m.insert("dd_content_wifi", "WiFi");
        m.insert("dd_content_vcard", "vCard/連絡先");
        m.insert("dd_content_calendar", "カレンダーイベント");
        m.insert("dd_content_gps", "GPS 位置情報");
        m.insert("dd_content_sms", "SMS");
        m.insert("dd_dot_rounded", "角丸");
        m.insert("dd_dot_square", "四角");
        m.insert("dd_dot_dots", "ドット");
        m.insert("dd_dot_diamond", "ダイヤモンド");
        m.insert("dd_dot_custom", "カスタム");
        m.insert("dd_corner_sq_rounded", "角丸");
        m.insert("dd_corner_sq_square", "四角");
        m.insert("dd_corner_sq_dot", "ドット");
        m.insert("dd_corner_sq_circle", "円");
        m.insert("dd_corner_dot_dot", "ドット");
        m.insert("dd_corner_dot_square", "四角");
        m.insert("dd_corner_dot_circle", "円");
        m.insert("dd_corner_dot_rounded", "角丸");
        m.insert("dd_wifi_wpa", "WPA");
        m.insert("dd_wifi_wep", "WEP");
        m.insert("dd_wifi_none", "なし");
        m.insert("dd_ec_medium", "中 (M)");
        m.insert("dd_ec_low", "低 (L)");
        m.insert("dd_ec_quartile", "四分位 (Q)");
        m.insert("dd_ec_high", "高 (H)");
        m.insert("dd_module_medium", "中 (32px)");
        m.insert("dd_module_small", "小 (16px)");
        m.insert("dd_module_large", "大 (64px)");
        m.insert("dd_module_print", "印刷用 (128px)");
        m.insert("dd_grad_horizontal", "水平");
        m.insert("dd_grad_vertical", "垂直");
        m.insert("dd_grad_diagonal", "斜め");
        m.insert("dd_grad_radial", "放射状");
        m.insert("dd_logo_circle", "円");
        m.insert("dd_logo_rectangle", "長方形");
        m.insert("dd_logo_rounded", "角丸");
        m.insert("dd_frame_none", "なし");
        m.insert("dd_frame_simple", "シンプル");
        m.insert("dd_frame_rounded", "角丸");
        m.insert("dd_frame_banner", "バナー");
        m.insert("dd_palette_custom", "カスタム");
        m.insert("dd_palette_classic", "クラシック");
        m.insert("dd_palette_ocean", "オーシャン");
        m.insert("dd_palette_sunset", "サンセット");
        m.insert("dd_palette_forest", "フォレスト");
        m.insert("dd_palette_lavender", "ラベンダー");
        m.insert("dd_palette_fire", "ファイア");
        m.insert("dd_palette_aurora", "オーロラ");
        m.insert("dd_palette_pastel", "パステル");
        m.insert("dd_palette_neon", "ネオン");
        m.insert("dd_preset_custom", "カスタム");
        m.insert("dd_preset_classic", "クラシック");
        m.insert("dd_preset_rounded", "角丸");
        m.insert("dd_preset_dots", "ドット");
        m.insert("dd_preset_diamond", "ダイヤモンド");
        m.insert("dd_preset_minimal", "ミニマリスト");
        m.insert("dd_preset_retro", "レトロ");
        // Misc
        m.insert("btn_bg_select", "背景画像を選択");
        m.insert("btn_export_style_short", "スタイルをエクスポート");
        m.insert("btn_import_style_short", "スタイルをインポート");
        m.insert("btn_print_calc", "印刷サイズ計算ツール");
        m.insert("dlg_print_calc", "印刷サイズ計算ツール");
        m.insert("btn_apply", "適用");
        m.insert("print_calc_result", "{} x {} ピクセル（推奨モジュールサイズ: ~{}px）");
        m.insert("label_dpi", "DPI:");
        m.insert("check_transparent_bg", "透明な背景");
        m.insert("check_gradient", "グラデーションを有効化");
        m.insert("check_shadow", "シャドウを有効化");
        m.insert("check_logo_vectorize", "ロゴをベクター化");
        m.insert("check_logo_bg_transparent", "背景を透明にする");
        m.insert("check_logo_clear_area", "領域をクリア");
        m.insert("check_radius_sync", "半径を同期");
        m.insert("placeholder_template_name", "テンプレート名...");
        m.insert("placeholder_top_text", "上部テキスト");
        m.insert("placeholder_bottom_text", "下部テキスト");
        m.insert("tooltip_undo", "元に戻す (Ctrl+Z)");
        m.insert("tooltip_redo", "やり直す (Ctrl+Y)");
        // Feature 2: Transparency Checkerboard
        m.insert("tooltip_preview_bg", "プレビュー背景を切り替え");
        // Feature 3: i18n for hardcoded strings - tooltips
        m.insert("tooltip_content_type", "コンテンツタイプを選択");
        m.insert("tooltip_qr_content", "QR コードのコンテンツ");
        m.insert("tooltip_wifi_ssid", "WiFi ネットワーク名");
        m.insert("tooltip_wifi_password", "WiFi パスワード");
        m.insert("tooltip_wifi_encryption", "暗号化");
        m.insert("tooltip_vcard_name", "連絡先の名前");
        m.insert("tooltip_vcard_phone", "電話番号");
        m.insert("tooltip_vcard_email", "メールアドレス");
        m.insert("tooltip_vcard_org", "組織/会社");
        m.insert("tooltip_vcard_url", "ウェブサイト URL");
        m.insert("tooltip_cal_title", "イベントのタイトル");
        m.insert("tooltip_cal_hour", "時");
        m.insert("tooltip_cal_minute", "分");
        m.insert("tooltip_cal_location", "イベントの場所");
        m.insert("tooltip_gps_lat", "緯度");
        m.insert("tooltip_gps_lon", "経度");
        m.insert(
            "tooltip_gps_search",
            "場所を入力して Enter を押してください",
        );

        m.insert("tooltip_sms_phone", "SMS 宛先番号");
        m.insert("tooltip_sms_message", "SMS メッセージ本文");
        m.insert("tooltip_preset_select", "スタイルプリセットを選択");
        m.insert(
            "tooltip_template_save",
            "現在のスタイルとコンテンツをテンプレートとして保存",
        );
        m.insert(
            "tooltip_template_load",
            "保存済みテンプレートを読み込む（スタイル＋コンテンツ）",
        );
        m.insert("tooltip_template_delete", "選択したテンプレートを削除");
        m.insert("tooltip_dot_style", "データドットのスタイル");
        m.insert("tooltip_corner_sq_style", "角の四角形のスタイル");
        m.insert("tooltip_corner_dot_style", "角のドットのスタイル");
        m.insert(
            "tooltip_custom_dot_svg",
            "カスタムドット形状の SVG パスデータ（座標 0..1）",
        );
        m.insert("tooltip_transparent_bg", "背景を透明にする");
        m.insert("tooltip_gradient_enable", "グラデーションを有効にする");
        m.insert("tooltip_gradient_dir", "グラデーションの方向");
        m.insert("tooltip_palette", "カラーパレットを選択");
        m.insert("tooltip_ec_level", "誤り訂正レベル");
        m.insert("tooltip_module_size", "モジュールサイズ");
        m.insert("tooltip_quiet_zone", "QR コード周囲のクワイエットゾーン");
        m.insert("tooltip_module_gap", "モジュール間のギャップ");
        m.insert("tooltip_shadow_enable", "シャドウ効果を有効にする");
        m.insert("tooltip_shadow_offset", "シャドウのオフセット");
        m.insert("tooltip_logo_select", "ロゴ画像を選択");
        m.insert("tooltip_logo_remove", "ロゴを削除");
        m.insert("tooltip_logo_size", "QR コードに対するロゴのサイズ");
        m.insert("tooltip_logo_shape", "ロゴの形");
        m.insert("tooltip_logo_radius_sync", "内側と外側の半径をリンク");
        m.insert("tooltip_logo_color", "ロゴの着色（Alpha = 強さ）");
        m.insert("tooltip_logo_border_width", "ロゴ周囲の枠線の太さ");
        m.insert("tooltip_logo_border_color", "ロゴの枠線の色");
        m.insert(
            "tooltip_logo_vectorize",
            "ラスター画像（PNG/JPG）をベクターパスに変換",
        );
        m.insert(
            "tooltip_logo_vectorize_bg",
            "ベクター化されたロゴの背景色: Alpha=0 で背景を削除、Alpha>0 でこの色に置き換え",
        );
        m.insert(
            "tooltip_logo_bg_transparent",
            "QR背景色に関係なく、ロゴ領域の背景を透明にする",
        );
        m.insert(
            "tooltip_logo_clear_area",
            "ロゴ周辺の QR モジュールを再配置（誤り訂正が必要）",
        );
        m.insert(
            "tooltip_logo_padding",
            "再配置を改善するためのロゴ周囲の追加パディング（モジュール単位）",
        );
        m.insert(
            "tooltip_outer_radius",
            "外側フレームの半径（0 = 四角、0.5 = 最大角丸）",
        );
        m.insert(
            "tooltip_inner_radius",
            "内側フレームの半径（0 = 四角、0.5 = 最大角丸）",
        );
        m.insert("tooltip_top_text", "QR コードの上のテキスト");
        m.insert("tooltip_bottom_text", "QR コードの下のテキスト");
        m.insert("tooltip_text_color", "テキストの色");
        m.insert("tooltip_frame_style", "フレームスタイル");
        m.insert("tooltip_frame_color", "フレームの色");
        m.insert("tooltip_frame_width", "モジュール単位のフレームの太さ");
        m.insert(
            "tooltip_frame_outer_radius",
            "外側フレームの半径（0 = 四角、0.5 = 最大角丸）",
        );
        m.insert("tooltip_bg_select", "背景画像を選択");
        m.insert("tooltip_bg_remove", "背景画像を削除");
        m.insert(
            "tooltip_export_style",
            "現在のスタイル設定を JSON としてエクスポート",
        );
        m.insert("tooltip_import_style", "JSON からスタイル設定をインポート");
        m.insert("tooltip_print_calc", "印刷用のピクセルサイズを計算");
        m.insert("tooltip_copy_png", "QR コードをクリップボードにコピー");
        m.insert("tooltip_save_png", "QR コードを PNG で保存");
        m.insert(
            "tooltip_copy_svg",
            "QR コードを SVG としてクリップボードにコピー",
        );
        m.insert("tooltip_save_svg", "QR コードを SVG で保存");
        m.insert("tooltip_save_gif", "アニメーション QR コードを GIF で保存");
        m.insert(
            "tooltip_save_pdf",
            "QR コードを PDF としてエクスポート（A4、印刷対応）",
        );
        m.insert(
            "tooltip_label_sheet",
            "複数の QR コードを A4 のラベルとして配置",
        );
        m.insert("tooltip_batch", "複数の QR コードを一括エクスポート");
        m.insert("tooltip_export_more", "その他のエクスポートオプション");
        m.insert("tooltip_sidebar_toggle", "サイドバーの切り替え");
        // Feature 3: labels
        m.insert("label_start_date", "開始日");
        m.insert("label_end_date", "終了日");
        m.insert("label_time", "時間:");
        m.insert("label_quiet_zone", "クワイエットゾーン (0-10)");
        m.insert("label_module_gap", "モジュールギャップ (0-0.4)");
        m.insert("label_shadow_offset", "シャドウオフセット (1.0-5.0)");
        m.insert("label_logo_size", "ロゴサイズ (0.1-0.6)");
        m.insert("label_outer_radius", "外側の半径");
        m.insert("label_inner_radius", "内側の半径");
        m.insert("label_logo_border_width", "ロゴの枠線の太さ (0-20)");
        m.insert("label_logo_padding", "ロゴパディング:");
        m.insert("label_frame_width", "フレームの太さ (1-10)");
        m.insert("label_frame_outer_radius", "外側の半径");
        m.insert("label_svg_path", "SVG パス (d 属性):");
        m.insert("label_custom_dot_hint", "ヒント: 座標は 0 から 1 の範囲です。例:\n• 星: M0.5,0 L0.62,0.38 L1,0.38 L0.69,0.62 L0.81,1 L0.5,0.76 L0.19,1 L0.31,0.62 L0,0.38 L0.38,0.38 Z\n• ハート: M0.5,0.9 L0.1,0.5 C0.1,0.1 0.5,0.1 0.5,0.4 C0.5,0.1 0.9,0.1 0.9,0.5 Z");
        m.insert("placeholder_custom_dot", "例: M0,0 L1,0 L1,1 L0,1 Z");
        m.insert("label_print_width", "幅 (cm):");
        m.insert("label_print_height", "高さ (cm):");
        // Feature 6: Content validation
        m.insert("validation_invalid_email", "無効なメールアドレス");
        m.insert(
            "validation_invalid_lat",
            "緯度は -90 から 90 の間である必要があります",
        );
        m.insert(
            "validation_invalid_lon",
            "経度は -180 から 180 の間である必要があります",
        );
        m.insert("validation_invalid_phone", "無効な電話番号");
        // Feature 9: Font selection
        m.insert("label_font", "フォント");
        m.insert("label_font_size", "フォントサイズ");
        // Scan verification
        m.insert("btn_verify_scan", "確認中…");
        m.insert(
            "scan_status_good",
            "スキャン可能 — すべてのチェックに合格しました",
        );
        m.insert("scan_status_limited", "スキャン可能性が制限されています");
        m.insert(
            "scan_status_bad",
            "スキャン不可 — コードをデコードできませんでした",
        );
        m.insert(
            "scan_tooltip",
            "コントラスト、ロゴのカバー率、およびコードがデコード可能かどうかを検証します",
        );
        m.insert(
            "scan_detail_low_contrast",
            "低コントラスト ({:.1}:1、推奨 ≥ 4.5:1)",
        );
        m.insert(
            "scan_detail_logo_ec",
            "誤り訂正レベルに対してロゴが大きすぎます",
        );
        m.insert(
            "scan_detail_large_gap",
            "モジュールギャップが非常に大きいです",
        );
        m.insert(
            "scan_detail_styled_corners",
            "スタイル付きコーナー — スマートフォンのスキャナーはこれらを確実に認識します",
        );

        // Dialog titles for file choosers
        m.insert("dlg_select_logo", "ロゴを選択");
        m.insert("dlg_select_bg", "背景画像を選択");
        m.insert("dlg_save_label_sheet", "ラベルシートを保存");
        m.insert("dlg_select_csv", "CSVファイルを選択");
        m.insert("dlg_select_folder", "フォルダーを選択");

        // Dialog buttons
        m.insert("btn_open", "開く");
        m.insert("btn_select", "選択");

        // File filter names
        m.insert("filter_images", "画像ファイル");
        m.insert("filter_json", "JSONファイル");
        m.insert("filter_csv_txt", "CSV/TXT");

        // Status messages (file operations)
        m.insert("status_style_exported", "スタイルをエクスポートしました");
        m.insert("status_pdf_saved", "PDFを保存しました");
        m.insert("status_pdf_error", "PDFエクスポートエラー");
        m.insert("status_label_sheet_saved", "ラベルシートを保存しました");
        m.insert("status_label_sheet_error", "ラベルシートエクスポートエラー");
        m.insert("status_png_saved", "PNGを保存しました");
        m.insert("status_svg_saved", "SVGを保存しました");
        m.insert("status_gif_saved", "GIFを保存しました");
        m.insert(
            "status_gif_gradient_only",
            "GIFはグラデーション使用時のみ利用可能",
        );
        m.insert(
            "status_batch_exported",
            "{}個のQRコードをエクスポートしました",
        );
        m.insert("status_saved_as", "{}として保存しました");
        m.insert(
            "status_enter_template_name",
            "テンプレート名を入力してください",
        );
        m.insert(
            "status_template_deleted_fmt",
            "テンプレート'{}'を削除しました",
        );
        m.insert(
            "status_render_error",
            "エラー：QRコードのレンダリングに失敗しました",
        );
        m.insert("status_copied", "クリップボードにコピーしました");
        m.insert("status_copied_svg", "SVGをクリップボードにコピーしました");

        // Batch/Label dialog labels
        m.insert("batch_data_label", "QRデータ（1行に1つ）:");
        m.insert(
            "batch_csv_hint",
            "（最初の列をQRデータとして使用、ヘッダー行はスキップ）",
        );
        m.insert("batch_format", "形式:");
        m.insert("batch_csv_filter", "CSV/TXT");
        m.insert("batch_folder_label", "フォルダー:");
        m.insert("batch_folder_selected", "フォルダー: {}");

        // Label sheet dialog
        m.insert("lbl_columns", "列");
        m.insert("lbl_rows", "行");
        m.insert("lbl_margin_mm", "マージン (mm)");
        m.insert("lbl_spacing_mm", "間隔 (mm)");
        m.insert("lbl_sheet_info", "A4ページに複数のQRコードを配置して印刷");
        m.insert("label_sheet_a4_info", "QRコードはA4ページに配置されます。");

        // Dialog buttons for batch/label
        m.insert("btn_cancel", "キャンセル");
        m.insert("btn_export", "エクスポート");
        m.insert("btn_save", "保存");

        // Dialog titles (FileChooserDialog)
        m.insert("dlg_save_pdf", "PDFとして保存");
        m.insert("dlg_save_png", "PNGとして保存");
        m.insert("dlg_save_svg", "SVGとして保存");
        m.insert("dlg_save_gif", "GIFとして保存");
        m.insert("dlg_import_style", "スタイルをインポート");
        m.insert("dlg_export_style", "スタイルをエクスポート");
        m.insert("dlg_batch_export", "一括エクスポート");
        m.insert("dlg_label_sheet", "ラベルシート");

        m
    }

    fn korean() -> HashMap<&'static str, &'static str> {
        let mut m = HashMap::new();
        // Tab labels
        m.insert("tab_content", "콘텐츠");
        m.insert("tab_style", "스타일");
        m.insert("btn_export_more", "내보내기 ▾");
        // Section headers
        // Content types
        // Buttons
        m.insert("btn_copy", "복사");
        m.insert("btn_save_png", "PNG로 저장");
        m.insert("btn_save_svg", "SVG로 저장");
        m.insert("btn_copy_svg", "SVG 복사");
        m.insert("btn_save_gif", "GIF로 저장");
        m.insert("btn_save_pdf", "PDF로 저장");
        m.insert("btn_label_sheet", "라벨 시트...");
        m.insert("btn_batch", "일괄 내보내기...");
        m.insert("btn_select_image", "이미지 선택");
        m.insert("btn_remove", "제거");
        m.insert("btn_clear_all", "모두 지우기");
        m.insert("tooltip_clear_all", "모든 입력 필드 초기화");
        // Labels
        // WiFi fields
        m.insert("wifi_password", "비밀번호");
        // vCard fields
        m.insert("vcard_name", "이름");
        m.insert("vcard_phone", "전화번호");
        m.insert("vcard_email", "이메일");
        m.insert("vcard_org", "소속 기관");
        m.insert("vcard_url", "웹사이트 URL");
        // Calendar fields
        m.insert("cal_title", "제목");
        m.insert("cal_location", "위치");
        // GPS fields
        m.insert("gps_lat", "위도");
        m.insert("gps_lon", "경도");
        m.insert("gps_search", "위치 검색…");

        // SMS fields
        m.insert("sms_phone", "전화번호");
        m.insert("sms_message", "메시지");
        // Frame styles
        // Gradient directions
        // Status messages
        m.insert(
            "status_template_saved_full",
            "템플릿 '{}'을(를) 저장했습니다 (스타일 + 콘텐츠)",
        );
        m.insert(
            "status_template_saved_style",
            "템플릿 '{}'을(를) 저장했습니다 (스타일만)",
        );
        m.insert(
            "status_template_loaded_full",
            "템플릿 '{}'을(를) 불러왔습니다 (스타일 + 콘텐츠)",
        );
        m.insert(
            "status_template_loaded_style",
            "템플릿 '{}'을(를) 불러왔습니다 (스타일만)",
        );
        // Dialog titles
        // Batch dialog
        // Label sheet dialog
        // EC levels
        // Dot styles
        // Corner square styles
        // Logo shapes
        // Harmonies
        m.insert("harmony_title", "색상 조화 (배경으로)");
        m.insert("color_fg", "전경");
        m.insert("color_bg", "배경");
        m.insert("color_corner", "모서리 색상");
        m.insert("color_gradient", "그라데이션 대상");
        m.insert(
            "dnd_logo_imported",
            "드래그 앤 드롭으로 로고를 가져왔습니다",
        );
        // QR Info
        m.insert("qrinfo_version", "버전");
        m.insert("qrinfo_modules", "모듈");
        m.insert("qrinfo_ec", "오류 정정");
        m.insert("qrinfo_capacity", "데이터 용량");
        m.insert("qrinfo_data_loss", "데이터 손실");
        m.insert("qrinfo_bytes", "바이트");
        m.insert("qrinfo_scan_dist", "스캔 거리");
        m.insert("qrinfo_at_dpi", "300 DPI 기준");
        m.insert("qrinfo_no_data", "데이터 없음");
        m.insert("qrinfo_data_too_long", "QR 코드에 너무 긴 데이터입니다");
        // Expander sections (as used in build_ui)
        m.insert("exp_content", "콘텐츠");
        m.insert("exp_presets", "스타일 프리셋");
        m.insert("exp_templates", "템플릿");
        m.insert("check_save_content", "콘텐츠 포함");
        m.insert(
            "tooltip_save_content",
            "활성화하면 현재 콘텐츠(텍스트, WiFi, vCard 등)가 스타일과 함께 저장됩니다",
        );
        m.insert("exp_pattern", "패턴 및 모서리");
        m.insert("exp_colors", "색상");
        m.insert("exp_settings", "설정");
        m.insert("exp_qr_info", "QR 정보");
        m.insert("exp_advanced", "고급");
        m.insert("exp_logo", "로고 (중앙)");
        m.insert("exp_outer_text", "QR 코드 주변 텍스트");
        m.insert("exp_frame", "프레임");
        m.insert("exp_bg", "배경");
        m.insert("exp_import", "가져오기/내보내기");
        // Preview
        m.insert("preview_label", "실시간 미리보기");
        // Generate button
        // Dropdown items as used in build_ui
        m.insert("dd_content_text", "텍스트");
        m.insert("dd_content_wifi", "WiFi");
        m.insert("dd_content_vcard", "vCard/연락처");
        m.insert("dd_content_calendar", "일정 이벤트");
        m.insert("dd_content_gps", "GPS 위치");
        m.insert("dd_content_sms", "SMS");
        m.insert("dd_dot_rounded", "둥근");
        m.insert("dd_dot_square", "사각형");
        m.insert("dd_dot_dots", "점");
        m.insert("dd_dot_diamond", "다이아몬드");
        m.insert("dd_dot_custom", "사용자 정의");
        m.insert("dd_corner_sq_rounded", "둥근");
        m.insert("dd_corner_sq_square", "사각형");
        m.insert("dd_corner_sq_dot", "점");
        m.insert("dd_corner_sq_circle", "원형");
        m.insert("dd_corner_dot_dot", "점");
        m.insert("dd_corner_dot_square", "사각형");
        m.insert("dd_corner_dot_circle", "원형");
        m.insert("dd_corner_dot_rounded", "둥근");
        m.insert("dd_wifi_wpa", "WPA");
        m.insert("dd_wifi_wep", "WEP");
        m.insert("dd_wifi_none", "없음");
        m.insert("dd_ec_medium", "보통 (M)");
        m.insert("dd_ec_low", "낮음 (L)");
        m.insert("dd_ec_quartile", "사분위 (Q)");
        m.insert("dd_ec_high", "높음 (H)");
        m.insert("dd_module_medium", "보통 (32px)");
        m.insert("dd_module_small", "작게 (16px)");
        m.insert("dd_module_large", "크게 (64px)");
        m.insert("dd_module_print", "인쇄용 (128px)");
        m.insert("dd_grad_horizontal", "가로");
        m.insert("dd_grad_vertical", "세로");
        m.insert("dd_grad_diagonal", "대각선");
        m.insert("dd_grad_radial", "방사형");
        m.insert("dd_logo_circle", "원형");
        m.insert("dd_logo_rectangle", "사각형");
        m.insert("dd_logo_rounded", "둥근");
        m.insert("dd_frame_none", "없음");
        m.insert("dd_frame_simple", "단순");
        m.insert("dd_frame_rounded", "둥근");
        m.insert("dd_frame_banner", "배너");
        m.insert("dd_palette_custom", "사용자 정의");
        m.insert("dd_palette_classic", "클래식");
        m.insert("dd_palette_ocean", "오션");
        m.insert("dd_palette_sunset", "선셋");
        m.insert("dd_palette_forest", "포레스트");
        m.insert("dd_palette_lavender", "라벤더");
        m.insert("dd_palette_fire", "파이어");
        m.insert("dd_palette_aurora", "오로라");
        m.insert("dd_palette_pastel", "파스텔");
        m.insert("dd_palette_neon", "네온");
        m.insert("dd_preset_custom", "사용자 정의");
        m.insert("dd_preset_classic", "클래식");
        m.insert("dd_preset_rounded", "둥근");
        m.insert("dd_preset_dots", "점");
        m.insert("dd_preset_diamond", "다이아몬드");
        m.insert("dd_preset_minimal", "미니멀");
        m.insert("dd_preset_retro", "레트로");
        // Misc
        m.insert("btn_bg_select", "배경 이미지 선택");
        m.insert("btn_export_style_short", "스타일 내보내기");
        m.insert("btn_import_style_short", "스타일 가져오기");
        m.insert("btn_print_calc", "인쇄 크기 계산기");
        m.insert("dlg_print_calc", "인쇄 크기 계산기");
        m.insert("btn_apply", "적용");
        m.insert("print_calc_result", "{} x {} 픽셀 (권장 모듈 크기: ~{}px)");
        m.insert("label_dpi", "DPI:");
        m.insert("check_transparent_bg", "투명 배경");
        m.insert("check_gradient", "그라데이션 사용");
        m.insert("check_shadow", "그림자 사용");
        m.insert("check_logo_vectorize", "로고 벡터화");
        m.insert("check_logo_bg_transparent", "투명 배경");
        m.insert("check_logo_clear_area", "영역 비우기");
        m.insert("check_radius_sync", "반경 동기화");
        m.insert("placeholder_template_name", "템플릿 이름...");
        m.insert("placeholder_top_text", "상단 텍스트");
        m.insert("placeholder_bottom_text", "하단 텍스트");
        m.insert("tooltip_undo", "실행 취소 (Ctrl+Z)");
        m.insert("tooltip_redo", "다시 실행 (Ctrl+Y)");
        // Feature 2: Transparency Checkerboard
        m.insert("tooltip_preview_bg", "미리보기 배경 전환");
        // Feature 3: i18n for hardcoded strings - tooltips
        m.insert("tooltip_content_type", "콘텐츠 유형 선택");
        m.insert("tooltip_qr_content", "QR 코드 콘텐츠");
        m.insert("tooltip_wifi_ssid", "WiFi 네트워크 이름");
        m.insert("tooltip_wifi_password", "WiFi 비밀번호");
        m.insert("tooltip_wifi_encryption", "암호화");
        m.insert("tooltip_vcard_name", "연락처 이름");
        m.insert("tooltip_vcard_phone", "전화번호");
        m.insert("tooltip_vcard_email", "이메일 주소");
        m.insert("tooltip_vcard_org", "소속 기관/회사");
        m.insert("tooltip_vcard_url", "웹사이트 URL");
        m.insert("tooltip_cal_title", "이벤트 제목");
        m.insert("tooltip_cal_hour", "시");
        m.insert("tooltip_cal_minute", "분");
        m.insert("tooltip_cal_location", "이벤트 위치");
        m.insert("tooltip_gps_lat", "위도");
        m.insert("tooltip_gps_lon", "경도");
        m.insert("tooltip_gps_search", "위치를 입력하고 Enter를 누르세요");

        m.insert("tooltip_sms_phone", "SMS 수신 번호");
        m.insert("tooltip_sms_message", "SMS 메시지 텍스트");
        m.insert("tooltip_preset_select", "스타일 프리셋 선택");
        m.insert(
            "tooltip_template_save",
            "현재 스타일 + 콘텐츠를 템플릿으로 저장",
        );
        m.insert(
            "tooltip_template_load",
            "저장된 템플릿 불러오기 (스타일 + 콘텐츠)",
        );
        m.insert("tooltip_template_delete", "선택한 템플릿 삭제");
        m.insert("tooltip_dot_style", "데이터 도트 스타일");
        m.insert("tooltip_corner_sq_style", "모서리 사각형 스타일");
        m.insert("tooltip_corner_dot_style", "모서리 도트 스타일");
        m.insert(
            "tooltip_custom_dot_svg",
            "사용자 정의 도트 모양의 SVG 경로 데이터 (좌표 0..1)",
        );
        m.insert("tooltip_transparent_bg", "배경을 투명하게 설정");
        m.insert("tooltip_gradient_enable", "그라데이션 사용");
        m.insert("tooltip_gradient_dir", "그라데이션 방향");
        m.insert("tooltip_palette", "색상 팔레트 선택");
        m.insert("tooltip_ec_level", "오류 정정 수준");
        m.insert("tooltip_module_size", "모듈 크기");
        m.insert("tooltip_quiet_zone", "QR 코드 주변 여백 영역");
        m.insert("tooltip_module_gap", "모듈 사이 간격");
        m.insert("tooltip_shadow_enable", "그림자 효과 사용");
        m.insert("tooltip_shadow_offset", "그림자 오프셋");
        m.insert("tooltip_logo_select", "로고 이미지 선택");
        m.insert("tooltip_logo_remove", "로고 제거");
        m.insert("tooltip_logo_size", "QR 코드 대비 로고 크기");
        m.insert("tooltip_logo_shape", "로고 모양");
        m.insert("tooltip_logo_radius_sync", "안쪽/바깥쪽 반경 연결");
        m.insert("tooltip_logo_color", "로고 착색 (알파 = 강도)");
        m.insert("tooltip_logo_border_width", "로고 주변 테두리 너비");
        m.insert("tooltip_logo_border_color", "로고 테두리 색상");
        m.insert(
            "tooltip_logo_vectorize",
            "래스터 로고(PNG/JPG)를 벡터 경로로 변환",
        );
        m.insert(
            "tooltip_logo_vectorize_bg",
            "벡터화된 로고 배경 색상: 알파=0이면 배경 제거, 알파>0이면 이 색상으로 교체",
        );
        m.insert(
            "tooltip_logo_bg_transparent",
            "QR 배경색과 관계없이 로고 영역 배경을 투명하게 만들기",
        );
        m.insert(
            "tooltip_logo_clear_area",
            "로고 주변 QR 모듈 재배치 (오류 정정 필요)",
        );
        m.insert(
            "tooltip_logo_padding",
            "로고 주변 추가 여백 (모듈 단위, 더 나은 재배치를 위해)",
        );
        m.insert(
            "tooltip_outer_radius",
            "바깥쪽 프레임 반경 (0 = 사각형, 0.5 = 최대 둥글게)",
        );
        m.insert(
            "tooltip_inner_radius",
            "안쪽 프레임 반경 (0 = 사각형, 0.5 = 최대 둥글게)",
        );
        m.insert("tooltip_top_text", "QR 코드 위 텍스트");
        m.insert("tooltip_bottom_text", "QR 코드 아래 텍스트");
        m.insert("tooltip_text_color", "텍스트 색상");
        m.insert("tooltip_frame_style", "프레임 스타일");
        m.insert("tooltip_frame_color", "프레임 색상");
        m.insert("tooltip_frame_width", "모듈 단위 프레임 너비");
        m.insert(
            "tooltip_frame_outer_radius",
            "바깥쪽 프레임 반경 (0 = 사각형, 0.5 = 최대 둥글게)",
        );
        m.insert("tooltip_bg_select", "배경 이미지 선택");
        m.insert("tooltip_bg_remove", "배경 이미지 제거");
        m.insert(
            "tooltip_export_style",
            "현재 스타일 설정을 JSON으로 내보내기",
        );
        m.insert("tooltip_import_style", "JSON에서 스타일 설정 가져오기");
        m.insert("tooltip_print_calc", "인쇄용 픽셀 크기 계산");
        m.insert("tooltip_copy_png", "QR 코드를 클립보드에 복사");
        m.insert("tooltip_save_png", "QR 코드를 PNG로 저장");
        m.insert("tooltip_copy_svg", "QR 코드를 SVG로 클립보드에 복사");
        m.insert("tooltip_save_svg", "QR 코드를 SVG로 저장");
        m.insert("tooltip_save_gif", "애니메이션 QR 코드를 GIF로 저장");
        m.insert(
            "tooltip_save_pdf",
            "QR 코드를 PDF로 내보내기 (A4, 인쇄 가능)",
        );
        m.insert(
            "tooltip_label_sheet",
            "A4 용지에 여러 QR 코드를 라벨로 배치",
        );
        m.insert("tooltip_batch", "여러 QR 코드를 한 번에 내보내기");
        m.insert("tooltip_export_more", "더 많은 내보내기 옵션");
        m.insert("tooltip_sidebar_toggle", "사이드바 전환");
        // Feature 3: labels
        m.insert("label_start_date", "시작 날짜");
        m.insert("label_end_date", "종료 날짜");
        m.insert("label_time", "시간:");
        m.insert("label_quiet_zone", "여백 영역 (0-10)");
        m.insert("label_module_gap", "모듈 간격 (0-0.4)");
        m.insert("label_shadow_offset", "그림자 오프셋 (1.0-5.0)");
        m.insert("label_logo_size", "로고 크기 (0.1-0.6)");
        m.insert("label_outer_radius", "바깥쪽 반경");
        m.insert("label_inner_radius", "안쪽 반경");
        m.insert("label_logo_border_width", "로고 테두리 너비 (0-20)");
        m.insert("label_logo_padding", "로고 여백:");
        m.insert("label_frame_width", "프레임 너비 (1-10)");
        m.insert("label_frame_outer_radius", "바깥쪽 반경");
        m.insert("label_svg_path", "SVG 경로 (d 속성):");
        m.insert(
            "label_custom_dot_hint",
            "팁: 좌표 범위는 0에서 1입니다. 예시:
• 별: M0.5,0 L0.62,0.38 L1,0.38 L0.69,0.62 L0.81,1 L0.5,0.76 L0.19,1 L0.31,0.62 L0,0.38 L0.38,0.38 Z
• 하트: M0.5,0.9 L0.1,0.5 C0.1,0.1 0.5,0.1 0.5,0.4 C0.5,0.1 0.9,0.1 0.9,0.5 Z",
        );
        m.insert("placeholder_custom_dot", "예: M0,0 L1,0 L1,1 L0,1 Z");
        m.insert("label_print_width", "너비 (cm):");
        m.insert("label_print_height", "높이 (cm):");
        // Feature 6: Content validation
        m.insert("validation_invalid_email", "잘못된 이메일 주소");
        m.insert(
            "validation_invalid_lat",
            "위도는 -90에서 90 사이여야 합니다",
        );
        m.insert(
            "validation_invalid_lon",
            "경도는 -180에서 180 사이여야 합니다",
        );
        m.insert("validation_invalid_phone", "잘못된 전화번호");
        // Feature 9: Font selection
        m.insert("label_font", "글꼴");
        m.insert("label_font_size", "글꼴 크기");
        // Scan verification
        m.insert("btn_verify_scan", "확인 중…");
        m.insert("scan_status_good", "스캔 가능 — 모든 검사 통과");
        m.insert("scan_status_limited", "스캔 가능성 제한");
        m.insert("scan_status_bad", "스캔 불가 — 코드를 디코딩할 수 없습니다");
        m.insert(
            "scan_tooltip",
            "대비, 로고 적용 범위 및 코드 디코딩 가능 여부를 확인합니다",
        );
        m.insert(
            "scan_detail_low_contrast",
            "낮은 대비 ({:.1}:1, 권장 ≥ 4.5:1)",
        );
        m.insert(
            "scan_detail_logo_ec",
            "로고가 오류 정정 수준에 비해 너무 큽니다",
        );
        m.insert("scan_detail_large_gap", "모듈 간격이 매우 큽니다");
        m.insert(
            "scan_detail_styled_corners",
            "스타일이 적용된 모서리 — 스마트폰 스캐너에서 안정적으로 인식됩니다",
        );

        // Dialog titles for file choosers
        m.insert("dlg_select_logo", "로고 선택");
        m.insert("dlg_select_bg", "배경 이미지 선택");
        m.insert("dlg_save_label_sheet", "라벨 시트 저장");
        m.insert("dlg_select_csv", "CSV 파일 선택");
        m.insert("dlg_select_folder", "폴더 선택");

        // Dialog buttons
        m.insert("btn_open", "열기");
        m.insert("btn_select", "선택");

        // File filter names
        m.insert("filter_images", "이미지 파일");
        m.insert("filter_json", "JSON 파일");
        m.insert("filter_csv_txt", "CSV/TXT");

        // Status messages (file operations)
        m.insert("status_style_exported", "스타일 내보내기 완료");
        m.insert("status_pdf_saved", "PDF 저장됨");
        m.insert("status_pdf_error", "PDF 내보내기 오류");
        m.insert("status_label_sheet_saved", "라벨 시트 저장됨");
        m.insert("status_label_sheet_error", "라벨 시트 내보내기 오류");
        m.insert("status_png_saved", "PNG 저장됨");
        m.insert("status_svg_saved", "SVG 저장됨");
        m.insert("status_gif_saved", "GIF 저장됨");
        m.insert(
            "status_gif_gradient_only",
            "그라데이션 사용 시에만 GIF 사용 가능",
        );
        m.insert("status_batch_exported", "{}개 QR 코드 내보내기 완료");
        m.insert("status_saved_as", "{}(으)로 저장됨");
        m.insert("status_enter_template_name", "템플릿 이름을 입력하세요");
        m.insert("status_template_deleted_fmt", "템플릿 '{}' 삭제됨");
        m.insert(
            "status_render_error",
            "오류: QR 코드를 렌더링할 수 없습니다",
        );
        m.insert("status_copied", "클립보드에 복사됨");
        m.insert("status_copied_svg", "SVG가 클립보드에 복사됨");

        // Batch/Label dialog labels
        m.insert("batch_data_label", "QR 데이터 (한 줄에 하나):");
        m.insert(
            "batch_csv_hint",
            "(첫 번째 열이 QR 데이터로 사용, 헤더 행 건너뜀)",
        );
        m.insert("batch_format", "형식:");
        m.insert("batch_csv_filter", "CSV/TXT");
        m.insert("batch_folder_label", "폴더:");
        m.insert("batch_folder_selected", "폴더: {}");

        // Label sheet dialog
        m.insert("lbl_columns", "열");
        m.insert("lbl_rows", "행");
        m.insert("lbl_margin_mm", "여백 (mm)");
        m.insert("lbl_spacing_mm", "간격 (mm)");
        m.insert("lbl_sheet_info", "A4 페이지에 여러 QR 코드 배치하여 인쇄");
        m.insert("label_sheet_a4_info", "QR 코드가 A4 페이지에 배치됩니다.");

        // Dialog buttons for batch/label
        m.insert("btn_cancel", "취소");
        m.insert("btn_export", "내보내기");
        m.insert("btn_save", "저장");

        // Dialog titles (FileChooserDialog)
        m.insert("dlg_save_pdf", "PDF로 저장");
        m.insert("dlg_save_png", "PNG로 저장");
        m.insert("dlg_save_svg", "SVG로 저장");
        m.insert("dlg_save_gif", "GIF로 저장");
        m.insert("dlg_import_style", "스타일 가져오기");
        m.insert("dlg_export_style", "스타일 내보내기");
        m.insert("dlg_batch_export", "일괄 내보내기");
        m.insert("dlg_label_sheet", "라벨 시트");

        m
    }

    fn chinese_cn() -> HashMap<&'static str, &'static str> {
        let mut m = HashMap::new();
        // Tab labels
        m.insert("tab_content", "内容");
        m.insert("tab_style", "样式");
        m.insert("btn_export_more", "导出 ▾");
        // Section headers
        // Content types
        // Buttons
        m.insert("btn_copy", "复制");
        m.insert("btn_save_png", "保存为 PNG");
        m.insert("btn_save_svg", "保存为 SVG");
        m.insert("btn_copy_svg", "复制 SVG");
        m.insert("btn_save_gif", "保存为 GIF");
        m.insert("btn_save_pdf", "保存为 PDF");
        m.insert("btn_label_sheet", "标签页…");
        m.insert("btn_batch", "批量导出…");
        m.insert("btn_select_image", "选择图片");
        m.insert("btn_remove", "移除");
        m.insert("btn_clear_all", "全部清除");
        m.insert("tooltip_clear_all", "重置所有输入字段");
        // Labels
        // WiFi fields
        m.insert("wifi_password", "密码");
        // vCard fields
        m.insert("vcard_name", "姓名");
        m.insert("vcard_phone", "电话号码");
        m.insert("vcard_email", "邮箱");
        m.insert("vcard_org", "组织");
        m.insert("vcard_url", "网站 URL");
        // Calendar fields
        m.insert("cal_title", "标题");
        m.insert("cal_location", "地点");
        // GPS fields
        m.insert("gps_lat", "纬度");
        m.insert("gps_lon", "经度");
        m.insert("gps_search", "搜索位置…");

        // SMS fields
        m.insert("sms_phone", "电话号码");
        m.insert("sms_message", "消息");
        // Frame styles
        // Gradient directions
        // Status messages
        m.insert(
            "status_template_saved_full",
            "模板 '{}' 已保存 (样式 + 内容)",
        );
        m.insert("status_template_saved_style", "模板 '{}' 已保存 (仅样式)");
        m.insert(
            "status_template_loaded_full",
            "模板 '{}' 已加载 (样式 + 内容)",
        );
        m.insert("status_template_loaded_style", "模板 '{}' 已加载 (仅样式)");
        // Dialog titles
        // Batch dialog
        // Label sheet dialog
        // EC levels
        // Dot styles
        // Corner square styles
        // Logo shapes
        // Harmonies
        m.insert("harmony_title", "色彩和谐（作为背景）");
        m.insert("color_fg", "前景色");
        m.insert("color_bg", "背景色");
        m.insert("color_corner", "定位角颜色");
        m.insert("color_gradient", "渐变目标色");
        m.insert("dnd_logo_imported", "标志已通过拖放导入");
        // QR Info
        m.insert("qrinfo_version", "版本");
        m.insert("qrinfo_modules", "模块");
        m.insert("qrinfo_ec", "纠错等级");
        m.insert("qrinfo_capacity", "数据容量");
        m.insert("qrinfo_data_loss", "数据丢失");
        m.insert("qrinfo_bytes", "字节");
        m.insert("qrinfo_scan_dist", "扫描距离");
        m.insert("qrinfo_at_dpi", "在 300 DPI 下");
        m.insert("qrinfo_no_data", "无数据");
        m.insert("qrinfo_data_too_long", "数据过长，无法生成 QR 码");
        // Expander sections (as used in build_ui)
        m.insert("exp_content", "内容");
        m.insert("exp_presets", "样式预设");
        m.insert("exp_templates", "模板");
        m.insert("check_save_content", "包含内容");
        m.insert(
            "tooltip_save_content",
            "启用后，当前内容（文本、WiFi、vCard 等）将与样式一起保存",
        );
        m.insert("exp_pattern", "图案与定位角");
        m.insert("exp_colors", "颜色");
        m.insert("exp_settings", "设置");
        m.insert("exp_qr_info", "QR 信息");
        m.insert("exp_advanced", "高级");
        m.insert("exp_logo", "标志（居中）");
        m.insert("exp_outer_text", "QR 码周围文字");
        m.insert("exp_frame", "边框");
        m.insert("exp_bg", "背景");
        m.insert("exp_import", "导入/导出");
        // Preview
        m.insert("preview_label", "实时预览");
        // Generate button
        // Dropdown items as used in build_ui
        m.insert("dd_content_text", "文本");
        m.insert("dd_content_wifi", "WiFi");
        m.insert("dd_content_vcard", "vCard/联系人");
        m.insert("dd_content_calendar", "日历事件");
        m.insert("dd_content_gps", "GPS 位置");
        m.insert("dd_content_sms", "短信");
        m.insert("dd_dot_rounded", "圆角");
        m.insert("dd_dot_square", "方形");
        m.insert("dd_dot_dots", "圆点");
        m.insert("dd_dot_diamond", "菱形");
        m.insert("dd_dot_custom", "自定义");
        m.insert("dd_corner_sq_rounded", "圆角");
        m.insert("dd_corner_sq_square", "方形");
        m.insert("dd_corner_sq_dot", "点");
        m.insert("dd_corner_sq_circle", "圆形");
        m.insert("dd_corner_dot_dot", "点");
        m.insert("dd_corner_dot_square", "方形");
        m.insert("dd_corner_dot_circle", "圆形");
        m.insert("dd_corner_dot_rounded", "圆角");
        m.insert("dd_wifi_wpa", "WPA");
        m.insert("dd_wifi_wep", "WEP");
        m.insert("dd_wifi_none", "无");
        m.insert("dd_ec_medium", "中等 (M)");
        m.insert("dd_ec_low", "低 (L)");
        m.insert("dd_ec_quartile", "四分之一 (Q)");
        m.insert("dd_ec_high", "高 (H)");
        m.insert("dd_module_medium", "中 (32px)");
        m.insert("dd_module_small", "小 (16px)");
        m.insert("dd_module_large", "大 (64px)");
        m.insert("dd_module_print", "打印 (128px)");
        m.insert("dd_grad_horizontal", "水平");
        m.insert("dd_grad_vertical", "垂直");
        m.insert("dd_grad_diagonal", "对角线");
        m.insert("dd_grad_radial", "径向");
        m.insert("dd_logo_circle", "圆形");
        m.insert("dd_logo_rectangle", "矩形");
        m.insert("dd_logo_rounded", "圆角");
        m.insert("dd_frame_none", "无");
        m.insert("dd_frame_simple", "简单");
        m.insert("dd_frame_rounded", "圆角");
        m.insert("dd_frame_banner", "横幅");
        m.insert("dd_palette_custom", "自定义");
        m.insert("dd_palette_classic", "经典");
        m.insert("dd_palette_ocean", "海洋");
        m.insert("dd_palette_sunset", "日落");
        m.insert("dd_palette_forest", "森林");
        m.insert("dd_palette_lavender", "薰衣草");
        m.insert("dd_palette_fire", "火焰");
        m.insert("dd_palette_aurora", "极光");
        m.insert("dd_palette_pastel", "粉彩");
        m.insert("dd_palette_neon", "霓虹");
        m.insert("dd_preset_custom", "自定义");
        m.insert("dd_preset_classic", "经典");
        m.insert("dd_preset_rounded", "圆角");
        m.insert("dd_preset_dots", "圆点");
        m.insert("dd_preset_diamond", "菱形");
        m.insert("dd_preset_minimal", "极简");
        m.insert("dd_preset_retro", "复古");
        // Misc
        m.insert("btn_bg_select", "选择背景图片");
        m.insert("btn_export_style_short", "导出样式");
        m.insert("btn_import_style_short", "导入样式");
        m.insert("btn_print_calc", "打印尺寸计算器");
        m.insert("dlg_print_calc", "打印尺寸计算器");
        m.insert("btn_apply", "应用");
        m.insert("print_calc_result", "{} x {} 像素（推荐模块大小：~{}px）");
        m.insert("label_dpi", "DPI：");
        m.insert("check_transparent_bg", "透明背景");
        m.insert("check_gradient", "启用渐变");
        m.insert("check_shadow", "启用阴影");
        m.insert("check_logo_vectorize", "矢量化标志");
        m.insert("check_logo_bg_transparent", "透明背景");
        m.insert("check_logo_clear_area", "清除区域");
        m.insert("check_radius_sync", "同步圆角");
        m.insert("placeholder_template_name", "模板名称…");
        m.insert("placeholder_top_text", "顶部文字");
        m.insert("placeholder_bottom_text", "底部文字");
        m.insert("tooltip_undo", "撤销 (Ctrl+Z)");
        m.insert("tooltip_redo", "重做 (Ctrl+Y)");
        // Feature 2: Transparency Checkerboard
        m.insert("tooltip_preview_bg", "切换预览背景");
        // Feature 3: i18n for hardcoded strings - tooltips
        m.insert("tooltip_content_type", "选择内容类型");
        m.insert("tooltip_qr_content", "QR 码内容");
        m.insert("tooltip_wifi_ssid", "WiFi 网络名称");
        m.insert("tooltip_wifi_password", "WiFi 密码");
        m.insert("tooltip_wifi_encryption", "加密方式");
        m.insert("tooltip_vcard_name", "联系人姓名");
        m.insert("tooltip_vcard_phone", "电话号码");
        m.insert("tooltip_vcard_email", "邮箱地址");
        m.insert("tooltip_vcard_org", "组织/公司");
        m.insert("tooltip_vcard_url", "网站 URL");
        m.insert("tooltip_cal_title", "事件标题");
        m.insert("tooltip_cal_hour", "小时");
        m.insert("tooltip_cal_minute", "分钟");
        m.insert("tooltip_cal_location", "事件地点");
        m.insert("tooltip_gps_lat", "纬度");
        m.insert("tooltip_gps_lon", "经度");
        m.insert("tooltip_gps_search", "输入位置后按回车键");

        m.insert("tooltip_sms_phone", "短信接收号码");
        m.insert("tooltip_sms_message", "短信内容");
        m.insert("tooltip_preset_select", "选择样式预设");
        m.insert("tooltip_template_save", "将当前样式和内容保存为模板");
        m.insert("tooltip_template_load", "加载已保存的模板（样式和内容）");
        m.insert("tooltip_template_delete", "删除选中的模板");
        m.insert("tooltip_dot_style", "数据点样式");
        m.insert("tooltip_corner_sq_style", "定位角样式");
        m.insert("tooltip_corner_dot_style", "定位点样式");
        m.insert(
            "tooltip_custom_dot_svg",
            "自定义点形状的 SVG 路径数据（坐标范围 0..1）",
        );
        m.insert("tooltip_transparent_bg", "使背景透明");
        m.insert("tooltip_gradient_enable", "启用渐变");
        m.insert("tooltip_gradient_dir", "渐变方向");
        m.insert("tooltip_palette", "选择调色板");
        m.insert("tooltip_ec_level", "纠错等级");
        m.insert("tooltip_module_size", "模块大小");
        m.insert("tooltip_quiet_zone", "QR 码周围的静区");
        m.insert("tooltip_module_gap", "模块之间的间距");
        m.insert("tooltip_shadow_enable", "启用阴影效果");
        m.insert("tooltip_shadow_offset", "阴影偏移");
        m.insert("tooltip_logo_select", "选择标志图片");
        m.insert("tooltip_logo_remove", "移除标志");
        m.insert("tooltip_logo_size", "标志相对于 QR 码的大小");
        m.insert("tooltip_logo_shape", "标志形状");
        m.insert("tooltip_logo_radius_sync", "关联内外圆角");
        m.insert("tooltip_logo_color", "标志着色（Alpha 值 = 着色强度）");
        m.insert("tooltip_logo_border_width", "标志周围的边框宽度");
        m.insert("tooltip_logo_border_color", "标志边框颜色");
        m.insert(
            "tooltip_logo_vectorize",
            "将位图标志（PNG/JPG）转换为矢量路径",
        );
        m.insert(
            "tooltip_logo_vectorize_bg",
            "矢量化标志背景色：Alpha=0 移除背景，Alpha>0 替换为此颜色",
        );
        m.insert(
            "tooltip_logo_bg_transparent",
            "使标志区域背景透明，不受QR背景色影响",
        );
        m.insert(
            "tooltip_logo_clear_area",
            "在标志周围重新排列 QR 模块（需要纠错支持）",
        );
        m.insert(
            "tooltip_logo_padding",
            "标志周围额外边距（以模块为单位），改善重新排列效果",
        );
        m.insert(
            "tooltip_outer_radius",
            "外框圆角（0 = 方形，0.5 = 最大圆角）",
        );
        m.insert(
            "tooltip_inner_radius",
            "内框圆角（0 = 方形，0.5 = 最大圆角）",
        );
        m.insert("tooltip_top_text", "QR 码上方文字");
        m.insert("tooltip_bottom_text", "QR 码下方文字");
        m.insert("tooltip_text_color", "文字颜色");
        m.insert("tooltip_frame_style", "边框样式");
        m.insert("tooltip_frame_color", "边框颜色");
        m.insert("tooltip_frame_width", "边框宽度（以模块为单位）");
        m.insert(
            "tooltip_frame_outer_radius",
            "外框圆角（0 = 方形，0.5 = 最大圆角）",
        );
        m.insert("tooltip_bg_select", "选择背景图片");
        m.insert("tooltip_bg_remove", "移除背景图片");
        m.insert("tooltip_export_style", "将当前样式设置导出为 JSON");
        m.insert("tooltip_import_style", "从 JSON 导入样式设置");
        m.insert("tooltip_print_calc", "计算打印所需的像素大小");
        m.insert("tooltip_copy_png", "复制 QR 码到剪贴板");
        m.insert("tooltip_save_png", "保存 QR 码为 PNG");
        m.insert("tooltip_copy_svg", "复制 QR 码为 SVG 到剪贴板");
        m.insert("tooltip_save_svg", "保存 QR 码为 SVG");
        m.insert("tooltip_save_gif", "保存动态 QR 码为 GIF");
        m.insert("tooltip_save_pdf", "导出 QR 码为 PDF（A4，适用于打印）");
        m.insert("tooltip_label_sheet", "在 A4 纸上排列多个 QR 码作为标签");
        m.insert("tooltip_batch", "一次导出多个 QR 码");
        m.insert("tooltip_export_more", "更多导出选项");
        m.insert("tooltip_sidebar_toggle", "切换侧边栏");
        // Feature 3: labels
        m.insert("label_start_date", "开始日期");
        m.insert("label_end_date", "结束日期");
        m.insert("label_time", "时间：");
        m.insert("label_quiet_zone", "静区 (0-10)");
        m.insert("label_module_gap", "模块间距 (0-0.4)");
        m.insert("label_shadow_offset", "阴影偏移 (1.0-5.0)");
        m.insert("label_logo_size", "标志大小 (0.1-0.6)");
        m.insert("label_outer_radius", "外圆角");
        m.insert("label_inner_radius", "内圆角");
        m.insert("label_logo_border_width", "标志边框宽度 (0-20)");
        m.insert("label_logo_padding", "标志边距：");
        m.insert("label_frame_width", "边框宽度 (1-10)");
        m.insert("label_frame_outer_radius", "外圆角");
        m.insert("label_svg_path", "SVG 路径（d 属性）：");
        m.insert("label_custom_dot_hint", "提示：坐标范围为 0 到 1。示例：\n• 星形：M0.5,0 L0.62,0.38 L1,0.38 L0.69,0.62 L0.81,1 L0.5,0.76 L0.19,1 L0.31,0.62 L0,0.38 L0.38,0.38 Z\n• 心形：M0.5,0.9 L0.1,0.5 C0.1,0.1 0.5,0.1 0.5,0.4 C0.5,0.1 0.9,0.1 0.9,0.5 Z");
        m.insert("placeholder_custom_dot", "例如 M0,0 L1,0 L1,1 L0,1 Z");
        m.insert("label_print_width", "宽度 (cm)：");
        m.insert("label_print_height", "高度 (cm)：");
        // Feature 6: Content validation
        m.insert("validation_invalid_email", "无效的邮箱地址");
        m.insert("validation_invalid_lat", "纬度必须在 -90 到 90 之间");
        m.insert("validation_invalid_lon", "经度必须在 -180 到 180 之间");
        m.insert("validation_invalid_phone", "无效的电话号码");
        // Feature 9: Font selection
        m.insert("label_font", "字体");
        m.insert("label_font_size", "字体大小");
        // Scan verification
        m.insert("btn_verify_scan", "正在检查…");
        m.insert("scan_status_good", "可扫描 — 所有检查通过");
        m.insert("scan_status_limited", "扫描能力有限");
        m.insert("scan_status_bad", "不可扫描 — 无法解码");
        m.insert("scan_tooltip", "验证对比度、标志覆盖范围以及码是否可解码");
        m.insert(
            "scan_detail_low_contrast",
            "对比度过低 ({:.1}:1，建议 ≥ 4.5:1)",
        );
        m.insert("scan_detail_logo_ec", "标志过大，超出纠错等级支持范围");
        m.insert("scan_detail_large_gap", "模块间距过大");
        m.insert(
            "scan_detail_styled_corners",
            "已设置定位角样式 — 智能手机扫描器可可靠识别",
        );

        // Dialog titles for file choosers
        m.insert("dlg_select_logo", "选择Logo");
        m.insert("dlg_select_bg", "选择背景图片");
        m.insert("dlg_save_label_sheet", "保存标签页");
        m.insert("dlg_select_csv", "选择CSV文件");
        m.insert("dlg_select_folder", "选择文件夹");

        // Dialog buttons
        m.insert("btn_open", "打开");
        m.insert("btn_select", "选择");

        // File filter names
        m.insert("filter_images", "图片文件");
        m.insert("filter_json", "JSON文件");
        m.insert("filter_csv_txt", "CSV/TXT");

        // Status messages (file operations)
        m.insert("status_style_exported", "样式已导出");
        m.insert("status_pdf_saved", "PDF已保存");
        m.insert("status_pdf_error", "PDF导出错误");
        m.insert("status_label_sheet_saved", "标签页已保存");
        m.insert("status_label_sheet_error", "标签页导出错误");
        m.insert("status_png_saved", "PNG已保存");
        m.insert("status_svg_saved", "SVG已保存");
        m.insert("status_gif_saved", "GIF已保存");
        m.insert("status_gif_gradient_only", "GIF仅在渐变模式下可用");
        m.insert("status_batch_exported", "{}个二维码已导出");
        m.insert("status_saved_as", "已保存为{}");
        m.insert("status_enter_template_name", "请输入模板名称");
        m.insert("status_template_deleted_fmt", "模板'{}'已删除");
        m.insert("status_render_error", "错误：无法渲染二维码");
        m.insert("status_copied", "已复制到剪贴板");
        m.insert("status_copied_svg", "SVG已复制到剪贴板");

        // Batch/Label dialog labels
        m.insert("batch_data_label", "二维码数据（每行一个）：");
        m.insert("batch_csv_hint", "（第一列作为二维码数据，跳过标题行）");
        m.insert("batch_format", "格式：");
        m.insert("batch_csv_filter", "CSV/TXT");
        m.insert("batch_folder_label", "文件夹：");
        m.insert("batch_folder_selected", "文件夹：{}");

        // Label sheet dialog
        m.insert("lbl_columns", "列");
        m.insert("lbl_rows", "行");
        m.insert("lbl_margin_mm", "边距 (mm)");
        m.insert("lbl_spacing_mm", "间距 (mm)");
        m.insert("lbl_sheet_info", "在A4页面上排列多个二维码以便打印");
        m.insert("label_sheet_a4_info", "二维码将排列在A4页面上。");

        // Dialog buttons for batch/label
        m.insert("btn_cancel", "取消");
        m.insert("btn_export", "导出");
        m.insert("btn_save", "保存");

        // Dialog titles (FileChooserDialog)
        m.insert("dlg_save_pdf", "保存为PDF");
        m.insert("dlg_save_png", "保存为PNG");
        m.insert("dlg_save_svg", "保存为SVG");
        m.insert("dlg_save_gif", "保存为GIF");
        m.insert("dlg_import_style", "导入样式");
        m.insert("dlg_export_style", "导出样式");
        m.insert("dlg_batch_export", "批量导出");
        m.insert("dlg_label_sheet", "标签页");

        m
    }
}

/// Get the path for persisting the language preference.
pub fn get_lang_path() -> Option<std::path::PathBuf> {
    let config_dir = dirs::config_dir()?;
    let dir = config_dir.join("qr_studio");
    std::fs::create_dir_all(&dir).ok()?;
    Some(dir.join("lang.txt"))
}

/// Load the saved language preference. Returns `Lang::De` as default.
pub fn load_lang() -> Lang {
    if let Some(path) = get_lang_path() {
        if let Ok(content) = std::fs::read_to_string(&path) {
            match content.trim() {
                "en" => Lang::En,
                "es" => Lang::Es,
                "fr" => Lang::Fr,
                "it" => Lang::It,
                "pt_BR" => Lang::PtBr,
                "ja" => Lang::Ja,
                "ko" => Lang::Ko,
                "zh_CN" => Lang::ZhCn,
                _ => Lang::De,
            }
        } else {
            Lang::De
        }
    } else {
        Lang::De
    }
}

/// Save the language preference to disk.
pub fn save_lang(lang: Lang) {
    if let Some(path) = get_lang_path() {
        let tag = match lang {
            Lang::De => "de",
            Lang::En => "en",
            Lang::Es => "es",
            Lang::Fr => "fr",
            Lang::It => "it",
            Lang::PtBr => "pt_BR",
            Lang::Ja => "ja",
            Lang::Ko => "ko",
            Lang::ZhCn => "zh_CN",
        };
        let _ = std::fs::write(&path, tag);
    }
}
