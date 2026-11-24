# Upstream Aurora Sync Policy

This document is the binding engineering contract for synchronizing releases or
pull requests from upstream `Aurora-Viewer/Aurora-Viewer` into **SunSet
Viewer**.

## 1. Golden Rule: Precedence of SunSet-Exclusive Implementations

When merging or cherry-picking from upstream Aurora:

1. **Always preserve SunSet-exclusive features** — never let an upstream commit
   overwrite, weaken, or hide a SunSet feature.
2. **Always reject Aurora's avatar hitbox implementation** (`Aurora-Viewer#4050`:
   *feat(avatar): avatar hitbox debug overlay*, Dev Toolbox tool `O10`,
   `DebugView::avatar_hitboxes`, and any follow-up commits touching it).
3. **Always keep SunSet's Combat Hitbox & Marker stack** (`Page::Combat` with
   the crossed-swords icon, `CombatSettings`, `scene/combat.rs`,
   `ui/debug_overlay.rs` combat rendering, `world/avatar_marks.rs`, and
   `ui/context.rs::avatar_marker_submenu`).
4. **Exception clause:** SunSet's hitbox or any SunSet-exclusive feature may be
   replaced by an upstream Aurora equivalent **only after explicit confirmation
   from the project owner (Adriano Severino)**. Without explicit owner approval,
   SunSet's implementation always wins.

---

## 2. Permanent Upstream Exclusions (Do Not Merge / Must Strip on Every Sync)

Check and remove these items whenever merging an Aurora release tag:

| Item | Upstream Origin | Required Action in SunSet |
| --- | --- | --- |
| Dev Toolbox `O10` / `DebugView::avatar_hitboxes` / `dev-toolbox-o10` | `Aurora-Viewer#4050` (and follow-ups e.g. `#4470` hitbox touch) | Strip from `settings.rs`, `dev_toolbox/tools.rs`, `app/dev_toolbox.rs`, `app.rs`, `ui/debug_overlay.rs`, `ui/minimap.rs` (free `avatar_color`), `docs/DEV_TOOLBOX.md`, `TASKS.md`, and all `crates/aurora-i18n/locales/*.ftl`. |
| Upstream signed-updater enforcement (`.sig` Ed25519 / `RELEASE_KEYS`) | `Aurora-Viewer#4632` | Keep SunSet's SHA-256 + HTTPS updater targeting `SunSet-Viewer/SunSet-Viewer` in `crates/aurora-updater/` and `release.yml`; drop `signature.rs` and `scripts/setup-release-signing.ps1` / `scripts/sign-release.ps1` / `.github/actions/release-admin/` unless the owner specifically requests SunSet signing keys. |
| Nightly Aurora workflows & `aurora-tools.cmd` | Aurora CI/maintenance scripts | Keep removed in SunSet (do not re-introduce from upstream merges). |
| Aurora News Entries in `news/*.md` | `news/en.md`, `news/fr.md`, `news/ja.md` | Keep SunSet's release news and links to `SunSet-Viewer/SunSet-Viewer`. |

---

## 3. SunSet-Exclusive Features Ownership Matrix

| Feature Area | Key Files & Symbols | Mechanical Guards (`sunset_tests.rs` & other tests) |
| --- | --- | --- |
| **Combat Tab & 3D Hitboxes (up to Draw Distance / 128 m+)** | `crates/aurora-viewer/src/ui/options.rs` (`Page::Combat`), `crates/aurora-viewer/src/ui/options/pages/combat.rs`, `crates/aurora-viewer/src/scene/combat.rs`, `crates/aurora-viewer/src/ui/debug_overlay.rs` | `test_combat_settings_and_preferences_page_presence`, `test_combat_hitbox_and_scene_blocking_behavior`, `test_aurora_upstream_hitbox_o10_is_excluded` |
| **Individual Avatar Color Markers (`Marcar cor`)** | `crates/aurora-viewer/src/world/avatar_marks.rs`, `crates/aurora-viewer/src/settings.rs` (`avatar_marks`), `crates/aurora-viewer/src/ui/context.rs` (`avatar_marker_submenu`, `SetAvatarMark`), `crates/aurora-viewer/src/ui/colors.rs` (`resident_name`), `crates/aurora-viewer/src/ui/minimap.rs` | `test_avatar_marks_individual_colors_precedence`, `test_avatar_marker_color_context_menu_and_precedence_preserved` |
| **Floating-Text Distance & Fade (`llSetText`)** | `crates/aurora-viewer/src/settings.rs` (`MAX_FLOATING_TEXT_FADE_DISTANCE = 128.0`, `MAX_FLOATING_TEXT_FADE_RANGE = 64.0`), `crates/aurora-viewer/src/ui/options/pages/layout.rs`, `crates/aurora-viewer/src/ui/hud.rs` | `test_floating_text_fade_settings_sanitized_and_extended_combat_range`, `hud::floating_text_fade_uses_settings_distances` |
| **Draw Distance 8192 m, SpeedRez & `/dd`** | `crates/aurora-viewer/src/settings.rs` (`DRAW_DISTANCE_MAX = 8192.0`, `draw_stepping`) | `test_draw_distance_sanitize_clamps_to_8192`, `test_quick_prefs_slider_draw_distance_range_allows_8192`, `test_speedrez_draw_distance_stepping_supports_8192m` |
| **Movement 100× & Native Dash** | `crates/aurora-viewer/src/movement/strokes.rs` (`dash_animation`), `crates/aurora-viewer/src/settings.rs` (`MovementDashSettings`), `assets/animations/sunset-dash.bvh` | `test_movement_dash_settings_and_multiplier_up_to_100x`, `test_movement_preferences_sanitize_run_and_fly_100x`, `test_sunset_dash_animation_loads_and_loops_at_priority_4` |
| **SunSet Skins & Live Theme Picker** | `crates/aurora-viewer/src/skin_presets.rs`, `crates/aurora-viewer/src/ui/options/pages/appearance.rs` | `test_skin_defaults_and_presets_contract`, `test_skin_i18n_keys_present_across_all_locales`, `test_disable_glass_preserves_dark_and_light_sunset_palettes` |
| **AppData Isolation & Coexistence** | `crates/aurora-viewer/src/app/paths.rs`, `SunSet\SunSetViewer`, `SunSetViewerCache`, `sunset-<pid>.log`, `SunSet.Viewer` AUMID | `test_sunset_user_data_dirs_isolated_from_official_aurora`, `test_migrate_legacy_aurora_profile_and_media_plugins_non_destructive`, `test_sunset_updater_feed_points_to_sunset_repo` |
| **15-Language Parity (`en`, `pt-BR`, `es`, `fr`, `ja`)** | `crates/aurora-i18n/locales/*.ftl` | `test_localization_pt_br_has_zero_missing_keys_compared_to_en`, `test_localization_es_has_zero_missing_keys_compared_to_en`, `test_all_fifteen_languages_bundled_with_native_portuguese_and_spanish` |
| **SunSet Updater & CI Pipeline** | `crates/aurora-updater/src/{feed,lib,package}.rs`, `.github/workflows/release.yml`, `scripts/{check,build-release,common,new-task}.ps1` | `test_sunset_updater_feed_points_to_sunset_repo`, `./scripts/check.ps1` |

---

## 4. Repeatable Upstream Sync Procedure

Follow these steps whenever syncing a new upstream Aurora release tag (e.g. `vX.Y.Z`):

1. **Create an isolated worktree** (never work directly on `main`):
   `./scripts/new-task.ps1 sync/aurora-vX.Y.Z`
2. **Fetch upstream into a uniquely named tag** so it never collides with
   SunSet's own `vX.Y.Z` release tag:
   `git fetch --no-tags upstream refs/tags/vX.Y.Z:refs/tags/upstream-vX.Y.Z`
3. **Inspect excluded PRs before merging**:
   Examine any upstream commits touching `avatar_hitboxes` / `O10`, updater
   signing (`signature.rs`), or SunSet-modified modules (`combat`,
   `avatar_marks`, `skin_presets`, `paths`, `floating_text_fade`, `draw_distance`,
   `movement`).
4. **Merge `--no-commit --no-ff upstream-vX.Y.Z`** and resolve every conflict in
   favor of preserving upstream bugfixes/features while keeping all SunSet
   features intact and stripping excluded upstream changes (`O10`, Aurora
   hitbox, Aurora news, upstream-only updater signing).
5. **Reconcile `crates/aurora-i18n/locales/*.ftl`** from the key diff between the
   previous `upstream-*` tag and the new `upstream-vX.Y.Z` tag: keep all SunSet
   custom keys and branding, translate all added/changed keys into `pt-BR.ftl`
   and `es.ftl`, and verify exact key parity across `en`, `pt-BR`, `es`, `fr`,
   `ja` with zero excluded keys (`dev-toolbox-o10`).
6. **Check `Cargo.lock` & `THIRD-PARTY-LICENSES.md`**: if dependencies changed,
   update `THIRD-PARTY-LICENSES.md` with `./scripts/third-party-licenses.ps1`.
7. **Run full workspace validation**: `./scripts/check.ps1` (`cargo fmt`,
   `cargo clippy --workspace --all-targets --locked -- -D warnings -A clippy::nonminimal-bool`,
   and `cargo test --workspace --locked` — never partial `-p` runs).
8. **Update release notes**: `CHANGELOG.md` and all `news/*.md` (`pt-BR`, `pt`,
   `es`, `en`, `fr`, `ja`).
9. **Validate in demo mode (`AURORA_DEMO=1`)** and get explicit human
   verification before merging to `main` or tagging a release.
