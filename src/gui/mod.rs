pub fn gui(project: crate::core::project::Project) {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
