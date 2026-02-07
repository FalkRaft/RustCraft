use bevy::input::ButtonInput;
use bevy::window::{MonitorSelection, PresentMode, VideoModeSelection};
use bevy::{prelude::*, window::WindowMode};
use log::info;

use crate::data::{DebugFlags, FpsCap, FpsMode, GlobalFlags, GlobalSettings};

/// Input handling:
/// - F11 toggles maximize
/// - F3 toggles debug overlay
/// - F2 cycles present modes
/// - F1 cycles FPS cap presets
pub fn input_system(
    mut windows: Query<&mut Window>,
    keys: Res<ButtonInput<KeyCode>>,
    mut cap: ResMut<FpsCap>,
    mut global_settings: ResMut<GlobalSettings>,
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
        if keys.just_released(KeyCode::F3) {
            global_settings.flags.toggle(GlobalFlags::DEBUG_OVERLAY);
            info!("Debug overlay is set to {}.", global_settings.flags.contains(GlobalFlags::DEBUG_OVERLAY));
        }

        if keys.all_pressed([KeyCode::F3, KeyCode::ControlLeft]) {
            warn!(
                "CRASH TEST: {:?} + {:?} pressed, crashing in 10 seconds...",
                KeyCode::F3,
                KeyCode::ControlLeft
            );
            std::thread::sleep(std::time::Duration::from_secs(1));
            for i in 0..10 {
                warn!("CRASH TEST: Crashing in {}s", 10 - i);
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
            std::process::abort();
        }

        if keys.all_pressed([KeyCode::ControlLeft, KeyCode::KeyQ]) {
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

            if global_settings.dbg_flags.contains(DebugFlags::VSYNC) {
                cap.mode = FpsMode::VSync;
            }

            info!("FPS cap mode changed: {:?}", cap.mode);
        }
    }
}
