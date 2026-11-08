//! Movement speed multiplier & Dash controls floater.

use super::widgets::Floater;
use crate::settings::MovementDashSettings;
use crate::theme::Palette;
use egui::{RichText, Vec2};

pub const WINDOW: &str = "movement_dash";

pub enum Action {
    PerformDash { distance: f32 },
}

pub fn show(
    ctx: &egui::Context,
    p: &Palette,
    settings: &mut MovementDashSettings,
    open: &mut bool,
) -> Option<Action> {
    if !*open {
        return None;
    }
    let mut action = None;
    let mut floater = Floater::new(
        WINDOW,
        aurora_i18n::tr!("movement-dash-title"),
        egui::pos2(120.0, 180.0),
        Vec2::new(320.0, 360.0),
    );
    floater.min_size = Vec2::new(280.0, 320.0);
    floater.show(ctx, p, open, |ui| {
        // Toggle Mestre (Flag ON/OFF)
        ui.horizontal(|ui| {
            let label = if settings.enabled {
                aurora_i18n::tr!("movement-dash-enabled")
            } else {
                aurora_i18n::tr!("movement-dash-disabled")
            };
            ui.checkbox(&mut settings.enabled, RichText::new(label).strong());
        });
        ui.separator();

        // Seção: Velocidade do Avatar
        ui.label(RichText::new(aurora_i18n::tr!("movement-dash-speed-section")).strong());
        ui.horizontal(|ui| {
            for &s in &[1.0, 2.0, 5.0, 10.0, 20.0] {
                let is_selected = (settings.speed_multiplier - s).abs() < 0.05;
                if ui.selectable_label(is_selected, format!("{s:.0}x")).clicked() {
                    settings.speed_multiplier = s;
                }
            }
        });
        ui.horizontal(|ui| {
            ui.label(aurora_i18n::tr!("movement-dash-speed-custom"));
            ui.add(
                egui::DragValue::new(&mut settings.speed_multiplier)
                    .range(0.1..=100.0)
                    .speed(0.1)
                    .suffix("x"),
            );
        });

        ui.add_space(6.0);
        ui.separator();

        // Seção: Salto / Dash
        ui.label(RichText::new(aurora_i18n::tr!("movement-dash-distance-section")).strong());
        ui.horizontal(|ui| {
            for &d in &[25.0, 50.0, 100.0, 200.0, 500.0] {
                let is_selected = (settings.dash_distance - d).abs() < 0.5;
                if ui.selectable_label(is_selected, format!("{d:.0}m")).clicked() {
                    settings.dash_distance = d;
                }
            }
        });
        ui.horizontal(|ui| {
            ui.label(aurora_i18n::tr!("movement-dash-distance-custom"));
            ui.add(
                egui::DragValue::new(&mut settings.dash_distance)
                    .range(1.0..=8192.0)
                    .speed(1.0)
                    .suffix(" m"),
            );
        });

        ui.add_space(4.0);
        if ui.button(RichText::new(aurora_i18n::tr!("movement-dash-trigger-btn")).strong()).clicked() {
            action = Some(Action::PerformDash { distance: settings.dash_distance });
        }

        ui.add_space(6.0);
        ui.separator();

        // Seção: Atalhos em Mouselook
        ui.label(RichText::new(aurora_i18n::tr!("movement-dash-mouselook-section")).strong());
        ui.checkbox(&mut settings.mouselook_key_e, aurora_i18n::tr!("movement-dash-mouselook-key-e"));
        ui.checkbox(&mut settings.mouselook_middle_click, aurora_i18n::tr!("movement-dash-mouselook-middle-click"));
    });
    action
}

