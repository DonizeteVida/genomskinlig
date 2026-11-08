mod floating_image;

pub use floating_image::FloatingImage;

pub fn percentage_slider(value: &mut u8) -> egui::Slider<'_> {
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
