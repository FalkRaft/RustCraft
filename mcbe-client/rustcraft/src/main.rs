#![feature(default_field_values)]
#![recursion_limit = "256"]

use bevy::{
    prelude::*,
    window::{CompositeAlphaMode, CursorOptions, ExitCondition, PresentMode},
};
use bevy_egui::EguiPlugin;
use bevy_egui_kbgp::KbgpPlugin;
use std::f32;

mod data;
mod egui_dbg;
mod fps;
mod input;
mod setup;
mod ui;
mod update;
mod window;

use crate::input::input_system;
use crate::setup::setup;
use crate::update::update;
use crate::window::BevyWindowPlugin;
use crate::{
    data::GlobalSettings,
    fps::{fps_counter_system, fps_title_system, frame_cap_system_improved, frame_start_system},
};
use crate::{
    data::{FpsCap, FpsState, FrameStart},
    ui::GameUIPlugin,
};

/// Application entry
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgba(1.0, 0.2, 0.25, 0.75)))
        // FPS tracking resource
        .insert_resource(FpsState::default())
        // debug toggle (starts disabled)
        .insert_resource(GlobalSettings::default())
        // FPS cap resource (start with VSync)
        .insert_resource(FpsCap::default())
        // frame start timestamp resource (initialized to now)
        .insert_resource(FrameStart::now())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_cursor_options: Some(CursorOptions {
                visible: true,
                ..Default::default()
            }),
            exit_condition: ExitCondition::OnAllClosed,
            close_when_requested: true,
            primary_window: Some(Window {
                // default to AutoVsync; switching to explicit Fifo/Immediate is handled by the input handler
                // Hide the OS titlebar at startup — we'll restore it after 5 seconds in the `update` system.
                present_mode: PresentMode::AutoVsync,
                title: "RustCraft".into(),
                resize_constraints: WindowResizeConstraints {
                    min_width: 800.0,
                    min_height: 600.0,
                    max_width: f32::INFINITY,
                    max_height: f32::INFINITY,
                },
                resizable: true,
                fullsize_content_view: false,
                titlebar_shown: true,
                titlebar_show_title: true,
                titlebar_show_buttons: true,
                titlebar_transparent: false,
                transparent: false,
                composite_alpha_mode: CompositeAlphaMode::PostMultiplied,
                ..Default::default()
            }),
            ..Default::default()
        }))
        // add egui overlay
        // .add_plugins(EguiPlugin {
        //     enable_multipass_for_primary_context: false, // deprecated
        //     ui_render_order: bevy_egui::UiRenderOrder::EguiAboveBevyUi,
        //     bindless_mode_array_size: NonZero::new(1024),
        // })
        .add_plugins(EguiPlugin::default())
        .add_plugins(KbgpPlugin)
        .add_plugins(BevyWindowPlugin::default())
        .add_plugins(GameUIPlugin)
        // startup
        .add_systems(PreStartup, setup)
        // record frame start early in the frame
        .add_systems(PreUpdate, frame_start_system)
        // main update systems
        .add_systems(PreUpdate, input_system)
        .add_systems(Update, fps_counter_system)
        .add_systems(Update, fps_title_system)
        .add_systems(Update, update)
        // .add_systems(PostUpdate, (egui_debug_system, ui::ui_system))
        // frame cap runs late in the frame
        // UI system is registered by `EguiUIPlugin`; do not duplicate scheduling here
        .add_systems(PostUpdate, frame_cap_system_improved)
        .run();
}
