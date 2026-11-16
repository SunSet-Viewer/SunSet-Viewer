use super::super::cx::{Custom, Cx, field};
use super::super::{COMBO_W, binding_cell};

pub fn page(cx: &mut Cx) {
    use aurora_i18n::tr;
    if let Some((a, slot)) = cx.st.clear.take() {
        cx.s.keybinds.set(a, slot, None);
        cx.changed = true;
    }
    let p = cx.p;
    cx.group(tr!("ui-options-combat-hitboxes"), |cx| {
        cx.note(tr!("ui-options-combat-local-only"));
        cx.toggle(
            tr!("ui-options-combat-show-hitboxes"),
            tr!("ui-options-combat-show-hitboxes-help"),
            &field!(combat.show_hitboxes),
        );
    });
    if crate::settings::COMBAT_AUTO_AIM_ENABLED {
        cx.group(tr!("ui-options-combat-auto-aim-group"), |cx| {
            cx.note(tr!("ui-options-combat-auto-aim-note"));
            cx.toggle(
                tr!("ui-options-combat-auto-head-aim"),
                tr!("ui-options-combat-auto-head-aim-help"),
                &field!(combat.auto_head_aim),
            );
            let aim_modes = [
                (
                    crate::settings::CombatAimMode::HeadAndBody,
                    tr!("ui-options-combat-auto-aim-mode-head-body"),
                    false,
                ),
                (
                    crate::settings::CombatAimMode::HeadOnly,
                    tr!("ui-options-combat-auto-aim-mode-head-only"),
                    false,
                ),
            ];
            cx.choice(
                tr!("ui-options-combat-auto-aim-mode"),
                tr!("ui-options-combat-auto-aim-mode-help"),
                &field!(combat.auto_aim_mode),
                &aim_modes,
            );
            cx.slider(
                tr!("ui-options-combat-hold-interval"),
                tr!("ui-options-combat-hold-interval-help"),
                &field!(combat.auto_aim_hold_interval_ms),
                50..=400,
                " ms",
            );
            cx.slider(
                tr!("ui-options-combat-lead-compensation"),
                tr!("ui-options-combat-lead-compensation-help"),
                &field!(combat.auto_aim_lead_compensation),
                0.0..=2.0,
                "×",
            );
            cx.toggle(
                tr!("ui-options-combat-target-cycle"),
                tr!("ui-options-combat-target-cycle-help"),
                &field!(combat.mouselook_target_cycle),
            );
            cx.slider(
                tr!("ui-options-combat-auto-aim-range"),
                tr!("ui-options-combat-auto-aim-range-help"),
                &field!(combat.auto_aim_range),
                10.0..=200.0,
                " m",
            );
            let toggle_aim = crate::keybinds::Action::ToggleAutoHeadAim;
            let t_toggle = Custom {
                path: "keybinds",
                modified: move |s: &crate::settings::Settings, d: &crate::settings::Settings| {
                    s.keybinds.get(toggle_aim) != d.keybinds.get(toggle_aim)
                },
                reset: move |s: &mut crate::settings::Settings, d: &crate::settings::Settings| {
                    for (slot, binding) in d.keybinds.get(toggle_aim).into_iter().enumerate() {
                        s.keybinds.set(toggle_aim, slot, binding);
                    }
                },
            };
            cx.wide().row_with(
                tr!("ui-options-combat-auto-aim-key"),
                tr!("ui-options-combat-auto-aim-key-help"),
                &t_toggle,
                |ui, s, st, _| {
                    let kb = s.keybinds.clone();
                    for slot in 0..2 {
                        binding_cell(ui, &p, &kb, st, toggle_aim, slot);
                    }
                    false
                },
            );
            let next_target = crate::keybinds::Action::CombatNextTarget;
            let t_cycle = Custom {
                path: "keybinds",
                modified: move |s: &crate::settings::Settings, d: &crate::settings::Settings| {
                    s.keybinds.get(next_target) != d.keybinds.get(next_target)
                },
                reset: move |s: &mut crate::settings::Settings, d: &crate::settings::Settings| {
                    for (slot, binding) in d.keybinds.get(next_target).into_iter().enumerate() {
                        s.keybinds.set(next_target, slot, binding);
                    }
                },
            };
            cx.wide().row_with(
                tr!("ui-options-combat-next-target-key"),
                tr!("ui-options-combat-next-target-key-help"),
                &t_cycle,
                |ui, s, st, _| {
                    let kb = s.keybinds.clone();
                    for slot in 0..2 {
                        binding_cell(ui, &p, &kb, st, next_target, slot);
                    }
                    false
                },
            );
        });
        cx.group(tr!("ui-options-combat-friendly-fire-group"), |cx| {
            cx.note(tr!("ui-options-combat-friendly-fire-note"));
            cx.toggle(
                tr!("ui-options-combat-friendly-fire-enable"),
                tr!("ui-options-combat-friendly-fire-enable-help"),
                &field!(combat.friendly_fire_protection),
            );
            let ff_off = !cx.s.combat.friendly_fire_protection;
            let preset_words: String = cx
                .st
                .contact_set_presets
                .iter()
                .map(|(n, _)| n.as_str())
                .collect::<Vec<_>>()
                .join(" ");
            let no_presets = cx.st.contact_set_presets.is_empty();
            let preset_active = !no_presets
                && cx
                    .s
                    .combat
                    .friendly_fire_preset
                    .as_deref()
                    .is_some_and(|name| cx.st.contact_set_presets.iter().any(|(preset, _)| preset == name));

            cx.greyed(ff_off);
            cx.keywords(&preset_words).wide().row_with(
                tr!("ui-options-combat-friendly-fire-preset"),
                tr!("ui-options-combat-friendly-fire-preset-help"),
                &field!(combat.friendly_fire_preset),
                |ui, s, st, _| {
                    let mut changed = false;
                    let selected_valid = s
                        .combat
                        .friendly_fire_preset
                        .as_deref()
                        .filter(|name| st.contact_set_presets.iter().any(|(preset, _)| preset == name))
                        .map(str::to_owned);
                    let selected_label = s
                        .combat
                        .friendly_fire_preset
                        .as_deref()
                        .unwrap_or_else(|| tr!("ui-options-combat-friendly-fire-preset-none"))
                        .to_owned();
                    ui.vertical(|ui| {
                        egui::ComboBox::from_id_salt(ui.scope_id().with("friendly_fire_preset"))
                            .width(COMBO_W)
                            .selected_text(&selected_label)
                            .show_ui(ui, |ui| {
                                if ui
                                    .selectable_label(
                                        s.combat.friendly_fire_preset.is_none(),
                                        tr!("ui-options-combat-friendly-fire-preset-none"),
                                    )
                                    .clicked()
                                    && s.combat.friendly_fire_preset.is_some()
                                {
                                    s.combat.friendly_fire_preset = None;
                                    changed = true;
                                }
                                for (name, rgba) in &st.contact_set_presets {
                                    let is_selected = selected_valid.as_deref() == Some(name.as_str());
                                    let swatch_color = egui::Color32::from_rgb(rgba[0], rgba[1], rgba[2]);
                                    let clicked = ui
                                        .horizontal(|ui| {
                                            let (rect, _) = ui.allocate_exact_size(egui::vec2(10.0, 10.0), egui::Sense::hover());
                                            ui.painter().circle_filled(rect.center(), 4.5, swatch_color);
                                            ui.selectable_label(is_selected, name).clicked()
                                        })
                                        .inner;
                                    if clicked && s.combat.friendly_fire_preset.as_deref() != Some(name.as_str()) {
                                        s.combat.friendly_fire_preset = Some(name.clone());
                                        changed = true;
                                    }
                                }
                            });
                        if st.contact_set_presets.is_empty() {
                            ui.label(
                                egui::RichText::new(tr!("ui-options-combat-friendly-fire-no-presets"))
                                    .small()
                                    .color(p.muted),
                            );
                        }
                    });
                    changed
                },
            );
            cx.greyed(ff_off || preset_active);
            cx.color(
                tr!("ui-options-combat-friendly-fire-color"),
                tr!("ui-options-combat-friendly-fire-color-help"),
                &field!(combat.friendly_fire_color),
            );
            cx.greyed(false);
        });
    }
    cx.group(tr!("ui-options-combat-aiming"), |cx| {
        cx.note(tr!("ui-options-combat-server-damage"));
        cx.toggle(
            tr!("ui-options-combat-local-detection"),
            tr!("ui-options-combat-local-detection-help"),
            &field!(combat.local_hit_detection),
        );
        cx.toggle(
            tr!("ui-options-combat-scene-blocks"),
            tr!("ui-options-combat-scene-blocks-help"),
            &field!(combat.scene_objects_block_rays),
        );
        cx.toggle(
            tr!("ui-options-combat-show-raycast"),
            tr!("ui-options-combat-show-raycast-help"),
            &field!(combat.show_raycast),
        );
    });
    cx.group(tr!("ui-options-floating-text"), |cx| {
        cx.borrow(
            super::super::Page::Layout,
            1,
            &["floating_text_fade_distance", "floating_text_fade_range"],
        );
    });
}
