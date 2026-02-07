use bevy::prelude::*;
use bitflags::bitflags;
use std::{time::{Duration, Instant}};
use sysinfo::System;

bitflags! {
    pub struct GlobalFlags: u8 {
        const IS_DEBUG = 1 << 0;
        const IS_LOADING = 1 << 1;
        const IS_SIGNED_IN = 1 << 2;
        const IS_MOBILE = 1 << 3;
        const DEBUG_OVERLAY = 1 << 4;
        const IS_FIFO = 1 << 5;
        const IS_DARK_MODE = 1 << 6;
        const IN_GAME = 1 << 7;
    }
}

bitflags! {
    pub struct DebugFlags: u8 {
        const VSYNC = 1 << 0;
        const FPS = 1 << 1;
        const CPU = 1 << 2;
        const MEM = 1 << 3;
        const VMEM = 1 << 4;
        const DISK = 1 << 5;
        const FILES = 1 << 6;
        const RUNTIME = 1 << 7;
    }
}

#[derive(Resource)]
pub struct GlobalSettings {
    pub flags: GlobalFlags,
    pub button_width_multiplier: f32,
    pub button_height_multiplier: f32,
    pub dbg_flags: DebugFlags,
}

impl Default for GlobalSettings {
    fn default() -> Self {
        Self {
            flags: GlobalFlags::empty(),
            button_width_multiplier: 1.0 / 20.0,
            button_height_multiplier: 1.0 / 30.0,
            dbg_flags: DebugFlags::empty(),
        }
    }
}

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
