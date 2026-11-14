//! Tests unitários de validação e não-regressão para funcionalidades exclusivas do SunSet Viewer.
//!
//! Cobre os 7 eixos prioritários definidos pelo desenvolvedor:
//! 1. Alcance Visual até 8.192m (`dd <metros>`, `Settings::sanitized`, slider e SpeedRez)
//! 2. Controle de Movimento 100x e Dash Instantâneo (`MovementDashSettings`, `movement.rs`)
//! 3. Hitboxes 3D de Combate e Raycast Local (`CombatSettings`, `combat.rs`, página `Page::Combat`)
//! 4. Cores Individuais de Hitbox e Minimapa (`avatar_marks`, precedência sobre Contact Sets)
//! 5. Localização 100% Nativa em Português (`pt-BR.ftl` vs `en.ftl` paridade perfeita de chaves)
//! 6. Identidade Visual e Branding do SunSet Viewer (`LOGO_PNG`, `sunset-logo.png`)
//! 7. Pipelines CI/CD e Scripts de Compilação (`release.yml`, `check.ps1`)

#![allow(clippy::field_reassign_with_default)]

use crate::movement;
use crate::scene::avatar::AvatarLibrary;
use crate::scene::combat::{AvatarHit, CombatRayHit, CombatSceneHit, combat_hit};
use crate::settings::draw_stepping;
use crate::settings::{CombatSettings, MovementDashSettings, Settings};
use crate::ui::minimap::MiniMap;
use crate::ui::options::Page;
use crate::ui::quick_prefs::Slot;
use crate::world::World;
use egui::Color32;
use glam::Vec3;
use std::collections::HashSet;
use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant};
use uuid::Uuid;

// ============================================================================
// 1. Alcance Visual até 8.192m
// ============================================================================

#[test]
fn test_draw_distance_sanitize_clamps_to_8192() {
    let mut s = Settings::default();

    // Valores válidos abaixo e acima do padrão (padrão é 96m)
    s.draw_distance = 10.0;
    let s = s.sanitized();
    assert_eq!(s.draw_distance, 32.0, "Distância mínima deve ser 32m");

    let mut s = Settings::default();
    s.draw_distance = 5000.0;
    let s = s.sanitized();
    assert_eq!(s.draw_distance, 5000.0, "Distância de 5000m (dd 5000+) deve ser preservada");

    let mut s = Settings::default();
    s.draw_distance = 8192.0;
    let s = s.sanitized();
    assert_eq!(s.draw_distance, 8192.0, "Distância máxima de 8192m deve ser preservada");

    let mut s = Settings::default();
    s.draw_distance = 12000.0;
    let s = s.sanitized();
    assert_eq!(s.draw_distance, 8192.0, "Valores além de 8192m devem ser limitados a 8192m");
}

#[test]
fn test_quick_prefs_slider_draw_distance_range_allows_8192() {
    let slot = Slot::DrawDistance;
    let range = slot.range();
    assert_eq!(*range.start(), 32.0, "Início do slider deve ser 32m");
    assert_eq!(*range.end(), 8192.0, "Fim do slider deve atingir 8192m");

    let mut s = Settings::default();
    slot.set(&mut s, 8192.0);
    assert_eq!(s.draw_distance, 8192.0, "Slot::DrawDistance deve aceitar 8192m");
}

#[test]
fn test_speedrez_draw_distance_stepping_supports_8192m() {
    let now = Instant::now();
    let interval = Duration::from_secs(20);
    let (steps, initial_step) = draw_stepping::start(None, 8192.0, interval, now);

    assert_eq!(initial_step, 32.0, "SpeedRez deve iniciar em 32m");
    assert_eq!(steps.user, 8192.0, "Alvo final do SpeedRez deve ser 8192m");

    // Simulando passos de duplicação até 8192m
    let mut current_steps = steps;
    let mut current_val = initial_step;
    let mut iterations = 0;
    while current_val < 8192.0 && iterations < 20 {
        iterations += 1;
        let tick_time = current_steps.next + Duration::from_millis(1);
        match draw_stepping::tick(current_steps, current_val, true, false, interval, tick_time) {
            draw_stepping::Tick::Step(next, next_val) => {
                assert!(next_val > current_val, "Cada passo deve aumentar a distância");
                assert!(next_val <= 8192.0, "Nenhum passo pode ultrapassar 8192m");
                current_steps = next;
                current_val = next_val;
            }
            draw_stepping::Tick::End(final_val) => {
                current_val = final_val;
                break;
            }
            draw_stepping::Tick::Wait => panic!("Não deve esperar quando o tempo já passou"),
        }
    }
    assert_eq!(current_val, 8192.0, "SpeedRez deve alcançar exatamente o teto de 8192m");
}

// ============================================================================
// 2. Controle de Movimento 100x & Dash Instantâneo
// ============================================================================

#[test]
fn test_movement_dash_settings_and_multiplier_up_to_100x() {
    let md = MovementDashSettings::default();
    assert!(!md.enabled, "Dash vem desligado por padrão");
    assert_eq!(md.speed_multiplier, 1.0, "Velocidade padrão é 1.0x");
    assert_eq!(md.dash_distance, 100.0, "Distância padrão de dash é 100.0m");
    assert!(md.mouselook_key_e, "Atalho da tecla E em mouselook ativo");
    assert!(md.mouselook_middle_click, "Atalho do botão do meio do mouse ativo");
}

#[test]
fn test_movement_preferences_sanitize_run_and_fly_100x() {
    let mut pref = movement::Preferences::default();
    pref.run_multiplier = 100.0;
    pref.fly_multiplier = 100.0;
    pref.sanitize();
    assert_eq!(pref.run_multiplier, 100.0, "run_multiplier até 100x suportado");
    assert_eq!(pref.fly_multiplier, 100.0, "fly_multiplier até 100x suportado");

    pref.run_multiplier = 250.0;
    pref.fly_multiplier = 300.0;
    pref.sanitize();
    assert_eq!(pref.run_multiplier, 100.0, "run_multiplier deve limitar a 100x");
    assert_eq!(pref.fly_multiplier, 100.0, "fly_multiplier deve limitar a 100x");
}

// ============================================================================
// 3. Hitboxes 3D de Combate & Raycast Local
// ============================================================================

#[test]
fn test_combat_settings_and_preferences_page_presence() {
    let cs = CombatSettings::default();
    assert!(!cs.show_hitboxes);
    assert!(!cs.scene_objects_block_rays);

    // Validação da contagem de páginas de Preferências incluindo Combat
    assert_eq!(Page::ALL.len(), 23, "Page::ALL deve conter exatamente 23 páginas");
    assert!(Page::ALL.contains(&Page::Combat), "Page::Combat deve estar registrada em Page::ALL");
    assert_eq!(Page::Combat.key(), "combat");
    assert!(!Page::Combat.tabs().is_empty(), "Page::Combat deve possuir abas registradas");
}

#[test]
fn test_combat_hitbox_and_scene_blocking_behavior() {
    // Cenário 1: Obstáculo de cenário mais próximo que o avatar
    let avatar_impact = AvatarHit {
        avatar: Uuid::from_u128(100),
        joint: "mChest".into(),
        distance: 15.0,
        position: Vec3::new(0.0, 0.0, 15.0),
    };
    let scene_impact = CombatSceneHit {
        distance: 8.0,
        point: Vec3::new(0.0, 0.0, 8.0),
        object: None,
        terrain: true,
    };

    let hit = combat_hit(Some(avatar_impact.clone()), Some(scene_impact), Vec3::ZERO, Vec3::Z, 50.0);
    assert!(
        matches!(hit, CombatRayHit::Scene(CombatSceneHit { distance, terrain: true, .. }) if (distance - 8.0).abs() < 0.001),
        "Oclusão de cenário deve bloquear o raycast quando estiver à frente do avatar"
    );

    // Cenário 2: Obstáculo atrás do avatar
    let far_scene = CombatSceneHit {
        distance: 25.0,
        point: Vec3::new(0.0, 0.0, 25.0),
        object: Some(1),
        terrain: false,
    };
    let hit = combat_hit(Some(avatar_impact), Some(far_scene), Vec3::ZERO, Vec3::Z, 50.0);
    assert!(
        matches!(hit, CombatRayHit::Avatar(AvatarHit { avatar, .. }) if avatar == Uuid::from_u128(100)),
        "Avatar deve ser atingido quando o obstáculo de cenário estiver atrás dele"
    );
}

// ============================================================================
// 4. Cores Individuais de Hitbox e Minimapa (avatar_marks)
// ============================================================================

#[test]
fn test_avatar_marks_individual_colors_precedence() {
    let mut world = World::new(Arc::new(AvatarLibrary::load()));
    let map = MiniMap::default();
    let target_id = Uuid::from_u128(0x1234_5678);

    // Cor individual customizada pelo usuário (Marcar cor)
    let custom_color = Color32::from_rgb(255, 60, 20);
    world.avatar_marks.insert(target_id, custom_color);

    let resolved_color = map.avatar_color(&world, &target_id);
    assert_eq!(
        resolved_color, custom_color,
        "avatar_marks deve ter precedência máxima sobre cores de contato"
    );

    // Ao remover o mark individual, volta para a cor padrão ou do conjunto
    world.avatar_marks.remove(&target_id);
    let fallback_color = map.avatar_color(&world, &target_id);
    assert_ne!(
        fallback_color, custom_color,
        "Após remoção, a cor deve reverter para o padrão do minimapa"
    );
}

// ============================================================================
// 5. Localização 100% em Português (pt-BR.ftl)
// ============================================================================

#[test]
fn test_localization_pt_br_has_zero_missing_keys_compared_to_en() {
    let en_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../aurora-i18n/locales/en.ftl");
    let pt_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../aurora-i18n/locales/pt-BR.ftl");

    let en_content = std::fs::read_to_string(&en_path).expect("en.ftl deve existir");
    let pt_content = std::fs::read_to_string(&pt_path).expect("pt-BR.ftl deve existir");

    let extract_keys = |content: &str| -> HashSet<String> {
        let mut keys = HashSet::new();
        for line in content.lines() {
            let trimmed = line.trim_start();
            if trimmed.starts_with('#') || trimmed.is_empty() {
                continue;
            }
            if let Some((k, _)) = trimmed.split_once('=') {
                let key = k.trim();
                if !key.is_empty() && !key.contains(' ') && !key.starts_with('{') && !key.starts_with('[') {
                    keys.insert(key.to_string());
                }
            }
        }
        keys
    };

    let en_keys = extract_keys(&en_content);
    let pt_keys = extract_keys(&pt_content);

    let missing: Vec<_> = en_keys.difference(&pt_keys).cloned().collect();
    assert!(
        missing.is_empty(),
        "pt-BR.ftl deve ter paridade total com en.ftl. Chaves ausentes ({}): {:?}",
        missing.len(),
        missing
    );
}

#[test]
fn test_localization_es_has_zero_missing_keys_compared_to_en() {
    let en_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../aurora-i18n/locales/en.ftl");
    let es_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../aurora-i18n/locales/es.ftl");

    let en_content = std::fs::read_to_string(&en_path).expect("en.ftl deve existir");
    let es_content = std::fs::read_to_string(&es_path).expect("es.ftl deve existir");

    let extract_keys = |content: &str| -> HashSet<String> {
        let mut keys = HashSet::new();
        for line in content.lines() {
            let trimmed = line.trim_start();
            if trimmed.starts_with('#') || trimmed.is_empty() {
                continue;
            }
            if let Some((k, _)) = trimmed.split_once('=') {
                let key = k.trim();
                if !key.is_empty() && !key.contains(' ') && !key.starts_with('{') && !key.starts_with('[') {
                    keys.insert(key.to_string());
                }
            }
        }
        keys
    };

    let en_keys = extract_keys(&en_content);
    let es_keys = extract_keys(&es_content);

    let missing: Vec<_> = en_keys.difference(&es_keys).cloned().collect();
    assert!(
        missing.is_empty(),
        "es.ftl deve ter paridade total com en.ftl. Chaves ausentes ({}): {:?}",
        missing.len(),
        missing
    );
}

// ============================================================================
// 6. Identidade Visual e Branding SunSet
// ============================================================================

#[test]
fn test_sunset_branding_logo_and_assets_validity() {
    let logo_bytes = crate::app::LOGO_PNG;
    assert!(logo_bytes.len() > 1000, "sunset-logo.png deve ter conteúdo válido");

    // Validação da assinatura mágica de arquivo PNG: 0x89 0x50 0x4E 0x47 0x0D 0x0A 0x1A 0x0A
    let png_magic = [0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];
    assert_eq!(&logo_bytes[0..8], &png_magic, "LOGO_PNG deve ser uma imagem PNG válida");

    let branding_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets/branding/sunset-logo.png");
    assert!(branding_path.exists(), "sunset-logo.png deve existir em assets/branding");
}

// ============================================================================
// 7. Pipelines CI/CD e Scripts de Compilação
// ============================================================================

#[test]
fn test_ci_cd_workflows_and_build_scripts_integrity() {
    let release_yml_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../.github/workflows/release.yml");
    let check_ps1_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../scripts/check.ps1");

    let release_yml = std::fs::read_to_string(&release_yml_path).expect("release.yml deve existir");
    assert!(
        release_yml.contains("runs-on: macos-15") || release_yml.contains("runs-on: macos-14"),
        "release.yml deve usar runner macOS ARM64 suportado (macos-15 ou macos-14)"
    );
    assert!(
        release_yml.contains("SunSet-Viewer.pdb") || release_yml.contains("SunSet_Viewer.pdb"),
        "release.yml deve emitir símbolos PDB para o SunSet Viewer"
    );

    let check_ps1 = std::fs::read_to_string(&check_ps1_path).expect("check.ps1 deve existir");
    assert!(
        check_ps1.contains("clippy::nonminimal-bool"),
        "check.ps1 deve conter as flags de tolerância aprovadas para o workspace"
    );
}

// ============================================================================
// 8. Integração da Release Final Aurora v0.9.6 sem Regressão
// ============================================================================

#[test]
fn test_aurora_v096_final_features_integrated_without_regression() {
    // 1. Pacotes de LUT embutidos (Czar Fenris: 20 + Nanner Pudding: 45 = 65 LUTs)
    let packs = crate::snapshot::lut_packs::packs();
    assert_eq!(packs.len(), 2, "Devem existir 2 pacotes de LUT embutidos");
    let total_luts: usize = packs.iter().map(|p| p.tables.len()).sum();
    assert_eq!(total_luts, 65, "Devem existir 65 LUTs embutidos no total");

    // 2. Tipos de renderização (Advanced › Rendering types)
    let mut hidden = crate::scene::render_types::Hidden::default();
    assert_eq!(hidden, Default::default(), "Nenhum tipo de renderização vem oculto por padrão");
    hidden.trees = true;
    assert!(hidden.trees, "Deve permitir alternar visibilidade de árvores");
    assert_eq!(crate::ui::bars::RENDER_TYPE_ACTIONS.len(), 10);
    assert_eq!(crate::ui::bars::RENDER_TYPE_ICONS.len(), 12);

    // 3. Preservação de Settings::sanitized() com outfit_portrait e draw_distance 8192m
    let mut s = Settings::default();
    s.draw_distance = 8192.0;
    s.outfit_portrait.hd = true;
    let s = s.sanitized();
    assert_eq!(
        s.draw_distance, 8192.0,
        "draw_distance de 8192m preservado junto com outfit_portrait"
    );
    assert!(s.outfit_portrait.hd, "Configuração outfit_portrait.hd preservada");
}

// ============================================================================
// 9. Skins, Firestorm Presets, SunSet Twilight and Typed Text
// ============================================================================

#[test]
fn test_skin_defaults_and_presets_contract() {
    use crate::skin_presets::{DEFAULT_SCHEME, DEFAULT_SKIN, families, find_scheme};
    use crate::theme::Theme;

    let defaults = Settings::default();
    assert_eq!(defaults.skin, DEFAULT_SKIN);
    assert_eq!(defaults.skin_theme, DEFAULT_SCHEME);

    let (aurora_fam, aurora_scheme) = find_scheme(&defaults.skin, &defaults.skin_theme);
    assert_eq!(aurora_fam.id, "aurora");
    assert_eq!(aurora_scheme.id, "default");
    assert_eq!(
        serde_json::to_value((aurora_scheme.build_theme)()).unwrap(),
        serde_json::to_value(Theme::default()).unwrap(),
        "Aurora default skin must stay byte-for-byte identical to Theme::default()"
    );

    let expected_families = [
        "aurora",
        "sunset",
        "firestorm",
        "ansastorm",
        "ansastorm_modern",
        "metaharper_modern",
        "starlight",
        "starlight_cui",
        "vintage",
    ];
    let present: Vec<&str> = families().iter().map(|f| f.id).collect();
    for id in expected_families {
        assert!(present.contains(&id), "Expected skin family '{id}' to exist");
    }

    for scheme_id in ["sunset_default", "rose", "rose_pastel", "violet", "purple", "lilac", "lilac_light"] {
        let (fam, scheme) = find_scheme("sunset", scheme_id);
        assert_eq!(fam.id, "sunset");
        assert_eq!(scheme.id, scheme_id);
    }

    let (_, rose_pastel) = find_scheme("sunset", "rose_pastel");
    assert!((rose_pastel.build_theme)().skin_palette().is_light());
    let (_, lilac_light) = find_scheme("sunset", "lilac_light");
    assert!((lilac_light.build_theme)().skin_palette().is_light());

    let mut dirty = Settings {
        skin: "SunSet".to_owned(),
        skin_theme: "twilight".to_owned(),
        ..Settings::default()
    };
    dirty = dirty.sanitized();
    assert_eq!((dirty.skin.as_str(), dirty.skin_theme.as_str()), ("sunset", "sunset_default"));

    let mut invalid = Settings {
        skin: "unknown-skin".to_owned(),
        skin_theme: "unknown-scheme".to_owned(),
        ..Settings::default()
    };
    invalid = invalid.sanitized();
    assert_eq!((invalid.skin.as_str(), invalid.skin_theme.as_str()), ("aurora", "default"));
}

#[test]
fn test_skin_i18n_keys_present_across_all_locales() {
    let locales_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../crates/aurora-i18n/locales");
    let required_keys = [
        "appearance-skin-preset-group",
        "appearance-skin-family",
        "appearance-skin-family-hint",
        "appearance-skin-scheme",
        "appearance-skin-scheme-hint",
        "appearance-skin-applied-live",
        "appearance-skin-reset-aurora",
        "appearance-skin-preview",
        "appearance-skin-preview-button",
        "appearance-skin-preview-friend",
        "appearance-skin-preview-typed",
        "skin-family-aurora",
        "skin-family-sunset",
        "skin-scheme-aurora-default",
        "skin-scheme-sunset-default",
        "skin-scheme-sunset-rose",
        "skin-scheme-sunset-rose-pastel",
        "skin-scheme-sunset-violet",
        "skin-scheme-sunset-purple",
        "skin-scheme-sunset-lilac",
        "skin-scheme-sunset-lilac-light",
    ];
    for file in ["en.ftl", "pt-BR.ftl", "es.ftl", "fr.ftl", "ja.ftl"] {
        let content = std::fs::read_to_string(locales_dir.join(file)).expect("locale file exists");
        for key in required_keys {
            let prefix = format!("{key} =");
            assert!(content.lines().any(|l| l.starts_with(&prefix)), "{file} missing skin key '{key}'");
        }
    }
}

#[test]
fn test_floating_text_fade_settings_sanitized_and_extended_combat_range() {
    use crate::settings::{
        DEFAULT_FLOATING_TEXT_FADE_DISTANCE, DEFAULT_FLOATING_TEXT_FADE_RANGE, MAX_FLOATING_TEXT_FADE_DISTANCE,
        MAX_FLOATING_TEXT_FADE_RANGE, MIN_FLOATING_TEXT_FADE_RANGE,
    };

    let defaults = Settings::default();
    assert_eq!(defaults.floating_text_fade_distance, DEFAULT_FLOATING_TEXT_FADE_DISTANCE);
    assert_eq!(defaults.floating_text_fade_range, DEFAULT_FLOATING_TEXT_FADE_RANGE);

    let clamped = Settings {
        floating_text_fade_distance: 200.0,
        floating_text_fade_range: 100.0,
        ..Settings::default()
    }
    .sanitized();
    assert_eq!(clamped.floating_text_fade_distance, MAX_FLOATING_TEXT_FADE_DISTANCE);
    assert_eq!(clamped.floating_text_fade_range, MAX_FLOATING_TEXT_FADE_RANGE);

    let min_clamped = Settings {
        floating_text_fade_distance: -5.0,
        floating_text_fade_range: 0.0,
        ..Settings::default()
    }
    .sanitized();
    assert_eq!(min_clamped.floating_text_fade_distance, 0.0);
    assert_eq!(min_clamped.floating_text_fade_range, MIN_FLOATING_TEXT_FADE_RANGE);

    let nan_reset = Settings {
        floating_text_fade_distance: f32::NAN,
        floating_text_fade_range: f32::INFINITY,
        ..Settings::default()
    }
    .sanitized();
    assert_eq!(nan_reset.floating_text_fade_distance, DEFAULT_FLOATING_TEXT_FADE_DISTANCE);
    assert_eq!(nan_reset.floating_text_fade_range, DEFAULT_FLOATING_TEXT_FADE_RANGE);

    assert_eq!(crate::ui::hud::floating_text_fade(60.0, &clamped), Some(1.0));
    assert!(crate::ui::hud::floating_text_fade(95.0, &clamped).is_some());
    assert_eq!(crate::ui::hud::floating_text_fade(96.0, &clamped), None);
}
