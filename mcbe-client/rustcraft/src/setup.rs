use bevy::prelude::*;
use dark_light;
use sysinfo::System;

use crate::data::{GlobalFlags, GlobalSettings, SysInfo, SystemThemeState, ThemeMode};

pub fn setup(mut commands: Commands) {
    // spawn a simple 2D camera
    commands.spawn(Camera2d::default());

    let system_theme = dark_light::detect();
    let theme_mode = match system_theme {
        Ok(dark_light::Mode::Dark) => ThemeMode::Dark,
        Ok(dark_light::Mode::Light) => ThemeMode::Light,
        Ok(dark_light::Mode::Unspecified) | Err(_) => ThemeMode::Unspecified,
    };

    let mut global_settings = GlobalSettings::default();
    global_settings
        .flags
        .set(GlobalFlags::IS_DARK_MODE, theme_mode == ThemeMode::Dark);
    global_settings
        .flags
        .set(GlobalFlags::IS_MOBILE, cfg!(target_arch = "wasm32"));
    global_settings
        .flags
        .set(GlobalFlags::DEBUG_OVERLAY, cfg!(debug_assertions));

    commands.insert_resource(SysInfo {
        sys: System::new_all(),
    });

    // insert the initial system theme
    commands.insert_resource(SystemThemeState { mode: theme_mode });
}
