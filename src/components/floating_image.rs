use super::percentage_slider;

pub struct FloatingImage {
    pub open: bool,
    transparency: u8,
    path: String,
}

impl FloatingImage {
    pub fn new(path: String) -> Self {
        Self {
            open: true,
            transparency: 127,
            path,
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        use egui::Widget;

        egui::Window::new(&self.path)
            .frame(egui::Frame::new())
            .open(&mut self.open)
            .show(ctx, |ui| {
                percentage_slider(&mut self.transparency).ui(ui);
                egui::Image::new(&self.path)
                    .fit_to_original_size(1.0)
                    .tint(egui::Color32::from_white_alpha(self.transparency))
                    .ui(ui);
            });
    }
}
