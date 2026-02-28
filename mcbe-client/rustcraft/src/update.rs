use crate::data::{GlobalFlags, GlobalSettings, SystemThemeState, ThemeMode};
use bevy::prelude::*;
use thread_priority::{ThreadPriority, set_current_thread_priority};

pub fn update(mut windows: Query<&mut Window>, mut system_theme_state: ResMut<SystemThemeState>) {
    if let Ok(mut window) = windows.single_mut() {
        window.resize_constraints.min_width = 800.0;
        window.resize_constraints.min_height = 600.0;

        assert!(set_current_thread_priority(ThreadPriority::Max).is_ok());

        // detect current system theme
        let detected_mode = match dark_light::detect() {
            Ok(dark_light::Mode::Dark) => ThemeMode::Dark,
            Ok(dark_light::Mode::Light) => ThemeMode::Light,
            Ok(dark_light::Mode::Unspecified) | Err(_) => ThemeMode::Unspecified,
        };

        let mut global_settings = GlobalSettings::default();
        global_settings
            .flags
            .set(GlobalFlags::IS_DARK_MODE, detected_mode == ThemeMode::Dark);
        global_settings
            .flags
            .set(GlobalFlags::IS_MOBILE, cfg!(target_arch = "wasm32"));
        global_settings
            .flags
            .set(GlobalFlags::DEBUG_OVERLAY, cfg!(debug_assertions));
        global_settings
            .flags
            .set(GlobalFlags::IS_FIFO, GlobalFlags::IN_GAME.bits() != 0);

        system_theme_state.mode = detected_mode;
        // window.present_mode = global_settings.game_settings.present_mode;
    }
}
