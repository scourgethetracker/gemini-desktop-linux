use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");

    let init_script = r#"
        document.addEventListener('keydown', function(e) {
            if (e.ctrlKey) {
                let currentZoom = parseFloat(document.body.style.zoom) || 1.0;
                if (e.key === '=' || e.key === '+') {
                    document.body.style.zoom = currentZoom + 0.1;
                } else if (e.key === '-') {
                    document.body.style.zoom = currentZoom - 0.1;
                } else if (e.key === '0') {
                    document.body.style.zoom = 1.0;
                }
            }
        });
    "#;

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, shortcut, event| {
                    if event.state == ShortcutState::Pressed {
                        if shortcut.matches(Modifiers::CONTROL, Code::Space) {
                            let chatbar = app.get_webview_window("chatbar");
                            let main = app.get_webview_window("main");

                            if let (Some(chatbar_window), Some(main_window)) = (chatbar, main) {
                                let is_chatbar_visible = chatbar_window.is_visible().unwrap_or(false);
                                if is_chatbar_visible {
                                    let _ = chatbar_window.hide();
                                    let _ = main_window.show();
                                    let _ = main_window.set_focus();
                                } else {
                                    let _ = main_window.hide();
                                    let _ = chatbar_window.show();
                                    let _ = chatbar_window.set_focus();
                                }
                            }
                        }
                    }
                })
                .build(),
        )
        .setup(move |app| {
            // Register global shortcut
            let ctrl_space = Shortcut::new(Some(Modifiers::CONTROL), Code::Space);
            app.global_shortcut().register(ctrl_space)?;

            // Create main window
            let main_window = WebviewWindowBuilder::new(
                app,
                "main",
                WebviewUrl::External("https://gemini.google.com/".parse().unwrap()),
            )
            .title("Gemini Desktop")
            .inner_size(1024.0, 768.0)
            .initialization_script(init_script)
            .build()?;

            let app_handle = app.handle().clone();
            main_window.on_window_event(move |event| {
                if let tauri::WindowEvent::Destroyed = event {
                    app_handle.exit(0);
                }
            });

            // Create chatbar window
            WebviewWindowBuilder::new(
                app,
                "chatbar",
                WebviewUrl::External("https://gemini.google.com/".parse().unwrap()),
            )
            .title("Gemini Chat")
            .inner_size(600.0, 400.0)
            .decorations(false)
            .always_on_top(true)
            .visible(false)
            .skip_taskbar(true)
            .center()
            .transparent(true)
            .initialization_script(init_script)
            .build()?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
