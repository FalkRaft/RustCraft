use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use sysinfo::{Pid};
use crate::structs::{DebugState, FpsCap, FpsState, SysInfo};
use bevy::window::Window;
use bevy::time::Time;
use egui::Color32;

/// Render a small egui overlay when debug is enabled.
pub fn egui_debug_system(
    mut contexts: EguiContexts,
    debug: Res<DebugState>,
    fps: Res<FpsState>,
    windows: Query<&Window>,
    cap: Res<FpsCap>,
    time: Res<Time>,
    mut sysinfo: ResMut<SysInfo>,
) {
    if !debug.enabled {
        return;
    }

    sysinfo.sys.refresh_all();

    let ctx = contexts.ctx_mut().cloned().unwrap();
    // Create a floating, anchored window in the top-left corner
    egui::Window::new("RustCraft Debug")
        .anchor(egui::Align2::LEFT_TOP, egui::vec2(8.0, 40.0))
        .resizable(false)
        .frame(
            egui::Frame::NONE
                .fill(Color32::from_black_alpha(192))
                .into(),
        )
        .interactable(true)
        .show(&ctx, |ui| {
            if let Ok(window) = windows.single() {
                let (w, h) = (
                    window.resolution.width() as u32,
                    window.resolution.height() as u32,
                );
                if fps.latest_fps > 0.0 {
                    let frame_ms = 1000.0 / fps.latest_fps;
                    ui.label(egui::RichText::new(format!("FPS: {:.0}", fps.latest_fps)).strong());
                    ui.label(format!("Frame time: {:.2} ms", frame_ms));
                    ui.label(format!("Resolution: {}x{}", w, h));
                    ui.separator();
                } else {
                    ui.label(format!("Resolution: {}x{}", w, h));
                    ui.label("FPS: calculating...");
                    ui.separator();
                }

                let pid = std::process::id();
                if let Some(process) = sysinfo.sys.process(Pid::from_u32(pid)) {
                    let mem = process.memory();
                    let cpu = process.cpu_usage();
                    let disk = process.disk_usage();
                    let vmemory = process.virtual_memory();
                    let files = process.open_files();
                    let session = process.session_id();
                    let runtime = process.run_time();
                    ui.label(egui::RichText::new(format!("Process Usage")).strong());
                    ui.label(format!("Memory: {:?} MB", mem / 1048576));
                    ui.label(format!("CPU: {:?}%", cpu));
                    ui.label(format!("Virtual Memory: {:?} MB", vmemory / 1048576));
                    ui.label(format!("Open files: {:?}", files.unwrap()));
                    ui.label(format!("SessionID: {:?}", session.unwrap().as_u32()));
                    ui.label(format!("PID: {:?}", pid));
                    ui.label(format!("Runtime: {:?}s", runtime));
                    ui.label(format!(
                        "Disk Read Bytes: new/total => {}/{}",
                        disk.read_bytes, disk.total_read_bytes
                    ));
                    ui.label(format!(
                        "Disk Write Bytes: new/total => {}/{}",
                        disk.written_bytes, disk.total_written_bytes
                    ));
                    ui.separator();
                }

                ui.label(egui::RichText::new(format!("Window Info")).strong());
                ui.label(format!("Present mode: {:?}", window.present_mode));
                ui.label(format!("FPS cap: {:?}", cap.mode));
                ui.label(format!("Delta time: {:.3} s", time.delta_secs()));
                ui.label(format!("Uptime: {:.3}s", time.elapsed_secs_wrapped_f64()));
            } else {
                ui.label("No window found");
            }
        });
}