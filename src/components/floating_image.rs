use super::percentage_slider;

#[derive(PartialEq)]
enum AttachState {
    Get,
    Set(egui::Pos2),
}

pub struct FloatingImage {
    path: String,
    settings: bool,
    pub open: bool,
    transparency: u8,
    attach_state: Option<AttachState>,
}

impl FloatingImage {
    pub fn new(path: String) -> Self {
        Self {
            path,
            open: true,
            settings: false,
            transparency: 127,
            attach_state: None,
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        use egui::Widget;

        let attach_state_id = egui::Id::unique("attach_state_position");

        let response = if let Some(AttachState::Set(position)) = self
            .attach_state
            .take_if(|state| matches!(state, AttachState::Set(_)))
        {
            egui::Area::new(egui::Id::unique(&self.path)).current_pos(position)
        } else {
            egui::Area::new(egui::Id::unique(&self.path))
        }
        .show(ctx, |ui| {
            if self.settings {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    if ui.button("X").clicked() {
                        self.open = !self.open;
                    }

                    if ui.button("\u{2316}").clicked() {
                        self.attach_state = Some(AttachState::Get);
                        self.settings = false;
                    }

                    percentage_slider(&mut self.transparency).ui(ui);
                });
            }

            let image = egui::Image::new(&self.path)
                .fit_to_original_size(1.0)
                .tint(egui::Color32::from_white_alpha(self.transparency))
                .sense(egui::Sense::click())
                .ui(ui);

            if image.clicked() {
                if let Some(position) =
                    ctx.data_mut(|data| data.remove_temp::<egui::Pos2>(attach_state_id))
                {
                    self.attach_state = Some(AttachState::Set(position))
                }
            }

            if image.double_clicked() {
                self.settings = !self.settings;
            }
        });

        if let Some(AttachState::Get) = self
            .attach_state
            .take_if(|state| matches!(state, AttachState::Get))
        {
            ctx.data_mut(|data| data.insert_persisted(attach_state_id, response.response.rect.min));
        }
    }
}
