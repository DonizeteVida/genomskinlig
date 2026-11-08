use super::percentage_slider;

pub struct FloatingImage {
    path: String,
    settings: bool,
    pub open: bool,
    transparency: u8,
    position: (egui::Pos2, Option<egui::Pos2>),
}

impl FloatingImage {
    pub fn new(path: String) -> Self {
        Self {
            path,
            open: true,
            settings: false,
            transparency: 127,
            position: (egui::Pos2::ZERO, None),
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        use egui::Widget;

        let response = egui::Area::new(egui::Id::unique(&self.path))
            .current_pos(self.position.1.unwrap_or(self.position.0))
            .show(ctx, |ui| {
                if self.settings {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        if ui.button("X").clicked() {
                            self.open = !self.open;
                        }

                        if ui.button("\u{2316}").clicked() {
                            let id = egui::Id::unique("floating_image_anchor_position");

                            if let Some(position) =
                                ctx.data_mut(|data| data.get_persisted::<egui::Pos2>(id))
                            {
                                ctx.data_mut(|data| data.remove::<egui::Pos2>(id));
                                self.position.1 = Some(position);
                            } else {
                                ctx.data_mut(|data| data.insert_persisted(id, self.position.0));
                            }
                            self.settings = false;
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

        self.position.0 = response.response.rect.min;
        if Some(self.position.0) == self.position.1 {
            self.position.1 = None;
        }
    }
}
