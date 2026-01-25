use bevy::prelude::{Entity, Resource, Timer, TimerMode};
use sysinfo::{System};
use std::time::{Duration, Instant};

#[derive(Resource)]
pub struct TitleBar(pub Entity);

#[derive(Resource)]
pub struct SysInfo {
    pub sys: System,
}

/// Resource that stores the timestamp at the start of the current frame.
#[derive(Resource, Debug, Clone, Copy)]
pub struct FrameStart(Instant);

impl FrameStart {
    pub fn now() -> Self {
        FrameStart(Instant::now())
    }

    pub fn elapsed(&self) -> Duration {
        Instant::now().duration_since(self.0)
    }

    pub fn set_now(&mut self) {
        self.0 = Instant::now();
    }
}

/// Tracks frames over a 1 second interval and exposes the last computed FPS.
#[derive(Resource)]
pub struct FpsState {
    pub timer: Timer,
    pub frames: u32,
    pub latest_fps: f64,
}

impl Default for FpsState {
    fn default() -> Self {
        Self {
            // compute once per second
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            frames: 0,
            latest_fps: 0.0,
        }
    }
}

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Dark,
    Light,
    Unspecified,
}

#[derive(Resource, Debug)]
pub struct SystemThemeState {
    pub mode: ThemeMode,
}

/// Whether debug UI is enabled (starts false).
#[derive(Resource)]
pub struct DebugState {
    pub enabled: bool,
}

impl Default for DebugState {
    fn default() -> Self {
        Self { enabled: false }
    }
}

/// FPS cap modes and resource
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FpsMode {
    VSync,       // use PresentMode::Fifo (vsync active)
    Uncapped,    // no cap, Immediate present
    Manual(u32), // cap to N FPS by sleeping + spinning
}

#[derive(Resource)]
pub struct FpsCap {
    pub mode: FpsMode,
}

impl Default for FpsCap {
    fn default() -> Self {
        Self {
            mode: FpsMode::VSync,
        }
    }
}
