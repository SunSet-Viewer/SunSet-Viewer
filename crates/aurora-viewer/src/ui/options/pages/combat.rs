use super::super::cx::{Cx, field};

pub fn page(cx: &mut Cx) {
    use aurora_i18n::tr;
    cx.group(tr!("ui-options-combat-hitboxes"), |cx| {
        cx.note(tr!("ui-options-combat-local-only"));
        cx.toggle(
            tr!("ui-options-combat-show-hitboxes"),
            tr!("ui-options-combat-show-hitboxes-help"),
            &field!(combat.show_hitboxes),
        );
    });
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
