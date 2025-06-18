use bevy::prelude::*;
use bevy_egui::*;

pub fn ui_example_system(mut contexts: EguiContexts) {
    // Use the safer approach with proper error handling
    egui::Window::new("Sanity Check")
        .default_width(200.0)
        .default_height(100.0)
        .show(contexts.ctx_mut(), |ui| {
            ui.label("If you see this, egui is fine");

            // Safe way to get available space
            let available_space = ui.available_size();
            ui.label(format!(
                "Available: {:.1} x {:.1}",
                available_space.x, available_space.y
            ));
        });
}
