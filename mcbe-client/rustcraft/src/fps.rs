use crate::data::{FpsCap, FpsMode, FpsState, FrameStart, GlobalFlags, GlobalSettings};
use bevy::prelude::{Query, Res, ResMut, Time, Window};
use std::hint::spin_loop;
use std::time::Duration;

/// Count frames and compute FPS once per second into `FpsState`.
pub fn fps_counter_system(time: Res<Time>, mut fps: ResMut<FpsState>) {
    fps.frames = fps.frames.saturating_add(1);
    fps.timer.tick(time.delta());

    if fps.timer.just_finished() {
        // With a 1-second timer, frames counted during the interval == fps.
        fps.latest_fps = fps.frames as f64 / fps.timer.duration().as_secs_f64();
        fps.frames = 0;
    }
}

/// Record the frame start timestamp as early as possible.
pub fn frame_start_system(mut frame_start: ResMut<FrameStart>) {
    frame_start.set_now();
}

/// Improved frame cap: sleep + short spin for precision.
/// Runs in PostUpdate so it measures nearly the whole frame's work time.
pub fn frame_cap_system_improved(frame_start: Res<FrameStart>, cap: Res<FpsCap>) {
    if let FpsMode::Manual(target) = cap.mode {
        if target == 0 {
            return;
        }

        let target_secs = 1.0 / (target as f64);
        // measure elapsed since frame start
        let elapsed = frame_start.elapsed().as_secs_f64();

        if elapsed >= target_secs {
            // already took too long — nothing to do
            return;
        }

        let remaining = target_secs - elapsed;
        // Sleep buffer: wake a little earlier to allow precise spin waiting.
        let sleep_buffer = 0.007_f64; // 7 ms buffer

        if remaining > sleep_buffer {
            let to_sleep = remaining - sleep_buffer;
            std::thread::sleep(Duration::from_secs_f64(to_sleep));
        }

        // Busy-wait until the exact deadline
        while frame_start.elapsed().as_secs_f64() < target_secs {
            spin_loop();
        }
    }
}

/// Update the window title with FPS / frame time / resolution when debug is enabled.
/// When debug is disabled we keep a minimal title.
pub fn fps_title_system(
    mut windows: Query<&mut Window>,
    fps: Res<FpsState>,
    cap: Res<FpsCap>,
    global_settings: Res<GlobalSettings>,
) {
    if let Ok(mut window) = windows.single_mut() {
        if global_settings.flags.contains(GlobalFlags::DEBUG_OVERLAY) && fps.latest_fps > 0.0 {
            let frame_ms = 1000.0 / fps.latest_fps;
            // Use logical resolution from the window if available.
            let (w, h) = (
                window.resolution.width() as u32,
                window.resolution.height() as u32,
            );
            let cap_str = match cap.mode {
                FpsMode::VSync => "VSync".to_string(),
                FpsMode::Uncapped => "Uncapped".to_string(),
                FpsMode::Manual(n) => format!("{} FPS", n),
            };
            window.title = format!(
                "RustCraft - FPS: {:.0} - {:.2} ms - {}x{} - {} - DEBUG",
                fps.latest_fps, frame_ms, w, h, cap_str
            );
        } else {
            // Minimal title
            window.title = "RustCraft".into();
        }
    }
}
