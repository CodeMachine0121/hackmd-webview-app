use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }

      // Get the main window
      let _window = app.get_webview_window("main").unwrap();
      
      // Navigate directly to HackMD.io
      #[cfg(debug_assertions)]
      println!("Debug mode: Load dev URL from tauri.conf.json");
      
      #[cfg(not(debug_assertions))]
      _window.eval("window.location.replace('https://hackmd.io');").unwrap();

      Ok(())
    })
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_http::init())
    .invoke_handler(tauri::generate_handler![])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
