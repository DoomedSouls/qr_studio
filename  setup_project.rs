cat <<'EOF' > setup_project.rs use std::fs; use std::path::Path; use
std::io::Write; fn main() -> std::io::Result<()> { let base_path =
format!("{}/QR Code/qr_studio", std::env::var("HOME").unwrap()); let src_path =
format!("{}/src", base_path); println!("🚀 Starte Projekt-Installation nach:
{}", base_path); // 1. Ordner erstellen fs::create_dir_all(&src_path)?;
println!("✅ Ordner erstellt."); // 2. Cargo.toml schreiben let cargo_toml =
r#"[package] name = "qr_studio" version = "0.1.0" edition = "2021"
[dependencies] gtk4 = { version = "0.7", package = "gtk4" } adw = { version =
"0.5", package = "libadwaita" } qrcode = "0.12" image = "0.24" "#; let mut
cargo_file = fs::File::create(format!("{}/Cargo.toml", base_path))?;
cargo_file.write_all(cargo_toml.as_bytes())?; println!("✅ Cargo.toml
erstellt."); // 3. main.rs schreiben let main_rs = r#"use adw::prelude::*; use
adw::{Application, ApplicationWindow, HeaderBar}; use gtk::{Alignment, Box,
Button, Entry, Label, Orientation, Picture}; use std::cell::RefCell; use
std::rc::Rc; struct AppState { preview_picture: Picture, } fn main() { let app =
Application::builder() .application_id("com.example.qr_studio") .build();
app.connect_activate(build_ui); app.run(); } fn build_ui(app: &Application) {
let window = ApplicationWindow::builder() .application(app) .title("QR Code
Studio") .default_width(800) .default_height(600) .build(); let main_box =
Box::new(Orientation::Vertical, 0); window.set_content(Some(&main_box)); let
header_bar = HeaderBar::new(); main_box.append(&header_bar); let split_view =
gtk::Paned::new(Orientation::Horizontal); main_box.append(&split_view); let
sidebar = Box::new(Orientation::Vertical, 12); sidebar.set_margin_all(12);
sidebar.set_width_request(300); let title_label = Label::builder()
.label("Konfiguration") .css_classes(["title-4"]) .build();
sidebar.append(&title_label); let text_entry = Entry::builder()
.placeholder_text("https://google.com") .build(); sidebar.append(&text_entry);
let action_button = Button::builder() .label("QR Generieren")
.css_classes(["suggested-action"]) .margin_top(20) .build();
sidebar.append(&action_button); split_view.set_start_child(Some(&sidebar)); let
content_box = Box::new(Orientation::Vertical, 12);
content_box.set_valign(Alignment::Center);
content_box.set_halign(Alignment::Center); let preview_label = Label::builder()
.label("Vorschau") .css_classes(["title-2"]) .build();
content_box.append(&preview_label); let preview_picture = Picture::new();
preview_picture.set_size_request(300, 300);
content_box.append(&preview_picture);
split_view.set_end_child(Some(&content_box)); action_button.connect_clicked({
let text_entry = text_entry.clone(); move |_| { let text =
text_entry.text().to_string(); if !text.is_empty() { println!("QR wird generiert
für: {}", text); } } }); window.present(); } "#; let mut main_file =
fs::File::create(format!("{}/src/main.rs", base_path))?;
main_file.write_all(main_rs.as_bytes())?; println!("✅ src/main.rs erstellt.");
println!("\n🎉 ALLES KLAR! Dein Projekt ist bereit."); println!("Gib jetzt ein:
cd \"{}\"", base_path); println!("Und dann: cargo run", base_path); } EOF #
Jetzt führen wir den Installer mit rustc aus rustc setup_project.rs -o
setup_project ./setup_project rm setup_project setup_project.rs
