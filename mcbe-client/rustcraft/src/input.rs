use bevy::{prelude::{Query, Res, ResMut, Window, KeyCode}, window::{WindowMode}};
use bevy::input::ButtonInput;
use bevy::window::{MonitorSelection, VideoModeSelection, PresentMode};
use log::info;

use crate::structs::{DebugState, FpsCap, FpsMode};

/// Input handling:
/// - F11 toggles maximize
/// - F3 toggles debug overlay
/// - F2 cycles present modes
/// - F1 cycles FPS cap presets
pub fn input_system(
    mut windows: Query<&mut Window>,
    keys: Res<ButtonInput<KeyCode>>,
    mut debug: ResMut<DebugState>,
    mut cap: ResMut<FpsCap>,
) {
    // assume single primary window
    if let Ok(mut window) = windows.single_mut() {
        // toggle maximized with F11
        let mut maximized = false;
        if keys.just_pressed(KeyCode::F11) || keys.just_pressed(KeyCode::F10) {
            maximized = !maximized;
            window.set_maximized(maximized);
            info!("Window maximized: {}", maximized);
        }

        // toggle debug overlay with F3
        if keys.just_pressed(KeyCode::F3) {
            debug.enabled = !debug.enabled;
            let enabled = debug.enabled;
            info!(
                "Debug overlay {}",
                if enabled { "enabled" } else { "disabled" }
            );
        }

        if keys.just_pressed(KeyCode::Escape) {
            std::process::exit(0);
        }

        if keys.just_pressed(KeyCode::F4) {
            window.mode = match window.mode {
                WindowMode::Windowed => {
                    WindowMode::Fullscreen(MonitorSelection::Current, VideoModeSelection::Current)
                }
                WindowMode::Fullscreen(MonitorSelection::Current, VideoModeSelection::Current) => {
                    WindowMode::Windowed
                }
                WindowMode::BorderlessFullscreen(MonitorSelection::Current) => WindowMode::Windowed,
                _ => WindowMode::Windowed,
            };
            info!("Toggled {:?} mode", window.mode);
        }

        // cycle present modes with F2
        if keys.just_pressed(KeyCode::F2) {
            let modes = [
                PresentMode::Fifo,
                PresentMode::FifoRelaxed,
                PresentMode::Immediate,
                PresentMode::AutoVsync,
                PresentMode::AutoNoVsync,
                PresentMode::Mailbox,
            ];
            let current = window.present_mode;
            let idx = modes
                .iter()
                .position(|m| m == &current)
                .unwrap_or_else(|| 0);
            let next = modes[(idx + 1) % modes.len()];
            window.present_mode = next;
            info!("Changed present mode to {:?}", next);
        }

        // cycle FPS cap presets with F1
        if keys.just_pressed(KeyCode::F1) {
            const PRESETS: &[FpsMode] = &[
                FpsMode::VSync,
                FpsMode::Uncapped,
                FpsMode::Manual(1000),
                FpsMode::Manual(720),
                FpsMode::Manual(540),
                FpsMode::Manual(480),
                FpsMode::Manual(360),
                FpsMode::Manual(240),
                FpsMode::Manual(144),
                FpsMode::Manual(120),
                FpsMode::Manual(60),
                FpsMode::Manual(30),
                FpsMode::Manual(20),
                FpsMode::Manual(15),
                FpsMode::Manual(10),
                FpsMode::Manual(5),
            ];

            let current = cap.mode;
            let next = PRESETS
                .iter()
                .cycle()
                .skip_while(|p| **p != current)
                .nth(1)
                .copied()
                .unwrap_or(PRESETS[0]);
            cap.mode = next;

            // Choose an appropriate present mode for VSync vs not-VSync:
            match cap.mode {
                FpsMode::VSync => {
                    // explicit vsync (Fifo) is preferred to avoid tearing
                    window.present_mode = PresentMode::Fifo;
                }
                FpsMode::Uncapped | FpsMode::Manual(_) => {
                    // immediate allows manual sleeping/spin to control pacing
                    window.present_mode = PresentMode::Immediate;
                }
            }

            info!("FPS cap mode changed: {:?}", cap.mode);
        }
    }
}