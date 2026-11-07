fn main() -> anyhow::Result<()> {
    let options = eframe::NativeOptions {
        ..Default::default()
    };

    let mut files = None;

    eframe::run_ui_native("Genomskinlig", options, move |ui, frame| {
        if ui.button("Open file").clicked() {
            files = rfd::FileDialog::new().pick_files();
        }

        if let Some(ref files) = files {
            for file in files {
                ui.monospace(file.display().to_string());
            }
        }
    })?;

    Ok(())
}
