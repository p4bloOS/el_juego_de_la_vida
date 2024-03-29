#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Esconde la ventana de consola en Windows en release


// Cuando se compila en nativo:
#[cfg(not(target_arch = "wasm32"))]
fn main() { 
    // Trazas en la salida estándar (si se ejecuta con `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "El juego de la vida",
        native_options,
        Box::new(|cc| Box::new(el_juego_de_la_vida::TemplateApp::new(cc))),
    ).unwrap();
}

// Cuando se compila para web usando trunk.
#[cfg(target_arch = "wasm32")]
fn main() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "the_canvas_id", // hardcode it
            web_options,
            Box::new(|cc| Box::new(el_juego_de_la_vida::TemplateApp::new(cc))),
        )
        .await
        .expect("failed to start eframe");
    });
}
