fn main() -> anyhow::Result<()> {
    let options = eframe::NativeOptions {
        ..Default::default()
    };

    eframe::run_ui_native("Genomskinlig", options, |ui, frame| {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Genomskinlig");
        });
    })?;

    Ok(())
}
