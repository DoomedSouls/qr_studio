fn main() {
    // Compile GResource bundle from data/resources.gresource.xml
    // The shortcuts UI is bundled as gtk/help-overlay.ui which GTK
    // automatically associates with win.show-help-overlay action
    // and Ctrl+? accelerator.
    // Only needed for the GUI build — skipped in headless CLI mode.
    #[cfg(feature = "gui")]
    glib_build_tools::compile_resources(
        &["data"],
        "data/resources.gresource.xml",
        "io.github.SlobCoder.qr_studio.gresource",
    );
}
