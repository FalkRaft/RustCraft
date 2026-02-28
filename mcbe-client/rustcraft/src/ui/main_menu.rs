pub fn main_menu_ui(ui: &mut egui::Ui) {
    ui.centered_and_justified(|ui| {
        ui.add_space(100.0);
        if ui.button("Settings").clicked() {
            #[cfg(debug_assertions)]
            todo!("Do settings screen");

            #[cfg(not(debug_assertions))]
            info!("Do settings screen");
        }
    });
}
