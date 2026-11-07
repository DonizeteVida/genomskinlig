struct App {
    transparency: u8,
    files: Option<Vec<String>>,
}

impl Default for App {
    fn default() -> App {
        App {
            transparency: 255,
            files: None,
        }
    }
}

impl eframe::App for App {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Color32::from_rgba_unmultiplied(0, 0, 0, self.transparency).to_normalized_gamma_f32()
    }

    fn ui(&mut self, ui: &mut egui::Ui, _: &mut eframe::Frame) {
        use egui::Widget;

        let Self {
            transparency,
            files,
        } = self;

        egui::Slider::new(transparency, 0..=255)
            .custom_formatter(|number, _| {
                format!("{value:.0}%", value = (number as f32 / 255.0) * 100.0)
            })
            .custom_parser(|str| {
                if str.chars().last() == Some('%') {
                    &str[..str.len() - 1]
                } else {
                    &str
                }
                .parse::<f64>()
                .ok()
                .map(|number| (number / 100.0) * 255.0)
            })
            .text("Transparency")
            .ui(ui);

        if ui.button("Open file").clicked() {
            *files = rfd::FileDialog::new().pick_files().as_ref().map(|v| {
                v.iter()
                    .map(|pb| format!("file://{path}", path = pb.display()))
                    .collect::<Vec<String>>()
            });
        }

        egui::ScrollArea::both().show(ui, |ui| {
            if let Some(ref files) = self.files {
                for file in files {
                    ui.monospace(file);
                    egui::Image::new(file).corner_radius(10).ui(ui);
                }
            }
        });
    }
}

fn main() -> anyhow::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_transparent(true)
            .with_has_shadow(false),
        ..Default::default()
    };

    eframe::run_native(
        "Genomskinlig",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<App>::default())
        }),
    )?;

    Ok(())
}
