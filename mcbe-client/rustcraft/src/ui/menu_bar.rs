use bevy::{prelude::*, window::PresentMode};
use bevy_egui::egui;
use bevy_egui_kbgp::KbgpEguiResponseExt;
use egui::containers::menu::MenuConfig;

use crate::{
    data::{FpsCap, FpsMode, GlobalFlags, GlobalSettings, RecentFile},
    ui::FileDialogChannel,
};

pub fn menu_bar_ui(
    ui: &mut egui::Ui,
    global_settings: &mut GlobalSettings,
    file_dialog: &FileDialogChannel,
    fps_cap: &mut FpsCap,
    window: &mut Window,
) {
    egui::MenuBar::new()
        .config(MenuConfig::new().close_behavior(egui::PopupCloseBehavior::CloseOnClick))
        .ui(ui, |ui| {
            ui.menu_button("App", |ui| {
                if ui.button("Quit").kbgp_initial_focus().clicked() {
                    std::process::exit(0);
                }
                ui.menu_button("Settings", |ui| {
                    if ui
                        .button("Toggle Debug Mode")
                        .kbgp_initial_focus()
                        .clicked()
                    {
                        global_settings.flags.toggle(GlobalFlags::DEBUG_OVERLAY);
                    }
                    ui.add(egui::Slider::new(
                        &mut global_settings.game_settings.render_distance,
                        4..=96,
                    ))
                    .kbgp_initial_focus()
                    .labelled_by(ui.label("Render Distance").id);
                    ui.add(egui::Slider::new(
                        &mut global_settings.game_settings.fps_cap,
                        0..=255,
                    ))
                    .kbgp_initial_focus()
                    .labelled_by(ui.label("FPS Cap").id);
                    fps_cap.mode =
                        FpsMode::Manual(global_settings.game_settings.fps_cap as u32 * 10);
                    fps_cap_ui(global_settings, ui, window);
                });
            });

            ui.menu_button("File", |ui| {
                // OPEN FILE — threaded dialog, main thread receives via channel
                if ui.button("Open").kbgp_initial_focus().clicked() {
                    let tx = file_dialog.sender.clone();

                    std::thread::spawn(move || {
                        let result = rfd::FileDialog::new().pick_file();
                        tx.send(result).ok();
                    });
                }

                ui.menu_button("Open Recent", |ui| {
                    let recent_files_opened = global_settings.recent_files_opened.clone();
                    for file in recent_files_opened {
                        let label = if let Some(ext) = &file.extension {
                            format!("{} ({})", file.name, ext)
                        } else {
                            file.name.clone()
                        };

                        if ui
                            .add(egui::Button::new(label).wrap_mode(egui::TextWrapMode::Extend))
                            .kbgp_initial_focus()
                            .clicked()
                        {
                            #[cfg(debug_assertions)]
                            todo!("Handle file opening not implemented!");

                            #[cfg(not(debug_assertions))]
                            info!("Handle file opening not implemented!");
                        }
                    }
                });
            });
        });
}

fn fps_cap_ui(global_settings: &mut GlobalSettings, ui: &mut egui::Ui, window: &mut Window) {
    ui.menu_button("Present Mode", |ui| {
        if ui
            .button(format!("{:?}", PresentMode::AutoVsync))
            .kbgp_initial_focus()
            .clicked()
        {
            global_settings.game_settings.present_mode = PresentMode::AutoVsync;
            window.present_mode = PresentMode::AutoVsync;
        }
        if ui
            .button(format!("{:?}", PresentMode::AutoNoVsync))
            .kbgp_initial_focus()
            .clicked()
        {
            global_settings.game_settings.present_mode = PresentMode::AutoNoVsync;
            window.present_mode = PresentMode::AutoNoVsync;
        }
        if ui
            .button(format!("{:?}", PresentMode::Fifo))
            .kbgp_initial_focus()
            .clicked()
        {
            global_settings.game_settings.present_mode = PresentMode::Fifo;
            window.present_mode = PresentMode::Fifo;
        }
        if ui
            .button(format!("{:?}", PresentMode::FifoRelaxed))
            .kbgp_initial_focus()
            .clicked()
        {
            global_settings.game_settings.present_mode = PresentMode::FifoRelaxed;
            window.present_mode = PresentMode::FifoRelaxed;
        }
        if ui
            .button(format!("{:?}", PresentMode::Immediate))
            .kbgp_initial_focus()
            .clicked()
        {
            global_settings.game_settings.present_mode = PresentMode::Immediate;
            window.present_mode = PresentMode::Immediate;
        }
        if ui
            .button(format!("{:?}", PresentMode::Mailbox))
            .kbgp_initial_focus()
            .clicked()
        {
            global_settings.game_settings.present_mode = PresentMode::Mailbox;
            window.present_mode = PresentMode::Mailbox;
        }
    });
}

pub fn file_dialog_system(
    mut global_settings: ResMut<GlobalSettings>,
    file_dialog: Res<FileDialogChannel>,
) {
    if let Ok(Some(path)) = file_dialog.receiver.try_recv() {
        let file = RecentFile {
            name: path.file_name().unwrap().to_string_lossy().to_string(),
            extension: path.extension().map(|e| e.to_string_lossy().to_string()),
            path,
        };

        add_recent_file(&mut global_settings, file);
    }
}

fn add_recent_file(settings: &mut GlobalSettings, file: RecentFile) {
    // Remove any existing entry with the same path
    settings.recent_files_opened.retain(|f| f.path != file.path);

    // Insert at the front (most recent first)
    settings.recent_files_opened.insert(0, file);
}
