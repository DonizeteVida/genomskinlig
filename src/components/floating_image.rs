use super::percentage_slider;

pub struct FloatingImage {
    pub open: bool,
    settings: bool,
    transparency: u8,
    path: String,
}

impl FloatingImage {
    pub fn new(path: String) -> Self {
        Self {
            open: true,
            settings: false,
            transparency: 127,
            path,
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        use egui::Widget;

        egui::Area::new(egui::Id::unique(&self.path)).show(ctx, |ui| {
            if self.settings {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    if ui.button("X").clicked() {
                        self.open = !self.open;
                    }

                    percentage_slider(&mut self.transparency).ui(ui);
                });
            }

            if egui::Image::new(&self.path)
                .fit_to_original_size(1.0)
                .tint(egui::Color32::from_white_alpha(self.transparency))
                .sense(egui::Sense::click())
                .ui(ui)
                .double_clicked()
            {
                self.settings = !self.settings;
            }
        });
    }
}
