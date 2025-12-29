#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{GlobalShortcutManager, Manager};

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      // Wir holen uns das Hauptfenster
      let window = app.get_window("main").unwrap();
      
      // Klonen für den Zugriff innerhalb der Tastenkombination
      let window_clone = window.clone();

      // Hier registrieren wir den Hotkey: ALT + R (für Raze Clan)
      let mut shortcut = app.global_shortcut_manager();
      shortcut
        .register("Alt+R", move || {
          // Logik: Wenn sichtbar -> Verstecken. Wenn unsichtbar -> Zeigen.
          if window_clone.is_visible().unwrap() {
            window_clone.hide().unwrap();
          } else {
            window_clone.show().unwrap();
            window_clone.set_focus().unwrap();
          }
        })
        .expect("Tastenkombination konnte nicht registriert werden!");

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("Fehler beim Starten der App");
}
