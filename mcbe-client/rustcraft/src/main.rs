use bevy::{
    prelude::*,
    window::{CompositeAlphaMode, CursorOptions, ExitCondition, PresentMode},
};
use bevy_egui::EguiPlugin;
use std::f32;

mod egui_dbg;
mod fps;
mod input;
mod setup;
mod structs;
mod update;

use crate::egui_dbg::egui_debug_system;
use crate::fps::{
    fps_counter_system, fps_title_system, frame_cap_system_improved, frame_start_system,
};
use crate::input::input_system;
use crate::setup::setup;
use crate::structs::{DebugState, FpsCap, FpsState, FrameStart};
use crate::update::update;

/// Application entry
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgba(1.0, 0.2, 0.25, 0.5)))
        // FPS tracking resource
        .insert_resource(FpsState::default())
        // debug toggle (starts disabled)
        .insert_resource(DebugState::default())
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
                fullsize_content_view: true,
                titlebar_shown: true,
                titlebar_show_title: true,
                titlebar_show_buttons: true,
                titlebar_transparent: true,
                transparent: false,
                composite_alpha_mode: CompositeAlphaMode::Auto,
                ..Default::default()
            }),
            ..Default::default()
        }))
        // add egui overlay
        .add_plugins(EguiPlugin::default())
        // startup
        .add_systems(Startup, setup)
        // record frame start early in the frame
        .add_systems(PreUpdate, frame_start_system)
        // main update systems
        .add_systems(Update, input_system)
        .add_systems(Update, fps_counter_system)
        .add_systems(Update, fps_title_system)
        .add_systems(Update, egui_debug_system)
        .add_systems(Update, update)
        // frame cap runs late in the frame
        .add_systems(PostUpdate, frame_cap_system_improved)
        .run();
}
