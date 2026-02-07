use bevy::{
    prelude::*,
    window::{PrimaryWindow},
};
use bevy_egui::EguiContexts;
use egui::{CornerRadius, containers::menu::MenuConfig};

use crate::data::{GlobalFlags, GlobalSettings};

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (ui_system, crate::egui_dbg::egui_debug_system));
    }
}

pub fn ui_system(
    mut window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut contexts: EguiContexts,
    mut global_settings: ResMut<GlobalSettings>,
) {
    if time.elapsed_secs() >= 5.0 {
        for window in window_query.iter_mut() {
            let (w, h) = (window.width(), window.height());

            for ctx in &mut contexts.ctx_mut().into_iter() {
                // Style
                let mut style = (*ctx.style()).clone();
                style.compact_menu_style = false;
                style.visuals.widgets.noninteractive.bg_fill = egui::Color32::from_black_alpha(128);
                style.visuals.widgets.inactive.bg_fill = egui::Color32::from_black_alpha(192);
                style.visuals.widgets.active.bg_fill = egui::Color32::from_black_alpha(255);
                style.visuals.widgets.hovered.bg_fill = egui::Color32::from_black_alpha(224);
                style.visuals.window_shadow = egui::epaint::Shadow::NONE;
                style.visuals.menu_corner_radius = CornerRadius::ZERO;
                style.visuals.widgets.active.corner_radius = CornerRadius::ZERO;
                style.visuals.widgets.inactive.corner_radius = CornerRadius::ZERO;
                style.visuals.widgets.noninteractive.corner_radius = CornerRadius::ZERO;
                style.visuals.widgets.hovered.corner_radius = CornerRadius::ZERO;
                ctx.set_style(style.clone());

                egui::TopBottomPanel::top("menu-bar").show(ctx, |ui| {
                    egui::MenuBar::new()
                        .config(
                            MenuConfig::new()
                                .close_behavior(egui::PopupCloseBehavior::CloseOnClick)
                                .style(style),
                        )
                        .ui(ui, |ui| {
                            ui.menu_button("App", |ui| {
                                if ui.button("Quit").clicked() {
                                    std::process::exit(0);
                                }
                            });
                            ui.menu_button("File", |ui| {
                                if ui.button("Open").clicked() {
                                    info!("TODO: Functionality.");
                                }
                            });
                            ui.menu_button("Settings", |ui| {
                                if ui.button("Toggle Debug Mode").clicked() {
                                    global_settings.flags.toggle(GlobalFlags::DEBUG_OVERLAY);
                                }
                            });
                        });
                });

                egui::CentralPanel::default().show(&ctx, |ui| {
                    ui.centered_and_justified(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(20.0);
                            ui.heading("Welcome to RustCraft!");
                            ui.add_space(10.0);
                            ui.label("This is a Minecraft Bedrock Edition client written in Rust.");
                            ui.label("Use the WASD keys to move around, and mouse to look.");
                            ui.add_space(10.0);
                            ui.label(
                                "This is an early prototype with many features still missing.",
                            );
                            ui.label(
                                "Expect bugs and crashes, and feel free to contribute on GitHub!",
                            );
                            ui.add_space(20.0);
                        });
                    });
                });
            }
        }
    }
}
