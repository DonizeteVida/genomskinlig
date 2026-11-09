mod components;

use components::{FloatingImage, percentage_slider};

struct App {
    transparency: u8,
    floating_images: Vec<FloatingImage>,
}

impl App {
    fn new() -> Self {
        Self {
            transparency: 127,
            floating_images: vec![],
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
            floating_images,
        } = self;

        percentage_slider(transparency).ui(ui);

        if ui.button("Open file").clicked() {
            if let Some(new_floating_images) =
                rfd::FileDialog::new().pick_files().map(|path_bufs| {
                    path_bufs
                        .into_iter()
                        .flat_map(url::Url::from_file_path)
                        .map(|url| url.to_string())
                        .map(FloatingImage::new)
                })
            {
                floating_images.extend(new_floating_images);
            }
        }

        for floating_image in floating_images {
            floating_image.ui(ui.ctx());
        }

        self.floating_images.retain(|fi| fi.open);
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
            Ok(Box::new(App::new()))
        }),
    )?;

    Ok(())
}
