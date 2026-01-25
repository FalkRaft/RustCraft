use bevy::prelude::*;
use bevy::window::{MonitorSelection, VideoModeSelection, WindowMode};
use dark_light;
use log::info;
use crate::structs::{SystemThemeState, ThemeMode, TitleBar};

pub fn update(
    mut windows: Query<&mut Window>,
    time: Res<Time>,
    titlebar: Res<TitleBar>,
    mut bgcolour: Query<&mut BackgroundColor>,
    mut theme_state: ResMut<SystemThemeState>,
) {
    if let Ok(mut window) = windows.single_mut() {
        window.resize_constraints.min_width = 800.0;
        window.resize_constraints.min_height = 600.0;

        // detect current system theme
        let detected_mode = match dark_light::detect() {
            Ok(dark_light::Mode::Dark) => ThemeMode::Dark,
            Ok(dark_light::Mode::Light) => ThemeMode::Light,
            Ok(dark_light::Mode::Unspecified) | Err(_) => ThemeMode::Unspecified,
        };

        // if the theme changed, update the titlebar colour
        if detected_mode != theme_state.mode {
            theme_state.mode = detected_mode;
            if let Ok(mut colour) = bgcolour.get_mut(titlebar.0) {
                // choose alpha depending on whether the titlebar is transparent currently
                let alpha = if window.titlebar_transparent {
                    0.75
                } else {
                    1.0
                };
                *colour = match detected_mode {
                    ThemeMode::Dark => BackgroundColor(Color::srgba(0.2, 0.2, 0.2, alpha)),
                    ThemeMode::Light | ThemeMode::Unspecified => {
                        BackgroundColor(Color::srgba(0.9, 0.9, 0.9, alpha))
                    }
                };
            }
            info!(
                "System theme changed: {:?} — updated titlebar colour",
                detected_mode
            );
        }

        // Only on desktop targets: show titlebar after 5s (your existing logic)
        if cfg!(any(
            target_os = "windows",
            target_os = "macos",
            target_os = "linux"
        )) {
            if window.titlebar_transparent && time.elapsed_secs() >= 5.0 {
                window.titlebar_transparent = false;
                if let Ok(mut colour) = bgcolour.get_mut(titlebar.0) {
                    let system_theme = dark_light::detect();
                    *colour = match system_theme {
                        Ok(dark_light::Mode::Dark) => {
                            BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 1.0))
                        }
                        Ok(dark_light::Mode::Light) => {
                            BackgroundColor(Color::srgba(0.9, 0.9, 0.9, 1.0))
                        }
                        Ok(dark_light::Mode::Unspecified) => {
                            BackgroundColor(Color::srgba(0.9, 0.9, 0.9, 1.0))
                        }
                        Err(_) => BackgroundColor(Color::srgba(0.9, 0.9, 0.9, 1.0)),
                    };
                }
                // info!("Titlebar shown after {:.2} seconds", time.elapsed_secs());
            }

            if window.mode
                == WindowMode::Fullscreen(MonitorSelection::Current, VideoModeSelection::Current)
                || window.mode == WindowMode::BorderlessFullscreen(MonitorSelection::Current)
            {
                // In fullscreen modes, hide the titlebar
                if !window.titlebar_transparent {
                    window.titlebar_transparent = true;
                    if let Ok(mut colour) = bgcolour.get_mut(titlebar.0) {
                        let system_theme = dark_light::detect();
                        *colour = match system_theme {
                            Ok(dark_light::Mode::Dark) => {
                                BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.0))
                            }
                            Ok(dark_light::Mode::Light) => {
                                BackgroundColor(Color::srgba(0.9, 0.9, 0.9, 0.0))
                            }
                            Ok(dark_light::Mode::Unspecified) => {
                                BackgroundColor(Color::srgba(0.9, 0.9, 0.9, 0.0))
                            }
                            Err(_) => BackgroundColor(Color::srgba(0.9, 0.9, 0.9, 0.0)),
                        };
                    }
                }
            }
        }
    }
}