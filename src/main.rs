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

struct FloatingImage {
    transparency: u8,
    path: String,
}

impl FloatingImage {
    fn new(path: String) -> Self {
        Self {
            transparency: 200,
            path,
        }
    }

    fn ui(&mut self, ctx: &egui::Context) {
        use egui::Widget;

        egui::Window::new(&self.path)
            .frame(egui::Frame::new())
            .show(ctx, |ui| {
                percentage_slider(&mut self.transparency).ui(ui);
                egui::Image::new(&self.path)
                    .tint(egui::Color32::from_white_alpha(self.transparency))
                    .ui(ui);
            });
    }
}

fn percentage_slider(value: &mut u8) -> egui::Slider<'_> {
    egui::Slider::new(value, 0..=255)
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
            let mut new_floating_images = rfd::FileDialog::new()
                .pick_files()
                .as_ref()
                .map(|path_bufs| {
                    path_bufs
                        .iter()
                        .map(|pb| format!("file://{path}", path = pb.display()))
                        .map(FloatingImage::new)
                        .collect::<Vec<_>>()
                })
                .unwrap_or(vec![]);

            floating_images.append(&mut new_floating_images);
        }

        for floating_image in floating_images {
            floating_image.ui(ui.ctx());
        }
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
