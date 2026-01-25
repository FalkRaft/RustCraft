use bevy::prelude::*;
use bevy::ui::ZIndex as GlobalZIndex;
use dark_light;
use sysinfo::System;
use crate::structs::{TitleBar, SysInfo, ThemeMode, SystemThemeState};

pub fn setup(mut commands: Commands) {
    // spawn a simple 2D camera
    commands.spawn(Camera2d::default());

    let system_theme = dark_light::detect();
    let theme_mode = match system_theme {
        Ok(dark_light::Mode::Dark) => ThemeMode::Dark,
        Ok(dark_light::Mode::Light) => ThemeMode::Light,
        Ok(dark_light::Mode::Unspecified) | Err(_) => ThemeMode::Unspecified,
    };

    let colour = match theme_mode {
        ThemeMode::Dark => BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.75)),
        ThemeMode::Light | ThemeMode::Unspecified => {
            BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.75))
        }
    };

    #[cfg(target_os = "macos")]
    let node = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(32.0),
                ..Default::default()
            },
            BackgroundColor(colour.0),
            GlobalZIndex(i32::MAX),
        ))
        .id();

    #[cfg(target_os = "windows")]
    let node = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(40.0),
                ..Default::default()
            },
            BackgroundColor(colour.0),
            GlobalZIndex(i32::MAX),
        ))
        .id();

    #[cfg(target_os = "linux")]
    let node = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(40.0),
                ..Default::default()
            },
            BackgroundColor(colour.0),
            GlobalZIndex(i32::MAX),
        ))
        .id();

    commands.insert_resource(TitleBar(node));
    commands.insert_resource(SysInfo {
        sys: System::new_all(),
    });

    // insert the initial system theme
    commands.insert_resource(SystemThemeState { mode: theme_mode });
}