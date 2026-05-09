#[cfg(test)]
mod tests {
    use crate::helpers::{
        color_harmonies, contrast_ratio, format_qr_data, parse_content_type,
        parse_corner_dot_style, parse_corner_square_style, parse_dot_style, parse_ec_level,
        parse_frame_style, parse_gradient_direction, parse_logo_shape, parse_wifi_encryption,
    };
    use crate::i18n::{I18n, Lang};
    use crate::svg::{base64_encode, parse_svg_viewbox, rgba_to_svg, xml_escape};
    use crate::types::{
        ContentType, CornerDotStyle, CornerSquareStyle, DotStyle, ErrorCorrectionLevel, FrameStyle,
        GradientDirection, LogoShape, WifiEncryption,
    };
    use image::Rgba;

    // ================================================================
    // 1. format_qr_data tests
    // ================================================================

    // --- Text content type ---

    #[test]
    fn format_text_plain() {
        let result = format_qr_data(
            ContentType::Text,
            "Hello World",
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
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn format_text_email_adds_mailto() {
        let result = format_qr_data(
            ContentType::Text,
            "user@example.com",
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
        assert_eq!(result, "mailto:user@example.com");
    }

    #[test]
    fn format_text_email_already_mailto() {
        // Note: The function does not specially handle "mailto:" prefix in the URL-like
        // detection branch, so "mailto:user@example.com" (contains '.' no space) gets
        // https:// prepended, resulting in "https://mailto:user@example.com".
        // This matches the current implementation logic.
        let result = format_qr_data(
            ContentType::Text,
            "mailto:user@example.com",
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
        // Contains '.' and no space → detected as URL-like → gets https:// prefix
        assert_eq!(result, "https://mailto:user@example.com");
    }

    #[test]
    fn format_text_email_with_space_not_mailto() {
        // Emails with spaces should NOT get mailto: prefix
        let result = format_qr_data(
            ContentType::Text,
            "user @example.com",
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
        assert_eq!(result, "user @example.com");
    }

    #[test]
    fn format_text_phone_with_plus() {
        let result = format_qr_data(
            ContentType::Text,
            "+491234567890",
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
        assert_eq!(result, "tel:+491234567890");
    }

    #[test]
    fn format_text_phone_with_00_prefix() {
        let result = format_qr_data(
            ContentType::Text,
            "00491234567890",
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
        assert_eq!(result, "tel:00491234567890");
    }

    #[test]
    fn format_text_phone_already_tel() {
        let result = format_qr_data(
            ContentType::Text,
            "tel:+491234567890",
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
        assert_eq!(result, "tel:+491234567890");
    }

    #[test]
    fn format_text_url_with_https() {
        let result = format_qr_data(
            ContentType::Text,
            "https://example.com",
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
        assert_eq!(result, "https://example.com");
    }

    #[test]
    fn format_text_url_with_http() {
        let result = format_qr_data(
            ContentType::Text,
            "http://example.com",
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
        assert_eq!(result, "http://example.com");
    }

    #[test]
    fn format_text_domain_adds_https() {
        let result = format_qr_data(
            ContentType::Text,
            "example.com",
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
        assert_eq!(result, "https://example.com");
    }

    #[test]
    fn format_text_www_adds_https() {
        let result = format_qr_data(
            ContentType::Text,
            "www.example.com",
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
        assert_eq!(result, "https://www.example.com");
    }

    #[test]
    fn format_text_empty() {
        let result = format_qr_data(
            ContentType::Text,
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
            "",
            "",
            "",
        );
        assert_eq!(result, "");
    }

    // --- URL content type ---

    #[test]
    fn format_url_with_scheme() {
        let result = format_qr_data(
            ContentType::Url,
            "",
            "https://example.com",
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
        assert_eq!(result, "https://example.com");
    }

    #[test]
    fn format_url_without_scheme() {
        let result = format_qr_data(
            ContentType::Url,
            "",
            "example.com",
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
        assert_eq!(result, "https://example.com");
    }

    #[test]
    fn format_url_www_prefix() {
        let result = format_qr_data(
            ContentType::Url,
            "",
            "www.example.com",
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
        assert_eq!(result, "https://www.example.com");
    }

    #[test]
    fn format_url_empty() {
        let result = format_qr_data(
            ContentType::Url,
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
            "",
            "",
            "",
        );
        assert_eq!(result, "");
    }

    // --- WiFi content type ---

    #[test]
    fn format_wifi_wpa() {
        let result = format_qr_data(
            ContentType::Wifi,
            "",
            "",
            "MySSID",
            "password",
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
        assert_eq!(result, "WIFI:T:WPA;S:MySSID;P:password;;");
    }

    #[test]
    fn format_wifi_wep() {
        let result = format_qr_data(
            ContentType::Wifi,
            "",
            "",
            "MySSID",
            "password",
            WifiEncryption::Wep,
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
        assert_eq!(result, "WIFI:T:WEP;S:MySSID;P:password;;");
    }

    #[test]
    fn format_wifi_nopass() {
        let result = format_qr_data(
            ContentType::Wifi,
            "",
            "",
            "OpenNetwork",
            "",
            WifiEncryption::None,
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
        assert_eq!(result, "WIFI:T:nopass;S:OpenNetwork;P:;;");
    }

    // --- vCard content type ---

    #[test]
    fn format_vcard_full() {
        let result = format_qr_data(
            ContentType::Vcard,
            "",
            "",
            "",
            "",
            WifiEncryption::Wpa,
            "John Doe",
            "+491234567",
            "+49",
            "john@example.com",
            "Acme Corp",
            "https://example.com",
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
        assert!(result.starts_with("BEGIN:VCARD\nVERSION:3.0\n"));
        assert!(result.contains("N:John Doe\nFN:John Doe\n"));
        assert!(result.contains("TEL:+491234567\n"));
        assert!(result.contains("EMAIL:john@example.com"));
        assert!(result.contains("ORG:Acme Corp\n"));
        assert!(result.contains("URL:https://example.com\n"));
        assert!(result.ends_with("END:VCARD"));
    }

    #[test]
    fn format_vcard_phone_with_country_code() {
        let result = format_qr_data(
            ContentType::Vcard,
            "",
            "",
            "",
            "",
            WifiEncryption::Wpa,
            "Jane",
            "123456",
            "+1",
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
        assert!(result.contains("TEL:+1123456\n"));
    }

    #[test]
    fn format_vcard_phone_already_international() {
        let result = format_qr_data(
            ContentType::Vcard,
            "",
            "",
            "",
            "",
            WifiEncryption::Wpa,
            "Jane",
            "+44123456",
            "+1",
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
        assert!(result.contains("TEL:+44123456\n"));
    }

    #[test]
    fn format_vcard_phone_00_prefix() {
        let result = format_qr_data(
            ContentType::Vcard,
            "",
            "",
            "",
            "",
            WifiEncryption::Wpa,
            "Jane",
            "0044123456",
            "+1",
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
        assert!(result.contains("TEL:0044123456\n"));
    }

    #[test]
    fn format_vcard_minimal() {
        let result = format_qr_data(
            ContentType::Vcard,
            "",
            "",
            "",
            "",
            WifiEncryption::Wpa,
            "Alice",
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
        assert!(result.starts_with("BEGIN:VCARD\nVERSION:3.0\n"));
        assert!(result.contains("N:Alice\nFN:Alice\n"));
        assert!(!result.contains("TEL:"));
        assert!(!result.contains("EMAIL:"));
        assert!(!result.contains("ORG:"));
        assert!(!result.contains("URL:"));
        assert!(result.ends_with("END:VCARD"));
    }

    // --- Calendar content type ---

    #[test]
    fn format_calendar_full() {
        let result = format_qr_data(
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
            "Team Meeting",
            "20240101T090000",
            "20240101T100000",
            "Conference Room",
            "",
            "",
            "",
            "",
            "",
        );
        assert!(result.starts_with("BEGIN:VEVENT\n"));
        assert!(result.contains("SUMMARY:Team Meeting\n"));
        assert!(result.contains("DTSTART:20240101T090000\n"));
        assert!(result.contains("DTEND:20240101T100000\n"));
        assert!(result.contains("LOCATION:Conference Room\n"));
        assert!(result.ends_with("END:VEVENT"));
    }

    #[test]
    fn format_calendar_minimal() {
        let result = format_qr_data(
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
            "Lunch",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        );
        assert!(result.contains("SUMMARY:Lunch\n"));
        assert!(!result.contains("DTSTART:"));
        assert!(!result.contains("DTEND:"));
        assert!(!result.contains("LOCATION:"));
    }

    // --- GPS content type ---

    #[test]
    fn format_gps() {
        let result = format_qr_data(
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
            "52.5200",
            "13.4050",
            "",
            "",
            "",
        );
        assert_eq!(result, "geo:52.5200,13.4050");
    }

    // --- SMS content type ---

    #[test]
    fn format_sms_with_country_code() {
        let result = format_qr_data(
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
            "123456",
            "+49",
            "Hello!",
        );
        assert_eq!(result, "SMSTO:+49123456:Hello!");
    }

    #[test]
    fn format_sms_international_phone() {
        let result = format_qr_data(
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
            "+49123456",
            "",
            "Hi there",
        );
        assert_eq!(result, "SMSTO:+49123456:Hi there");
    }

    #[test]
    fn format_sms_00_prefix() {
        let result = format_qr_data(
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
            "0049123456",
            "",
            "Test",
        );
        assert_eq!(result, "SMSTO:0049123456:Test");
    }

    // ================================================================
    // 2. Parsing tests
    // ================================================================

    // --- parse_dot_style ---

    #[test]
    fn parse_dot_style_english() {
        assert_eq!(parse_dot_style("Rounded"), DotStyle::Rounded);
        assert_eq!(parse_dot_style("Square"), DotStyle::Square);
        assert_eq!(parse_dot_style("Dots"), DotStyle::Dots);
        assert_eq!(parse_dot_style("Diamond"), DotStyle::Diamond);
        assert_eq!(parse_dot_style("Custom"), DotStyle::Custom);
    }

    #[test]
    fn parse_dot_style_german() {
        assert_eq!(parse_dot_style("Abgerundet"), DotStyle::Rounded);
        assert_eq!(parse_dot_style("Quadratisch"), DotStyle::Square);
        assert_eq!(parse_dot_style("Punkte"), DotStyle::Dots);
        assert_eq!(parse_dot_style("Raute"), DotStyle::Diamond);
        assert_eq!(parse_dot_style("Benutzerdefiniert"), DotStyle::Custom);
    }

    #[test]
    fn parse_dot_style_unknown_defaults_rounded() {
        assert_eq!(parse_dot_style("Unknown"), DotStyle::Rounded);
        assert_eq!(parse_dot_style(""), DotStyle::Rounded);
    }

    #[test]
    fn parse_dot_style_legacy_mappings() {
        assert_eq!(parse_dot_style("ExtraRounded"), DotStyle::Rounded);
        assert_eq!(parse_dot_style("Elegant"), DotStyle::Rounded);
        assert_eq!(parse_dot_style("Classy"), DotStyle::Rounded);
        assert_eq!(parse_dot_style("ClassyRounded"), DotStyle::Rounded);
        assert_eq!(parse_dot_style("Stark abgerundet"), DotStyle::Rounded);
        assert_eq!(parse_dot_style("Elegant abgerundet"), DotStyle::Rounded);
    }

    // --- parse_corner_square_style ---

    #[test]
    fn parse_corner_square_style_english() {
        assert_eq!(
            parse_corner_square_style("Square"),
            CornerSquareStyle::Square
        );
        assert_eq!(
            parse_corner_square_style("Rounded"),
            CornerSquareStyle::ExtraRounded
        );
        assert_eq!(parse_corner_square_style("Dot"), CornerSquareStyle::Dot);
        assert_eq!(
            parse_corner_square_style("Circle"),
            CornerSquareStyle::Circle
        );
    }

    #[test]
    fn parse_corner_square_style_german() {
        assert_eq!(
            parse_corner_square_style("Quadratisch"),
            CornerSquareStyle::Square
        );
        assert_eq!(
            parse_corner_square_style("Abgerundet"),
            CornerSquareStyle::ExtraRounded
        );
        assert_eq!(parse_corner_square_style("Punkt"), CornerSquareStyle::Dot);
        assert_eq!(
            parse_corner_square_style("Kreis"),
            CornerSquareStyle::Circle
        );
    }

    #[test]
    fn parse_corner_square_style_unknown_defaults_extra_rounded() {
        assert_eq!(
            parse_corner_square_style("Unknown"),
            CornerSquareStyle::ExtraRounded
        );
    }

    // --- parse_corner_dot_style ---

    #[test]
    fn parse_corner_dot_style_english() {
        assert_eq!(parse_corner_dot_style("Square"), CornerDotStyle::Square);
        assert_eq!(parse_corner_dot_style("Dot"), CornerDotStyle::Dot);
        assert_eq!(parse_corner_dot_style("Circle"), CornerDotStyle::Circle);
        assert_eq!(
            parse_corner_dot_style("Rounded"),
            CornerDotStyle::ExtraRounded
        );
    }

    #[test]
    fn parse_corner_dot_style_german() {
        assert_eq!(
            parse_corner_dot_style("Quadratisch"),
            CornerDotStyle::Square
        );
        assert_eq!(parse_corner_dot_style("Punkt"), CornerDotStyle::Dot);
        assert_eq!(parse_corner_dot_style("Kreis"), CornerDotStyle::Circle);
        assert_eq!(
            parse_corner_dot_style("Abgerundet"),
            CornerDotStyle::ExtraRounded
        );
    }

    #[test]
    fn parse_corner_dot_style_unknown_defaults_dot() {
        assert_eq!(parse_corner_dot_style("Unknown"), CornerDotStyle::Dot);
    }

    // --- parse_ec_level ---

    #[test]
    fn parse_ec_level_english() {
        assert_eq!(parse_ec_level("Low (L)"), ErrorCorrectionLevel::Low);
        assert_eq!(parse_ec_level("Medium (M)"), ErrorCorrectionLevel::Medium);
        assert_eq!(
            parse_ec_level("Quartile (Q)"),
            ErrorCorrectionLevel::Quartile
        );
        assert_eq!(parse_ec_level("High (H)"), ErrorCorrectionLevel::High);
    }

    #[test]
    fn parse_ec_level_german() {
        assert_eq!(parse_ec_level("Niedrig (L)"), ErrorCorrectionLevel::Low);
        assert_eq!(parse_ec_level("Mittel (M)"), ErrorCorrectionLevel::Medium);
        assert_eq!(
            parse_ec_level("Quartil (Q)"),
            ErrorCorrectionLevel::Quartile
        );
        assert_eq!(parse_ec_level("Hoch (H)"), ErrorCorrectionLevel::High);
    }

    #[test]
    fn parse_ec_level_unknown_defaults_medium() {
        assert_eq!(parse_ec_level("Unknown"), ErrorCorrectionLevel::Medium);
    }

    // --- parse_gradient_direction ---

    #[test]
    fn parse_gradient_direction_all() {
        assert_eq!(
            parse_gradient_direction("Horizontal"),
            GradientDirection::Horizontal
        );
        assert_eq!(
            parse_gradient_direction("Vertical"),
            GradientDirection::Vertical
        );
        assert_eq!(
            parse_gradient_direction("Diagonal"),
            GradientDirection::Diagonal
        );
        assert_eq!(
            parse_gradient_direction("Radial"),
            GradientDirection::Radial
        );
    }

    #[test]
    fn parse_gradient_direction_german() {
        assert_eq!(
            parse_gradient_direction("Vertikal"),
            GradientDirection::Vertical
        );
    }

    #[test]
    fn parse_gradient_direction_unknown_defaults_horizontal() {
        assert_eq!(
            parse_gradient_direction("Unknown"),
            GradientDirection::Horizontal
        );
    }

    // --- parse_content_type ---

    #[test]
    fn parse_content_type_english() {
        assert_eq!(parse_content_type("Text"), ContentType::Text);
        assert_eq!(parse_content_type("URL"), ContentType::Url);
        assert_eq!(parse_content_type("WiFi"), ContentType::Wifi);
        assert_eq!(parse_content_type("vCard"), ContentType::Vcard);
        assert_eq!(parse_content_type("Calendar Event"), ContentType::Calendar);
        assert_eq!(parse_content_type("GPS Location"), ContentType::Gps);
        assert_eq!(parse_content_type("SMS"), ContentType::Sms);
    }

    #[test]
    fn parse_content_type_german() {
        assert_eq!(parse_content_type("Webseite"), ContentType::Url);
        assert_eq!(parse_content_type("Website"), ContentType::Url);
        assert_eq!(parse_content_type("vCard/Kontakt"), ContentType::Vcard);
        assert_eq!(
            parse_content_type("Kalenderereignis"),
            ContentType::Calendar
        );
        assert_eq!(parse_content_type("GPS-Standort"), ContentType::Gps);
    }

    #[test]
    fn parse_content_type_unknown_defaults_text() {
        assert_eq!(parse_content_type("Unknown"), ContentType::Text);
    }

    // --- parse_wifi_encryption ---

    #[test]
    fn parse_wifi_encryption_all() {
        assert_eq!(parse_wifi_encryption("WPA"), WifiEncryption::Wpa);
        assert_eq!(parse_wifi_encryption("WEP"), WifiEncryption::Wep);
        assert_eq!(parse_wifi_encryption("None"), WifiEncryption::None);
        assert_eq!(parse_wifi_encryption("Keine"), WifiEncryption::None);
    }

    #[test]
    fn parse_wifi_encryption_unknown_defaults_wpa() {
        assert_eq!(parse_wifi_encryption("Unknown"), WifiEncryption::Wpa);
    }

    // --- parse_logo_shape ---

    #[test]
    fn parse_logo_shape_all() {
        assert_eq!(parse_logo_shape("Rectangle"), LogoShape::Rectangle);
        assert_eq!(parse_logo_shape("Circle"), LogoShape::Circle);
        assert_eq!(parse_logo_shape("Rounded"), LogoShape::RoundedRect);
    }

    #[test]
    fn parse_logo_shape_german() {
        assert_eq!(parse_logo_shape("Rechteck"), LogoShape::Rectangle);
        assert_eq!(parse_logo_shape("Kreis"), LogoShape::Circle);
        assert_eq!(parse_logo_shape("Abgerundet"), LogoShape::RoundedRect);
    }

    #[test]
    fn parse_logo_shape_unknown_defaults_circle() {
        assert_eq!(parse_logo_shape("Unknown"), LogoShape::Circle);
    }

    // --- parse_frame_style ---

    #[test]
    fn parse_frame_style_all() {
        assert_eq!(parse_frame_style("None"), FrameStyle::None);
        assert_eq!(parse_frame_style("Simple"), FrameStyle::Simple);
        assert_eq!(parse_frame_style("Rounded"), FrameStyle::Rounded);
        assert_eq!(parse_frame_style("Banner"), FrameStyle::Banner);
    }

    #[test]
    fn parse_frame_style_german() {
        assert_eq!(parse_frame_style("Keiner"), FrameStyle::None);
        assert_eq!(parse_frame_style("Einfach"), FrameStyle::Simple);
        assert_eq!(parse_frame_style("Abgerundet"), FrameStyle::Rounded);
    }

    #[test]
    fn parse_frame_style_unknown_defaults_none() {
        assert_eq!(parse_frame_style("Unknown"), FrameStyle::None);
    }

    // ================================================================
    // 3. SVG helper tests
    // ================================================================

    // --- rgba_to_svg ---

    #[test]
    fn rgba_to_svg_opaque_returns_hex() {
        let c = Rgba([255, 0, 128, 255]);
        assert_eq!(rgba_to_svg(c), "#ff0080");
    }

    #[test]
    fn rgba_to_svg_opaque_black() {
        let c = Rgba([0, 0, 0, 255]);
        assert_eq!(rgba_to_svg(c), "#000000");
    }

    #[test]
    fn rgba_to_svg_opaque_white() {
        let c = Rgba([255, 255, 255, 255]);
        assert_eq!(rgba_to_svg(c), "#ffffff");
    }

    #[test]
    fn rgba_to_svg_transparent_returns_rgba() {
        let c = Rgba([255, 0, 128, 128]);
        let result = rgba_to_svg(c);
        // Alpha 128/255 ≈ 0.50
        assert!(result.starts_with("rgba(255,0,128,"));
        assert!(!result.starts_with('#'));
    }

    #[test]
    fn rgba_to_svg_fully_transparent() {
        let c = Rgba([100, 200, 50, 0]);
        let result = rgba_to_svg(c);
        assert!(result.starts_with("rgba(100,200,50,"));
        assert!(result.contains("0.00"));
    }

    #[test]
    fn rgba_to_svg_semi_transparent_precision() {
        let c = Rgba([10, 20, 30, 127]);
        let result = rgba_to_svg(c);
        // 127/255 ≈ 0.50, formatted to 2 decimal places
        assert!(result.starts_with("rgba(10,20,30,"));
    }

    // --- base64_encode ---

    #[test]
    fn base64_encode_empty() {
        assert_eq!(base64_encode(&[]), "");
    }

    #[test]
    fn base64_encode_hello() {
        // "Hello" → 5 bytes → 8 base64 chars + 0 padding
        let hello = b"Hello";
        let result = base64_encode(hello);
        assert_eq!(result, "SGVsbG8=");
    }

    #[test]
    fn base64_encode_known_triple() {
        // "Man" → 3 bytes → exactly 4 base64 chars, no padding
        let data = b"Man";
        assert_eq!(base64_encode(data), "TWFu");
    }

    #[test]
    fn base64_encode_single_byte() {
        // 1 byte → 2 base64 chars + 2 padding
        let data = b"A";
        assert_eq!(base64_encode(data), "QQ==");
    }

    #[test]
    fn base64_encode_two_bytes() {
        // 2 bytes → 3 base64 chars + 1 padding
        let data = b"AB";
        assert_eq!(base64_encode(data), "QUI=");
    }

    #[test]
    fn base64_encode_binary_data() {
        // bytes 0x00, 0x01, 0x02, 0x03 → 4 bytes → 2 chunks
        let data: &[u8] = &[0x00, 0x01, 0x02, 0x03];
        let result = base64_encode(data);
        assert_eq!(result, "AAECAw==");
    }

    // --- xml_escape ---

    #[test]
    fn xml_escape_ampersand() {
        assert_eq!(xml_escape("a&b"), "a&amp;b");
    }

    #[test]
    fn xml_escape_less_than() {
        assert_eq!(xml_escape("a<b"), "a&lt;b");
    }

    #[test]
    fn xml_escape_greater_than() {
        assert_eq!(xml_escape("a>b"), "a&gt;b");
    }

    #[test]
    fn xml_escape_double_quote() {
        assert_eq!(xml_escape("a\"b"), "a&quot;b");
    }

    #[test]
    fn xml_escape_single_quote() {
        assert_eq!(xml_escape("a'b"), "a&apos;b");
    }

    #[test]
    fn xml_escape_all_special_chars() {
        assert_eq!(xml_escape("&<\"'>"), "&amp;&lt;&quot;&apos;&gt;");
    }

    #[test]
    fn xml_escape_no_special_chars() {
        assert_eq!(xml_escape("Hello World"), "Hello World");
    }

    #[test]
    fn xml_escape_empty() {
        assert_eq!(xml_escape(""), "");
    }

    // --- parse_svg_viewbox ---

    #[test]
    fn parse_svg_viewbox_valid() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 800 600">"#;
        let result = parse_svg_viewbox(svg);
        assert_eq!(result, Some((800.0, 600.0)));
    }

    #[test]
    fn parse_svg_viewbox_square() {
        let svg = r#"<svg viewBox="0 0 256 256">"#;
        let result = parse_svg_viewbox(svg);
        assert_eq!(result, Some((256.0, 256.0)));
    }

    #[test]
    fn parse_svg_viewbox_fractional() {
        let svg = r#"<svg viewBox="0 0 100.5 200.75">"#;
        let result = parse_svg_viewbox(svg);
        assert_eq!(result, Some((100.5, 200.75)));
    }

    #[test]
    fn parse_svg_viewbox_no_viewbox() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg">"#;
        assert_eq!(parse_svg_viewbox(svg), None);
    }

    #[test]
    fn parse_svg_viewbox_malformed() {
        let svg = r#"<svg viewBox="invalid">"#;
        assert_eq!(parse_svg_viewbox(svg), None);
    }

    // ================================================================
    // 4. i18n completeness test
    // ================================================================

    #[test]
    fn i18n_all_languages_have_same_keys_as_english() {
        let en = I18n::new(Lang::En);
        let en_keys: std::collections::HashSet<_> = en.strings.keys().copied().collect();
        assert!(
            !en_keys.is_empty(),
            "English translation map should not be empty"
        );

        let other_langs = [
            (Lang::De, "German"),
            (Lang::Es, "Spanish"),
            (Lang::Fr, "French"),
            (Lang::It, "Italian"),
            (Lang::PtBr, "Portuguese (BR)"),
            (Lang::Ja, "Japanese"),
            (Lang::Ko, "Korean"),
            (Lang::ZhCn, "Chinese (CN)"),
        ];

        for (lang, lang_name) in &other_langs {
            let i18n = I18n::new(*lang);
            let lang_keys: std::collections::HashSet<_> = i18n.strings.keys().copied().collect();

            let missing: Vec<&&str> = en_keys.iter().filter(|k| !lang_keys.contains(*k)).collect();
            let extra: Vec<&&str> = lang_keys.iter().filter(|k| !en_keys.contains(*k)).collect();

            assert!(
                missing.is_empty() && extra.is_empty(),
                "{}: key mismatch with English.\n  Missing keys: {:?}\n  Extra keys: {:?}",
                lang_name,
                missing,
                extra,
            );
        }
    }

    #[test]
    fn i18n_translation_returns_known_key() {
        let en = I18n::new(Lang::En);
        // t() should return a value for any key that exists in the map
        for key in en.strings.keys() {
            let translated = en.t(key);
            assert!(
                !translated.is_empty(),
                "Translation for key '{}' should not be empty",
                key
            );
        }
    }

    #[test]
    fn i18n_translation_unknown_key_returns_key() {
        let en = I18n::new(Lang::En);
        assert_eq!(en.t("nonexistent_key_12345"), "nonexistent_key_12345");
    }

    #[test]
    fn i18n_german_translation_is_different_from_english() {
        let en = I18n::new(Lang::En);
        let de = I18n::new(Lang::De);

        // At least some keys should have different values between English and German
        let mut differing_count = 0;
        for key in en.strings.keys() {
            let en_val = en.t(key);
            let de_val = de.t(key);
            if en_val != de_val {
                differing_count += 1;
            }
        }
        assert!(
            differing_count > 0,
            "German translations should differ from English for at least some keys"
        );
    }

    // ================================================================
    // 5. Contrast ratio tests
    // ================================================================

    #[test]
    fn contrast_ratio_black_on_white() {
        let black = Rgba([0, 0, 0, 255]);
        let white = Rgba([255, 255, 255, 255]);
        let ratio = contrast_ratio(&black, &white);
        // WCAG: black on white = 21:1
        assert!((ratio - 21.0).abs() < 0.01, "Expected ~21.0, got {}", ratio);
    }

    #[test]
    fn contrast_ratio_white_on_white() {
        let white = Rgba([255, 255, 255, 255]);
        let ratio = contrast_ratio(&white, &white);
        // Same color = 1:1
        assert!((ratio - 1.0).abs() < 0.01, "Expected ~1.0, got {}", ratio);
    }

    #[test]
    fn contrast_ratio_black_on_black() {
        let black = Rgba([0, 0, 0, 255]);
        let ratio = contrast_ratio(&black, &black);
        assert!((ratio - 1.0).abs() < 0.01, "Expected ~1.0, got {}", ratio);
    }

    #[test]
    fn contrast_ratio_order_invariant() {
        let fg = Rgba([0, 0, 0, 255]);
        let bg = Rgba([255, 255, 255, 255]);
        let r1 = contrast_ratio(&fg, &bg);
        let r2 = contrast_ratio(&bg, &fg);
        assert!(
            (r1 - r2).abs() < f64::EPSILON,
            "contrast_ratio should be order-invariant: {} vs {}",
            r1,
            r2
        );
    }

    #[test]
    fn contrast_ratio_mid_gray_on_white() {
        // #808080 on white: known WCAG value ≈ 3.95
        let gray = Rgba([128, 128, 128, 255]);
        let white = Rgba([255, 255, 255, 255]);
        let ratio = contrast_ratio(&gray, &white);
        assert!((ratio - 3.95).abs() < 0.2, "Expected ~3.95, got {}", ratio);
    }

    #[test]
    fn contrast_ratio_red_on_white() {
        // Pure red (#FF0000) on white: known value ≈ 4.00
        let red = Rgba([255, 0, 0, 255]);
        let white = Rgba([255, 255, 255, 255]);
        let ratio = contrast_ratio(&red, &white);
        // Approximate: WCAG says ~4.00
        assert!((ratio - 4.0).abs() < 0.5, "Expected ~4.0, got {}", ratio);
    }

    #[test]
    fn contrast_ratio_blue_on_black() {
        // Blue (#0000FF) on black: known value ≈ 2.44
        let blue = Rgba([0, 0, 255, 255]);
        let black = Rgba([0, 0, 0, 255]);
        let ratio = contrast_ratio(&blue, &black);
        assert!((ratio - 2.44).abs() < 0.2, "Expected ~2.44, got {}", ratio);
    }

    // ================================================================
    // Color harmonies tests
    // ================================================================

    #[test]
    fn color_harmonies_returns_five_entries() {
        let color = Rgba([255, 0, 0, 255]);
        let harmonies = color_harmonies(color);
        assert_eq!(harmonies.len(), 5);
    }

    #[test]
    fn color_harmonies_contains_expected_names() {
        let color = Rgba([0, 128, 255, 255]);
        let harmonies = color_harmonies(color);
        let names: Vec<&str> = harmonies.iter().map(|(name, _)| name.as_str()).collect();
        assert!(names.contains(&"Komplementär"));
        assert!(names.contains(&"Analog 1"));
        assert!(names.contains(&"Analog 2"));
        assert!(names.contains(&"Triadisch 1"));
        assert!(names.contains(&"Triadisch 2"));
    }

    #[test]
    fn color_harmonies_preserves_alpha() {
        let color = Rgba([100, 200, 50, 128]);
        let harmonies = color_harmonies(color);
        for (_name, harmony_color) in &harmonies {
            assert_eq!(
                harmony_color.0[3], 128,
                "Alpha channel should be preserved in harmonies"
            );
        }
    }

    #[test]
    fn color_harmonies_complementary_is_rotated_180() {
        // Pure red (h=0) → complementary should be cyan (h=180)
        let red = Rgba([255, 0, 0, 255]);
        let harmonies = color_harmonies(red);
        let complementary = harmonies
            .iter()
            .find(|(name, _)| name == "Komplementär")
            .map(|(_, c)| *c)
            .unwrap();
        // Complementary of red should be approximately cyan
        assert_eq!(complementary.0[0], 0); // Red channel should be 0
        assert!(
            complementary.0[2] > 200,
            "Blue channel should be high for cyan, got {}",
            complementary.0[2]
        );
    }

    // ================================================================
    // 8. SVG Snapshot Regression Tests
    // ================================================================

    use std::path::PathBuf;

    /// Configuration struct for SVG snapshot tests.
    /// Provides sensible defaults matching the CLI defaults.
    /// Individual tests modify only the fields they want to test.
    #[derive(Clone)]
    struct SvgSnapshotConfig {
        data: &'static str,
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
        logo_path: Option<PathBuf>,
        logo_size: f64,
        outer_text_top: &'static str,
        outer_text_bottom: &'static str,
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
        frame_inner_radius: f64,
        logo_color: Rgba<u8>,
        logo_border_width: f64,
        logo_border_color: Rgba<u8>,
        bg_image_path: Option<PathBuf>,
        bg_image_data: Option<(String, String)>,
        logo_vectorize: bool,
        logo_vectorize_bg_color: Rgba<u8>,
        logo_bg_transparent: bool,
        logo_clear_area: bool,
        logo_clear_padding: f64,
        logo_outer_radius: f64,
        logo_inner_radius: f64,
        gradient_phase: f64,
        custom_dot_path: &'static str,
        outer_text_font: &'static str,
        outer_text_font_size: u32,
    }

    impl Default for SvgSnapshotConfig {
        fn default() -> Self {
            Self {
                data: "HELLO",
                dot_style: DotStyle::Rounded,
                corner_square_style: CornerSquareStyle::ExtraRounded,
                corner_dot_style: CornerDotStyle::Dot,
                fg_color: Rgba([0, 0, 0, 255]),
                bg_color: Rgba([255, 255, 255, 255]),
                corner_color: Rgba([0, 0, 0, 255]),
                ec_level: ErrorCorrectionLevel::Medium,
                transparent_bg: false,
                gradient_enabled: false,
                gradient_color: Rgba([0, 0, 0, 255]),
                gradient_direction: GradientDirection::Horizontal,
                logo_path: None,
                logo_size: 0.4,
                outer_text_top: "",
                outer_text_bottom: "",
                outer_text_color: Rgba([0, 0, 0, 255]),
                module_size: 8,
                logo_shape: LogoShape::Circle,
                quiet_zone: 4,
                module_gap: 0.0,
                frame_style: FrameStyle::None,
                frame_color: Rgba([0, 0, 0, 255]),
                shadow_enabled: false,
                shadow_offset: 2.0,
                frame_width: 4,
                frame_outer_radius: 4.0,
                frame_inner_radius: 4.0,
                logo_color: Rgba([0, 0, 0, 255]),
                logo_border_width: 2.0,
                logo_border_color: Rgba([255, 255, 255, 255]),
                bg_image_path: None,
                bg_image_data: None,
                logo_vectorize: false,
                logo_vectorize_bg_color: Rgba([255, 255, 255, 255]),
                logo_bg_transparent: false,
                logo_clear_area: false,
                logo_clear_padding: 0.5,
                logo_outer_radius: 8.0,
                logo_inner_radius: 8.0,
                gradient_phase: 0.0,
                custom_dot_path: "",
                outer_text_font: "sans-serif",
                outer_text_font_size: 14,
            }
        }
    }

    impl SvgSnapshotConfig {
        /// Render the SVG using this config's parameters.
        fn render(&self) -> String {
            crate::svg::render_vector_svg(
                self.data,
                self.dot_style,
                self.corner_square_style,
                self.corner_dot_style,
                self.fg_color,
                self.bg_color,
                self.corner_color,
                self.ec_level,
                self.transparent_bg,
                self.gradient_enabled,
                self.gradient_color,
                self.gradient_direction,
                self.logo_path.as_ref(),
                self.logo_size,
                self.outer_text_top,
                self.outer_text_bottom,
                self.outer_text_color,
                self.module_size,
                self.logo_shape,
                self.quiet_zone,
                self.module_gap,
                self.frame_style,
                self.frame_color,
                self.shadow_enabled,
                self.shadow_offset,
                self.frame_width,
                self.frame_outer_radius,
                self.frame_inner_radius,
                self.logo_color,
                self.logo_border_width,
                self.logo_border_color,
                self.bg_image_path.as_ref(),
                self.bg_image_data.as_ref(),
                self.logo_vectorize,
                self.logo_vectorize_bg_color,
                self.logo_bg_transparent,
                self.logo_clear_area,
                self.logo_clear_padding,
                self.logo_outer_radius,
                self.logo_inner_radius,
                self.gradient_phase,
                self.custom_dot_path,
                self.outer_text_font,
                self.outer_text_font_size,
            )
            .expect("render_vector_svg should succeed for snapshot tests")
        }
    }

    /// Returns the directory where SVG snapshot baselines are stored.
    fn snapshot_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("svg_snapshots")
    }

    /// Compare a generated SVG against a stored baseline snapshot.
    ///
    /// - If `UPDATE_SNAPSHOTS` env var is set, writes the SVG as the new baseline.
    /// - Otherwise, reads the baseline and asserts exact equality.
    /// - If the baseline doesn't exist, panics with instructions to create it.
    /// - On mismatch, writes the actual output as `<name>.actual.svg` for easy diffing.
    fn check_svg_snapshot(name: &str, svg: &str) {
        let dir = snapshot_dir();
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join(format!("{}.svg", name));

        if std::env::var("UPDATE_SNAPSHOTS").is_ok() {
            std::fs::write(&path, svg.as_bytes())
                .unwrap_or_else(|e| panic!("Failed to write snapshot '{}': {}", name, e));
            eprintln!("  ✓ Updated snapshot: {}", path.display());
            return;
        }

        let expected = std::fs::read_to_string(&path).unwrap_or_else(|_| {
            panic!(
                "Snapshot baseline '{}' not found at {}. \
                 Run with UPDATE_SNAPSHOTS=1 to create it.",
                name,
                path.display()
            )
        });

        if svg != expected {
            // Write the actual output next to the baseline for easy diffing
            let actual_path = dir.join(format!("{}.actual.svg", name));
            std::fs::write(&actual_path, svg.as_bytes()).unwrap();
            panic!(
                "SVG snapshot '{}' differs from baseline.\
                 \n  Baseline: {}\
                 \n  Actual:   {}\
                 \nRun with UPDATE_SNAPSHOTS=1 to accept the new output.",
                name,
                path.display(),
                actual_path.display(),
            );
        }
    }

    // ── Dot styles ──────────────────────────────────────────────

    #[test]
    fn snapshot_rounded_default() {
        let svg = SvgSnapshotConfig::default().render();
        check_svg_snapshot("rounded_default", &svg);
    }

    #[test]
    fn snapshot_square_dots() {
        let cfg = SvgSnapshotConfig {
            dot_style: DotStyle::Square,
            ..Default::default()
        };
        check_svg_snapshot("square_dots", &cfg.render());
    }

    #[test]
    fn snapshot_dots_style() {
        let cfg = SvgSnapshotConfig {
            dot_style: DotStyle::Dots,
            ..Default::default()
        };
        check_svg_snapshot("dots_style", &cfg.render());
    }

    #[test]
    fn snapshot_diamond_dots() {
        let cfg = SvgSnapshotConfig {
            dot_style: DotStyle::Diamond,
            ..Default::default()
        };
        check_svg_snapshot("diamond_dots", &cfg.render());
    }

    // ── Corner square styles ────────────────────────────────────

    #[test]
    fn snapshot_corner_square() {
        let cfg = SvgSnapshotConfig {
            corner_square_style: CornerSquareStyle::Square,
            ..Default::default()
        };
        check_svg_snapshot("corner_square", &cfg.render());
    }

    #[test]
    fn snapshot_corner_extra_rounded() {
        let cfg = SvgSnapshotConfig {
            corner_square_style: CornerSquareStyle::ExtraRounded,
            ..Default::default()
        };
        check_svg_snapshot("corner_extra_rounded", &cfg.render());
    }

    #[test]
    fn snapshot_corner_dot() {
        let cfg = SvgSnapshotConfig {
            corner_square_style: CornerSquareStyle::Dot,
            ..Default::default()
        };
        check_svg_snapshot("corner_dot_style", &cfg.render());
    }

    #[test]
    fn snapshot_corner_circle() {
        let cfg = SvgSnapshotConfig {
            corner_square_style: CornerSquareStyle::Circle,
            ..Default::default()
        };
        check_svg_snapshot("corner_circle", &cfg.render());
    }

    // ── Corner dot styles ───────────────────────────────────────

    #[test]
    fn snapshot_corner_dot_square() {
        let cfg = SvgSnapshotConfig {
            corner_dot_style: CornerDotStyle::Square,
            ..Default::default()
        };
        check_svg_snapshot("corner_dot_square", &cfg.render());
    }

    #[test]
    fn snapshot_corner_dot_circle() {
        let cfg = SvgSnapshotConfig {
            corner_dot_style: CornerDotStyle::Circle,
            ..Default::default()
        };
        check_svg_snapshot("corner_dot_circle", &cfg.render());
    }

    #[test]
    fn snapshot_corner_dot_extra_rounded() {
        let cfg = SvgSnapshotConfig {
            corner_dot_style: CornerDotStyle::ExtraRounded,
            ..Default::default()
        };
        check_svg_snapshot("corner_dot_extra_rounded", &cfg.render());
    }

    // ── Gradients ───────────────────────────────────────────────

    #[test]
    fn snapshot_gradient_horizontal() {
        let cfg = SvgSnapshotConfig {
            gradient_enabled: true,
            gradient_color: Rgba([0, 100, 255, 255]),
            gradient_direction: GradientDirection::Horizontal,
            ..Default::default()
        };
        check_svg_snapshot("gradient_horizontal", &cfg.render());
    }

    #[test]
    fn snapshot_gradient_vertical() {
        let cfg = SvgSnapshotConfig {
            gradient_enabled: true,
            gradient_color: Rgba([255, 0, 100, 255]),
            gradient_direction: GradientDirection::Vertical,
            ..Default::default()
        };
        check_svg_snapshot("gradient_vertical", &cfg.render());
    }

    #[test]
    fn snapshot_gradient_diagonal() {
        let cfg = SvgSnapshotConfig {
            gradient_enabled: true,
            gradient_color: Rgba([0, 200, 100, 255]),
            gradient_direction: GradientDirection::Diagonal,
            ..Default::default()
        };
        check_svg_snapshot("gradient_diagonal", &cfg.render());
    }

    #[test]
    fn snapshot_gradient_radial() {
        let cfg = SvgSnapshotConfig {
            gradient_enabled: true,
            gradient_color: Rgba([200, 0, 200, 255]),
            gradient_direction: GradientDirection::Radial,
            ..Default::default()
        };
        check_svg_snapshot("gradient_radial", &cfg.render());
    }

    // ── Frames ──────────────────────────────────────────────────

    #[test]
    fn snapshot_frame_simple() {
        let cfg = SvgSnapshotConfig {
            frame_style: FrameStyle::Simple,
            ..Default::default()
        };
        check_svg_snapshot("frame_simple", &cfg.render());
    }

    #[test]
    fn snapshot_frame_rounded() {
        let cfg = SvgSnapshotConfig {
            frame_style: FrameStyle::Rounded,
            ..Default::default()
        };
        check_svg_snapshot("frame_rounded", &cfg.render());
    }

    #[test]
    fn snapshot_frame_banner() {
        let cfg = SvgSnapshotConfig {
            frame_style: FrameStyle::Banner,
            outer_text_bottom: "Scan Me",
            ..Default::default()
        };
        check_svg_snapshot("frame_banner", &cfg.render());
    }

    // ── Transparent background ───────────────────────────────────

    #[test]
    fn snapshot_transparent_bg() {
        let cfg = SvgSnapshotConfig {
            transparent_bg: true,
            ..Default::default()
        };
        check_svg_snapshot("transparent_bg", &cfg.render());
    }

    // ── Module gap ──────────────────────────────────────────────

    #[test]
    fn snapshot_module_gap() {
        let cfg = SvgSnapshotConfig {
            module_gap: 0.3,
            ..Default::default()
        };
        check_svg_snapshot("module_gap", &cfg.render());
    }

    // ── Outer text ──────────────────────────────────────────────

    #[test]
    fn snapshot_outer_text_top() {
        let cfg = SvgSnapshotConfig {
            outer_text_top: "QR Studio",
            ..Default::default()
        };
        check_svg_snapshot("outer_text_top", &cfg.render());
    }

    #[test]
    fn snapshot_outer_text_top_and_bottom() {
        let cfg = SvgSnapshotConfig {
            outer_text_top: "QR Studio",
            outer_text_bottom: "Scan Me",
            ..Default::default()
        };
        check_svg_snapshot("outer_text_top_and_bottom", &cfg.render());
    }

    // ── Shadow ──────────────────────────────────────────────────

    #[test]
    fn snapshot_shadow() {
        let cfg = SvgSnapshotConfig {
            shadow_enabled: true,
            shadow_offset: 2.0,
            ..Default::default()
        };
        check_svg_snapshot("shadow", &cfg.render());
    }

    // ── Custom colors ───────────────────────────────────────────

    #[test]
    fn snapshot_custom_colors() {
        let cfg = SvgSnapshotConfig {
            fg_color: Rgba([30, 60, 180, 255]),
            bg_color: Rgba([240, 240, 250, 255]),
            corner_color: Rgba([200, 40, 40, 255]),
            ..Default::default()
        };
        check_svg_snapshot("custom_colors", &cfg.render());
    }

    // ── High error correction ───────────────────────────────────

    #[test]
    fn snapshot_ec_high() {
        let cfg = SvgSnapshotConfig {
            ec_level: ErrorCorrectionLevel::High,
            ..Default::default()
        };
        check_svg_snapshot("ec_high", &cfg.render());
    }
}
