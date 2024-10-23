pub fn gui(project: Option<crate::core::project::Project>) {
    let _ = tauri::Builder::default()
        .setup(|app| {
            tauri::WindowBuilder::new(app, "label", tauri::WindowUrl::App("index.html".into()))
                .data_directory(None)
                .title("Conduct")
                .build()?;

            Ok(())
        })
        .run(tauri::generate_context!());
}
