use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPrimaryContextPass};
use crossbeam_channel::{Receiver, Sender};
use egui::{CornerRadius, Id, Memory};
use main_menu::main_menu_ui;
use menu_bar::menu_bar_ui;

pub mod main_menu;
pub mod menu_bar;

use crate::data::{FpsCap, GlobalSettings};

pub struct GameUIPlugin;

#[derive(Resource)]
pub struct FileDialogChannel {
    pub sender: Sender<Option<std::path::PathBuf>>,
    pub receiver: Receiver<Option<std::path::PathBuf>>,
}

#[derive(Resource, Default)]
pub struct MenuBarVisibility {
    pub hidden: bool,
}

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        let (sender, receiver) = crossbeam_channel::unbounded();
        app.insert_resource(FileDialogChannel { sender, receiver })
            .insert_resource(MenuBarVisibility::default());
        app.add_systems(Update, menu_bar::file_dialog_system);
        app.add_systems(
            EguiPrimaryContextPass,
            (ui_system, crate::egui_dbg::egui_debug_system),
        );
        // app.add_systems(Update, (ui_system, crate::egui_dbg::egui_debug_system));
    }
}

pub fn ui_system(
    mut contexts: EguiContexts,
    mut global_settings: ResMut<GlobalSettings>,
    file_dialog: Res<FileDialogChannel>,
    mut fps_cap: ResMut<FpsCap>,
    mut windows: Query<&mut Window>,
    keys: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    mut menu_bar_visibility: ResMut<MenuBarVisibility>,
) {
    for mut window in &mut windows.iter_mut() {
        let ctx = contexts.ctx_mut().unwrap();
        let mut style = (*ctx.style()).clone();
        style.visuals.widgets.active.corner_radius = CornerRadius::ZERO;
        style.visuals.widgets.hovered.corner_radius = CornerRadius::ZERO;
        style.visuals.widgets.inactive.corner_radius = CornerRadius::ZERO;
        style.visuals.widgets.noninteractive.corner_radius = CornerRadius::ZERO;
        style.visuals.widgets.open.corner_radius = CornerRadius::ZERO;
        ctx.set_style(style);
        ctx.all_styles_mut(|style| {
            style.visuals.widgets.active.corner_radius = CornerRadius::ZERO;
            style.visuals.widgets.hovered.corner_radius = CornerRadius::ZERO;
            style.visuals.widgets.inactive.corner_radius = CornerRadius::ZERO;
            style.visuals.widgets.noninteractive.corner_radius = CornerRadius::ZERO;
            style.visuals.widgets.open.corner_radius = CornerRadius::ZERO;
        });

        let maybe_gamepad = gamepads.iter().next();

        let f3_pressed = keys.pressed(KeyCode::AltLeft) || keys.just_pressed(KeyCode::AltRight);
        let t_pressed = keys.pressed(KeyCode::KeyT);
        let f3_just = keys.just_pressed(KeyCode::AltLeft) || keys.just_pressed(KeyCode::AltRight);
        let t_just = keys.just_pressed(KeyCode::KeyT);

        // Combo is true if *either* key was just pressed **while** the other is already held
        let toggle_menu_bar = (f3_just && t_pressed)        // F3 pressed last
            || (t_just && f3_pressed)        // T pressed last
            || maybe_gamepad
            .map(|gp| gp.just_pressed(GamepadButton::Select))
            .unwrap_or(false);

        if toggle_menu_bar {
            menu_bar_visibility.hidden = !menu_bar_visibility.hidden;
            ctx.memory(|memory: &Memory| memory.clone())
                .request_focus(Id::new("menu_bar_focus"));
        }

        if !menu_bar_visibility.hidden {
            egui::TopBottomPanel::top("menu").show(ctx, |ui| {
                menu_bar_ui(
                    ui,
                    &mut global_settings,
                    &file_dialog,
                    &mut fps_cap,
                    &mut window,
                );
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            main_menu_ui(ui);
        });
    }
}
