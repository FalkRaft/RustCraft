use bevy::prelude::*;
use bevy::time::Time;
use bevy::window::Window;
use bevy_egui::{EguiContexts, egui};
use egui::Color32;
use sysinfo::Pid;

use crate::data::{DebugFlags, FpsCap, FpsMode, FpsState, GlobalFlags, GlobalSettings, SysInfo};

/// Render a small egui overlay when debug is enabled.
pub fn egui_debug_system(
    mut contexts: EguiContexts,
    fps: Res<FpsState>,
    mut windows: Query<&mut Window>,
    mut cap: ResMut<FpsCap>,
    time: Res<Time>,
    mut sysinfo: ResMut<SysInfo>,
    mut global_settings: ResMut<GlobalSettings>,
) {
    if !global_settings.flags.contains(GlobalFlags::DEBUG_OVERLAY) {
        return;
    }

    sysinfo.sys.refresh_all();

    for ctx in &mut contexts.ctx_mut().into_iter() {
        // Create a floating, anchored window in the top-left corner
        egui::Window::new("RustCraft Debug")
            .anchor(egui::Align2::LEFT_TOP, egui::vec2(0.0, 22.0))
            .resizable(false)
            .frame(
                egui::Frame::NONE
                    .fill(Color32::from_black_alpha(192))
                    .into(),
            )
            .interactable(true)
            .show(&ctx, |ui| {
                for mut window in &mut windows.iter_mut() {
                    let (w, h) = (
                        window.resolution.width() as u32,
                        window.resolution.height() as u32,
                    );
                    if fps.latest_fps > 0.0 {
                        let frame_ms = 1000.0 / fps.latest_fps;
                        ui.label(
                            egui::RichText::new(format!("FPS: {:.0}", fps.latest_fps)).strong(),
                        );
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
                        if global_settings.dbg_flags.contains(DebugFlags::CPU) {
                            ui.label(format!("CPU: {:?}%", cpu));
                        }
                        if global_settings.dbg_flags.contains(DebugFlags::MEM) {
                            ui.label(format!("Memory: {:?} MB", mem / 1048576));
                        }
                        if global_settings.dbg_flags.contains(DebugFlags::VMEM) {
                            ui.label(format!("Virtual Memory: {:?} MB", vmemory / 1048576));
                        }
                        if global_settings.dbg_flags.contains(DebugFlags::FILES) {
                            ui.label(format!("Open files: {:?}", files.unwrap()));
                        }
                        if global_settings.dbg_flags.contains(DebugFlags::RUNTIME) {
                            ui.label(format!("Runtime: {:?}s", runtime));
                        }
                        if global_settings.dbg_flags.contains(DebugFlags::DISK) {
                            ui.label(format!(
                                "Disk Read Bytes: new/total => {}/{}",
                                disk.read_bytes, disk.total_read_bytes
                            ));
                            ui.label(format!(
                                "Disk Write Bytes: new/total => {}/{}",
                                disk.written_bytes, disk.total_written_bytes
                            ));
                        }
                        ui.label(format!("SessionID: {:?}", session.unwrap().as_u32()));
                        ui.label(format!("PID: {:?}", pid));
                        ui.separator();
                    }

                    ui.label(egui::RichText::new(format!("Window Info")).strong());
                    ui.label(format!("Present mode: {:?}", window.present_mode));
                    ui.label(format!("FPS cap: {:?}", cap.mode));
                    ui.label(format!("Delta time: {:.3} s", time.delta_secs()));
                    ui.label(format!("Uptime: {:.3}s", time.elapsed_secs_wrapped_f64()));
                    ui.separator();
                    ui.label(egui::RichText::new(format!("Settings")).strong());
                    // For each flag:
                    // 1. Copy the current value into a local bool
                    // 2. Pass that bool to the checkbox
                    // 3. If changed, write the updated value back into the bitflags

                    // VSync
                    let mut vsync = global_settings.dbg_flags.contains(DebugFlags::VSYNC);
                    if ui.checkbox(&mut vsync, "VSync").changed() {
                        global_settings.dbg_flags.set(DebugFlags::VSYNC, vsync);
                    }

                    if vsync {
                        cap.mode = FpsMode::VSync;
                        window.present_mode = bevy::window::PresentMode::AutoVsync;
                    }

                    // FPS
                    let mut fps = global_settings.dbg_flags.contains(DebugFlags::FPS);
                    if ui.checkbox(&mut fps, "FPS").changed() {
                        global_settings.dbg_flags.set(DebugFlags::FPS, fps);
                    }

                    // CPU
                    let mut cpu = global_settings.dbg_flags.contains(DebugFlags::CPU);
                    if ui.checkbox(&mut cpu, "CPU").changed() {
                        global_settings.dbg_flags.set(DebugFlags::CPU, cpu);
                    }

                    // Memory
                    let mut mem = global_settings.dbg_flags.contains(DebugFlags::MEM);
                    if ui.checkbox(&mut mem, "Memory").changed() {
                        global_settings.dbg_flags.set(DebugFlags::MEM, mem);
                    }

                    // Virtual Memory
                    let mut vmem = global_settings.dbg_flags.contains(DebugFlags::VMEM);
                    if ui.checkbox(&mut vmem, "Virtual Memory").changed() {
                        global_settings.dbg_flags.set(DebugFlags::VMEM, vmem);
                    }

                    // Disk I/O
                    let mut disk = global_settings.dbg_flags.contains(DebugFlags::DISK);
                    if ui.checkbox(&mut disk, "Disk I/O").changed() {
                        global_settings.dbg_flags.set(DebugFlags::DISK, disk);
                    }

                    // Files Open
                    let mut files = global_settings.dbg_flags.contains(DebugFlags::FILES);
                    if ui.checkbox(&mut files, "Files Open").changed() {
                        global_settings.dbg_flags.set(DebugFlags::FILES, files);
                    }

                    // Runtime
                    let mut runtime = global_settings.dbg_flags.contains(DebugFlags::RUNTIME);
                    if ui.checkbox(&mut runtime, "Runtime").changed() {
                        global_settings.dbg_flags.set(DebugFlags::RUNTIME, runtime);
                    }
                }
            });
    }
}
